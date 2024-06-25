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
use super::ParsedType;
use super::PassingStyle;

pub(crate) fn passing_style_of(ty: &ParsedType) -> &PassingStyle {
    use ParsedType::*;

    match ty {
        Boolean(passing_style) => passing_style,
        U8(passing_style) => passing_style,
        U16(passing_style) => passing_style,
        U32(passing_style) => passing_style,
        U64(passing_style) => passing_style,
        I8(passing_style) => passing_style,
        I16(passing_style) => passing_style,
        I32(passing_style) => passing_style,
        I64(passing_style) => passing_style,
        F32(passing_style) => passing_style,
        F64(passing_style) => passing_style,
        Utf8Str(passing_style) => passing_style,
        Utf8String(passing_style) => passing_style,
        Vector(_, passing_style) => passing_style,
        Record(_, passing_style) => passing_style,
    }
}
