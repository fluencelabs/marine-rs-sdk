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

use super::ParsedType;
use super::FnArgGlueCodeGenerator;
use crate::new_ident;
use crate::wasm_type::WasmType;

use quote::quote;

/// Describes various parts of a function prolog.
pub(crate) struct FnPrologDescriptor {
    pub(crate) raw_arg_names: Vec<syn::Ident>,
    pub(crate) raw_arg_types: Vec<WasmType>,
    pub(crate) prolog: proc_macro2::TokenStream,
    pub(crate) args: Vec<syn::Ident>,
}

/// This trait could be used to generate various parts needed to construct prolog of an export
/// function. They are marked with # in the following example:
/// ```
/// quote! {
///     fn foo(#(#raw_arg_names: #raw_arg_types),*) {
///        #prolog
///        let result = original_foo(#(#args), *);
///        ...
///     }
/// }
/// ```
pub(crate) trait FnPrologGlueCodeGenerator {
    fn generate_prolog(&self) -> FnPrologDescriptor;
}

impl FnPrologGlueCodeGenerator for Vec<(String, ParsedType)> {
    fn generate_prolog(&self) -> FnPrologDescriptor {
        let mut prolog = proc_macro2::TokenStream::new();
        let mut args: Vec<syn::Ident> = Vec::with_capacity(self.len());
        let mut raw_arg_names = Vec::with_capacity(self.len());
        let mut raw_arg_types = Vec::with_capacity(self.len());

        let mut input_type_id = 0;
        for arg in self {
            let type_prolog = generate_type_prolog(&arg.1, input_type_id, input_type_id);
            let curr_raw_arg_types = arg.generate_arguments();

            args.push(new_ident!(format!("converted_arg_{}", input_type_id)));

            raw_arg_names.extend(
                curr_raw_arg_types
                    .iter()
                    .enumerate()
                    .map(|(id, _)| new_ident!(format!("arg_{}", input_type_id + id))),
            );

            input_type_id += curr_raw_arg_types.len();
            raw_arg_types.extend(curr_raw_arg_types);
            prolog.extend(type_prolog);
        }

        FnPrologDescriptor {
            raw_arg_names,
            raw_arg_types,
            prolog,
            args,
        }
    }
}

fn generate_type_prolog(
    ty: &ParsedType,
    generated_arg_id: usize,
    supplied_arg_start_id: usize,
) -> proc_macro2::TokenStream {
    let generated_arg_id = new_ident!(format!("converted_arg_{}", generated_arg_id));

    match ty {
        ParsedType::Boolean => {
            let supplied_arg_start_id = new_ident!(format!("arg_{}", supplied_arg_start_id));
            quote! {
                let #generated_arg_id = #supplied_arg_start_id != 0;
            }
        }
        ty if !ty.is_complex_type() => {
            let supplied_arg_start_id = new_ident!(format!("arg_{}", supplied_arg_start_id));
            quote! {
                let #generated_arg_id = #supplied_arg_start_id as _;
            }
        }
        ty => {
            // all complex types are represented with pointer and size
            let ptr = new_ident!(format!("arg_{}", supplied_arg_start_id));
            let size = new_ident!(format!("arg_{}", supplied_arg_start_id + 1));
            match ty {
                ParsedType::Utf8String => quote! {
                    let #generated_arg_id = String::from_raw_parts(#ptr as _, #size as _ , #size as _);
                },
                ParsedType::ByteVector => quote! {
                    let #generated_arg_id = Vec::from_raw_parts(#ptr as _, #size as _, #size as _);
                },
                ParsedType::Record(record_name) => {
                    let record_ident = new_ident!(record_name);
                    quote! {
                        let #generated_arg_id = #record_ident::__fce_generated_deserialize(#ptr as _);
                    }
                }
                _ => panic!(
                    "perhaps new type's been added to ParsedType, and this match became incomplete"
                ),
            }
        }
    }
}
