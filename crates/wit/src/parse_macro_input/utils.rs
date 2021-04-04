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
