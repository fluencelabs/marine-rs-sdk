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

use super::ParseMacroInput;
use crate::ast_types;
use crate::ast_types::AstRecordField;
use crate::ast_types::AstRecordFields;
use crate::ast_types::MarineAst;
use crate::syn_error;
use crate::parsed_type::ParsedType;

use syn::Result;
use syn::spanned::Spanned;

impl ParseMacroInput for syn::ItemStruct {
    fn parse_macro_input(self) -> Result<MarineAst> {
        check_record(&self)?;

        let fields = match &self.fields {
            syn::Fields::Named(named_fields) => &named_fields.named,
            _ => return syn_error!(self.span(), "only named fields are allowed in structs"),
        };

        let fields = fields_into_ast(fields, &self.ident)?;
        let fields = AstRecordFields::Named(fields);

        let name = self.ident.to_string();
        let ast_record_item = ast_types::AstRecord {
            name,
            fields,
            original: self,
        };
        let ast_record_item = MarineAst::Record(Box::new(ast_record_item));

        Ok(ast_record_item)
    }
}

fn check_record(record: &syn::ItemStruct) -> Result<()> {
    if record.generics.lt_token.is_some()
        || record.generics.gt_token.is_some()
        || record.generics.where_clause.is_some()
    {
        return syn_error!(
            record.span(),
            "#[marine] couldn't be applied to a struct with generics or lifetimes"
        );
    }

    Ok(())
}

fn fields_into_ast(
    fields: &syn::punctuated::Punctuated<syn::Field, syn::Token![,]>,
    record_ident: &syn::Ident,
) -> Result<Vec<AstRecordField>> {
    fields
        .iter()
        .map(|field| {
            maybe_warn_about_non_doc_attributes(field, record_ident);

            let name = field.ident.as_ref().map(|ident| {
                ident
                    .to_string()
                    .split(' ')
                    .last()
                    .unwrap_or_default()
                    .to_string()
            });
            let ty = ParsedType::from_type(&field.ty)?;

            let record_field = AstRecordField { name, ty };
            Ok(record_field)
        })
        .collect::<Result<Vec<_>>>()
}

/// Prints an error if a field has an any attribute except doc.
fn maybe_warn_about_non_doc_attributes(field: &syn::Field, record_ident: &syn::Ident) {
    for attr in field.attrs.iter() {
        match attr.parse_meta() {
            Ok(meta) if is_doc_attribute(&meta) => continue,
            _ => {}
        }

        // TODO: print message with a span when diagnostic API stabilized
        // https://github.com/rust-lang/rust/issues/54140
        match &field.ident {
            Some(ident) => eprintln!(
                r#"warning: field "{}" of struct "{}" has an attribute which could cause compatibility issues"#,
                ident, record_ident
            ),
            None => eprintln!(
                r#"warning: field of struct "{}" has an attribute which could cause compatibility issues"#,
                record_ident
            ),
        };
    }
}

fn is_doc_attribute(meta: &syn::Meta) -> bool {
    const DOC_ATTR_NAME: &str = "doc";

    meta.path().is_ident(DOC_ATTR_NAME)
}
