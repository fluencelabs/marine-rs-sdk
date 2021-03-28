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

use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::quote;

pub(super) fn fce_test_impl(attrs: TokenStream, func_input: syn::ItemFn) -> TokenStream {
    use darling::FromMeta;

    let attrs = syn::parse_macro_input!(attrs as syn::AttributeArgs);
    let attrs = match FCETestAttributes::from_list(&attrs) {
        Ok(v) => v,
        Err(e) => {
            return TokenStream::from(e.write_errors());
        }
    };

    generate_test_glue_code(func_input, &attrs.config_path).into()
}

fn generate_test_glue_code(func: syn::ItemFn, config_path: &str) -> TokenStream2 {
    let fce_ctor = generate_fce_ctor(config_path);
    let original_block = func.block;
    let signature = func.sig;

    quote! {
        #[test]
        #signature {
            #fce_ctor

            #original_block
        }
    }
}

fn generate_fce_ctor(config_path: &str) -> TokenStream2 {
    let config_path = quote! { #config_path };

    let tmp_file_path = std::env::temp_dir();
    let random_uuid = uuid::Uuid::new_v4().to_string();
    let service_id = quote! { #random_uuid };

    let tmp_file_path = tmp_file_path.join(random_uuid);
    let tmp_file_path = tmp_file_path.to_string_lossy().to_string();
    let tmp_file_path = quote! { #tmp_file_path };

    quote! {
        let mut __fce__generated_fce_config = fluence_test::internal::TomlAppServiceConfig::load(#config_path.to_string())
            .unwrap_or_else(|e| panic!("app service located at `{}` config can't be loaded: {}", #config_path, e));
        __fce__generated_fce_config.service_base_dir = Some(#tmp_file_path.to_string());

        let mut fce = fluence_test::internal::AppService::new_with_empty_facade(__fce__generated_fce_config, #service_id, std::collections::HashMap::new())
            .unwrap_or_else(|e| panic!("app service can't be created: {}", e));
    }
}
