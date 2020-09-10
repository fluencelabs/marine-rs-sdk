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
        try_to_ast_signature(self.sig.clone(), self.vis.clone()).map(|signature| {
            FCEAst::Function(AstFunctionItem {
                signature,
                original: Some(self),
            })
        })
    }
}

pub(super) fn try_to_ast_signature(
    signature: syn::Signature,
    visibility: syn::Visibility,
) -> Result<fce_ast_types::AstFunctionSignature> {
    use crate::parsed_type::ParsedType;
    use syn::spanned::Spanned;
    use quote::ToTokens;

    check_function(&signature)?;

    let syn::Signature { inputs, output, .. } = signature;

    let arguments = inputs
        .iter()
        .map(|arg| -> Result<(String, ParsedType)> {
            let pat = match arg {
                syn::FnArg::Typed(arg) => arg,
                _ => {
                    return Err(syn::Error::new(
                        arg.span(),
                        "`self` argument types aren't supported",
                    ))
                }
            };
            Ok((
                pat.pat
                    .to_token_stream()
                    .to_string()
                    .split(' ')
                    .last()
                    .unwrap_or_default()
                    .to_string(),
                ParsedType::from_type(pat.ty.as_ref())?,
            ))
        })
        .collect::<Result<Vec<(_, _)>>>()?;

    let output_type = ParsedType::from_return_type(&output)?;

    let ast_function_item = fce_ast_types::AstFunctionSignature {
        visibility: Some(visibility),
        name: signature.ident.to_string(),
        arguments,
        output_type,
    };

    Ok(ast_function_item)
}

/// Check whether the #[fce] macro could be applied to a function.
fn check_function(signature: &syn::Signature) -> Result<()> {
    use syn::Error;
    use syn::spanned::Spanned;

    let syn::Signature {
        constness,
        unsafety,
        abi,
        variadic,
        generics,
        ..
    } = signature;

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
            signature.span(),
            "FCE export function shouldn't use template parameters",
        ));
    }
    if variadic.is_some() {
        return Err(Error::new(
            variadic.span(),
            "FCE export function shouldn't use variadic interface",
        ));
    }

    // TODO: check for a lifetime
    Ok(())
}
