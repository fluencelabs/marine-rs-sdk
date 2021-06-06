/*
 * copyright 2020 sdk labs limited
 *
 * licensed under the apache license, version 2.0 (the "license");
 * you may not use this file except in compliance with the license.
 * you may obtain a copy of the license at
 *
 *     http://www.apache.org/licenses/license-2.0
 *
 * unless required by applicable law or agreed to in writing, software
 * distributed under the license is distributed on an "as is" basis,
 * without warranties or conditions of any kind, either express or implied.
 * see the license for the specific language governing permissions and
 * limitations under the license.
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
