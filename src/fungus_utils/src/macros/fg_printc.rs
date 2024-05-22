#[macro_export]
macro_rules! fg_printc_error {
    ($($arg:tt)*) => {
        println!("\x1b[31m[ERROR::]\x1b[0m {}", format!($($arg)*));
    };
}
#[macro_export]
macro_rules! fg_printc_success {
    ($($arg:tt)*) => {
        println!("\x1b[32m[SUCCESS::]\x1b[0m {}", format!($($arg)*));
    };
}
#[macro_export]
macro_rules! fg_printc_info {
    ($($arg:tt)*) => {
        println!("\x1b[34m[INFO::]\x1b[0m {}", format!($($arg)*));
    };
}

#[macro_export]
macro_rules! fg_printc_debug {
    ($($arg:tt)*) => {
        println!("\x1b[35m[DEBUG::]\x1b[0m {}", format!($($arg)*));
    };
}
