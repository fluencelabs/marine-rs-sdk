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
use crate::wasm_type::RustType;
use crate::parsed_type::PassingStyle;

use quote::quote;

/// Describes various parts of a function prolog.
pub(crate) struct FnPrologDescriptor {
    pub(crate) raw_arg_names: Vec<syn::Ident>,
    pub(crate) raw_arg_types: Vec<RustType>,
    pub(crate) prolog: proc_macro2::TokenStream,
    pub(crate) args: Vec<proc_macro2::TokenStream>,
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
        let mut args: Vec<proc_macro2::TokenStream> = Vec::with_capacity(self.len());
        let mut raw_arg_names = Vec::with_capacity(self.len());
        let mut raw_arg_types = Vec::with_capacity(self.len());

        let mut input_type_id = 0;
        for arg in self {
            let type_prolog = generate_type_lifting_prolog(&arg.1, input_type_id, input_type_id);
            let curr_raw_arg_types = arg.generate_arguments();
            let passing_style = passing_style_of(&arg.1);

            let arg = new_ident!(format!("converted_arg_{}", input_type_id));
            let arg = quote! { #passing_style #arg };
            args.push(arg);

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

fn generate_type_lifting_prolog(
    ty: &ParsedType,
    generated_arg_id: usize,
    supplied_arg_start_id: usize,
) -> proc_macro2::TokenStream {
    let generated_arg_id = new_ident!(format!("converted_arg_{}", generated_arg_id));

    match ty {
        ParsedType::Boolean(_) => {
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
                ParsedType::Utf8String(_) => quote! {
                    let #generated_arg_id = String::from_raw_parts(#ptr as _, #size as _ , #size as _);
                },
                ParsedType::Vector(ty, _) => {
                    let generated_deserializer_name =
                        format!("__fce_generated_vec_deserializer_{}", supplied_arg_start_id);
                    let generated_deserializer_ident = new_ident!(generated_deserializer_name);
                    let vector_deserializer = super::vector_utils::generate_vector_deserializer(
                        ty,
                        &generated_deserializer_name,
                    );

                    quote! {
                        #vector_deserializer
                        let #generated_arg_id = #generated_deserializer_ident(#ptr as _, #size as _);
                    }
                }
                ParsedType::Record(record_name, _) => {
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

fn passing_style_of(ty: &ParsedType) -> &PassingStyle {
    use ParsedType::*;

    match ty {
        Boolean(passing_style) => passing_style,
        U8(passing_style) => passing_style,
        U16(passing_style) => passing_style,
        U32(passing_style) => passing_style,
        U64(passing_style) => passing_style,
        I8(passing_style) => passing_style,
        I16(passing_style) => passing_style,
        I32(passing_style) => passing_style,
        I64(passing_style) => passing_style,
        F32(passing_style) => passing_style,
        F64(passing_style) => passing_style,
        Utf8Str(passing_style) => passing_style,
        Utf8String(passing_style) => passing_style,
        Vector(_, passing_style) => passing_style,
        Record(_, passing_style) => passing_style,
    }
}
