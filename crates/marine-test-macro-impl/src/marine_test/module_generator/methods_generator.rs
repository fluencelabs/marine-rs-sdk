/*
 * Copyright 2021 Fluence Labs Limited
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

use super::methods_generator_utils::*;
use crate::TResult;

use marine_it_parser::it_interface::IFunctionSignature;
use marine_it_parser::it_interface::IRecordTypes;

use itertools::Itertools;

pub(super) fn generate_module_methods<'m, 'r>(
    module_name: &str,
    method_signatures: impl ExactSizeIterator<Item = &'m IFunctionSignature>,
    records: &'r IRecordTypes,
) -> TResult<Vec<proc_macro2::TokenStream>> {
    use CallParametersSettings::*;

    let methods_count = 2 * method_signatures.len();
    method_signatures
        .sorted_by(|sig1, sig2| sig1.name.cmp(&sig2.name))
        .try_fold::<_, _, TResult<_>>(
            Vec::with_capacity(methods_count),
            |mut methods, signature| {
                let default_cp = generate_module_method(module_name, &signature, Default, records)?;
                let user_cp =
                    generate_module_method(module_name, &signature, UserDefined, records)?;

                methods.push(default_cp);
                methods.push(user_cp);

                Ok(methods)
            },
        )
}
