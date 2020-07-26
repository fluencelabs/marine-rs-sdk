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

use crate::new_ident;
use crate::parsed_type::ParsedType;
use crate::fce_ast_types;
use crate::token_stream_generator::GENERATED_RECORD_SERIALIZER_PREFIX;

use quote::quote;

pub(super) struct RecordSerializerDescriptor {
    pub(super) serializer: proc_macro2::TokenStream,
    pub(super) record_type: syn::Ident,
}

/// This trait could be used to generate various parts of a record serializer func.
pub(super) trait RecordSerializerGlueCodeGenerator {
    fn generate_serializer(&self, record_name: &str) -> RecordSerializerDescriptor;
}

impl RecordSerializerGlueCodeGenerator for fce_ast_types::AstRecordItem {
    fn generate_serializer(&self, record_name: &str) -> RecordSerializerDescriptor {
        let mut serializer = proc_macro2::TokenStream::new();
        for (id, field) in self.fields.iter().enumerate() {
            let field_ident = field_ident(field, id);

            let field_serialization = match &field.ty {
                ParsedType::F64 => {
                    quote! {
                        raw_record.push(#field_ident.to_bits());
                    }
                }
                ParsedType::Utf8String | ParsedType::ByteVector => {
                    quote! {
                        raw_record.push(#field_ident.as_ptr() as _);
                        raw_record.push(#field_ident.len() as _);
                    }
                }
                ParsedType::Record(record_name) => {
                    let serializer_name =
                        new_ident!(GENERATED_RECORD_SERIALIZER_PREFIX.to_string() + &record_name);
                    quote! {
                        raw_record.push(#serializer_name(#field_ident) as _);
                    }
                }
                _ => quote! {
                    raw_record.push(#field_ident as u64);
                },
            };

            serializer.extend(field_serialization);
        }
        let record_type = new_ident!(record_name);

        RecordSerializerDescriptor {
            serializer,
            record_type,
        }
    }
}

fn field_ident(field: &fce_ast_types::AstRecordField, id: usize) -> proc_macro2::TokenStream {
    match &field.name {
        Some(name) => {
            let name = new_ident!(name);
            quote! { record.#name }
        }
        None => {
            let id = new_ident!(format!("{}", id));
            quote! { record.#id }
        }
    }
}
