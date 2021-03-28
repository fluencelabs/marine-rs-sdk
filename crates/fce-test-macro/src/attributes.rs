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

use syn::parse::Parse;
use syn::parse::ParseStream;

#[derive(Debug, Default, Clone)]
pub(crate) struct FCETestAttributes {
    pub(crate) config_path: String,
}

impl Parse for FCETestAttributes {
    fn parse(input: ParseStream<'_>) -> syn::Result<Self> {
        let config_file_path = parse_config_file_path(input)?;
        let attributes = FCETestAttributes {
            config_path: config_file_path,
        };

        Ok(attributes)
    }
}

pub(crate) fn parse_config_file_path(token_stream: ParseStream<'_>) -> syn::Result<String> {
    let attr_name = token_stream.step(|cursor| match cursor.ident() {
        Some((ident, rem)) => Ok((ident, rem)),
        None => Err(cursor.error("Expected a valid identifier")),
    })?;

    match attr_name.to_string().as_str() {
        "config" => {
            // trying to parse `=`
            token_stream.parse::<syn::token::Eq>()?;

            match token_stream.parse::<syn::Ident>() {
                Ok(config_file_path) => Ok(config_file_path.to_string()),
                Err(e) => Err(syn::Error::new(
                    attr_name.span(),
                    format!("failed to parse a config file path: {}", e),
                )),
            }
        }

        attr => Err(syn::Error::new(
            attr_name.span(),
            format!("Expected 'config' identifier, but {} found", attr),
        )),
    }
}
