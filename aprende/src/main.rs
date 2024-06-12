mod error_handling;
use error_handling::error_handler;

pub fn main() {
    hello();
    error_handler::file_loader();
}

pub fn hello() -> &'static str {
    println!("Running...");
    "world"
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn main_test() {
        assert_eq!(hello(), "world");
    }
}
