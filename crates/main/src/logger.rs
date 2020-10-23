/*
 * Copyright 2018 Fluence Labs Limited
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

//! This module allows log messages from the Wasm side. It is implemented as a logging facade for
//! crate [`log`].
//!
//! # Examples
//!
//! This example initializes [`WasmLogger`] if target arch is Wasm and [`simple_logger`] otherwise.
//! Macros from crate [`log`] are used as a logging facade.
//!
//! ```
//!     use fluence::sdk::*;
//!     use log::{error, trace};
//!     use simple_logger;
//!
//!     fn main() {
//!         if cfg!(target_arch = "wasm32") {
//!             logger::WasmLogger::init_with_level(log::Level::Info).unwrap();
//!         } else {
//!             simple_logger::init_with_level(log::Level::Info).unwrap();
//!         }
//!
//!         error!("This message will be logged.");
//!         trace!("This message will not be logged.");
//!     }
//!
//! ```
//!
//! This example provides methods for [`WasmLogger`] initialization only for Wasm target without
//! specifying logger level:
//!
//! ```
//!     use fluence::sdk::*;
//!     use log::info;
//!
//!     /// This method initializes WasmLogger and should be called at the start of the application.
//!     #[no_mangle]
//!     #[cfg(target_arch = "wasm32")]
//!     fn init_logger() {
//!         logger::WasmLogger::init().unwrap();
//!         info!("If you can see this message that logger was successfully initialized.");
//!     }
//!
//! ```
//!
//! [`WasmLogger`]: struct.WasmLogger.html
//! [`log`]: https://docs.rs/log
//! [`simple_logger`]: https://docs.rs/simple_logger
//! [`static_lazy`]: https://docs.rs/lazy_static
//! [`lazy_static::initialize()`]: https://docs.rs/lazy_static/1.3.0/lazy_static/fn.initialize.html
//! [`backend app debugging`]: https://fluence.dev/docs/debugging

use std::collections::HashMap;

/// The Wasm Logger.
///
/// This struct implements the [`Log`] trait from the [`log`] crate, which allows it to act as a
/// logger.
///
/// For initialization of WasmLogger as a default logger please see [`init()`]
/// and [`init_with_level()`]
///
/// [log-crate-url]: https://docs.rs/log/
/// [`Log`]: https://docs.rs/log/0.4.6/log/trait.Log.html
/// [`init_with_level()`]: struct.WasmLogger.html#method.init_with_level
/// [`init()`]: struct.WasmLogger.html#method.init
pub struct WasmLogger {
    level: log::Level,
    target_map: Option<HashMap<&'static str, i64>>,
}

#[allow(dead_code)]
impl WasmLogger {
    /// Initializes the global logger with a [`WasmLogger`] instance, sets
    /// `max_log_level` to a given log level.
    ///
    /// ```
    /// # use fluence::sdk::*;
    /// # use log::info;
    /// #
    /// # fn main() {
    /// if cfg!(target_arch = "wasm32") {
    ///     logger::WasmLogger::init_with_level(log::Level::Error).unwrap();
    /// }
    /// error!("This message will be logged.");
    /// info!("This message will not be logged.");
    /// # }
    /// ```
    pub fn init_with_level(level: log::Level) -> Result<(), log::SetLoggerError> {
        let logger = WasmLogger {
            level,
            target_map: None,
        };
        log::set_boxed_logger(Box::new(logger))?;
        log::set_max_level(level.to_level_filter());
        Ok(())
    }

    /// Sets mapping between logging targets and numbers
    /// Used to efficiently enable & disable logs per target on the host
    pub fn with_target_map(&mut self, map: HashMap<&'static str, i64>) {
        self.target_map = Some(map);
    }

    /// Initializes the global logger with a [`WasmLogger`] instance, sets
    /// `max_log_level` to `Level::Info`.
    ///
    /// ```
    /// # use fluence::sdk::*;
    /// # use log::info;
    /// #
    /// # fn main() {
    /// if cfg!(target_arch = "wasm32") {
    ///     fluence::logger::WasmLogger::init().unwrap();
    /// }
    ///
    /// error!("This message will be logged.");
    /// trace!("This message will not be logged.");
    /// # }
    /// ```
    pub fn init() -> Result<(), log::SetLoggerError> {
        WasmLogger::init_with_level(log::Level::Info)
    }
}

impl log::Log for WasmLogger {
    #[inline]
    fn enabled(&self, metadata: &log::Metadata<'_>) -> bool {
        metadata.level() <= self.level
    }

    #[inline]
    fn log(&self, record: &log::Record<'_>) {
        if !self.enabled(record.metadata()) {
            return;
        }

        let level = record.metadata().level() as i32;
        let target = *self
            .target_map
            .as_ref()
            .and_then(|m| m.get(record.metadata().target()))
            .unwrap_or(&0);
        let msg = record.args().to_string();

        log_utf8_string(level, target, msg.as_ptr() as _, msg.len() as _);
    }

    // in our case flushing is performed by the VM itself
    #[inline]
    fn flush(&self) {}
}

#[cfg(target_arch = "wasm32")]
pub fn log_utf8_string(level: i32, target: i64, msg_ptr: i32, msg_size: i32) {
    unsafe { log_utf8_string_impl(level, target, msg_ptr, msg_size) };
}

#[cfg(not(target_arch = "wasm32"))]
pub fn log_utf8_string(level: i32, target: i64, msg_ptr: i32, msg_size: i32) {
    use std::str::from_utf8_unchecked;
    use core::slice::from_raw_parts;

    let level = level_from_i32(level);
    let msg = unsafe { from_utf8_unchecked(from_raw_parts(msg_ptr as _, msg_size as _)) };
    println!("[{}] {} {}", level, target, msg);
}

/// log_utf8_string should be provided directly by a host.
#[cfg(target_arch = "wasm32")]
#[link(wasm_import_module = "host")]
extern "C" {
    // Writes a byte string of size bytes that starts from ptr to a logger
    #[link_name = "log_utf8_string"]
    fn log_utf8_string_impl(level: i32, target: i64, msg_ptr: i32, msg_size: i32);
}

#[allow(dead_code)]
fn level_from_i32(level: i32) -> log::Level {
    match level {
        1 => log::Level::Error,
        2 => log::Level::Warn,
        3 => log::Level::Info,
        4 => log::Level::Debug,
        5 => log::Level::Trace,
        _ => log::Level::max(),
    }
}
