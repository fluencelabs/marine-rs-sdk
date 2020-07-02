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
pub(crate) struct AstFunctionItem {
    pub(crate) name: String,
    pub(crate) input_types: Vec<ParsedType>,
    // fce supports only one return value now,
    // waiting for adding multi-value support in Wasmer.
    pub(crate) output_type: ParsedType,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub(crate) struct AstRecordItem {
    pub(crate) fields: Vec<ParsedType>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub(crate) struct AstExternModItem {
    pub(crate) namespace: String,
    // only imports are possible here
    pub(crate) imports: Vec<AstFunctionItem>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub(crate) enum FCEAst {
    Function(AstFunctionItem),
    Record(AstRecordItem),
    ExternMod(AstExternModItem),
}
