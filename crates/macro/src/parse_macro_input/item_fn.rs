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
use crate::fce_ast_types::{FCEAst, AstFunctionItem};

use syn::Result;

impl ParseMacroInput for syn::ItemFn {
    fn parse_macro_input(self) -> Result<FCEAst> {
        parse_function(self.sig.clone(), self.vis.clone()).map(|f| {
            FCEAst::Function(AstFunctionItem {
                signature: f,
                original: Some(self),
            })
        })
    }
}

pub(super) fn parse_function(
    function_sig: syn::Signature,
    function_vis: syn::Visibility,
) -> Result<fce_ast_types::AstFunctionSignature> {
    use crate::parsed_type::ParsedType;

    check_func(&function_sig, function_vis)?;

    let syn::Signature { inputs, output, .. } = function_sig;

    let input_types = inputs
        .iter()
        .map(ParsedType::from_fn_arg)
        .collect::<Result<Vec<_>>>()?;

    let output_type = ParsedType::from_return_type(&output)?;

    let ast_function_item = fce_ast_types::AstFunctionSignature {
        name: function_sig.ident.to_string(),
        input_types,
        output_type,
    };

    Ok(ast_function_item)
}

/// Check whether the #[fce] macro could be applied to a function.
fn check_func(function_sig: &syn::Signature, function_vis: syn::Visibility) -> Result<()> {
    use syn::Error;
    use syn::spanned::Spanned;

    let syn::Signature {
        constness,
        unsafety,
        abi,
        variadic,
        generics,
        ..
    } = function_sig;

    if let Some(constness) = constness {
        return Err(Error::new(
            constness.span,
            "FCE export function shouldn't be constant",
        ));
    }
    if let Some(unsafety) = unsafety {
        return Err(Error::new(
            unsafety.span,
            "FCE export function shouldn't be unsafe",
        ));
    }
    if let Some(abi) = abi {
        return Err(Error::new(
            abi.extern_token.span,
            "FCE export function shouldn't have any custom linkage",
        ));
    }
    if !generics.params.is_empty() || generics.where_clause.is_some() {
        return Err(Error::new(
            function_sig.span(),
            "FCE export function shouldn't use template parameters",
        ));
    }
    if let Some(_) = variadic {
        return Err(Error::new(
            variadic.span(),
            "FCE export function shouldn't use variadic interface",
        ));
    }

    // TODO: check for a lifetime

    match function_vis {
        syn::Visibility::Public(_) => Ok(()),
        _ => Err(Error::new(
            variadic.span(),
            "FCE export function should be public",
        )),
    }
}
