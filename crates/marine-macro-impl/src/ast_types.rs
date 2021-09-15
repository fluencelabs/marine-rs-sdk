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
    Function(AstFn),
    ExternMod(AstExternMod),
    Record(AstRecord),
}
