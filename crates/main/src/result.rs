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

//! Contains ad-hoc implementations of returning complex data types from function calls
//! by two global variables that contain pointer and size. Will be refactored after multi-value
//! support in Wasmer.

//use std::sync::atomic::AtomicUsize;
//use std::cell::RefCell;
//use std::any::Any;

//static mut RESULT_PTR: AtomicUsize = AtomicUsize::new(0);

//thread_local!(static OBJECTS_TO_RELEASE: RefCell<Vec<Box<dyn Any>>> = RefCell::new(Vec::new()));

#[no_mangle]
pub unsafe fn get_result_ptr() -> usize {

 0 as _
   // *RESULT_PTR.get_mut()
}
