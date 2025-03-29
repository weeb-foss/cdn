//! ANSI terminal text styling library with type-safe style composition
//!
//! Provides a robust system for styling terminal text using ANSI escape codes
//! with:
//! - Compile-time type checking
//! - Method chaining for style combinations
//! - Zero-cost abstractions
//! - RGB true color support
//!
//! # Features
//! - Color styling (RGB 24-bit)
//! - Text decorations (bold, underline)
//! - Style composition and chaining
//! - Automatic reset handling
//! - Generic display type support
//!
//! # Usage
//! ```no_run
//! use logger::colors::{Colorize, StyledText};
//!
//! // Basic styling
//! println!("{}", "Error".red().bold());
//!
//! // Number styling
//! println!("{}", 42.5.cyan().underline());
//!
//! // Style composition
//! let warning = "Warning".yellow()
//!     .bold()
//!     .underline();
//! println!("{}", warning);
//! ```
//!
//! # Technical Notes
//! - Requires terminal with ANSI escape code support
//! - Uses CSI (Control Sequence Introducer) sequences
//! - Reset code (`\x1b[0m`) automatically applied after each styled value

use std::fmt::{Display, Formatter, Result as FmtResult};

/// Represents ANSI escape codes for styling terminal text
///
/// # Variant Types
/// - **Colors**: Use RGB true color codes (24-bit)
/// - **Decorations**: Use standard SGR (Select Graphic Rendition) codes
///
/// # ANSI Code Reference
/// | Variant     | Escape Sequence          | Description               |
/// |-------------|--------------------------|---------------------------|
/// | `Reset`     | `\x1b[0m`                | Reset all styles          |
/// | `Red`       | `\x1b[38;2;235;111;146m` | RGB(235, 111, 146) text   |
/// | `Bold`      | `\x1b[1m`                | Increased intensity/bold  |
/// | `Underline` | `\x1b[4m`                | Underlined text           |
///
/// # Implementation Details
/// Converts enum variants to their corresponding ANSI escape sequences
/// when formatted using the `Display` trait.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AnsiCode {
    /// Reset all styles to terminal defaults
    /// ANSI: `\x1b[0m`
    Reset,

    /// Red text (RGB: 235, 111, 146)
    /// ANSI: `\x1b[38;2;235;111;146m`
    Red,

    /// Green text (RGB: 49, 116, 143)
    /// ANSI: `\x1b[38;2;49;116;143m`
    Green,

    /// Yellow text (RGB: 246, 193, 119)
    /// ANSI: `\x1b[38;2;246;193;119m`
    Yellow,

    /// Purple text (RGB: 196, 167, 231)
    /// ANSI: `\x1b[38;2;196;167;231m`
    Purple,

    /// Cyan text (RGB: 156, 207, 216)
    /// ANSI: `\x1b[38;2;156;207;216m`
    Cyan,

    /// Gray text (RGB: 144, 140, 170)
    /// ANSI: `\x1b[38;2;144;140;170m`
    Gray,

    /// Bold/bright text style
    /// ANSI: `\x1b[1m`
    Bold,

    /// Underlined text
    /// ANSI: `\x1b[4m`
    Underline,
}

impl Display for AnsiCode {
    /// Converts `AnsiCode` variants to their corresponding escape sequences
    ///
    /// # Formatting Process
    /// 1. Matches enum variant to corresponding ANSI code
    /// 2. Writes raw escape sequence to formatter
    /// 3. Maintains proper CSI sequence structure
    ///
    /// # Panics
    /// Never panics - all enum variants are explicitly handled
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match self {
            AnsiCode::Reset => write!(f, "\x1b[0m"),
            AnsiCode::Red => write!(f, "\x1b[38;2;255;85;85m"), // #FF5555
            AnsiCode::Green => write!(f, "\x1b[38;2;80;250;123m"), // #50FA7B
            AnsiCode::Yellow => write!(f, "\x1b[38;2;241;250;140m"), // #F1FA8C
            AnsiCode::Purple => write!(f, "\x1b[38;2;189;147;249m"), // #BD93F9
            AnsiCode::Cyan => write!(f, "\x1b[38;2;139;233;253m"), // #8BE9FD
            AnsiCode::Gray => write!(f, "\x1b[38;2;136;136;136m"), // #888888
            AnsiCode::Bold => write!(f, "\x1b[1m"),
            AnsiCode::Underline => write!(f, "\x1b[4m"),
        }
    }
}

/// Composable text styling container
///
/// # Type Parameters
/// - `T`: The underlying displayable type implementing `Display`
///
/// # Composition Behavior
/// - Stores original value without modification
/// - Collects styling codes in insertion order
/// - Applies all styles when formatted
/// - Automatically resets styles after value
pub struct StyledText<T: Display> {
    /// Original displayable value being styled
    value: T,
    /// Ordered collection of ANSI styling codes
    styles: Vec<AnsiCode>,
    /// Reset code to apply after value (always `AnsiCode::Reset`)
    reset: AnsiCode,
}

impl<T: Display> StyledText<T> {
    /// Creates new styled text with initial style
    ///
    /// # Parameters
    /// - `value`: The displayable value to style
    /// - `style`: Initial ANSI style to apply
    ///
    /// # Initialization
    /// - Initializes `styles` vector with provided style
    /// - Sets `reset` to `AnsiCode::Reset`
    ///
    /// # Example
    /// ```
    /// use logger::colors::{StyledText, AnsiCode};
    /// let styled = StyledText::new("Hello", AnsiCode::Red);
    /// ```
    #[inline]
    fn new(value: T, style: AnsiCode) -> Self {
        Self {
            value,
            styles: vec![style],
            reset: AnsiCode::Reset,
        }
    }

    /// Adds bold style to the style stack
    ///
    /// # Method Behavior
    /// - Appends `AnsiCode::Bold` to styles list
    /// - Returns modified `StyledText` for chaining
    ///
    /// # Example
    /// ```
    /// use crate::helpers::misc::colors::{Colorize, AnsiCode};
    /// let text = "Alert".red().bold();
    /// ```
    pub fn bold(mut self) -> Self {
        self.styles.push(AnsiCode::Bold);
        self
    }

    /// Adds underline style to the style stack
    ///
    /// # Method Behavior
    /// - Appends `AnsiCode::Underline` to styles list
    /// - Returns modified `StyledText` for chaining
    ///
    /// # Example
    /// ```
    /// use crate::helpers::misc::colors::{Colorize, AnsiCode};
    /// let text = "URL".blue().underline();
    /// ```
    pub fn underline(mut self) -> Self {
        self.styles.push(AnsiCode::Underline);
        self
    }
}

impl<T: Display> Display for StyledText<T> {
    /// Formats styled text with proper ANSI code wrapping
    ///
    /// # Formatting Steps
    /// 1. Applies all stored ANSI codes in insertion order
    /// 2. Writes original value
    /// 3. Applies reset code
    ///
    /// # Error Handling
    /// Propagates any formatting errors from underlying writes
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        // Apply all accumulated styles
        for style in &self.styles {
            write!(f, "{}", style)?;
        }

        // Write original value
        write!(f, "{}", self.value)?;

        // Reset styling
        write!(f, "{}", self.reset)
    }
}

/// Generates color styling methods for the Colorize trait
///
/// # Macro Expansion Example
/// ```ignore
/// color_method!(red, AnsiCode::Red);
/// expands to:
/// fn red(self) -> StyledText<Self> {
///     StyledText::new(self, AnsiCode::Red)
/// }
/// ```
macro_rules! color_method {
    ($method:ident, $code:expr) => {
        #[inline]
        fn $method(self) -> StyledText<Self> {
            StyledText::new(self, $code)
        }
    };
}

/// Extension trait adding styling methods to all Display implementers
///
/// # Blanket Implementation
/// Implemented for all `T: Display`, meaning these methods are available on:
/// - Strings (&str, String)
/// - Numbers (i32, f64, etc.)
/// - Smart pointers (Box, Rc, Arc)
/// - Any custom Display implementers
///
/// # Method Guarantees
/// - All methods return `StyledText` wrappers
/// - Original value remains unmodified
/// - Methods can be safely chained
pub trait Colorize: Sized + Display {
    color_method!(red, AnsiCode::Red);
    color_method!(green, AnsiCode::Green);
    color_method!(yellow, AnsiCode::Yellow);
    color_method!(purple, AnsiCode::Purple);
    color_method!(cyan, AnsiCode::Cyan);
    color_method!(gray, AnsiCode::Gray);
}

/// Applies color styling methods to all Display implementers
impl<T: Display> Colorize for T {}
