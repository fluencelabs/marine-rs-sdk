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

//! Contains ad-hoc implementations of returning complex data types from function calls
//! by two global variables that contain pointer and size. Will be refactored after multi-value
//! support in Wasmer.

use std::sync::atomic::AtomicUsize;
use std::cell::RefCell;
use std::any::Any;

static mut RESULT_PTR: AtomicUsize = AtomicUsize::new(0);
static mut RESULT_SIZE: AtomicUsize = AtomicUsize::new(0);

thread_local!(static OBJECTS_TO_RELEASE: RefCell<Vec<Box<dyn Any>>> = RefCell::new(Vec::new()));

#[no_mangle]
pub unsafe fn get_result_ptr() -> usize {
    crate::debug_log!(format!(
        "sdk.get_result_ptr, returns {}\n",
        *RESULT_PTR.get_mut()
    ));

    *RESULT_PTR.get_mut()
}

#[no_mangle]
pub unsafe fn get_result_size() -> usize {
    crate::debug_log!(format!(
        "sdk.get_result_size, returns {}\n",
        *RESULT_SIZE.get_mut()
    ));

    *RESULT_SIZE.get_mut()
}

#[no_mangle]
pub unsafe fn set_result_ptr(ptr: usize) {
    crate::debug_log!(format!("sdk.set_result_ptr: {}\n", ptr));

    *RESULT_PTR.get_mut() = ptr;
}

#[no_mangle]
pub unsafe fn set_result_size(size: usize) {
    crate::debug_log!(format!("sdk.set_result_size: {}\n", size));

    *RESULT_SIZE.get_mut() = size;
}

#[no_mangle]
pub unsafe fn release_objects() {
    OBJECTS_TO_RELEASE.with(|objects| {
        let mut objects = objects.borrow_mut();
        while let Some(object) = objects.pop() {
            drop(object);
        }
    })
}

pub fn add_object_to_release(object: Box<dyn Any>) {
    OBJECTS_TO_RELEASE.with(|objects| {
        let mut objects = objects.borrow_mut();
        objects.push(object);
    });
}
