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
use super::passing_style_of;
use super::PassingStyle;
use crate::new_ident;
use crate::ast_types::AstFnArgument;

use quote::quote;

/// Describes various parts of a function epilog.
pub(crate) struct FnEpilogDescriptor {
    pub(crate) fn_return_type: proc_macro2::TokenStream,
    pub(crate) return_expression: proc_macro2::TokenStream,
    pub(crate) epilog: proc_macro2::TokenStream,
    pub(crate) objs_savings: proc_macro2::TokenStream,
}

/// Contains all ingredients needed for epilog creation.
pub(crate) struct FnEpilogIngredients<'i> {
    pub(crate) args: &'i [AstFnArgument],
    pub(crate) converted_args: &'i [syn::Ident],
    pub(crate) return_type: &'i Option<ParsedType>,
}

/// This trait could be used to generate various parts needed to construct epilog of an export
/// function. They are marked with # in the following example:
/// ```ignore
/// quote! {
///     pub unsafe fn foo(...) #fn_return_type {
///         ...
///         #return_expression original_foo(...);
///         #epilog
///     }
/// }
/// ```
pub(crate) trait FnEpilogGlueCodeGenerator {
    fn generate_fn_epilog(&self) -> FnEpilogDescriptor;
}

impl FnEpilogGlueCodeGenerator for FnEpilogIngredients<'_> {
    fn generate_fn_epilog(&self) -> FnEpilogDescriptor {
        FnEpilogDescriptor {
            fn_return_type: generate_fn_return_type(self.return_type),
            return_expression: generate_return_expression(self.return_type),
            epilog: generate_epilog(self.return_type),
            objs_savings: generate_objs_savings(self),
        }
    }
}

pub(crate) fn generate_fn_return_type(ty: &Option<ParsedType>) -> proc_macro2::TokenStream {
    let ty = match ty {
        Some(ParsedType::Boolean(_)) => Some("i32"),
        Some(ParsedType::I8(_)) => Some("i8"),
        Some(ParsedType::I16(_)) => Some("i16"),
        Some(ParsedType::I32(_)) => Some("i32"),
        Some(ParsedType::I64(_)) => Some("i64"),
        Some(ParsedType::U8(_)) => Some("u8"),
        Some(ParsedType::U16(_)) => Some("u16"),
        Some(ParsedType::U32(_)) => Some("u32"),
        Some(ParsedType::U64(_)) => Some("u64"),
        Some(ParsedType::F32(_)) => Some("f32"),
        Some(ParsedType::F64(_)) => Some("f64"),
        None
        | Some(ParsedType::Utf8Str(_))
        | Some(ParsedType::Utf8String(_))
        | Some(ParsedType::Vector(..))
        | Some(ParsedType::Record(..)) => None,
    };

    match ty {
        Some(ty) => {
            let ty = new_ident!(ty);
            quote! { -> #ty}
        }
        None => quote! {},
    }
}

pub(crate) fn generate_fn_original_return_type(
    ty: &Option<ParsedType>,
) -> proc_macro2::TokenStream {
    match &ty {
        Some(ty) => quote! {-> #ty},
        None => <_>::default(),
    }
}

pub(crate) fn generate_return_expression(ty: &Option<ParsedType>) -> proc_macro2::TokenStream {
    match ty {
        None => quote! {},
        _ => quote! {
            let result =
        },
    }
}

fn generate_epilog(ty: &Option<ParsedType>) -> proc_macro2::TokenStream {
    match ty {
        None => quote!(),
        Some(ParsedType::Record(..)) => {
            quote! {
                let result_ptr = result.__m_generated_serialize();
                marine_rs_sdk::internal::set_result_ptr(result_ptr as _);
            }
        }
        Some(ParsedType::Utf8Str(_)) | Some(ParsedType::Utf8String(_)) => {
            quote! {
                marine_rs_sdk::internal::set_result_ptr(result.as_ptr() as _);
                marine_rs_sdk::internal::set_result_size(result.len() as _);
            }
        }
        Some(ParsedType::Vector(ty, _)) => {
            let generated_serializer_name = "__m_generated_vec_serializer";
            let generated_serializer_ident = new_ident!(generated_serializer_name);
            let vector_serializer =
                super::vector_ser_der::generate_vector_ser(ty, generated_serializer_name);

            quote! {
                #vector_serializer
                {
                    let (serialized_vec_ptr, serialized_vec_size) = #generated_serializer_ident(&result);
                    marine_rs_sdk::internal::set_result_ptr(serialized_vec_ptr as _);
                    marine_rs_sdk::internal::set_result_size(serialized_vec_size as _);
                }
            }
        }
        Some(_) => quote! {
            return result as _;
        },
    }
}

/// If an export function returns a reference, this is probably a reference to one
/// of the function arguments. If that's the case, reference must be still valid after
/// the end of the function. Their deletion will be handled by IT with calling `release_objects`.
fn generate_objs_savings(ingredients: &FnEpilogIngredients<'_>) -> proc_macro2::TokenStream {
    match ingredients.return_type {
        // if type is empty we don't need to save arguments or objects
        Some(ty) if !ty.is_complex_type() => return proc_macro2::TokenStream::new(),
        None => return proc_macro2::TokenStream::new(),
        _ => {}
    };

    let passing_style = ingredients.return_type.as_ref().map(passing_style_of);

    match passing_style {
        // result will be deleted by IT side
        Some(PassingStyle::ByValue) => {
            quote! { marine_rs_sdk::internal::add_object_to_release(Box::new(result)); }
        }
        Some(PassingStyle::ByRef) | Some(PassingStyle::ByMutRef) => {
            generate_args_savings(ingredients.args, ingredients.converted_args)
        }
        None => quote! {},
    }
}

fn generate_args_savings(
    args: &[AstFnArgument],
    converted_args: &[syn::Ident],
) -> proc_macro2::TokenStream {
    debug_assert!(args.len() == converted_args.len());

    let mut res = proc_macro2::TokenStream::new();
    for (arg, converted_arg) in args.iter().zip(converted_args) {
        let arg_passing_style = passing_style_of(&arg.ty);
        match arg_passing_style {
            // such values will be deleted inside an export function because they are being moved
            PassingStyle::ByValue => {}
            _ => res.extend(quote! {
                marine_rs_sdk::internal::add_object_to_release(Box::new(#converted_arg));
            }),
        }
    }

    res
}
