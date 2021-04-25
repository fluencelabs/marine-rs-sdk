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
use crate::ast_types;
use crate::ast_types::FCEAst;
use crate::syn_error;

use syn::Result;
use syn::spanned::Spanned;

const LINK_DIRECTIVE_NAME: &str = "link";
const LINK_NAME_DIRECTIVE_NAME: &str = "link_name";
const WASM_IMPORT_MODULE_DIRECTIVE_NAME: &str = "wasm_import_module";

impl ParseMacroInput for syn::ItemForeignMod {
    fn parse_macro_input(self) -> Result<FCEAst> {
        check_foreign_section(&self)?;

        let wasm_import_module: Option<String> = parse_wasm_import_module(&self);
        let namespace = try_extract_namespace(wasm_import_module, &self)?;

        let imports = extract_import_functions(&self)?;
        check_imports(imports.iter().zip(self.items.iter().map(|i| i.span())))?;

        let extern_mod_item = ast_types::AstExternMod {
            namespace,
            imports,
            original: self,
        };
        Ok(FCEAst::ExternMod(extern_mod_item))
    }
}

fn check_foreign_section(foreign_mod: &syn::ItemForeignMod) -> Result<()> {
    match &foreign_mod.abi.name {
        Some(name) if name.value() != "C" => {
            syn_error!(foreign_mod.span(), "only 'C' abi is allowed")
        }
        _ => Ok(()),
    }
}

/// Tries to find and parse wasm module name from
///   #[link(wasm_import_module = "host")]
fn parse_wasm_import_module(foreign_mod: &syn::ItemForeignMod) -> Option<String> {
    foreign_mod
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
        .collect()
}

fn try_extract_namespace(
    attr: Option<String>,
    foreign_mod: &syn::ItemForeignMod,
) -> Result<String> {
    match attr {
        Some(namespace) if namespace.is_empty() => syn_error!(
            foreign_mod.span(),
            "import module name should be defined by 'wasm_import_module' directive"
        ),
        Some(namespace) => Ok(namespace),
        None => syn_error!(
            foreign_mod.span(),
            "import module name should be defined by 'wasm_import_module' directive"
        ),
    }
}

fn extract_import_functions(
    foreign_mod: &syn::ItemForeignMod,
) -> Result<Vec<ast_types::AstExternFn>> {
    foreign_mod
        .items
        .iter()
        .cloned()
        .map(parse_raw_foreign_item)
        .collect::<Result<_>>()
}

/// This function checks whether these imports contains inner references. In this case glue
/// code couldn't be generated.
fn check_imports<'i>(
    extern_fns: impl ExactSizeIterator<Item = (&'i ast_types::AstExternFn, proc_macro2::Span)>,
) -> Result<()> {
    use super::utils::contain_inner_ref;

    for (extern_fn, span) in extern_fns {
        if let Some(output_type) = &extern_fn.signature.output_type {
            if contain_inner_ref(output_type) {
                return crate::syn_error!(
                    span,
                    "import function can't return a value with references"
                );
            }
        }
    }

    Ok(())
}

fn parse_raw_foreign_item(raw_item: syn::ForeignItem) -> Result<ast_types::AstExternFn> {
    let function_item = match raw_item {
        syn::ForeignItem::Fn(function_item) => function_item,
        _ => {
            return syn_error!(
                raw_item.span(),
                "#[fce] could be applied only to a function, struct ot extern block"
            )
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

    let signature = super::item_fn::try_to_ast_signature(function_item.sig, function_item.vis)?;
    let ast_extern_fn_item = ast_types::AstExternFn {
        link_name,
        signature,
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
