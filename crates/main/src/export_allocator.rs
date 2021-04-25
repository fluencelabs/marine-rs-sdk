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

/// Allocates memory area of specified size and type and returns its address.
/// The allocated memory region is intended to be use as a Vec.
#[no_mangle]
pub unsafe fn allocate(elem_count: usize, elem_ty: usize) -> usize {
    if elem_count == 0 {
        // otherwise 1 would be returned due to the internals of Vec in Rust
        return 0;
    }

    let allocated_mem = allocate_impl(elem_count, elem_ty);
    crate::debug_log!(format!(
        "sdk.allocate: {} {} -> {}\n",
        elem_count, elem_ty, allocated_mem
    ));

    allocated_mem
}

fn allocate_impl(elem_count: usize, elem_ty: usize) -> usize {
    match elem_ty {
        0 => allocate_vec::<u8>(elem_count), // for booleans
        1 => allocate_vec::<u8>(elem_count),
        2 => allocate_vec::<u16>(elem_count),
        3 => allocate_vec::<u32>(elem_count),
        4 => allocate_vec::<u64>(elem_count),
        5 => allocate_vec::<i8>(elem_count),
        6 => allocate_vec::<i16>(elem_count),
        7 => allocate_vec::<i32>(elem_count),
        8 => allocate_vec::<i64>(elem_count),
        9 => allocate_vec::<f32>(elem_count),
        10 => allocate_vec::<f64>(elem_count),
        _ => 0,
    }
}

fn allocate_vec<T>(count: usize) -> usize {
    // TODO: handle OOM
    // This allocation scheme with vectors is needed to deal with internal Vec layout
    let vec = Vec::<T>::with_capacity(count);
    let offset = vec.as_ptr() as usize;
    std::mem::forget(vec);

    offset
}
