/*
 * Fluence Marine Rust SDK
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

#[macro_export]
/// Crates new syn::Ident with the given string and new call span
macro_rules! new_ident {
    ($string: expr) => {
        syn::Ident::new(&$string, proc_macro2::Span::call_site())
    };
}

#[macro_export]
macro_rules! prepare_global_data {
    ($mtype: ident, $self: ident, $name: expr, $data: ident, $data_size: ident, $global_static_name: ident, $section_name: ident) => {
        // TODO: change serialization protocol
        let mtype = $crate::export_ast_types::SDKAst::$mtype($self.clone().into());
        let $data = serde_json::to_vec(&mtype).unwrap();
        let $data_size = $data.len();
        let $data = syn::LitByteStr::new(&$data, proc_macro2::Span::call_site());

        let $global_static_name = $crate::new_ident!(format!(
            "{}{}",
            $crate::token_stream_generator::GENERATED_GLOBAL_PREFIX,
            $name.replace(".", "_"),
        ));
        let $section_name = format!(
            "{}{}",
            $crate::token_stream_generator::GENERATED_SECTION_PREFIX,
            $name.replace(".", "_"),
        );
    };
}

#[macro_export]
macro_rules! syn_error {
    ($span:expr, $message:expr) => {
        Err(syn::Error::new($span, $message))
    };
}

/// Calculate record size in an internal serialized view.
pub fn get_record_size<'a>(
    fields: impl Iterator<Item = &'a crate::parsed_type::ParsedType>,
) -> usize {
    use crate::parsed_type::ParsedType;

    let mut size = 0;

    for field in fields {
        size += match field {
            ParsedType::U8(_) | ParsedType::I8(_) | ParsedType::Boolean(_) => 1,
            ParsedType::U16(_) | ParsedType::I16(_) => 2,
            ParsedType::U32(_) | ParsedType::I32(_) | ParsedType::F32(_) => 4,
            ParsedType::U64(_) | ParsedType::I64(_) | ParsedType::F64(_) => 8,
            ParsedType::Record(..) => 4,
            ParsedType::Vector(..) | ParsedType::Utf8Str(_) | ParsedType::Utf8String(_) => 2 * 4,
        };
    }

    size
}

pub(crate) fn prepare_ident(str: String) -> String {
    str.chars()
        .map(|c| match c {
            '<' => '_',
            '&' => '_',
            '>' => '_',
            c => c,
        })
        .collect()
}
