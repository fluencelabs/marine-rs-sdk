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

use crate::fce_ast_types;

impl quote::ToTokens for fce_ast_types::AstRecordItem {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let original = &self.original;
        crate::prepare_global_data!(
            Record,
            self,
            self.name,
            data,
            data_size,
            global_static_name,
            section_name
        );

        let glue_code = quote::quote! {
            #original

            // #[cfg(target_arch = "wasm32")]
            #[doc(hidden)]
            #[allow(clippy::all)]
            #[link_section = #section_name]
            pub static #global_static_name: [u8; #data_size] = { *#data };

        };

        tokens.extend(glue_code);
    }
}
