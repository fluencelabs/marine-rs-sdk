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
