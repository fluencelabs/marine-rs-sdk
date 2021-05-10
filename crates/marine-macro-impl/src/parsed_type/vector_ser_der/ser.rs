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

use quote::quote;

pub(super) fn string_ser() -> proc_macro2::TokenStream {
    quote! {
        let mut result: Vec<u32> = Vec::with_capacity(arg.len());

        for value in arg {
            result.push(value.as_ptr() as _);
            result.push(value.len() as _);
        }

        let result_ptr = result.as_ptr();
        let result_len = result.len() / 2;
        fluence::internal::add_object_to_release(Box::new(result));

        (result_ptr as _, result_len as _)
    }
}

pub(super) fn vector_ser(arg_name: &str, ty: &ParsedType) -> proc_macro2::TokenStream {
    let ser_name = format!("{}_{}", arg_name, ty);
    let ser_name = crate::utils::prepare_ident(ser_name);
    let ser_ident = crate::new_ident!(ser_name);

    let inner_vector_ser = super::generate_vector_ser(ty, &ser_name);

    quote! {
        #inner_vector_ser

        let mut result: Vec<u32> = Vec::with_capacity(2 * arg.len());
        for value in arg {
            let (ptr, size) = #ser_ident(&value);
            result.push(ptr as _);
            result.push(size as _);
        }

        let result_ptr = result.as_ptr();
        let result_len = result.len() / 2;
        fluence::internal::add_object_to_release(Box::new(result));

        (result_ptr as _, result_len as _)
    }
}

pub(super) fn record_ser() -> proc_macro2::TokenStream {
    quote! {
        let mut result: Vec<u32> = Vec::with_capacity(arg.len());

        for value in arg {
            result.push(value.__fce_generated_serialize() as _);
        }

        let result_ptr = result.as_ptr();
        let result_len = result.len();
        fluence::internal::add_object_to_release(Box::new(result));

        (result_ptr as _, result_len as _)
    }
}
