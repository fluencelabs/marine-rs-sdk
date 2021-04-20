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

#[cfg(feature = "debug")]
use super::log;

/// Allocates memory area of specified size and type and returns its address.
/// The returned
#[no_mangle]
pub unsafe fn allocate(elem_count: usize, elem_ty: usize) -> usize {
    let allocated_mem = allocate_impl(elem_count, elem_ty);

    #[cfg(feature = "debug")]
    log(format!(
        "sdk.allocate: {} {} -> {}\n",
        elem_count, elem_ty, allocated_mem
    ));

    allocated_mem
}

fn allocate_impl(elem_count: usize, elem_ty: usize) -> usize {
    // TODO: handle OOM
    // Such allocation scheme is needed to deal with layout
    match elem_ty {
        0 => Vec::<u8>::with_capacity(elem_count).as_ptr() as usize, // for booleans
        1 => Vec::<u8>::with_capacity(elem_count).as_ptr() as usize,
        2 => Vec::<u16>::with_capacity(elem_count).as_ptr() as usize,
        3 => Vec::<u32>::with_capacity(elem_count).as_ptr() as usize,
        4 => Vec::<u64>::with_capacity(elem_count).as_ptr() as usize,
        5 => Vec::<i8>::with_capacity(elem_count).as_ptr() as usize,
        6 => Vec::<i16>::with_capacity(elem_count).as_ptr() as usize,
        7 => Vec::<i32>::with_capacity(elem_count).as_ptr() as usize,
        8 => Vec::<i64>::with_capacity(elem_count).as_ptr() as usize,
        9 => Vec::<f32>::with_capacity(elem_count).as_ptr() as usize,
        10 => Vec::<f64>::with_capacity(elem_count).as_ptr() as usize,
        _ => Vec::<u8>::with_capacity(0).as_ptr() as usize, // it'll allocate 0 bytes
    }
}
