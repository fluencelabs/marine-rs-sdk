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

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AstFunctionSignature {
    pub name: String,
    pub input_types: Vec<ParsedType>,
    // fce supports only one return value now,
    // waiting for adding multi-value support in Wasmer.
    pub output_type: ParsedType,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AstRecordItem {
    pub fields: Vec<ParsedType>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AstExternFnItem {
    pub link_name: Option<String>,
    // only imports are possible here
    pub signature: AstFunctionSignature,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AstExternModItem {
    pub namespace: String,

    // only imports are possible here
    pub imports: Vec<AstExternFnItem>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct AstFunctionItem {
    pub signature: AstFunctionSignature,

    // Option is needed only for skipping serialization/deserialization of syn::ItemFn
    #[serde(skip)]
    pub original: Option<syn::ItemFn>,
}

#[derive(Clone, Serialize, Deserialize)]
#[serde(tag = "ast_type")]
pub enum FCEAst {
    Function(AstFunctionItem),
    ExternMod(AstExternModItem),
    Record(AstRecordItem),
}
