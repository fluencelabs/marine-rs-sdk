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

use crate::parsed_type::ParsedType;

#[derive(Clone)]
pub(crate) struct AstFnArgument {
    pub(crate) name: String,
    pub(crate) ty: ParsedType,
}

#[derive(Clone)]
pub(crate) struct AstFnSignature {
    pub(crate) visibility: syn::Visibility,
    pub(crate) name: String,
    pub(crate) arguments: Vec<AstFnArgument>,
    // only one or zero return values are supported now,
    // waiting for adding multi-value support in Wasmer
    pub(crate) output_type: Option<ParsedType>,
}

#[derive(Clone)]
pub(crate) struct AstRecord {
    pub(crate) name: String,
    pub(crate) fields: AstRecordFields,
    pub(crate) original: syn::ItemStruct,
}

#[derive(Debug, Clone, PartialEq)]
pub(crate) enum AstRecordFields {
    Named(Vec<AstRecordField>),

    // named and unnamed variants have the same inner field types because of it's easy to handle it,
    // for additional info look at https://github.com/dtolnay/syn/issues/698
    #[allow(dead_code)] // at the moment tuple and unit structs aren't supported
    Unnamed(Vec<AstRecordField>),

    #[allow(dead_code)] // at the moment tuple and unit structs aren't supported
    Unit,
}

#[derive(Debug, Clone, PartialEq)]
pub(crate) struct AstRecordField {
    /// Name of the field. Can be `None` for tuples.
    pub(crate) name: Option<String>,
    pub(crate) ty: ParsedType,
}

#[derive(Clone)]
pub(crate) struct AstExternFn {
    pub(crate) link_name: Option<String>,
    // only imports are possible here
    pub(crate) signature: AstFnSignature,
}

#[derive(Clone)]
pub(crate) struct AstExternMod {
    pub(crate) namespace: String,
    // only imports are possible here
    pub(crate) imports: Vec<AstExternFn>,
}

#[derive(Clone)]
pub(crate) struct AstFn {
    pub(crate) signature: AstFnSignature,
    pub(crate) original: syn::ItemFn,
}

#[derive(Clone)]
pub(crate) enum MarineAst {
    Function(Box<AstFn>),
    ExternMod(AstExternMod),
    Record(Box<AstRecord>),
}
