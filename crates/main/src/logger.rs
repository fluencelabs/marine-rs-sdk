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

use log::Level as LogLevel;
use std::collections::HashMap;

/// By default, logger will be initialized with log level from this environment variable.
pub const WASM_LOG_ENV_NAME: &str = "WASM_LOG";

/// If WASM_LOG_ENV isn't set, then this level will be used as the default.
const WASM_DEFAULT_LOG_LEVEL: LogLevel = LogLevel::Info;

/// Mapping from logging namespace string to its bitmask.
/// TODO: use i64 for bitmask when wasmpack/bindgen issue with i64 is fixed.
///       Currently, i64 doesn't work on some versions of V8 because log_utf8_string function
///       isn't marked as #[wasm_bindgen]. In result, TS/JS code throws 'TypeError' on every log.
pub type TargetMap = HashMap<&'static str, i32>;

/// Mapping from module name to their log levels.
type ModuleMap = HashMap<String, log::Level>;

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
struct WasmLogger {
    target_map: TargetMap,
    modules_level: ModuleMap,
    default_log_level: log::Level,
}

/// The Wasm logger builder.
///
/// Build logger for the Fluence network, allows specifying target map and log level while building.
pub struct WasmLoggerBuilder {
    wasm_logger: WasmLogger,
}

impl WasmLoggerBuilder {
    /// Initializes a builder of the global logger. Set log level based on the WASM_LOG environment variable if it set,
    /// or [[WASM_DEFAULT_LOG_LEVEL]] otherwise. It is an initial method in this builder chain, please note,
    /// that logger wouldn't work without subsequent build() call.
    pub fn new() -> Self {
        use std::str::FromStr;

        let default_log_level = std::env::var(WASM_LOG_ENV_NAME)
            .map_or(WASM_DEFAULT_LOG_LEVEL, |log_level_str| {
                LogLevel::from_str(&log_level_str).unwrap_or(WASM_DEFAULT_LOG_LEVEL)
            });

        let wasm_logger = WasmLogger {
            target_map: HashMap::new(),
            modules_level: HashMap::new(),
            default_log_level,
        };

        Self { wasm_logger }
    }

    /// Set the log level.
    pub fn with_log_level(mut self, level: log::Level) -> Self {
        self.wasm_logger.default_log_level = level;
        self
    }

    /// Set mapping between logging targets and numbers.
    /// Used to efficiently enable & disable logs per target on the host.
    pub fn with_target_map(mut self, map: TargetMap) -> Self {
        self.wasm_logger.target_map = map;
        self
    }

    pub fn filter(mut self, module_name: impl Into<String>, level: log::Level) -> Self {
        self.wasm_logger
            .modules_level
            .insert(module_name.into(), level);
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
        let Self { wasm_logger } = self;

        log::set_boxed_logger(Box::new(wasm_logger))?;
        Ok(())
    }
}

impl log::Log for WasmLogger {
    #[inline]
    fn enabled(&self, metadata: &log::Metadata<'_>) -> bool {
        let allowed_level = match self.modules_level.get(metadata.target()) {
            Some(allowed_level) => allowed_level,
            None => &self.default_log_level,
        };

        metadata.level() <= *allowed_level
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
pub fn log_utf8_string(level: i32, target: i32, msg_ptr: i32, msg_size: i32) {
    unsafe { log_utf8_string_impl(level, target, msg_ptr, msg_size) };
}

#[cfg(not(target_arch = "wasm32"))]
pub fn log_utf8_string(level: i32, target: i32, msg_ptr: i32, msg_size: i32) {
    use std::str::from_utf8_unchecked;
    use core::slice::from_raw_parts;

    let level = level_from_i32(level);
    let msg = unsafe { from_utf8_unchecked(from_raw_parts(msg_ptr as _, msg_size as _)) };
    println!("[{}] {} {}", level, target, msg);
}

/// TODO: mark `log_utf8_string_impl` as #[wasm_bindgen], so it is polyfilled by bindgen
/// log_utf8_string should be provided directly by a host.
#[cfg(target_arch = "wasm32")]
#[link(wasm_import_module = "host")]
extern "C" {
    // Writes a byte string of size bytes that starts from ptr to a logger
    #[link_name = "log_utf8_string"]
    fn log_utf8_string_impl(level: i32, target: i32, msg_ptr: i32, msg_size: i32);
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

#[cfg(test)]
mod tests {
    use super::WasmLogger;
    use log::Log;

    use std::collections::HashMap;

    fn create_metadata(module_name: &str, level: log::Level) -> log::Metadata<'_> {
        log::MetadataBuilder::new()
            .level(level)
            .target(module_name)
            .build()
    }

    #[test]
    fn enabled_by_module_name() {
        let module_1_name = "module_1";
        let module_2_name = "module_2";

        let modules_level = maplit::hashmap!(
            module_1_name.to_string() => log::Level::Info,
            module_2_name.to_string() => log::Level::Warn,
        );

        let logger = WasmLogger {
            target_map: HashMap::new(),
            modules_level,
            default_log_level: log::Level::Error,
        };

        let allowed_metadata = create_metadata(module_1_name, log::Level::Info);
        assert!(logger.enabled(&allowed_metadata));

        let allowed_metadata = create_metadata(module_1_name, log::Level::Warn);
        assert!(logger.enabled(&allowed_metadata));

        let allowed_metadata = create_metadata(module_2_name, log::Level::Warn);
        assert!(logger.enabled(&allowed_metadata));

        let not_allowed_metadata = create_metadata(module_1_name, log::Level::Debug);
        assert!(!logger.enabled(&not_allowed_metadata));

        let not_allowed_metadata = create_metadata(module_2_name, log::Level::Info);
        assert!(!logger.enabled(&not_allowed_metadata));
    }

    #[test]
    fn default_log_level() {
        let modules_level = maplit::hashmap!(
            "module_1".to_string() => log::Level::Info,
        );

        let logger = WasmLogger {
            target_map: HashMap::new(),
            modules_level,
            default_log_level: log::Level::Warn,
        };

        let module_name = "some_module";
        let allowed_metadata = create_metadata(module_name, log::Level::Warn);
        assert!(logger.enabled(&allowed_metadata));

        let not_allowed_metadata = create_metadata(module_name, log::Level::Info);
        assert!(!logger.enabled(&not_allowed_metadata));
    }
}
