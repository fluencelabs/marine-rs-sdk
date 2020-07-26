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

#[macro_export]
/// Crates new syn::Ident with the given string and new call span
macro_rules! new_ident {
    ($string: expr) => {
        syn::Ident::new(&$string, proc_macro2::Span::call_site());
    };
}

#[macro_export]
macro_rules! prepare_global_data {
    ($fce_type: ident, $self: ident, $name: expr, $data: ident, $data_size: ident, $global_static_name: ident, $section_name: ident) => {
        // TODO: change serialization protocol
        let fce_type = fce_ast_types::FCEAst::$fce_type($self.clone());
        let $data = serde_json::to_vec(&fce_type).unwrap();
        let $data_size = $data.len();
        let $data = syn::LitByteStr::new(&$data, proc_macro2::Span::call_site());

        let $global_static_name = crate::new_ident!(
            crate::token_stream_generator::GENERATED_GLOBAL_PREFIX.to_string()
                + &$name.replace(".", "_")
        );
        let $section_name = crate::token_stream_generator::GENERATED_SECTION_PREFIX.to_string()
            + &$name.replace(".", "_");
    };
}

pub fn get_record_size<'a>(
    fields: impl Iterator<Item = &'a crate::parsed_type::ParsedType>,
) -> usize {
    use crate::parsed_type::ParsedType;

    let mut size = 0;

    for field in fields {
        let params_count = match field {
            ParsedType::ByteVector | ParsedType::Utf8String => 2,
            _ => 1,
        };

        size += std::mem::size_of::<u64>() * params_count;
    }

    size
}
