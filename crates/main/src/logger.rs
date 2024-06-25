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

//! This module allows log messages from the Wasm side. It is implemented as a logging facade for
//! crate [`log`].
//!
//! # Examples
//!
//! This example initializes [`WasmLogger`] with setting log level.
//! Macros from crate [`log`] are used as a logging facade.
//!
//! ```ignore
//!     use marine_rs_sdk::logger;
//!     use log::{error, trace};
//!     use simple_logger;
//!
//!     fn main() {
//!         logger::WasmLoggerBuilder::new()
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

use log::LevelFilter;
use std::collections::HashMap;

/// By default, logger will be initialized with log level from this environment variable.
pub const WASM_LOG_ENV_NAME: &str = "WASM_LOG";

/// If WASM_LOG_ENV isn't set, then this level will be used as the default.
const WASM_DEFAULT_LOG_LEVEL: LevelFilter = LevelFilter::Info;

/// Mapping from logging namespace string to its bitmask.
/// TODO: use i64 for bitmask when wasmpack/bindgen issue with i64 is fixed.
///       Currently, i64 doesn't work on some versions of V8 because log_utf8_string function
///       isn't marked as #[wasm_bindgen]. In result, TS/JS code throws 'TypeError' on every log.
pub type TargetMap = HashMap<&'static str, i32>;

/// This structure is used to save information about particular log level for a particular module.
#[derive(Debug)]
struct LogDirective {
    module_name: String,
    level: LevelFilter,
}

impl LogDirective {
    pub fn new(module_name: String, level: LevelFilter) -> Self {
        Self { module_name, level }
    }
}

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
    modules_directives: Vec<LogDirective>,
    default_log_level: LevelFilter,
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
                LevelFilter::from_str(&log_level_str).unwrap_or(WASM_DEFAULT_LOG_LEVEL)
            });

        let wasm_logger = WasmLogger {
            target_map: HashMap::new(),
            modules_directives: Vec::new(),
            default_log_level,
        };

        Self { wasm_logger }
    }

    /// Set the log level.
    pub fn with_log_level(mut self, level: LevelFilter) -> Self {
        self.wasm_logger.default_log_level = level;
        self
    }

    /// Set mapping between logging targets and numbers.
    /// Used to efficiently enable & disable logs per target on the host.
    pub fn with_target_map(mut self, map: TargetMap) -> Self {
        self.wasm_logger.target_map = map;
        self
    }

    pub fn filter(mut self, module_name: impl Into<String>, level: LevelFilter) -> Self {
        let module_name = module_name.into();
        let log_directive = LogDirective::new(module_name, level);

        self.wasm_logger.modules_directives.push(log_directive);
        self
    }

    /// Build the real logger.
    ///
    /// This method is a last one in this builder chain and MUST be called to set logger up.
    /// Returns a error
    ///
    /// ```ignore
    /// # use marine_rs_sdk::logger;
    /// # use log::info;
    /// #
    /// # fn main() {
    ///     logger::WasmLoggerBuilder::new()
    ///         .with_log_level(log::LevelFilter::Trace)
    ///         .with_target_map(<_>::default())
    ///         .build()
    ///         .unwrap();
    /// # }
    /// ```
    pub fn build(mut self) -> Result<(), log::SetLoggerError> {
        let max_level = self.max_log_level();
        self.sort_directives();

        let Self { wasm_logger } = self;

        log::set_boxed_logger(Box::new(wasm_logger))?;
        log::set_max_level(max_level);
        Ok(())
    }

    /// Sort supplied directive ny length of module names to make more efficient lookup at runtime.
    fn sort_directives(&mut self) {
        self.wasm_logger.modules_directives.sort_by(|l, r| {
            let llen = l.module_name.len();
            let rlen = r.module_name.len();

            rlen.cmp(&llen)
        });
    }

    fn max_log_level(&self) -> log::LevelFilter {
        let default_level = self.wasm_logger.default_log_level;
        let max_filter_level = self
            .wasm_logger
            .modules_directives
            .iter()
            .map(|d| d.level)
            .max()
            .unwrap_or(LevelFilter::Off);

        std::cmp::max(default_level, max_filter_level)
    }
}

impl log::Log for WasmLogger {
    #[inline]
    fn enabled(&self, metadata: &log::Metadata<'_>) -> bool {
        let target = metadata.target();

        for directive in self.modules_directives.iter() {
            if target.starts_with(&directive.module_name) {
                return metadata.level() <= directive.level;
            }
        }

        metadata.level() <= self.default_log_level
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

#[cfg(all(feature = "marine-abi", target_arch = "wasm32"))]
pub fn log_utf8_string(level: i32, target: i32, msg_ptr: i32, msg_size: i32) {
    unsafe { log_utf8_string_impl(level, target, msg_ptr, msg_size) };
}

#[cfg(not(all(feature = "marine-abi", target_arch = "wasm32")))]
pub fn log_utf8_string(level: i32, target: i32, msg_ptr: i32, msg_size: i32) {
    use std::str::from_utf8_unchecked;
    use core::slice::from_raw_parts;

    let level = level_from_i32(level);
    let msg = unsafe { from_utf8_unchecked(from_raw_parts(msg_ptr as _, msg_size as _)) };
    println!("[{}] {} {}", level, target, msg);
}

#[allow(unused_doc_comments)]
/// TODO: mark `log_utf8_string_impl` as #[wasm_bindgen], so it is polyfilled by bindgen
/// log_utf8_string should be provided directly by a host.
#[cfg(all(feature = "marine-abi", target_arch = "wasm32"))]
#[link(wasm_import_module = "__marine_host_api_v3")]
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
    use super::LogDirective;
    use super::WasmLoggerBuilder;
    use log::LevelFilter;
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

        let modules_directives = vec![
            LogDirective::new(module_1_name.to_string(), LevelFilter::Info),
            LogDirective::new(module_2_name.to_string(), LevelFilter::Warn),
        ];

        let logger = WasmLogger {
            target_map: HashMap::new(),
            modules_directives,
            default_log_level: LevelFilter::Error,
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
        let modules_directives = vec![LogDirective::new("module_1".to_string(), LevelFilter::Info)];

        let logger = WasmLogger {
            target_map: HashMap::new(),
            modules_directives,
            default_log_level: LevelFilter::Warn,
        };

        let module_name = "some_module";
        let allowed_metadata = create_metadata(module_name, log::Level::Warn);
        assert!(logger.enabled(&allowed_metadata));

        let not_allowed_metadata = create_metadata(module_name, log::Level::Info);
        assert!(!logger.enabled(&not_allowed_metadata));
    }

    #[test]
    fn longest_directive_first() {
        let module_1_name = "module_1";
        let module_2_name = "module_1::some_name::func_name";

        WasmLoggerBuilder::new()
            .filter(module_1_name, LevelFilter::Info)
            .filter(module_2_name, LevelFilter::Warn)
            .build()
            .unwrap();

        let logger = log::logger();

        let allowed_metadata = create_metadata(module_1_name, log::Level::Info);
        assert!(logger.enabled(&allowed_metadata));

        let not_allowed_metadata = create_metadata(module_2_name, log::Level::Info);
        assert!(!logger.enabled(&not_allowed_metadata));
    }
}
