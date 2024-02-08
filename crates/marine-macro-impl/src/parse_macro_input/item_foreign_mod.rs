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
use crate::ast_types::MarineAst;
use crate::syn_error;

use syn::Result;
use syn::spanned::Spanned;

const LINK_NAME_DIRECTIVE_NAME: &str = "link_name";
const HOST_IMPORT_DIRECTIVE_NAME: &str = "host_import";
const MODULE_IMPORT_DIRECTIVE_NAME: &str = "module_import";
const HOST_IMPORT_NAMESPACE: &str = "__marine_host_api_v1";

impl ParseMacroInput for syn::ItemForeignMod {
    fn parse_macro_input(self) -> Result<MarineAst> {
        check_foreign_section(&self)?;

        let wasm_import_module = parse_wasm_import_module(&self);
        let namespace = try_extract_namespace(wasm_import_module, &self)?;

        let imports = extract_import_functions(&self)?;
        check_imports(imports.iter().zip(self.items.iter().map(|i| i.span())))?;

        let extern_mod_item = ast_types::AstExternMod { namespace, imports };
        Ok(MarineAst::ExternMod(extern_mod_item))
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
/// #[module_import("module_name")] or #[host_import]
fn parse_wasm_import_module(foreign_mod: &syn::ItemForeignMod) -> Vec<String> {
    foreign_mod
        .attrs
        .iter()
        .filter_map(|attr| attr.parse_meta().ok())
        .filter_map(|meta| {
            if meta.path().is_ident(HOST_IMPORT_DIRECTIVE_NAME) {
                Some(HOST_IMPORT_NAMESPACE.to_string())
            } else if meta.path().is_ident(MODULE_IMPORT_DIRECTIVE_NAME) {
                parse_module_import_directive(meta)
            } else {
                None
            }
        })
        .collect()
}

fn parse_module_import_directive(meta: syn::Meta) -> Option<String> {
    let nested_meta = match meta {
        syn::Meta::List(mut meta_list) if meta_list.nested.len() == 1 => {
            Some(meta_list.nested.pop().unwrap())
        }
        _ => None,
    }?;

    match nested_meta.value() {
        syn::NestedMeta::Lit(syn::Lit::Str(lit)) => Some(lit.value()),
        _ => None,
    }
}

fn try_extract_namespace(
    mut attr: Vec<String>,
    foreign_mod: &syn::ItemForeignMod,
) -> Result<String> {
    if attr.len() == 0 {
        return syn_error!(
            foreign_mod.span(),
            "import module name should be defined by either '#[host_import]' or '#[module_import(\"module_name\")]' attributes"
        );
    }

    if attr.len() > 1 {
        return syn_error!(
            foreign_mod.span(),
            "only one of '#[host_import]' or '#[module_import(\"module_name\")]' attributes is allowed"
        );
    }

    let namespace = attr
        .pop()
        .expect("length of attribute vec should be checked before");

    if namespace.is_empty() {
        return syn_error!(
            foreign_mod.span(),
            "#[module_import(\"module_name\")] attribute should have a non-empty module name"
        );
    }

    Ok(namespace)
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
                "#[marine] could be applied only to a function, struct ot extern block"
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
