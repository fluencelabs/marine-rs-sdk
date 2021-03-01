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
use crate::wasm_type::RustType;
use crate::new_ident;
use crate::parsed_type::PassingStyle;

pub(crate) struct WrapperDescriptor {
    pub(crate) arg_names: Vec<syn::Ident>,
    pub(crate) arg_types: Vec<proc_macro2::TokenStream>,
    pub(crate) raw_args: Vec<proc_macro2::TokenStream>,
    pub(crate) arg_transforms: proc_macro2::TokenStream,
    pub(crate) arg_drops: proc_macro2::TokenStream,
}

pub(crate) struct ExternDescriptor {
    pub(crate) raw_arg_names: Vec<syn::Ident>,
    pub(crate) raw_arg_types: Vec<RustType>,
}

/// This trait could be used to generate various parts needed to construct prolog of an wrapper
/// function or extern block. They are marked with # in the following examples:
/// ```
/// quote! {
///     fn foo(#(#arg_names: #arg_types), *) {
///         let arg_1 = std::mem::ManuallyDrop::new(arg_1);
///         let result = original_foo(#(#raw_args), *);
///         std::mem::ManuallyDrop::drop(&mut arg_1);
///         ...
///     }
/// }
/// ```
///
/// ```
/// quote! {
///     extern "C" {
///         #[link_name = "foo_link_name"]
///         pub fn foo(#(#raw_arg_names: #raw_arg_types),*);
///     }
/// }
/// ```
pub(crate) trait ForeignModPrologGlueCodeGenerator {
    fn generate_wrapper_prolog(&self) -> WrapperDescriptor;
    fn generate_extern_prolog(&self) -> ExternDescriptor;
}

impl ForeignModPrologGlueCodeGenerator for Vec<(String, ParsedType)> {
    fn generate_wrapper_prolog(&self) -> WrapperDescriptor {
        use crate::parsed_type::foreign_mod_arg::ForeignModArgGlueCodeGenerator;
        use quote::ToTokens;

        let arg_types: Vec<proc_macro2::TokenStream> = self
            .iter()
            .map(|(_, input_type)| input_type.to_token_stream())
            .collect();

        let (arg_names, arg_transforms, arg_drops) = self
            .iter()
            .enumerate()
            .fold((Vec::new(), proc_macro2::TokenStream::new(), proc_macro2::TokenStream::new()), |(mut arg_names, mut arg_transforms, mut arg_drops), (id, (_, ty))| {
                let arg_name = format!("arg_{}", id);
                let arg_ident = new_ident!(arg_name);
                arg_names.push(arg_ident.clone());

                // arguments of following two types shouldn't be deleted after transformation to raw view
                match ty {
                    ParsedType::Utf8String(PassingStyle::ByValue) => {
                        arg_transforms.extend(quote::quote! { let mut #arg_ident = std::mem::ManuallyDrop::new(#arg_ident); });
                        arg_drops.extend(quote::quote! { std::mem::ManuallyDrop::drop(&mut #arg_ident); });
                    },
                    ParsedType::Vector(ty, _) => {
                        let generated_serializer_name = format!("__fce_generated_vec_serializer_{}", arg_name);
                        let generated_serializer_ident = new_ident!(generated_serializer_name);
                        let vector_serializer = super::vector_utils::generate_vector_serializer(ty, &generated_serializer_name);

                        let arg_transform = quote::quote! {
                            #vector_serializer

                            let #arg_ident = #generated_serializer_ident(#arg_ident);
                        };
                        arg_transforms.extend(arg_transform);

                    }
                    _ => {}
                }

                (arg_names, arg_transforms, arg_drops)
            });

        let raw_args: Vec<proc_macro2::TokenStream> = self
            .iter()
            .enumerate()
            .map(|(id, (_, input_type))| input_type.generate_raw_args(id))
            .collect();

        WrapperDescriptor {
            arg_names,
            arg_types,
            arg_transforms,
            arg_drops,
            raw_args,
        }
    }

    fn generate_extern_prolog(&self) -> ExternDescriptor {
        use crate::parsed_type::FnArgGlueCodeGenerator;

        let raw_arg_types: Vec<RustType> = self
            .iter()
            .map(|input_type| input_type.generate_arguments())
            .flatten()
            .collect();
        let raw_arg_names: Vec<syn::Ident> = raw_arg_types
            .iter()
            .enumerate()
            .map(|(id, _)| new_ident!(format!("arg_{}", id)))
            .collect();

        ExternDescriptor {
            raw_arg_names,
            raw_arg_types,
        }
    }
}
