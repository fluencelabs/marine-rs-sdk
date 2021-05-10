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

use crate::ParsedType;

use quote::quote;

pub(super) fn string_der() -> proc_macro2::TokenStream {
    quote! {
        let vec_passing_size = 2;
        let mut arg: Vec<u32> = Vec::from_raw_parts(offset as _, (vec_passing_size * size) as _, (vec_passing_size * size) as _);
        let mut arg = arg.into_iter();
        let mut result = Vec::with_capacity(arg.len() / 2);

        while let Some(offset) = arg.next() {
            let size = arg.next().unwrap();
            let value = String::from_raw_parts(offset as _, size as _, size as _);
            result.push(value);
        }

        result
    }
}

pub(super) fn vector_der(arg_name: &str, ty: &ParsedType) -> proc_macro2::TokenStream {
    let deserializer_name = format!("{}_{}", arg_name, ty);
    let deserializer_name = crate::utils::prepare_ident(deserializer_name);
    let deserializer_ident = crate::new_ident!(deserializer_name);

    let inner_vector_deserializer = super::generate_vector_der(&*ty, &deserializer_name);

    quote! {
        #inner_vector_deserializer

        let vec_passing_size = 2;
        let mut arg: Vec<u32> = Vec::from_raw_parts(offset as _, (vec_passing_size * size) as _, (vec_passing_size * size) as _);
        let mut result = Vec::with_capacity(arg.len());

        let mut arg = arg.into_iter();
        while let Some(offset) = arg.next() {
            let size = arg.next().unwrap();

            let value = #deserializer_ident(offset as _, size as _);
            result.push(value);
        }

        result
    }
}

pub(super) fn record_der(record_name: &str) -> proc_macro2::TokenStream {
    let record_name_ident = crate::new_ident!(record_name);

    quote! {
        let mut arg: Vec<u32> = Vec::from_raw_parts(offset as _, size as _, size as _);
        let mut result = Vec::with_capacity(arg.len());

        for offset in arg {
            let value = #record_name_ident::__m_generated_deserialize(offset as _);
            result.push(value);
        }

        result
    }
}
