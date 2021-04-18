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
use crate::ast_types::*;

use quote::quote;

#[derive(Default)]
pub(super) struct RecordDerDescriptor {
    pub(super) fields_der: proc_macro2::TokenStream,
    pub(super) record_ctor: proc_macro2::TokenStream,
}

/// This trait could be used to generate various parts of a record serializer func.
pub(super) trait RecordDerGlueCodeGenerator {
    fn generate_der(&self) -> RecordDerDescriptor;
}

impl RecordDerGlueCodeGenerator for AstRecordItem {
    fn generate_der(&self) -> RecordDerDescriptor {
        match &self.fields {
            AstRecordFields::Named(fields) => record_der_from_named(fields),
            AstRecordFields::Unnamed(fields) => record_der_from_unnamed(fields),
            AstRecordFields::Unit => RecordDerDescriptor::default(),
        }
    }
}

fn record_der_from_named(fields: &[AstRecordField]) -> RecordDerDescriptor {
    let builder = FieldValuesBuilder::build(fields.iter());
    let record_ctor = field_ctors_from_named(fields.iter(), builder.field_value_idents.iter());

    RecordDerDescriptor {
        fields_der: builder.fields_der,
        record_ctor,
    }
}

fn record_der_from_unnamed(fields: &[AstRecordField]) -> RecordDerDescriptor {
    let builder = FieldValuesBuilder::build(fields.iter());
    let record_ctor = field_ctor_from_unnamed(builder.field_value_idents.iter());

    RecordDerDescriptor {
        fields_der: builder.fields_der,
        record_ctor,
    }
}

struct FieldValuesBuilder {
    value_id: usize,
    fields_der: proc_macro2::TokenStream,
    field_value_idents: Vec<syn::Ident>,
}

/// Contains all necessary info to construct record fields.
struct FieldValuesOutcome {
    /// Generated deserializer for each record field.
    fields_der: proc_macro2::TokenStream,

    /// Idents of each record field.
    field_value_idents: Vec<syn::Ident>,
}

impl FieldValuesBuilder {
    pub(self) fn build<'a>(
        fields: impl ExactSizeIterator<Item = &'a AstRecordField>,
    ) -> FieldValuesOutcome {
        let values_builder = Self::new(fields.len());
        values_builder.build_impl(fields)
    }

    fn new(fields_count: usize) -> Self {
        Self {
            value_id: 0,
            fields_der: proc_macro2::TokenStream::new(),
            field_value_idents: Vec::with_capacity(fields_count),
        }
    }

    fn build_impl<'r>(
        mut self,
        fields: impl ExactSizeIterator<Item = &'r AstRecordField>,
    ) -> FieldValuesOutcome {
        for (id, ast_field) in fields.enumerate() {
            let field_value_ident = new_ident!(format!("field_{}", id));
            let field_der = self.field_der(ast_field, &field_value_ident);

            self.field_value_idents.push(field_value_ident);
            self.fields_der.extend(field_der);
        }

        let outcome = FieldValuesOutcome {
            fields_der: self.fields_der,
            field_value_idents: self.field_value_idents,
        };

        outcome
    }

    fn field_der(
        &mut self,
        ast_field: &AstRecordField,
        field: &syn::Ident,
    ) -> proc_macro2::TokenStream {
        let value_id = self.value_id;
        let der = match &ast_field.ty {
            ParsedType::Boolean(_) => quote! { let #field = raw_record[#value_id] != 0; },
            ParsedType::I8(_) => quote! { let #field = raw_record[#value_id] as i8; },
            ParsedType::I16(_) => quote! { let #field = raw_record[#value_id] as i16; },
            ParsedType::I32(_) => quote! { let #field = raw_record[#value_id] as i32; },
            ParsedType::I64(_) => quote! { let #field = raw_record[#value_id] as i64; },
            ParsedType::U8(_) => quote! { let #field = raw_record[#value_id] as u8; },
            ParsedType::U16(_) => quote! { let #field = raw_record[#value_id] as u16; },
            ParsedType::U32(_) => quote! { let #field = raw_record[#value_id] as u32; },
            ParsedType::U64(_) => quote! { let #field = raw_record[#value_id] as u64; },
            ParsedType::F32(_) => quote! { let #field = raw_record[#value_id] as f32; },
            ParsedType::F64(_) => quote! { let #field = f64::from_bits(raw_record[#value_id]); },
            ParsedType::Utf8Str(_) | ParsedType::Utf8String(_) => self.string_der(field),
            ParsedType::Vector(ty, _) => self.vector_der(ty, field),
            ParsedType::Record(name, _) => self.record_der(name, field),
        };

        self.value_id += 1;

        der
    }

    fn string_der(&mut self, field: &syn::Ident) -> proc_macro2::TokenStream {
        let ptr_id = self.value_id;
        let size_id = self.value_id + 1;
        self.value_id += 1;

        quote! {
            let #field = unsafe { String::from_raw_parts(raw_record[#ptr_id] as _, raw_record[#size_id] as _, raw_record[#size_id] as _) };
        }
    }

    fn vector_der(&mut self, ty: &ParsedType, field: &syn::Ident) -> proc_macro2::TokenStream {
        let generated_der_name = format!("__fce_generated_vec_deserializer_{}", self.value_id);
        let generated_der_name = crate::utils::prepare_ident(generated_der_name);
        let generated_der_ident = new_ident!(generated_der_name);

        let vector_deserializer =
            crate::parsed_type::generate_vector_deserializer(ty, &generated_der_name);

        let ptr_id = self.value_id;
        let size_id = self.value_id + 1;
        self.value_id += 1;

        quote! {
            #vector_deserializer
            let #field = unsafe { #generated_der_ident(raw_record[#ptr_id] as _, raw_record[#size_id] as _) };
        }
    }

    fn record_der(&mut self, name: &str, field: &syn::Ident) -> proc_macro2::TokenStream {
        let ptr_id = self.value_id;
        let record_ident = new_ident!(name);

        quote! {
            let #field = #record_ident::__fce_generated_deserialize(raw_record[#ptr_id] as _);
        }
    }
}

fn field_ctors_from_named<'a, 'v>(
    ast_fields: impl ExactSizeIterator<Item = &'a AstRecordField>,
    field_values: impl ExactSizeIterator<Item = &'v syn::Ident>,
) -> proc_macro2::TokenStream {
    let field_names = ast_fields
        .map(|ast_field| {
            new_ident!(ast_field
                .name
                .as_ref()
                .expect("all fields should have name"))
        })
        .collect::<Vec<_>>();

    quote! {
        Self {
            #(#field_names: #field_values),*
        }
    }
}

fn field_ctor_from_unnamed<'v>(
    field_values: impl ExactSizeIterator<Item = &'v syn::Ident>,
) -> proc_macro2::TokenStream {
    quote! {
        Self {
            #(#field_values),*
        }
    }
}
