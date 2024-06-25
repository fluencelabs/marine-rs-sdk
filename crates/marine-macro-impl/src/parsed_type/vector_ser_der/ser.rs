/*
 * Fluence Marine Rust SDK
 *
 * Copyright (C) 2024 Fluence DAO
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU Affero General Public License as
 * published by the Free Software Foundation version 3 of the
 * License.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU Affero General Public License for more details.
 *
 * You should have received a copy of the GNU Affero General Public License
 * along with this program.  If not, see <https://www.gnu.org/licenses/>.
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
        marine_rs_sdk::internal::add_object_to_release(Box::new(result));

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
        marine_rs_sdk::internal::add_object_to_release(Box::new(result));

        (result_ptr as _, result_len as _)
    }
}

pub(super) fn record_ser() -> proc_macro2::TokenStream {
    quote! {
        let mut result: Vec<u32> = Vec::with_capacity(arg.len());

        for value in arg {
            result.push(value.__m_generated_serialize() as _);
        }

        let result_ptr = result.as_ptr();
        let result_len = result.len();
        marine_rs_sdk::internal::add_object_to_release(Box::new(result));

        (result_ptr as _, result_len as _)
    }
}
