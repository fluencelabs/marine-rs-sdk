/*
 * Fluence Marine Rust SDK
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

impl RecordDerGlueCodeGenerator for AstRecord {
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
