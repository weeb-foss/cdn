use std::fmt::format;
use std::io::{Result, Write};

use flexi_logger::DeferredNow;
use log::Record;

use crate::colors::Colorize;

pub mod colors;

/// Formats a log record and writes it to the provided writer.
///
/// # Arguments
///
/// * `w` - A mutable reference to a writer where the formatted log will be
///   written.
/// * `now` - A mutable reference to a DeferredNow instance representing the
///   current time.
/// * `record` - A reference to the log record to be formatted.
///
/// # Example
///
/// ```rust
/// use log::{Record, Level};
/// use std::io::stdout;
/// use flexi_logger::DeferredNow;
///
/// let mut writer = stdout();
/// let now = DeferredNow::now();
/// let record = Record::builder()
///     .args(format_args!("Hello, world!"))
///     .level(Level::Info)
///     .target("my_target")
///     .build();
///
/// format_log(&mut writer, &mut now, &record).unwrap();
/// ```
pub fn format_log(w: &mut dyn Write, now: &mut DeferredNow, record: &Record) -> Result<()> {
    // Match the log level of the record to a colored string
    let level = match record.level() {
        log::Level::Error => "ERROR".red().bold(),
        log::Level::Warn => "WARN".yellow().bold(),
        log::Level::Info => "INFO".green(),
        log::Level::Debug => "DEBUG".purple(),
        log::Level::Trace => "TRACE".cyan(),
    };

    // Write the formatted log message to the writer
    write!(
        w,
        // Format: [HH:MM:SS YYYY-MM-DD LEVEL] > message
        "[{} {}] \u{203A} {}",
        // Current local time formatted as HH:MM:SS YYYY-MM-DD and colored gray
        now.format("%H:%M:%S %Y-%m-%d").to_string().gray(),
        // Colored log level
        level,
        // Log message formatted and colored gray
        format(*record.args()).gray()
    )
}
