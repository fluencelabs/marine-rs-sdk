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
use crate::ParsedType;
use crate::fce_ast_types::FCEAst;
use crate::fce_ast_types::AstFunctionItem;
use crate::fce_ast_types::AstFuncArgument;
use crate::parsed_type::passing_style_of;
use crate::syn_error;

use syn::Result;
use syn::spanned::Spanned;

impl ParseMacroInput for syn::ItemFn {
    fn parse_macro_input(self) -> Result<FCEAst> {
        let signature = try_to_ast_signature(self.sig.clone(), self.vis.clone())?;

        // this check specific only for export functions
        let parsed_args = signature
            .arguments
            .iter()
            .zip(self.sig.inputs.iter().map(|arg| arg.span()));
        check_parsed_functions(parsed_args)?;

        let ast_fn = FCEAst::Function(AstFunctionItem {
            signature,
            original: Some(self),
        });
        Ok(ast_fn)
    }
}

pub(super) fn try_to_ast_signature(
    signature: syn::Signature,
    visibility: syn::Visibility,
) -> Result<fce_ast_types::AstFunctionSignature> {
    use quote::ToTokens;

    check_function(&signature)?;

    let syn::Signature { inputs, output, .. } = signature;

    let arguments = inputs
        .iter()
        .map(|arg| -> Result<_> {
            let pat = match arg {
                syn::FnArg::Typed(arg) => arg,
                _ => {
                    return Err(syn::Error::new(
                        arg.span(),
                        "`self` argument types aren't supported",
                    ))
                }
            };

            let name = pat
                .pat
                .to_token_stream()
                .to_string()
                .split(' ')
                .last()
                .unwrap_or_default()
                .to_string();
            let ty = ParsedType::from_type(pat.ty.as_ref())?;
            let ast_arg = AstFuncArgument { name, ty };

            Ok(ast_arg)
        })
        .collect::<Result<Vec<_>>>()?;

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
#[rustfmt::skip]
fn check_function(signature: &syn::Signature) -> Result<()> {
    let syn::Signature {
        constness,
        unsafety,
        abi,
        variadic,
        generics,
        ..
    } = signature;

    if let Some(constness) = constness {
        return syn_error!(constness.span, "FCE export function shouldn't be constant");
    }
    if let Some(unsafety) = unsafety {
        return syn_error!(unsafety.span, "FCE export function shouldn't be unsafe");
    }
    if let Some(abi) = abi {
        return syn_error!(abi.extern_token.span, "FCE export function shouldn't have any custom linkage");
    }
    if generics.where_clause.is_some() {
        return syn_error!(signature.span(), "FCE export function shouldn't use template parameters");
    }
    if variadic.is_some() {
        return syn_error!(variadic.span(), "FCE export function shouldn't use variadic interface");
    }

    // TODO: check for a lifetime
    Ok(())
}

fn check_parsed_functions<'a>(
    args: impl ExactSizeIterator<Item = (&'a AstFuncArgument, proc_macro2::Span)>,
) -> Result<()> {
    for (arg, span) in args {
        if contains_inner_ref(&arg.ty) {
            return crate::syn_error!(
                span,
                "vector type in export functions should take arguments only by value"
            );
        }
    }

    return Ok(());
}

/// Returns true if the given type is a vector contains a reference inside it's parameter type.
/// F.e.
/// Vec<&String> => true
/// Vec<Vec<&Vec<String>>> => true
/// &Vec<String> => false
fn contains_inner_ref(ty: &ParsedType) -> bool {
    fn contains_inner_ref_impl(ty: &ParsedType) -> bool {
        use crate::parsed_type::PassingStyle;

        match ty {
            ParsedType::Vector(ty, passing_style) => match passing_style {
                PassingStyle::ByValue => contains_inner_ref_impl(ty),
                PassingStyle::ByRef | PassingStyle::ByMutRef => true,
            },
            _ => match passing_style_of(ty) {
                PassingStyle::ByValue => false,
                PassingStyle::ByRef | PassingStyle::ByMutRef => true,
            },
        }
    }

    match ty {
        ParsedType::Vector(ty, _) => contains_inner_ref_impl(ty),
        // Structs are checked while parsing
        _ => false,
    }
}
