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

use fce_wit_parser::WITParserError;
use fluence_app_service::AppServiceError;

use darling::Error as DarlingError;
use syn::Error as SynError;
use thiserror::Error as ThisError;

use std::path::PathBuf;

#[derive(Debug, ThisError)]
pub enum TestGeneratorError {
    #[error("Can't load Wasm modules into FCE: {0}")]
    WITParserError(#[from] WITParserError),

    #[error("{0}")]
    CorruptedITSection(#[from] CorruptedITSection),

    #[error("{0}")]
    SynError(#[from] SynError),

    #[error("Can't load Wasm modules from the provided config: {0}")]
    ConfigLoadError(#[from] AppServiceError),

    #[error("{0}")]
    AttributesError(#[from] DarlingError),

    #[error(
        "neither modules_dir attribute specified nor service config contains modules_dir, please specify one of them"
    )]
    ModulesDirUnspecified,

    #[error("a Wasm file compiled with newer version of sdk that supports multi-value")]
    ManyFnOutputsUnsupported,

    #[error("{0} is invalid UTF8 path")]
    InvalidUTF8Path(PathBuf),
}

#[derive(Debug, ThisError)]
pub enum CorruptedITSection {
    #[error("record with {0} is absent in embedded IT section")]
    AbsentRecord(u64),
}
