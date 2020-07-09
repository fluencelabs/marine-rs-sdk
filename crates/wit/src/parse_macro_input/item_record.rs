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

use super::ParseMacroInput;
use crate::{fce_ast_types, AstRecordField};
use crate::fce_ast_types::FCEAst;

use syn::Error;
use syn::Result;
use syn::spanned::Spanned;
use crate::parsed_type::ParsedType;

impl ParseMacroInput for syn::ItemStruct {
    fn parse_macro_input(self) -> Result<FCEAst> {
        check_record(&self)?;

        let fields = match &self.fields {
            syn::Fields::Named(named_fields) => &named_fields.named,
            syn::Fields::Unnamed(unnamed_fields) => &unnamed_fields.unnamed,
            _ => return Err(Error::new(self.span(), "only named field allowed")),
        };

        let fields = fields
            .iter()
            .map(|field| {
                check_field(field)?;
                let field_name = field.ident.as_ref().map(|ident| ident.to_string());
                let field_type = ParsedType::from_type(&field.ty)?;
                Ok(AstRecordField {
                    field_name,
                    field_type,
                })
            })
            .collect::<Result<Vec<_>>>()?;

        let name = self.ident.to_string();
        let ast_record_item = fce_ast_types::AstRecordItem {
            name,
            fields,
            original: Some(self),
        };

        Ok(FCEAst::Record(ast_record_item))
    }
}

fn check_record(record: &syn::ItemStruct) -> Result<()> {
    match record.vis {
        syn::Visibility::Public(_) => {}
        _ => {
            return Err(Error::new(
                record.span(),
                "#[fce] could be applied only to public struct",
            ))
        }
    };

    if record.generics.lt_token.is_some()
        || record.generics.gt_token.is_some()
        || record.generics.where_clause.is_some()
    {
        return Err(Error::new(
            record.span(),
            "#[fce] couldn't be applied to a struct with generics",
        ));
    }

    Ok(())
}

fn check_field(field: &syn::Field) -> Result<()> {
    match field.vis {
        syn::Visibility::Public(_) => {}
        _ => {
            return Err(Error::new(
                field.span(),
                "#[fce] could be applied only to struct with all public fields",
            ))
        }
    };

    if !field.attrs.is_empty() {
        return Err(Error::new(field.span(), "field attributes isn't allowed"));
    }

    Ok(())
}
