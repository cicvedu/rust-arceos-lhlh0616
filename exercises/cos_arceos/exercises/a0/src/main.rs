#![cfg_attr(feature = "axstd", no_std)]
#![cfg_attr(feature = "axstd", no_main)]

#[cfg(feature = "axstd")]
use axstd::println;



// TODO: Implement macro println_prefix.

#[macro_export]
macro_rules! println_prefix {
    ($prefix:expr,) => {
        println!("{}", $prefix);
    };
    ($prefix:expr, $fmt:expr, $($arg:tt)*) => {
        println!("{}{}", $prefix, format!($fmt, $($arg)*));
    };
}


#[cfg(feature = "axstd")]
use axstd::println_prefix;

#[cfg_attr(feature = "axstd", no_mangle)]
fn main() {
    println!("Hello, world!");

    let times = 2;
    println_prefix!("Stdout: ", "Hello, world![{}]", times);

    println!("\n[ArceOS Tutorial]: A0 okay!");
}
