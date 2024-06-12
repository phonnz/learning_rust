mod messager;

pub fn main() -> () {
    hello();
    messager::message();
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
