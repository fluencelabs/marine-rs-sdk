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
use crate::TResult;

use fluence_app_service::TomlAppServiceConfig;
use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::quote;

pub(super) fn fce_test_impl(attrs: TokenStream, func_input: syn::ItemFn) -> TokenStream {
    use darling::FromMeta;

    let attrs = syn::parse_macro_input!(attrs as syn::AttributeArgs);
    let attrs = match FCETestAttributes::from_list(&attrs) {
        Ok(v) => v,
        Err(e) => {
            return TokenStream::from(e.write_errors());
        }
    };

    generate_test_glue_code(func_input, &attrs.config_path).into()
}

fn generate_test_glue_code(func: syn::ItemFn, config_path: &str) -> TokenStream2 {
    let fce_ctor = generate_fce_ctor(config_path);
    let original_block = func.block;
    let signature = func.sig;

    quote! {
        #[test]
        #signature {
            #fce_ctor

            #original_block
        }
    }
}

fn generate_fce_ctor(config_path: &str) -> TokenStream2 {
    let config_path = quote! { #config_path };

    let tmp_file_path = std::env::temp_dir();
    let random_uuid = uuid::Uuid::new_v4().to_string();
    let service_id = quote! { #random_uuid };

    let tmp_file_path = tmp_file_path.join(random_uuid);
    let tmp_file_path = tmp_file_path.to_string_lossy().to_string();
    let tmp_file_path = quote! { #tmp_file_path };

    quote! {
        let mut __fce__generated_fce_config = fluence_test::internal::TomlAppServiceConfig::load(#config_path.to_string())
            .unwrap_or_else(|e| panic!("app service located at `{}` config can't be loaded: {}", #config_path, e));
        __fce__generated_fce_config.service_base_dir = Some(#tmp_file_path.to_string());

        let mut fce = fluence_test::internal::AppService::new_with_empty_facade(__fce__generated_fce_config, #service_id, std::collections::HashMap::new())
            .unwrap_or_else(|e| panic!("app service can't be created: {}", e));
    }
}

use fce_wit_parser::module_raw_interface;
use fce_wit_parser::interface::FCEModuleInterface;
use fce_wit_parser::interface::FCERecordTypes;
use fce_wit_parser::interface::FCEFunctionSignature;
use fce_wit_parser::interface::it::IFunctionArg;
use fce_wit_parser::interface::it::IRecordFieldType;
use fce_wit_parser::interface::it::IType;

use std::path::PathBuf;

fn generate_module_definition(
    module_name: &str,
    module_interface: &FCEModuleInterface,
) -> TResult<TokenStream2> {
    let module_name_ident = new_ident(module_name)?;
    let module_records = generate_records(&module_interface.record_types)?;
    let module_functions = generate_module_methods(
        module_name,
        module_interface.function_signatures.iter(),
        &module_interface.record_types,
    )?;

    let module_definition = quote! {
        pub mod #module_name_ident {
            #module_records

            struct #module_name_ident {
                pub fce: fluence_test::internal::AppService,
            }

            impl #module_name_ident {
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
        quote! { let arguments = fluence_test::internal::json!([#(#arguments)*,]) };
    Ok(arguments_serializer)
}

fn generate_function_call(module_name: &str, method_name: &str) -> TokenStream2 {
    quote! { self.call_module(#module_name, #method_name, arguments, <_>::default()).expect("call to FCE failed"); }
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
                let result: #ty = serde_json::from_value(result).expect("default deserializer shouldn't fail");
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
        let name = new_ident(&record.name)?;
        let fields = prepare_field(record.fields.deref(), records)?;

        let record = quote! {
            #[derive(Clone, fluence_test::internal::Serialize, fluence_test::internal::Deserialize)]
            struct #name {
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

        let field = quote! { #field_name: #field_type };
        result.extend(field);
    }

    Ok(result)
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
) -> TResult<Vec<(&str, FCEModuleInterface)>> {
    let module_paths = collect_module_paths(config);

    module_paths
        .into_iter()
        .map(|(name, path)| module_raw_interface(path).map(|interface| (name, interface)))
        .collect::<Result<Vec<_>, _>>()
        .map_err(Into::into)
}

fn collect_module_paths(config: &TomlAppServiceConfig) -> Vec<(&str, PathBuf)> {
    let base_dir = config
        .toml_faas_config
        .modules_dir
        .as_ref()
        .map(|p| PathBuf::from(p))
        .unwrap_or_default();

    config
        .toml_faas_config
        .module
        .iter()
        .map(|m| {
            let module_file_name = m.file_name.as_ref().unwrap_or_else(|| &m.name);
            let module_file_name = PathBuf::from(module_file_name);
            let module_path = base_dir.join(module_file_name);

            (m.name.as_str(), module_path)
        })
        .collect::<Vec<_>>()
}
