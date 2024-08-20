/*
 * Marine Rust SDK
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

use crate::new_ident;
use crate::parsed_type::ParsedType;
use crate::ast_types::AstRecord;
use crate::ast_types::AstRecordField;
use crate::ast_types::AstRecordFields;

use quote::quote;

/// This trait could be used to generate various parts of a record serializer func.
pub(super) trait RecordSerGlueCodeGenerator {
    fn generate_serializer(&self) -> proc_macro2::TokenStream;
}

impl RecordSerGlueCodeGenerator for AstRecord {
    fn generate_serializer(&self) -> proc_macro2::TokenStream {
        let mut serializer = proc_macro2::TokenStream::new();
        let fields = match &self.fields {
            AstRecordFields::Named(fields) => fields,
            AstRecordFields::Unnamed(fields) => fields,
            AstRecordFields::Unit => return proc_macro2::TokenStream::new(),
        };

        for (id, field) in fields.iter().enumerate() {
            let field_ident = field_ident(field, id);

            let field_serialization = match &field.ty {
                ParsedType::Boolean(_) => {
                    quote! { raw_record.push(*&#field_ident as _); }
                }
                ParsedType::Utf8Str(_) | ParsedType::Utf8String(_) => {
                    quote! {
                        let field_ident_ptr = #field_ident.as_ptr() as u32;
                        raw_record.extend(&field_ident_ptr.to_le_bytes());
                        raw_record.extend(&(#field_ident.len() as u32).to_le_bytes());
                    }
                }
                ParsedType::Vector(ty, _) => {
                    let generated_ser_name = format!(
                        "__m_generated_vec_serializer_{}_{}",
                        field.name.as_ref().unwrap(),
                        id
                    );

                    let generated_ser_ident = new_ident!(generated_ser_name);
                    let vector_ser =
                        crate::parsed_type::generate_vector_ser(ty, &generated_ser_name);
                    let serialized_field_ident = new_ident!(format!("serialized_arg_{}", id));

                    quote::quote! {
                        #vector_ser
                        let #serialized_field_ident = unsafe { #generated_ser_ident(&#field_ident) };

                        raw_record.extend(&#serialized_field_ident.0.to_le_bytes());
                        raw_record.extend(&#serialized_field_ident.1.to_le_bytes());
                    }
                }
                ParsedType::Record(..) => {
                    quote! {
                        let serialized_struct_ptr = #field_ident.__m_generated_serialize() as usize;
                        raw_record.extend(&serialized_struct_ptr.to_le_bytes());
                    }
                }
                _ => quote! {
                    raw_record.extend(&#field_ident.to_le_bytes());
                },
            };

            serializer.extend(field_serialization);
        }

        serializer
    }
}

fn field_ident(field: &AstRecordField, id: usize) -> proc_macro2::TokenStream {
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
