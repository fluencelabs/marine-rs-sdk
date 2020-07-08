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

use super::log;

use std::alloc::alloc as global_alloc;
use std::alloc::dealloc as global_dealloc;
use std::alloc::Layout;

/// Allocates memory area of specified size and returns its address.
/// Returns 0 if supplied size is too long.
#[no_mangle]
pub unsafe fn allocate(size: usize) -> usize {
    let layout = match Layout::from_size_align(size, std::mem::align_of::<u8>()) {
        Ok(layout) => layout,
        // in this case a err may occur only in a case of too long allocated size,
        // so just return 0
        Err(_) => return 0,
    };

    log(format!("sdk.allocate: {:?}\n", size));

    global_alloc(layout) as _
}

/// Deallocates memory area for provided memory pointer and size.
/// Does nothing if supplied size is too long.
#[no_mangle]
pub unsafe fn deallocate(ptr: *mut u8, size: usize) {
    let layout = match Layout::from_size_align(size, std::mem::align_of::<u8>()) {
        Ok(layout) => layout,
        // in this case a err may occur only in a case of too long allocated size,
        // so just done nothing
        Err(_) => return,
    };

    log(format!("sdk.deallocate: {:?} {}\n", ptr, size));

    global_dealloc(ptr, layout);
}
