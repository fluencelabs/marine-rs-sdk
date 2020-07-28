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

use quote::quote;

/// This trait could be used to generate various parts of a record serializer func.
pub(super) trait RecordSerializerGlueCodeGenerator {
    fn generate_serializer(&self) -> proc_macro2::TokenStream;
}

impl RecordSerializerGlueCodeGenerator for fce_ast_types::AstRecordItem {
    fn generate_serializer(&self) -> proc_macro2::TokenStream {
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
                        std::mem::forget(#field_ident);
                    }
                }
                ParsedType::Record(_) => {
                    quote! {
                        raw_record.push(#field_ident.__fce_generated_serialize() as _);
                    }
                }
                _ => quote! {
                    raw_record.push(#field_ident as u64);
                },
            };

            serializer.extend(field_serialization);
        }

        serializer
    }
}

fn field_ident(field: &fce_ast_types::AstRecordField, id: usize) -> proc_macro2::TokenStream {
    match &field.name {
        Some(name) => {
            let name = new_ident!(name);
            quote! { self.#name }
        }
        None => {
            let id = new_ident!(format!("{}", id));
            quote! { self.#id }
        }
    }
}
