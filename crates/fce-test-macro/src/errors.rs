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

#[derive(Debug, ThisError)]
pub(crate) enum TestGeneratorError {
    #[error("{0}")]
    WITParserError(#[from] WITParserError),

    #[error("{0}")]
    CorruptedITSection(#[from] CorruptedITSection),

    #[error("{0}")]
    SynError(#[from] SynError),

    #[error("{0}")]
    ConfigLoadError(#[from] AppServiceError),

    #[error("{0}")]
    AttributesError(#[from] DarlingError),
}

#[derive(Debug, ThisError)]
pub(crate) enum CorruptedITSection {
    #[error("record with {0} is absent in embedded IT section")]
    AbsentRecord(u64),
}

use proc_macro2::TokenStream;

impl quote::ToTokens for TestGeneratorError {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let error_as_text = format!("Error was encountered inside fce_test: {}", self);
        error_as_text.to_tokens(tokens);
    }
}
