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

use marine_macro_impl::marine;

use marine_macro_testing_utils::{items_from_file, stream_from_file, to_syn_item};

use std::path::Path;

pub fn test_marine_token_streams<FP, EP>(marine_path: FP, expanded_path: EP) -> bool
where
    FP: AsRef<Path>,
    EP: AsRef<Path>,
{
    let marine_item = stream_from_file(marine_path);
    let test_token_stream = quote::quote! { #marine_item };
    let marine_token_streams = marine(test_token_stream)
        .unwrap_or_else(|e| panic!("failed to apply the marine macro due {}", e));

    let expanded_item = items_from_file(expanded_path);
    let marine_item = to_syn_item(marine_token_streams);

    marine_item == expanded_item
}
