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

use crate::parsed_type::ParsedType;

use serde::Serialize;
use serde::Deserialize;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct FnArgument {
    pub name: String,
    pub ty: ParsedType,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct FnSignature {
    pub name: String,
    pub arguments: Vec<FnArgument>,
    pub output_types: Vec<ParsedType>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct RecordType {
    pub name: String,
    pub fields: RecordFields,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum RecordFields {
    Named(Vec<RecordField>),
    // named and unnamed variants have the same inner field types because of it's easy to handle it,
    // for additional info look at https://github.com/dtolnay/syn/issues/698
    Unnamed(Vec<RecordField>),
    Unit,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RecordField {
    // fields of tuple structs haven't got name
    pub name: Option<String>,
    pub ty: ParsedType,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ExternFnType {
    pub link_name: Option<String>,
    // only imports are possible here
    pub signature: FnSignature,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ExternModType {
    pub namespace: String,
    // only imports are possible here
    pub imports: Vec<ExternFnType>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct FnType {
    pub signature: FnSignature,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(tag = "ast_type")]
pub enum SDKAst {
    Function(FnType),
    ExternMod(ExternModType),
    Record(RecordType),
}

use crate::ast_types::{
    AstFn, AstFnSignature, AstFnArgument, AstExternMod, AstExternFn, AstRecordField, AstRecord,
    AstRecordFields,
};

impl From<AstFn> for SDKAst {
    fn from(ast_fn_item: AstFn) -> Self {
        let fn_item = ast_fn_item.into();
        Self::Function(fn_item)
    }
}

impl From<AstExternMod> for SDKAst {
    fn from(ast_extern_mod: AstExternMod) -> Self {
        let extern_mod = ast_extern_mod.into();
        Self::ExternMod(extern_mod)
    }
}

impl From<AstRecord> for SDKAst {
    fn from(ast_record_item: AstRecord) -> Self {
        let record_item = ast_record_item.into();
        Self::Record(record_item)
    }
}

impl From<AstFn> for FnType {
    fn from(ast_fn_item: AstFn) -> Self {
        let signature = ast_fn_item.signature.into();

        Self { signature }
    }
}

impl From<AstExternMod> for ExternModType {
    fn from(ast_extern_mod: AstExternMod) -> Self {
        let imports = ast_extern_mod.imports.into_iter().map(Into::into).collect();

        Self {
            namespace: ast_extern_mod.namespace,
            imports,
        }
    }
}

impl From<AstRecord> for RecordType {
    fn from(ast_record_item: AstRecord) -> Self {
        Self {
            name: ast_record_item.name,
            fields: ast_record_item.fields.into(),
        }
    }
}

impl From<AstRecordFields> for RecordFields {
    fn from(ast_record_item: AstRecordFields) -> Self {
        match ast_record_item {
            AstRecordFields::Named(fields) => {
                let fields = fields.into_iter().map(Into::into).collect();
                Self::Named(fields)
            }
            AstRecordFields::Unnamed(fields) => {
                let fields = fields.into_iter().map(Into::into).collect();
                Self::Unnamed(fields)
            }
            AstRecordFields::Unit => Self::Unit,
        }
    }
}

impl From<AstFnSignature> for FnSignature {
    fn from(ast_fn_sig: AstFnSignature) -> Self {
        // TODO: consider to do transmute here in case of optimization issues.
        let arguments = ast_fn_sig.arguments.into_iter().map(Into::into).collect();
        let output_type = match ast_fn_sig.output_type {
            Some(output_type) => vec![output_type],
            None => Vec::new(),
        };

        Self {
            name: ast_fn_sig.name,
            arguments,
            output_types: output_type,
        }
    }
}

impl From<AstFnArgument> for FnArgument {
    fn from(ast_fn_arg: AstFnArgument) -> Self {
        Self {
            name: ast_fn_arg.name,
            ty: ast_fn_arg.ty,
        }
    }
}

impl From<AstExternFn> for ExternFnType {
    fn from(ast_extern_item: AstExternFn) -> Self {
        Self {
            link_name: ast_extern_item.link_name,
            signature: ast_extern_item.signature.into(),
        }
    }
}

impl From<AstRecordField> for RecordField {
    fn from(ast_record_field: AstRecordField) -> Self {
        Self {
            name: ast_record_field.name,
            ty: ast_record_field.ty,
        }
    }
}
