use console::style;
use std::fmt;

pub static mut VERBOSE: bool = false;

#[allow(dead_code)]
pub fn enable_verbose() {
    unsafe {
        VERBOSE = true;
    }
}

pub mod impls {
    use super::*;

    #[allow(dead_code)]
    pub fn log_state_impl(source: &str, args: &fmt::Arguments<'_>) {
        print!("\r\x1b[2K{:>12} {}\r", style(source).green().bold(), args);
    }

    #[allow(dead_code)]
    pub fn log_trace_impl(source: &str, args: &fmt::Arguments<'_>) {
        unsafe {
            if VERBOSE {
                println!("{:>12} {}", style(source).blue().bold(), args);
            }
        }
    }

    #[allow(dead_code)]
    pub fn log_info_impl(source: &str, args: &fmt::Arguments<'_>) {
        println!("{:>12} {}", style(source).green().bold(), args);
    }

    #[allow(dead_code)]
    pub fn log_error_impl(source: &str, args: &fmt::Arguments<'_>) {
        println!("{:>12} {}", style(source).red().bold(), args);
    }

    #[allow(dead_code)]
    pub fn log_warn_impl(source: &str, args: &fmt::Arguments<'_>) {
        println!("{:>12} {}", style(source).yellow().bold(), args);
    }

    #[allow(dead_code)]
    pub fn stage_impl(args: &fmt::Arguments<'_>) {
        println!("{:>12} {}", style("Stage").cyan().bold(), args);
    }
}

#[macro_export]
macro_rules! log_trace {
    () => (
        println!()
    );

    ($target:expr, $($t:tt)*) => (
        impls::log_trace_impl($target, &format_args!($($t)*))
    )
}

#[macro_export]
macro_rules! log_info {
    () => (
        println!()
    );

    ($target:expr, $($t:tt)*) => (
        impls::log_info_impl($target, &format_args!($($t)*))
    )
}

#[macro_export]
macro_rules! log_error {
    ($target:expr, $($t:tt)*) => (
        impls::log_error_impl($target, &format_args!($($t)*))
    )
}

#[macro_export]
macro_rules! log_warn {
    ($target:expr, $($t:tt)*) => (
        impls::log_warn_impl($target, &format_args!($($t)*))
    )
}

#[macro_export]
macro_rules! log_state {
    ($target:expr, $($t:tt)*) => (
        impls::log_state_impl($target, &format_args!($($t)*))
    )
}

pub use log_error;
pub use log_info;
pub use log_state;
pub use log_trace;
pub use log_warn;

#[allow(dead_code)]
pub fn log_state_clear() {
    print!("\r\x1b[2K");
}
