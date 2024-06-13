mod error_handling;
use error_handling::error_handler;
use error_handling::error_handler::TransactionError;

pub fn main() -> Result<(), TransactionError> {
    // hello();
    error_handler::all_transactions();
    let trx = error_handler::get_transaction_to("nobody").ok_or("Could not find a transaction");
    println!("First transaction: {:?}", trx);
    Ok(())
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
