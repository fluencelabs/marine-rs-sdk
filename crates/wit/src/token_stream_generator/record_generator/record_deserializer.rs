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

pub(super) struct RecordDeserializerDescriptor {
    pub(super) deserializer: proc_macro2::TokenStream,
    pub(super) type_constructor: proc_macro2::TokenStream,
}

/// This trait could be used to generate various parts of a record serializer func.
pub(super) trait RecordDeserializerGlueCodeGenerator {
    fn generate_deserializer(&self) -> RecordDeserializerDescriptor;
}

impl RecordDeserializerGlueCodeGenerator for fce_ast_types::AstRecordItem {
    fn generate_deserializer(&self) -> RecordDeserializerDescriptor {
        let mut field_values = Vec::with_capacity(self.fields.len());
        let mut deserializer = proc_macro2::TokenStream::new();
        let mut value_id: usize = 0;

        for (id, ast_field) in self.fields.iter().enumerate() {
            let field = new_ident!(format!("field_{}", id));
            let field_d = match &ast_field.ty {
                ParsedType::Boolean => {
                    quote! {
                        let #field = raw_record[#value_id] != 0;
                    }
                }
                ParsedType::I8 => {
                    quote! {
                        let #field = raw_record[#value_id] as i8;
                    }
                }
                ParsedType::I16 => {
                    quote! {
                        let #field = raw_record[#value_id] as i16;
                    }
                }
                ParsedType::I32 => {
                    quote! {
                        let #field = raw_record[#value_id] as i32;
                    }
                }
                ParsedType::I64 => {
                    quote! {
                        let #field = raw_record[#value_id] as i64;
                    }
                }
                ParsedType::U8 => {
                    quote! {
                        let #field = raw_record[#value_id] as u8;
                    }
                }
                ParsedType::U16 => {
                    quote! {
                        let #field = raw_record[#value_id] as u16;
                    }
                }
                ParsedType::U32 => {
                    quote! {
                        let #field = raw_record[#value_id] as u32;
                    }
                }
                ParsedType::U64 => {
                    quote! {
                        let #field = raw_record[#value_id] as u64;
                    }
                }
                ParsedType::F32 => {
                    quote! {
                        let #field = raw_record[#value_id] as f32;
                    }
                }
                ParsedType::F64 => {
                    quote! {
                        let #field = f64::from_bits(raw_record[#value_id]);
                    }
                }
                ParsedType::Utf8String => {
                    let ptr_id = value_id;
                    let size_id = value_id + 1;
                    value_id += 1;

                    quote! {
                        let #field = unsafe { String::from_raw_parts(raw_record[#ptr_id] as _, raw_record[#size_id] as _, raw_record[#size_id] as _) };
                    }
                }
                ParsedType::Vector(ty) => {
                    let ptr_id = value_id;
                    let size_id = value_id + 1;
                    value_id += 1;

                    let generated_deserializer_name =
                        String::from("__fce_generated_vec_deserializer");
                    let generated_deserializer_ident = new_ident!(generated_deserializer_name);
                    let vector_deserializer = crate::parsed_type::generate_vector_deserializer(
                        ty,
                        &generated_deserializer_name,
                    );

                    quote! {
                        #vector_deserializer
                        let #field = unsafe { #generated_deserializer_ident(raw_record[#ptr_id] as _, raw_record[#size_id] as _) };
                    }
                }
                ParsedType::Record(record_name) => {
                    let ptr_id = value_id;
                    let record_ident = new_ident!(record_name);
                    quote! {
                        let #field = #record_ident::__fce_generated_deserialize(raw_record[#ptr_id] as _);
                    }
                }
            };

            field_values.push(field);
            deserializer.extend(field_d);
            value_id += 1;
        }

        let type_constructor = match self.fields.first() {
            Some(ast_field) if ast_field.name.is_some() => {
                let field_names = self
                    .fields
                    .iter()
                    .map(|field| {
                        new_ident!(field.name.clone().expect("all fields should have name"))
                    })
                    .collect::<Vec<_>>();

                quote! {
                    Self {
                        #(#field_names: #field_values),*
                    }
                }
            }
            Some(_) => {
                quote! {
                    Self (
                        #(#field_values),*
                    )
                }
            }
            _ => quote! {},
        };

        RecordDeserializerDescriptor {
            deserializer,
            type_constructor,
        }
    }
}
