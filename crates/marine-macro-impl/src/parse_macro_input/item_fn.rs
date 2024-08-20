/*
 * Marine Rust SDK
 *
 * Copyright (C) 2024 Fluence DAO
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU Affero General Public License as
 * published by the Free Software Foundation version 3 of the
 * License.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU Affero General Public License for more details.
 *
 * You should have received a copy of the GNU Affero General Public License
 * along with this program.  If not, see <https://www.gnu.org/licenses/>.
 */

use super::ParseMacroInput;
use crate::ast_types;
use crate::ParsedType;
use crate::ast_types::MarineAst;
use crate::ast_types::AstFn;
use crate::ast_types::AstFnArgument;
use crate::syn_error;

use syn::Result;
use syn::spanned::Spanned;

impl ParseMacroInput for syn::ItemFn {
    fn parse_macro_input(self) -> Result<MarineAst> {
        let signature = try_to_ast_signature(self.sig.clone(), self.vis.clone())?;

        // this check specific only for export functions
        let parsed_args = signature
            .arguments
            .iter()
            .zip(self.sig.inputs.iter().map(|arg| arg.span()));

        check_args(parsed_args)?;
        check_output_type(&signature.output_type, self.sig.output.span())?;

        let ast_fn = AstFn {
            signature,
            original: self,
        };
        let ast_fn = MarineAst::Function(Box::new(ast_fn));
        Ok(ast_fn)
    }
}

pub(super) fn try_to_ast_signature(
    signature: syn::Signature,
    visibility: syn::Visibility,
) -> Result<ast_types::AstFnSignature> {
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
            let ast_arg = AstFnArgument { name, ty };

            Ok(ast_arg)
        })
        .collect::<Result<Vec<_>>>()?;

    let output_type = ParsedType::from_return_type(&output)?;

    let ast_function_item = ast_types::AstFnSignature {
        visibility,
        name: signature.ident.to_string(),
        arguments,
        output_type,
    };

    Ok(ast_function_item)
}

/// Check whether the #[marine] macro could be applied to a function.
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
        return syn_error!(constness.span, "Marine export function shouldn't be constant");
    }
    if let Some(unsafety) = unsafety {
        return syn_error!(unsafety.span, "Marine export function shouldn't be unsafe");
    }
    if let Some(abi) = abi {
        return syn_error!(abi.extern_token.span, "Marine export function shouldn't have any custom linkage");
    }
    if generics.where_clause.is_some() {
        return syn_error!(signature.span(), "Marine export function shouldn't use template parameters");
    }
    if variadic.is_some() {
        return syn_error!(variadic.span(), "Marine export function shouldn't use variadic interface");
    }

    // TODO: check for a lifetime
    Ok(())
}

fn check_args<'a>(
    args: impl ExactSizeIterator<Item = (&'a AstFnArgument, proc_macro2::Span)>,
) -> Result<()> {
    for (arg, span) in args {
        if contains_inner_ref(&arg.ty) {
            return crate::syn_error!(
                span,
                "a vector type in arguments of export functions shouldn't contain references"
            );
        }
    }

    Ok(())
}

fn check_output_type(output_type: &Option<ParsedType>, span: proc_macro2::Span) -> Result<()> {
    let ty = match output_type {
        Some(ty) => ty,
        None => return Ok(()),
    };

    if contains_inner_ref(ty) {
        return crate::syn_error!(
            span,
            "a vector type in output types of export functions shouldn't contain references"
        );
    }

    Ok(())
}

/// Returns true if the given type is a vector contains a reference inside it's parameter type.
/// F.e.
/// Vec<&String> => true
/// Vec<Vec<&Vec<String>>> => true
/// &Vec<String> => false
fn contains_inner_ref(ty: &ParsedType) -> bool {
    use super::utils::contain_inner_ref;

    match ty {
        ParsedType::Vector(ty, _) => contain_inner_ref(ty),
        // Structs are checked while parsing
        _ => false,
    }
}
