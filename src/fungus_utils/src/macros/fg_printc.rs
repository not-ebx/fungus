#[macro_export]
macro_rules! fg_printc_error {
    ($($arg:tt)*) => {
        println!("\x1b[31m[ERROR::RESET]\x1b[0m {}", format!($($arg)*));
    };
}
#[macro_export]
macro_rules! fg_printc_success {
    ($($arg:tt)*) => {
        println!("\x1b[32m[SUCCESS::RESET]\x1b[0m {}", format!($($arg)*));
    };
}
#[macro_export]
macro_rules! fg_printc_info {
    ($($arg:tt)*) => {
        println!("\x1b[34m[INFO::RESET]\x1b[0m {}", format!($($arg)*));
    };
}

#[macro_export]
macro_rules! fg_printc_debug {
    ($($arg:tt)*) => {
        println!("\x1b[35m[DEBUG::RESET]\x1b[0m {}", format!($($arg)*));
    };
}
