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
use super::passing_style_of;
use crate::new_ident;
use crate::wasm_type::RustType;
use crate::ast_types::AstFnArgument;
use crate::parsed_type::PassingStyle;

use quote::quote;

/// Describes various parts of a function prolog.
pub(crate) struct FnPrologDescriptor {
    pub(crate) raw_arg_names: Vec<syn::Ident>,
    pub(crate) raw_arg_types: Vec<RustType>,
    pub(crate) prolog: proc_macro2::TokenStream,
    pub(crate) converted_arg_idents: Vec<syn::Ident>,
    pub(crate) args: Vec<proc_macro2::TokenStream>,
}

/// This trait could be used to generate various parts needed to construct prolog of an export
/// function. They are marked with # in the following example:
/// ```ignore
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

impl FnPrologGlueCodeGenerator for Vec<AstFnArgument> {
    fn generate_prolog(&self) -> FnPrologDescriptor {
        let mut raw_arg_names = Vec::with_capacity(self.len());
        let mut raw_arg_types = Vec::with_capacity(self.len());
        let mut prolog = proc_macro2::TokenStream::new();
        let mut converted_arg_idents = Vec::with_capacity(self.len());
        let mut args: Vec<proc_macro2::TokenStream> = Vec::with_capacity(self.len());

        let mut input_type_id = 0;
        for arg in self {
            let passing_style = passing_style_of(&arg.ty);
            let TypeLifter {
                converted_arg_ident,
                type_lifter_glue_code,
            } = generate_type_lifting_prolog(&arg.ty, passing_style, input_type_id, input_type_id);

            let curr_raw_arg_types = arg.generate_arguments();
            let arg = quote! { #passing_style #converted_arg_ident };
            args.push(arg);

            raw_arg_names.extend(
                curr_raw_arg_types
                    .iter()
                    .enumerate()
                    .map(|(id, _)| new_ident!(format!("arg_{}", input_type_id + id))),
            );

            input_type_id += curr_raw_arg_types.len();
            raw_arg_types.extend(curr_raw_arg_types);
            prolog.extend(type_lifter_glue_code);
            converted_arg_idents.push(converted_arg_ident);
        }

        FnPrologDescriptor {
            raw_arg_names,
            raw_arg_types,
            prolog,
            converted_arg_idents,
            args,
        }
    }
}

struct TypeLifter {
    pub(self) converted_arg_ident: syn::Ident,
    pub(self) type_lifter_glue_code: proc_macro2::TokenStream,
}

fn generate_type_lifting_prolog(
    ty: &ParsedType,
    passing_style: &PassingStyle,
    generated_arg_id: usize,
    supplied_arg_start_id: usize,
) -> TypeLifter {
    const CONVERTED_ARG_PREFIX: &str = "converted_arg";

    let converted_arg_ident = new_ident!(format!("{}_{}", CONVERTED_ARG_PREFIX, generated_arg_id));
    let type_modifier = converted_arg_modifier(passing_style);

    let type_lifter_glue_code = match ty {
        ParsedType::Boolean(_) => {
            let supplied_arg_start_id = new_ident!(format!("arg_{}", supplied_arg_start_id));
            quote! {
                let #type_modifier #converted_arg_ident = #supplied_arg_start_id != 0;
            }
        }
        ty if !ty.is_complex_type() => {
            let supplied_arg_start_id = new_ident!(format!("arg_{}", supplied_arg_start_id));
            quote! {
                let #type_modifier #converted_arg_ident = #supplied_arg_start_id as _;
            }
        }
        ty => {
            // all complex types are represented with pointer and size
            let ptr = new_ident!(format!("arg_{}", supplied_arg_start_id));
            let size = new_ident!(format!("arg_{}", supplied_arg_start_id + 1));
            match ty {
                ParsedType::Utf8Str(_) | ParsedType::Utf8String(_) => quote! {
                    let #type_modifier #converted_arg_ident = String::from_raw_parts(#ptr as _, #size as _ , #size as _);
                },
                ParsedType::Vector(ty, _) => {
                    let generated_der_name =
                        format!("__fce_generated_vec_deserializer_{}", supplied_arg_start_id);
                    let generated_der_name = crate::utils::prepare_ident(generated_der_name);
                    let generated_der_ident = new_ident!(generated_der_name);

                    let vector_deserializer =
                        super::vector_utils::generate_vector_deserializer(ty, &generated_der_name);

                    quote! {
                        #vector_deserializer
                        let #type_modifier #converted_arg_ident = #generated_der_ident(#ptr as _, #size as _);
                    }
                }
                ParsedType::Record(record_name, _) => {
                    let record_ident = new_ident!(record_name);
                    quote! {
                        let #type_modifier #converted_arg_ident = #record_ident::__fce_generated_deserialize(#ptr as _);
                    }
                }
                _ => panic!(
                    "perhaps new type's been added to ParsedType, and this match became incomplete"
                ),
            }
        }
    };

    TypeLifter {
        converted_arg_ident,
        type_lifter_glue_code,
    }
}

fn converted_arg_modifier(passing_style: &PassingStyle) -> proc_macro2::TokenStream {
    match passing_style {
        PassingStyle::ByMutRef => quote! { mut },
        _ => quote! {},
    }
}
