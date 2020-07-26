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

use super::GENERATED_RECORD_SERIALIZER_PREFIX;
use super::GENERATED_RECORD_DESERIALIZER_PREFIX;

use crate::new_ident;
use crate::fce_ast_types;
use crate::ParsedType;

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

        let serializer = generate_serializer(self);
        let deserializer = generate_deserializer(self);

        let glue_code = quote::quote! {
            #original

            #serializer

            #deserializer

            #[cfg(target_arch = "wasm32")]
            #[doc(hidden)]
            #[allow(clippy::all)]
            #[link_section = #section_name]
            pub static #global_static_name: [u8; #data_size] = { *#data };
        };

        tokens.extend(glue_code);
    }
}

fn field_ident(field: &fce_ast_types::AstRecordField, id: usize) -> proc_macro2::TokenStream {
    match &field.name {
        Some(name) => {
            let name = new_ident!(name);
            quote::quote! { record.#name }
        }
        None => {
            let id = new_ident!(format!("{}", id));
            quote::quote! { record.#id }
        }
    }
}

fn generate_serializer(record: &fce_ast_types::AstRecordItem) -> proc_macro2::TokenStream {
    let serializer_fn_name =
        new_ident!(GENERATED_RECORD_SERIALIZER_PREFIX.to_string() + &record.name);
    let ty = new_ident!(record.name);

    let mut serializer = proc_macro2::TokenStream::new();
    for (id, field) in record.fields.iter().enumerate() {
        let field_ident = field_ident(field, id);

        let field_serialization = match &field.ty {
            ParsedType::F64 => {
                quote::quote! {
                    raw_record.push(#field_ident.to_bits());
                }
            }
            ParsedType::Utf8String | ParsedType::ByteVector => {
                quote::quote! {
                    raw_record.push(#field_ident.as_ptr() as _);
                    raw_record.push(#field_ident.len() as _);
                }
            }
            ParsedType::Record(record_name) => {
                let serializer_name =
                    new_ident!(GENERATED_RECORD_SERIALIZER_PREFIX.to_string() + record_name);
                quote::quote! {
                    raw_record.push(#serializer_name(#field_ident) as _);
                }
            }
            _ => quote::quote! {
                raw_record.push(#field_ident as u64);
            },
        };

        serializer.extend(field_serialization);
    }

    quote::quote! {
        #[cfg(target_arch = "wasm32")]
        #[doc(hidden)]
        #[allow(clippy::all)]
        pub(crate) fn #serializer_fn_name(record: #ty) -> i32 {
            let mut raw_record = Vec::new();

            #serializer

            let raw_record_ptr = raw_record.as_ptr();
            std::mem::forget(raw_record);

            raw_record_ptr as _
        }
    }
}

fn generate_deserializer(record: &fce_ast_types::AstRecordItem) -> proc_macro2::TokenStream {
    let ret_type = new_ident!(record.name);
    let deserializer_fn_name =
        new_ident!(GENERATED_RECORD_DESERIALIZER_PREFIX.to_string() + &record.name);

    let mut field_values = Vec::with_capacity(record.fields.len());
    let mut deserializer = proc_macro2::TokenStream::new();
    let mut value_id: usize = 0;

    for (id, ast_field) in record.fields.iter().enumerate() {
        let field = new_ident!(format!("field_{}", id));
        let field_d = match &ast_field.ty {
            ParsedType::Boolean => {
                quote::quote! {
                    let #field = raw_record[#value_id] as bool;
                }
            }
            ParsedType::I8 => {
                quote::quote! {
                    let #field = raw_record[#value_id] as i8;
                }
            }
            ParsedType::I16 => {
                quote::quote! {
                    let #field = raw_record[#value_id] as i16;
                }
            }
            ParsedType::I32 => {
                quote::quote! {
                    let #field = raw_record[#value_id] as i32;
                }
            }
            ParsedType::I64 => {
                quote::quote! {
                    let #field = raw_record[#value_id] as i64;
                }
            }
            ParsedType::U8 => {
                quote::quote! {
                    let #field = raw_record[#value_id] as u8;
                }
            }
            ParsedType::U16 => {
                quote::quote! {
                    let #field = raw_record[#value_id] as u16;
                }
            }
            ParsedType::U32 => {
                quote::quote! {
                    let #field = raw_record[#value_id] as u32;
                }
            }
            ParsedType::U64 => {
                quote::quote! {
                    let #field = raw_record[#value_id] as u64;
                }
            }
            ParsedType::F32 => {
                quote::quote! {
                    let #field = raw_record[#value_id] as f32;
                }
            }
            ParsedType::F64 => {
                quote::quote! {
                    let #field = f64::from_bits(raw_record[#value_id as _]);
                }
            }
            ParsedType::Utf8String => {
                let ptr_id = value_id;
                let size_id = value_id + 1;
                value_id += 1;

                quote::quote! {
                    let #field = unsafe { String::from_raw_parts(raw_record[#ptr_id] as _, raw_record[#size_id] as _, raw_record[#size_id] as _) };
                }
            }
            ParsedType::ByteVector => {
                let ptr_id = value_id;
                let size_id = value_id + 1;
                value_id += 1;

                quote::quote! {
                    let #field = unsafe { Vec::from_raw_parts(raw_record[#ptr_id] as _, raw_record[#size_id] as _, raw_record[#size_id] as _) };
                }
            }
            ParsedType::Record(record_name) => {
                let ptr_id = value_id;
                let size_id = value_id + 1;
                value_id += 1;
                let deserializer_name =
                    new_ident!(GENERATED_RECORD_DESERIALIZER_PREFIX.to_string() + record_name);

                quote::quote! {
                    let #field = #deserializer_name(raw_record[#ptr_id] as _, raw_record[#size_id] as _);
                }
            }
        };

        field_values.push(field);
        deserializer.extend(field_d);
        value_id += 1;
    }

    let type_constructor = match record.fields.first() {
        Some(ast_field) if ast_field.name.is_some() => {
            let field_names = record
                .fields
                .iter()
                .map(|field| new_ident!(field.name.clone().expect("all fields should have name")))
                .collect::<Vec<_>>();

            quote::quote! {
                #ret_type {
                    #(#field_names: #field_values),*
                }
            }
        }
        Some(_) => {
            quote::quote! {
                #ret_type (
                    #(#field_values),*
                )
            }
        }
        _ => quote::quote! {},
    };

    quote::quote! {
        #[cfg(target_arch = "wasm32")]
        #[doc(hidden)]
        #[allow(clippy::all)]
        unsafe fn #deserializer_fn_name(offset: i32, size: i32) -> #ret_type {
            let raw_record: Vec<u64> = Vec::from_raw_parts(offset as _, size as _, size as _);

            #deserializer

            #type_constructor
        }
    }
}
