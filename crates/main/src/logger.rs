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
//! This example initializes [`WasmLogger`] with setting log level.
//! Macros from crate [`log`] are used as a logging facade.
//!
//! ```
//!     use fluence::sdk::*;
//!     use log::{error, trace};
//!     use simple_logger;
//!
//!     fn main() {
//!         logger::WasmLogger::new()
//!             .with_log_leve(log::Level::Info)
//!             .build()
//!             .unwrap();
//!
//!         error!("This message will be logged.");
//!         trace!("This message will not be logged.");
//!     }
//!
//! ```
//!
//! [`WasmLogger`]: struct.WasmLogger.html
//! [`log`]: https://docs.rs/log

pub type TargetMap = std::collections::HashMap<&'static str, i64>;

/// The Wasm Logger.
///
/// This struct implements the [`Log`] trait from the [`log`] crate, which allows it to act as a
/// logger.
///
/// Builder pattern is used here for logger initialization. Please be aware that build must be called
/// to set the logger up.
///
/// [log-crate-url]: https://docs.rs/log/
/// [`Log`]: https://docs.rs/log/0.4.11/log/trait.Log.html
pub struct WasmLogger {
    target_map: TargetMap,
}

pub struct WasmLoggerBuilder {
    target_map: TargetMap,
    log_level: log::Level,
}

// #[allow(dead_code)]
impl WasmLoggerBuilder {
    /// Initializes a builder of the global logger with log level set to `Level::Info`.
    /// It is a initial method in this builder chain, please note, that logger wouldn't work without
    /// subsequent build() call.
    pub fn new() -> Self {
        Self {
            log_level: log::Level::Info,
            target_map: <_>::default(),
        }
    }

    /// Set the log level.
    pub fn with_log_level(mut self, level: log::Level) -> Self {
        self.log_level = level;
        self
    }

    /// Set mapping between logging targets and numbers.
    /// Used to efficiently enable & disable logs per target on the host.
    pub fn with_target_map(mut self, map: TargetMap) -> Self {
        self.target_map = map;
        self
    }

    /// Build the real logger.
    ///
    /// This method is a last one in this builder chain and MUST be called to set logger up.
    /// Returns a error
    ///
    /// ```
    /// # use fluence::logger;
    /// # use log::info;
    /// #
    /// # fn main() {
    ///     logger::WasmLogger::new()
    ///         .with_log_level(log::Level::Trace)
    ///         .with_target_map(<_>::default())
    ///         .build()
    ///         .unwrap();
    /// # }
    /// ```
    pub fn build(self) -> Result<(), log::SetLoggerError> {
        let Self {
            log_level,
            target_map,
        } = self;

        let wasm_logger = WasmLogger { target_map };

        log::set_boxed_logger(Box::new(wasm_logger))?;
        log::set_max_level(log_level.to_level_filter());
        Ok(())
    }
}

impl log::Log for WasmLogger {
    #[inline]
    fn enabled(&self, metadata: &log::Metadata<'_>) -> bool {
        metadata.level() <= log::max_level()
    }

    #[inline]
    fn log(&self, record: &log::Record<'_>) {
        if !self.enabled(record.metadata()) {
            return;
        }

        let level = record.metadata().level() as i32;
        let default_target = 0;
        let target = *self
            .target_map
            .get(record.metadata().target())
            .unwrap_or(&default_target);
        let msg = record.args().to_string();

        log_utf8_string(level, target, msg.as_ptr() as _, msg.len() as _);
    }

    // in our case flushing is performed by a host itself
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
