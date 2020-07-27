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

mod record_serializer;
mod record_deserializer;

use record_serializer::*;
use record_deserializer::*;

use super::GENERATED_RECORD_SERIALIZER_PREFIX;
use super::GENERATED_RECORD_DESERIALIZER_PREFIX;

use crate::new_ident;
use crate::fce_ast_types;

impl quote::ToTokens for fce_ast_types::AstRecordItem {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let original = &self.original;
        crate::prepare_global_data!(
            Record,
            self,
            self.name,
            data,
            data_size,
            global_static_name,
            section_name
        );

        let serializer_fn = generate_serializer_fn(self);
        let deserializer_fn = generate_deserializer_fn(self);

        let glue_code = quote::quote! {
            #original

            #[cfg(target_arch = "wasm32")]
            #[doc(hidden)]
            #[allow(clippy::all)]
            #serializer_fn

            #[cfg(target_arch = "wasm32")]
            #[doc(hidden)]
            #[allow(clippy::all)]
            #deserializer_fn

            #[cfg(target_arch = "wasm32")]
            #[doc(hidden)]
            #[allow(clippy::all)]
            #[link_section = #section_name]
            pub static #global_static_name: [u8; #data_size] = { *#data };
        };

        tokens.extend(glue_code);
    }
}

fn generate_serializer_fn(record: &fce_ast_types::AstRecordItem) -> proc_macro2::TokenStream {
    let serializer_fn_name =
        new_ident!(GENERATED_RECORD_SERIALIZER_PREFIX.to_string() + &record.name);

    let RecordSerializerDescriptor {
        serializer,
        record_type,
    } = record.generate_serializer(&record.name);

    quote::quote! {
        pub(in crate) fn #serializer_fn_name(record: #record_type) -> i32 {
            let mut raw_record = Vec::new();

            #serializer

            let raw_record_ptr = raw_record.as_ptr();
            std::mem::forget(raw_record);

            raw_record_ptr as _
        }
    }
}

fn generate_deserializer_fn(record: &fce_ast_types::AstRecordItem) -> proc_macro2::TokenStream {
    let deserializer_fn_name =
        new_ident!(GENERATED_RECORD_DESERIALIZER_PREFIX.to_string() + &record.name);

    let RecordDeserializerDescriptor {
        deserializer,
        type_constructor,
        return_type,
    } = record.generate_deserializer(&record.name);

    let record_size =
        crate::utils::get_record_size(record.fields.iter().map(|ast_field| &ast_field.ty));

    quote::quote! {
        pub(in crate) unsafe fn #deserializer_fn_name(offset: i32) -> #return_type {
            let raw_record: Vec<u64> = Vec::from_raw_parts(offset as _, #record_size, #record_size);

            #deserializer

            std::mem::forget(raw_record);

            #type_constructor
        }
    }
}
