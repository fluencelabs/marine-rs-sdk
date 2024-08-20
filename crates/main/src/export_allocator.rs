/*
 * Marine Rust SDK
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

/// Allocates memory area of specified size and type and returns its address.
/// The allocated memory region is intended to be use as a Vec.
#[no_mangle]
pub unsafe fn allocate(elem_count: usize, elem_ty: usize) -> usize {
    if elem_count == 0 {
        // otherwise 1 would be returned due to the internals of Vec in Rust
        return 0;
    }

    #[allow(clippy::let_and_return)]
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
