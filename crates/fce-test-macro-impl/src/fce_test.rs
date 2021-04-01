/*
 * Copyright 2020 Fluence Labs Limited
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

use crate::attributes::FCETestAttributes;
use crate::{TResult, TestGeneratorError};

use fluence_app_service::TomlAppServiceConfig;
use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use quote::ToTokens;

pub fn fce_test_impl(attrs: TokenStream2, input: TokenStream2) -> TResult<TokenStream2> {
    use darling::FromMeta;

    // from https://github.com/dtolnay/syn/issues/788
    let parser = syn::punctuated::Punctuated::<syn::NestedMeta, syn::Token![,]>::parse_terminated;
    let attrs = parser.parse2(attrs)?;
    let attrs: Vec<syn::NestedMeta> = attrs.into_iter().collect();
    let attrs = FCETestAttributes::from_list(&attrs)?;

    let func_item = syn::parse2::<syn::ItemFn>(input)?;

    generate_test_glue_code(func_item, attrs)
}

fn generate_test_glue_code(
    func_item: syn::ItemFn,
    attrs: FCETestAttributes,
) -> TResult<TokenStream2> {
    let fce_config = TomlAppServiceConfig::load(&attrs.config_path)?;
    let modules_dir = match determine_modules_dir(&fce_config, attrs.modules_dir) {
        Some(modules_dir) => modules_dir,
        None => return Err(TestGeneratorError::ModulesDirUnspecified),
    };

    let fce_ctor = generate_fce_ctor(&attrs.config_path, &modules_dir);
    let module_interfaces = collect_module_interfaces(&fce_config, modules_dir)?;

    let module_definitions = generate_module_definitions(module_interfaces.iter())?;
    let module_iter = module_interfaces
        .iter()
        .map(|(module_name, _)| *module_name);
    let module_ctors = generate_module_ctors(module_iter)?;
    let original_block = func_item.block;
    let signature = func_item.sig;

    let glue_code = quote! {
        #[test]
        #signature {
            #module_definitions

            #fce_ctor

            #module_ctors

            #original_block
        }
    };

    Ok(glue_code)
}

fn generate_fce_ctor(config_path: &str, modules_dir: &PathBuf) -> TokenStream2 {
    let config_path = config_path.to_token_stream();
    let modules_dir = modules_dir.to_string_lossy().to_string();

    quote! {
        let tmp_dir = std::env::temp_dir();
        let service_id = fluence_test::internal::Uuid::new_v4().to_string();

        let tmp_dir = tmp_dir.join(&service_id);
        let tmp_dir = tmp_dir.to_string_lossy().to_string();
        std::fs::create_dir(&tmp_dir).expect("can't create a directory for service in tmp");

        let mut __fce_generated_fce_config = fluence_test::internal::TomlAppServiceConfig::load(#config_path.to_string())
            .unwrap_or_else(|e| panic!("app service located at `{}` config can't be loaded: {}", #config_path, e));
        __fce_generated_fce_config.service_base_dir = Some(tmp_dir);
        __fce_generated_fce_config.toml_faas_config.modules_dir = Some(#modules_dir.to_string());

        let fce = fluence_test::internal::AppService::new_with_empty_facade(__fce_generated_fce_config, service_id, std::collections::HashMap::new())
            .unwrap_or_else(|e| panic!("app service can't be created: {}", e));

        let fce = std::rc::Rc::new(std::cell::RefCell::new(fce));
    }
}

fn generate_module_ctors<'n>(
    module_names: impl ExactSizeIterator<Item = &'n str>,
) -> TResult<TokenStream2> {
    let mut module_ctors = Vec::with_capacity(module_names.len());
    for name in module_names {
        // TODO: optimize these two call because they are called twice for each module name
        // and internally allocate memory in format
        let module_name = generate_module_name(&name)?;
        let struct_name = generate_struct_name(&name)?;
        let name_for_user = new_ident(&name)?;

        let module_ctor =
            quote! { let mut #name_for_user = #module_name::#struct_name { fce: fce.clone() }; };
        module_ctors.push(module_ctor);
    }

    let module_ctors = quote! { #(#module_ctors),* };

    Ok(module_ctors)
}

use fce_wit_parser::module_raw_interface;
use fce_wit_parser::interface::FCEModuleInterface;
use fce_wit_parser::interface::FCERecordTypes;
use fce_wit_parser::interface::FCEFunctionSignature;
use fce_wit_parser::interface::it::IFunctionArg;
use fce_wit_parser::interface::it::IRecordFieldType;
use fce_wit_parser::interface::it::IType;

use std::path::PathBuf;
use syn::parse::Parser;

fn generate_module_definitions<'i>(
    module_interfaces: impl ExactSizeIterator<Item = &'i (&'i str, FCEModuleInterface)>,
) -> TResult<TokenStream2> {
    let mut module_definitions = Vec::with_capacity(module_interfaces.len());

    for interface in module_interfaces {
        let module_definition = generate_module_definition(&interface.0, &interface.1)?;
        module_definitions.push(module_definition);
    }

    let module_definitions = quote! { #(#module_definitions),*};

    Ok(module_definitions)
}

fn generate_module_definition(
    module_name: &str,
    module_interface: &FCEModuleInterface,
) -> TResult<TokenStream2> {
    let module_name_ident = generate_module_name(module_name)?;
    let struct_name_ident = generate_struct_name(module_name)?;
    let module_records = generate_records(&module_interface.record_types)?;
    let module_functions = generate_module_methods(
        module_name,
        module_interface.function_signatures.iter(),
        &module_interface.record_types,
    )?;

    let module_definition = quote! {
        pub mod #module_name_ident {
            #module_records

            pub struct #struct_name_ident {
                pub fce: std::rc::Rc<std::cell::RefCell<fluence_test::internal::AppService>>,
            }

            impl #struct_name_ident {
                #(#module_functions)*
            }
        }
    };

    Ok(module_definition)
}

fn generate_module_methods<'m, 'r>(
    module_name: &str,
    method_signatures: impl ExactSizeIterator<Item = &'m FCEFunctionSignature>,
    records: &'r FCERecordTypes,
) -> TResult<Vec<TokenStream2>> {
    let mut result = Vec::with_capacity(method_signatures.len());

    for signature in method_signatures {
        let func_name = new_ident(&signature.name)?;
        let arguments = generate_arguments(signature.arguments.iter(), records)?;
        let output_type = generate_output_type(&signature.outputs, records)?;
        let fce_call = generate_fce_call(module_name, &signature, records)?;

        let module_method = quote! {
            pub fn #func_name(&mut self, #(#arguments),*) #output_type {
                #fce_call
            }
        };

        result.push(module_method);
    }

    Ok(result)
}

fn generate_fce_call(
    module_name: &str,
    method_signature: &FCEFunctionSignature,
    records: &FCERecordTypes,
) -> TResult<TokenStream2> {
    let args = method_signature.arguments.iter().map(|a| a.name.as_str());
    let convert_arguments = generate_arguments_converter(args)?;

    let output_type = get_output_type(&method_signature.outputs);
    let set_result = generate_set_result(&output_type);
    let function_call = generate_function_call(module_name, &method_signature.name);
    let convert_result_to_output_type = generate_convert_to_output(&output_type, records)?;
    let ret = generate_ret(&output_type);

    let function_call = quote! {
        use std::ops::DerefMut;

        #convert_arguments

        #set_result #function_call

        #convert_result_to_output_type

        #ret
    };

    Ok(function_call)
}

fn generate_arguments_converter<'a>(
    args: impl ExactSizeIterator<Item = &'a str>,
) -> TResult<TokenStream2> {
    let mut arguments = Vec::with_capacity(args.len());

    for arg in args {
        let arg_ident = new_ident(arg)?;
        arguments.push(arg_ident);
    }

    let arguments_serializer =
        quote! { let arguments = fluence_test::internal::json!([#(#arguments),*]); };
    Ok(arguments_serializer)
}

fn generate_function_call(module_name: &str, method_name: &str) -> TokenStream2 {
    quote! { self.fce.as_ref().borrow_mut().call_with_module_name(#module_name, #method_name, arguments, <_>::default()).expect("call to FCE failed"); }
}

fn generate_set_result(output_type: &Option<&IType>) -> TokenStream2 {
    match output_type {
        Some(_) => quote! { let result = },
        None => TokenStream2::new(),
    }
}

fn generate_convert_to_output(
    output_type: &Option<&IType>,
    records: &FCERecordTypes,
) -> TResult<TokenStream2> {
    let result_stream = match output_type {
        Some(ty) => {
            let ty = itype_to_tokens(ty, records)?;
            quote! {
                let result: #ty = serde_json::from_value(result).expect("the default deserializer shouldn't fail");
            }
        }
        None => TokenStream2::new(),
    };

    Ok(result_stream)
}

fn generate_ret(output_type: &Option<&IType>) -> TokenStream2 {
    match output_type {
        Some(_) => quote! { result },
        None => TokenStream2::new(),
    }
}

fn generate_arguments<'a, 'r>(
    arguments: impl ExactSizeIterator<Item = &'a IFunctionArg>,
    records: &'r FCERecordTypes,
) -> TResult<Vec<TokenStream2>> {
    let mut result = Vec::with_capacity(arguments.len());
    for argument in arguments {
        let arg_name = new_ident(&argument.name)?;
        let arg_type = itype_to_tokens(&argument.ty, records)?;

        let arg = quote! { #arg_name: #arg_type };
        result.push(arg);
    }

    Ok(result)
}

fn generate_output_type(output_types: &[IType], records: &FCERecordTypes) -> TResult<TokenStream2> {
    let output_type = get_output_type(output_types);
    match output_type {
        None => Ok(TokenStream2::new()),
        Some(ty) => {
            let output_type = itype_to_tokens(&ty, records)?;
            let output_type = quote! { -> #output_type };

            Ok(output_type)
        }
    }
}

fn get_output_type(output_types: &[IType]) -> Option<&IType> {
    match output_types.len() {
        0 => None,
        1 => Some(&output_types[0]),
        _ => unimplemented!("function with more than 1 arguments aren't supported now"),
    }
}

fn generate_records(records: &FCERecordTypes) -> TResult<TokenStream2> {
    use std::ops::Deref;

    let mut result = TokenStream2::new();

    for (_, record) in records.iter() {
        let record_name_ident = generate_record_name(&record.name)?;
        let fields = prepare_field(record.fields.deref(), records)?;

        let record = quote! {
            #[derive(Clone, fluence_test::internal::Serialize, fluence_test::internal::Deserialize)]
            pub struct #record_name_ident {
                #fields
            }
        };
        result.extend(record);
    }

    Ok(result)
}

fn prepare_field(fields: &[IRecordFieldType], records: &FCERecordTypes) -> TResult<TokenStream2> {
    let mut result = TokenStream2::new();

    for field in fields {
        let field_name = new_ident(&field.name)?;
        let field_type = itype_to_tokens(&field.ty, records)?;

        let field = quote! { #field_name: #field_type, };
        result.extend(field);
    }

    Ok(result)
}

fn generate_module_name(module_name: &str) -> TResult<syn::Ident> {
    let extended_module_name = format!("__fce_generated_{}", module_name);
    new_ident(&extended_module_name)
}

fn generate_record_name(record_name: &str) -> TResult<syn::Ident> {
    let extended_record_name = format!("{}", record_name);
    new_ident(&extended_record_name)
}

fn generate_struct_name(struct_name: &str) -> TResult<syn::Ident> {
    let extended_struct_name = format!("FCEGeneratedStruct{}", struct_name);
    new_ident(&extended_struct_name)
}

fn new_ident(ident_str: &str) -> TResult<syn::Ident> {
    syn::parse_str::<syn::Ident>(ident_str).map_err(Into::into)
}

fn itype_to_tokens(itype: &IType, records: &FCERecordTypes) -> TResult<TokenStream2> {
    let token_stream = match itype {
        IType::Record(record_id) => {
            let record = records
                .get(record_id)
                .ok_or_else(|| crate::errors::CorruptedITSection::AbsentRecord(*record_id))?;
            let record_name = new_ident(&record.name)?;
            let token_stream = quote! { #record_name };
            token_stream
        }
        IType::Array(ty) => {
            let inner_ty_token_stream = itype_to_tokens(ty, records)?;
            let token_stream = quote! { Vec<#inner_ty_token_stream> };
            token_stream
        }
        IType::String => quote! { String },
        IType::S8 => quote! { i8 },
        IType::S16 => quote! { i16 },
        IType::S32 => quote! { i32 },
        IType::S64 => quote! { i64 },
        IType::U8 => quote! { u8 },
        IType::U16 => quote! { u16 },
        IType::U32 => quote! { u32 },
        IType::U64 => quote! { u64 },
        IType::I32 => quote! { i32 },
        IType::I64 => quote! { i64 },
        IType::F32 => quote! { f32 },
        IType::F64 => quote! { f64 },
        IType::Anyref => unimplemented!("anyref isn't supported and will be deleted from IType"),
    };

    Ok(token_stream)
}

fn collect_module_interfaces(
    config: &TomlAppServiceConfig,
    modules_dir: PathBuf,
) -> TResult<Vec<(&str, FCEModuleInterface)>> {
    let module_paths = collect_module_paths(config, modules_dir);
    println!("module paths: {:?}", module_paths);

    module_paths
        .into_iter()
        .map(|(name, path)| module_raw_interface(path).map(|interface| (name, interface)))
        .collect::<Result<Vec<_>, _>>()
        .map_err(Into::into)
}

fn collect_module_paths(
    config: &TomlAppServiceConfig,
    modules_dir: PathBuf,
) -> Vec<(&str, PathBuf)> {
    config
        .toml_faas_config
        .module
        .iter()
        .map(|m| {
            let module_file_name = m.file_name.as_ref().unwrap_or_else(|| &m.name);
            let module_file_name = PathBuf::from(module_file_name);
            // TODO: is it right to always have .wasm extension?
            let module_path = modules_dir.join(module_file_name).with_extension("wasm");

            (m.name.as_str(), module_path)
        })
        .collect::<Vec<_>>()
}

fn determine_modules_dir(
    config: &TomlAppServiceConfig,
    modules_dir: Option<String>,
) -> Option<PathBuf> {
    match modules_dir {
        Some(modules_dir) => Some(PathBuf::from(modules_dir)),
        None => config
            .toml_faas_config
            .modules_dir
            .as_ref()
            .map(|p| PathBuf::from(p)),
    }
}
