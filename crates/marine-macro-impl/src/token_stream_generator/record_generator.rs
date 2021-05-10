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
mod field_values_builder;

use field_values_builder::*;
use record_deserializer::*;
use record_serializer::*;

use crate::new_ident;
use crate::ast_types::AstRecord;
use crate::ast_types::AstRecordFields;

impl quote::ToTokens for AstRecord {
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
        let record_name = new_ident!(self.name);

        let serializer_fn = generate_serializer_fn(self);
        let deserializer_fn = generate_deserializer_fn(self);

        let glue_code = quote::quote! {
            #original

            #[cfg(target_arch = "wasm32")]
            #[doc(hidden)]
            #[allow(clippy::all)]
            impl #record_name {
                #serializer_fn

                #deserializer_fn
            }

            #[cfg(target_arch = "wasm32")]
            #[doc(hidden)]
            #[allow(clippy::all)]
            #[link_section = #section_name]
            pub static #global_static_name: [u8; #data_size] = { *#data };
        };

        tokens.extend(glue_code);
    }
}

fn generate_serializer_fn(record: &AstRecord) -> proc_macro2::TokenStream {
    let serializer = record.generate_serializer();
    let fields_count = match &record.fields {
        AstRecordFields::Named(fields) => fields.len(),
        AstRecordFields::Unnamed(fields) => fields.len(),
        AstRecordFields::Unit => return proc_macro2::TokenStream::new(),
    };

    quote::quote! {
        pub fn __m_generated_serialize(&self) -> *const u8 {
            // 4 is an average size of a possible record field
            let mut raw_record: Vec<u8> = Vec::with_capacity(4 * #fields_count);

            #serializer

            let raw_record_ptr = raw_record.as_ptr();
            fluence::internal::add_object_to_release(Box::new(raw_record));

            raw_record_ptr as _
        }
    }
}

fn generate_deserializer_fn(record: &AstRecord) -> proc_macro2::TokenStream {
    let RecordDerDescriptor {
        fields_der,
        record_ctor,
    } = record.generate_der();

    let fields = match &record.fields {
        AstRecordFields::Named(fields) => fields,
        AstRecordFields::Unnamed(fields) => fields,
        AstRecordFields::Unit => return proc_macro2::TokenStream::new(),
    };

    let record_size = crate::utils::get_record_size(fields.iter().map(|ast_field| &ast_field.ty));

    quote::quote! {
        pub unsafe fn __m_generated_deserialize(record_ptr: *const u8) -> Self {
            let raw_record: Vec<u8> = Vec::from_raw_parts(record_ptr as _, #record_size, #record_size);

            #fields_der

            #record_ctor
        }
    }
}
