/*
 * Copyright 2021 Fluence Labs Limited
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

use crate::fce_test::utils;
use crate::TResult;
use crate::TestGeneratorError;

use fce_wit_parser::interface::it::IType;
use fce_wit_parser::interface::it::IFunctionArg;
use fce_wit_parser::interface::FCERecordTypes;
use fce_wit_parser::interface::FCEFunctionSignature;

use proc_macro2::TokenStream;
use quote::quote;

pub(super) fn generate_module_methods<'m, 'r>(
    module_name: &str,
    method_signatures: impl ExactSizeIterator<Item = &'m FCEFunctionSignature>,
    records: &'r FCERecordTypes,
) -> TResult<Vec<TokenStream>> {
    method_signatures
        .map(|signature| -> TResult<_> {
            let func_name = utils::new_ident(&signature.name)?;
            let arguments = generate_arguments(signature.arguments.iter(), records)?;
            let output_type = generate_output_type(&signature.outputs, records)?;
            let fce_call = generate_fce_call(module_name, &signature, records)?;

            let module_method = quote! {
                pub fn #func_name(&mut self, #(#arguments),*) #output_type {
                    #fce_call
                }
            };

            Ok(module_method)
        })
        .collect::<TResult<Vec<_>>>()
}

fn generate_fce_call(
    module_name: &str,
    method_signature: &FCEFunctionSignature,
    records: &FCERecordTypes,
) -> TResult<TokenStream> {
    let args = method_signature.arguments.iter().map(|a| a.name.as_str());
    let convert_arguments = generate_arguments_converter(args)?;

    let output_type = get_output_type(&method_signature.outputs)?;
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

/// Generates type convertor to json because of AppService receives them in json.
fn generate_arguments_converter<'a>(
    args: impl ExactSizeIterator<Item = &'a str>,
) -> TResult<TokenStream> {
    let arg_idents: Vec<syn::Ident> = args.map(utils::new_ident).collect::<Result<_, _>>()?;

    let args_converter =
        quote! { let arguments = fluence_test::internal::serde_json::json!([#(#arg_idents),*]); };

    Ok(args_converter)
}

fn generate_function_call(module_name: &str, method_name: &str) -> TokenStream {
    quote! { self.fce.as_ref().borrow_mut().call_module(#module_name, #method_name, arguments, <_>::default()).expect("call to FCE failed"); }
}

fn generate_set_result(output_type: &Option<&IType>) -> TokenStream {
    match output_type {
        Some(_) => quote! { let result = },
        None => TokenStream::new(),
    }
}

fn generate_convert_to_output(
    output_type: &Option<&IType>,
    records: &FCERecordTypes,
) -> TResult<TokenStream> {
    let result_stream = match output_type {
        Some(ty) => {
            let ty = utils::itype_to_tokens(ty, records)?;
            quote! {
                let result: #ty = fluence_test::internal::serde_json::from_value(result).expect("the default deserializer shouldn't fail");
            }
        }
        None => TokenStream::new(),
    };

    Ok(result_stream)
}

fn generate_ret(output_type: &Option<&IType>) -> TokenStream {
    match output_type {
        Some(_) => quote! { result },
        None => TokenStream::new(),
    }
}

fn generate_arguments<'a, 'r>(
    arguments: impl ExactSizeIterator<Item = &'a IFunctionArg>,
    records: &'r FCERecordTypes,
) -> TResult<Vec<TokenStream>> {
    arguments
        .map(|argument| -> TResult<_> {
            let arg_name = utils::new_ident(&argument.name)?;
            let arg_type = utils::itype_to_tokens(&argument.ty, records)?;

            let arg = quote! { #arg_name: #arg_type };
            Ok(arg)
        })
        .collect::<TResult<Vec<_>>>()
}

fn generate_output_type(output_types: &[IType], records: &FCERecordTypes) -> TResult<TokenStream> {
    let output_type = get_output_type(output_types)?;
    match output_type {
        None => Ok(TokenStream::new()),
        Some(ty) => {
            let output_type = utils::itype_to_tokens(&ty, records)?;
            let output_type = quote! { -> #output_type };

            Ok(output_type)
        }
    }
}

fn get_output_type(output_types: &[IType]) -> TResult<Option<&IType>> {
    match output_types.len() {
        0 => Ok(None),
        1 => Ok(Some(&output_types[0])),
        _ => Err(TestGeneratorError::ManyFnOutputsUnsupported),
    }
}
