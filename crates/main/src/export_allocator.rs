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
/// The allocated memory region is intended to be use as a Vec.
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

macro_rules! alloc {
    ($ty:ty, $elem_count:expr) => {{
        let vec = Vec::<$ty>::with_capacity($elem_count);
        let offset = vec.as_ptr() as usize;
        std::mem::forget(vec);
        offset
    }};
}

fn allocate_impl(elem_count: usize, elem_ty: usize) -> usize {
    // TODO: handle OOM
    // Such allocation scheme is needed to deal with layout
    match elem_ty {
        0 => alloc!(u8, elem_count), // for booleans
        1 => alloc!(u8, elem_count),
        2 => alloc!(u16, elem_count),
        3 => alloc!(u32, elem_count),
        4 => alloc!(u64, elem_count),
        5 => alloc!(i8, elem_count),
        6 => alloc!(i16, elem_count),
        7 => alloc!(i32, elem_count),
        8 => alloc!(i64, elem_count),
        9 => alloc!(f32, elem_count),
        10 => alloc!(f64, elem_count),
        _ => alloc!(u8, 0), // it'll allocate 0 bytes
    }
}
