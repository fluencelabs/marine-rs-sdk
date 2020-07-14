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

use super::ParseMacroInput;
use crate::fce_ast_types;
use crate::fce_ast_types::FCEAst;

use syn::Error;
use syn::Result;
use syn::spanned::Spanned;

const LINK_DIRECTIVE_NAME: &str = "link";
const LINK_NAME_DIRECTIVE_NAME: &str = "link_name";
const WASM_IMPORT_MODULE_DIRECTIVE_NAME: &str = "wasm_import_module";

impl ParseMacroInput for syn::ItemForeignMod {
    fn parse_macro_input(self) -> Result<FCEAst> {
        match &self.abi.name {
            Some(name) if name.value() != "C" => {
                return Err(Error::new(self.span(), "only 'C' abi is allowed"))
            }
            _ => {}
        };

        let self_span = self.span();

        let imports = self
            .items
            .iter()
            .cloned()
            .map(parse_raw_foreign_item)
            .collect::<Result<_>>()?;

        // try to find and parse wasm module name from
        //   #[link(wasm_import_module = "host")]
        let wasm_import_module: Option<String> = self
            .attrs
            .iter()
            .filter_map(|attr| attr.parse_meta().ok())
            .filter(|meta| meta.path().is_ident(LINK_DIRECTIVE_NAME))
            .filter_map(|meta| {
                let pair = match meta {
                    syn::Meta::List(mut meta_list) if meta_list.nested.len() == 1 => {
                        meta_list.nested.pop().unwrap()
                    }
                    _ => return None,
                };
                Some(pair.into_tuple().0)
            })
            .filter_map(|nested| match nested {
                syn::NestedMeta::Meta(meta) => Some(meta),
                _ => None,
            })
            .filter(|meta| meta.path().is_ident(WASM_IMPORT_MODULE_DIRECTIVE_NAME))
            .map(extract_value)
            .collect();

        match wasm_import_module {
            Some(namespace) if namespace.is_empty() => Err(Error::new(
                self_span,
                "import module name should be defined by 'wasm_import_module' directive",
            )),
            Some(namespace) => {
                let extern_mod_item = fce_ast_types::AstExternModItem {
                    namespace,
                    imports,
                    original: Some(self),
                };
                Ok(FCEAst::ExternMod(extern_mod_item))
            }
            None => Err(Error::new(
                self_span,
                "import module name should be defined by 'wasm_import_module' directive",
            )),
        }
    }
}

fn parse_raw_foreign_item(raw_item: syn::ForeignItem) -> Result<fce_ast_types::AstExternFnItem> {
    let function_item = match raw_item {
        syn::ForeignItem::Fn(function_item) => function_item,
        _ => {
            return Err(Error::new(
                raw_item.span(),
                "#[fce] could be applied only to a function, struct ot extern block",
            ))
        }
    };

    // parse the link_name attribute
    //  #[link_name = "put"]
    //  fn ipfs_put(ptr: i32, size: i32);
    let link_name: Option<String> = function_item
        .attrs
        .iter()
        .filter_map(|attr| attr.parse_meta().ok())
        .filter(|meta| meta.path().is_ident(LINK_NAME_DIRECTIVE_NAME))
        .map(extract_value)
        .collect();

    let link_name = match link_name {
        Some(name) if name.is_empty() => None,
        v @ Some(_) => v,
        None => None,
    };

    let function = super::item_fn::parse_function(function_item.sig, function_item.vis)?;
    let ast_extern_fn_item = fce_ast_types::AstExternFnItem {
        link_name,
        signature: function,
    };

    Ok(ast_extern_fn_item)
}

fn extract_value(nested_meta: syn::Meta) -> Option<String> {
    match nested_meta {
        syn::Meta::NameValue(name_value) => match name_value.lit {
            syn::Lit::Str(str) => Some(str.value()),
            _ => None,
        },
        _ => None,
    }
}
