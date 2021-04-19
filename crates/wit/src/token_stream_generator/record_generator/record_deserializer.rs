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
use crate::ast_types::*;
use super::FieldValuesBuilder;

use proc_macro2::TokenStream;
use quote::quote;

#[derive(Default)]
pub(super) struct RecordDerDescriptor {
    pub(super) fields_der: TokenStream,
    pub(super) record_ctor: TokenStream,
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

fn field_ctors_from_named<'a, 'v>(
    ast_fields: impl ExactSizeIterator<Item = &'a AstRecordField>,
    field_values: impl ExactSizeIterator<Item = &'v syn::Ident>,
) -> TokenStream {
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
) -> TokenStream {
    quote! {
        Self {
            #(#field_values),*
        }
    }
}
