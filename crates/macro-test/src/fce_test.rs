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

use proc_macro2::TokenStream;
use quote::quote;

pub(super) fn fce_test_impl(
    attr: TokenStream,
    func_input: syn::ItemFn,
) -> Result<TokenStream, TokenStream> {
    use crate::attributes::FCETestAttributes;

    let attrs = syn::parse2::<FCETestAttributes>(attr).map_err(|e| e.into_compile_error())?;
    let generated_test = generate_test_glue_code(func_input, &attrs.config_path);

    Ok(generated_test)
}

fn generate_test_glue_code(func: syn::ItemFn, config_path: &str) -> TokenStream {
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

fn generate_fce_ctor(config_path: &str) -> TokenStream {
    let config_path = new_ident(config_path);

    let tmp_file_path = std::env::temp_dir();
    let random_uuid = uuid::Uuid::new_v4().to_string();
    let service_id = new_ident(&random_uuid);

    let tmp_file_path = tmp_file_path.join(random_uuid);
    let tmp_file_path = tmp_file_path.to_string_lossy().to_string();
    let tmp_file_path = new_ident(&tmp_file_path);

    quote! {
        let mut __fce__generated_fce_config = fluence_faas::TomlAppServiceConfig::load(#config_path)
            .unwrap_or_else(|e| panic!("app service located at `{}` config can't be loaded: {}", #config_path, e));
        __fce__generated_fce_config.service_base_dir = Some(#tmp_file_path);

        let fce = fce_app_service::AppService::new_with_empty_facade(__fce__generated_fce_config, #service_id, std::collections::HashMap::new())
            .unwrap_or_else(|e| panic!("app service can't be created: {}", e));
    }
}

fn new_ident(name: &str) -> syn::Ident {
    syn::Ident::new(name, proc_macro2::Span::call_site())
}
