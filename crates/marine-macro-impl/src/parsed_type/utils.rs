/*
 * Copyright 2021 Fluence Labs Limited
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
