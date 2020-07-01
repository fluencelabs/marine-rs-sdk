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

use crate::parse_macro_input::ParseMacroInput;
use crate::token_stream_generator::TokenStreamGenerator;

use proc_macro2::TokenStream;
use syn::Result;

pub(super) fn fce_impl(tokens: TokenStream) -> Result<TokenStream> {
    let item = syn::parse2::<syn::Item>(tokens)?;
    let fce_ast_item = item.parse_macro_input()?;
    fce_ast_item.generate_token_stream()

    /*
    let input_type = match sig.inputs.len() {
        0 => ParsedType::Empty,
        1 => ParsedType::from_fn_arg(sig.inputs.first().unwrap())?,
        _ => {
            return Err(Error::new(
                sig.inputs.span(),
                "The invocation handler shouldn't have more than one argument",
            ))
        }
    };

    let output_type = ParsedType::from_return_type(&sig.output)?;
    if output_type == ParsedType::Empty {
        return Err(Error::new(
            sig.output.span(),
            "The invocation handler should have the return value",
        ));
    }

    let ident = &sig.ident;
    let prolog = input_type.generate_fn_prolog();
    let prolog = match input_type {
        ParsedType::Empty => quote! {
            #prolog

            let result = #ident();
        },
        _ => quote! {
            #prolog

            let result = #ident(arg);
        },
    };
    let epilog = output_type.generate_fn_epilog();

    let resulted_invoke = quote! {
        #fn_item

        #[no_mangle]
        pub unsafe fn invoke(ptr: *mut u8, len: usize) -> std::ptr::NonNull<u8> {
            #prolog

            #epilog
        }
    };

    Ok(resulted_invoke)
     */
}
