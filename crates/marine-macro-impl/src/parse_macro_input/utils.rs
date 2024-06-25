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

use crate::ParsedType;
use crate::parsed_type::PassingStyle;
use crate::parsed_type::passing_style_of;

/// Checks whether a type contains a reference in one of types.
pub(super) fn contain_inner_ref(ty: &ParsedType) -> bool {
    let passing_style = passing_style_of(ty);
    match passing_style {
        PassingStyle::ByValue => {}
        PassingStyle::ByRef | PassingStyle::ByMutRef => return true,
    };

    match ty {
        ParsedType::Vector(ty, _) => contain_inner_ref(ty),
        _ => false,
    }
}
