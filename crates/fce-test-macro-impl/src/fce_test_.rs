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

use crate::attributes::FCETestAttributes;
use crate::TResult;
use crate::TestGeneratorError;

use fluence_app_service::TomlAppServiceConfig;
use proc_macro2::TokenStream;
use quote::quote;
use quote::ToTokens;



use fce_wit_parser::module_raw_interface;
use fce_wit_parser::interface::FCEModuleInterface;
use fce_wit_parser::interface::FCERecordTypes;
use fce_wit_parser::interface::FCEFunctionSignature;
use fce_wit_parser::interface::it::IFunctionArg;
use fce_wit_parser::interface::it::IRecordFieldType;
use fce_wit_parser::interface::it::IType;

use std::path::PathBuf;
use syn::parse::Parser;




