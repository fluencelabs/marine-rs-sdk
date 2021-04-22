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

use crate::parsed_type::ParsedType;

use serde::Serialize;
use serde::Deserialize;

#[derive(Clone, Serialize, Deserialize)]
pub struct FnArgument {
    pub name: String,
    pub ty: ParsedType,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct FnSignature {
    pub name: String,
    pub arguments: Vec<FnArgument>,
    pub output_types: Vec<ParsedType>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct RecordItem {
    pub name: String,
    pub fields: RecordFields,
}

#[derive(Clone, Serialize, Deserialize)]
pub enum RecordFields {
    Named(Vec<RecordField>),
    // named and unnamed variants have the same inner field types because of it's easy to handle it,
    // for additional info look at https://github.com/dtolnay/syn/issues/698
    Unnamed(Vec<RecordField>),
    Unit,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RecordField {
    // fields of tuple structs haven't got name
    pub name: Option<String>,
    pub ty: ParsedType,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct ExternFnItem {
    pub link_name: Option<String>,
    // only imports are possible here
    pub signature: FnSignature,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct ExternModItem {
    pub namespace: String,
    // only imports are possible here
    pub imports: Vec<ExternFnItem>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct FnItem {
    pub signature: FnSignature,
}

#[derive(Clone, Serialize, Deserialize)]
#[serde(tag = "ast_type")]
pub enum SDKAst {
    Function(FnItem),
    ExternMod(ExternModItem),
    Record(RecordItem),
}

use crate::ast_types::{
    AstFnItem, AstFnSignature, AstFnArgument, AstExternModItem, AstExternFnItem, AstRecordField,
    AstRecordItem, AstRecordFields,
};

impl From<AstFnItem> for SDKAst {
    fn from(ast_fn_item: AstFnItem) -> Self {
        let fn_item = ast_fn_item.into();
        Self::Function(fn_item)
    }
}

impl From<AstExternModItem> for SDKAst {
    fn from(ast_extern_mod: AstExternModItem) -> Self {
        let extern_mod = ast_extern_mod.into();
        Self::ExternMod(extern_mod)
    }
}

impl From<AstRecordItem> for SDKAst {
    fn from(ast_record_item: AstRecordItem) -> Self {
        let record_item = ast_record_item.into();
        Self::Record(record_item)
    }
}

impl From<AstFnItem> for FnItem {
    fn from(ast_fn_item: AstFnItem) -> Self {
        let signature = ast_fn_item.signature.into();

        Self { signature }
    }
}

impl From<AstExternModItem> for ExternModItem {
    fn from(ast_extern_mod: AstExternModItem) -> Self {
        let imports = ast_extern_mod.imports.into_iter().map(Into::into).collect();

        Self {
            namespace: ast_extern_mod.namespace,
            imports,
        }
    }
}

impl From<AstRecordItem> for RecordItem {
    fn from(ast_record_item: AstRecordItem) -> Self {
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

impl From<AstExternFnItem> for ExternFnItem {
    fn from(ast_extern_item: AstExternFnItem) -> Self {
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
