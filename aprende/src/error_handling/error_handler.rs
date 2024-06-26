use serde_derive::*;

#[derive(Debug)]
pub enum TransactionError {
    LoadError(std::io::Error),
    ParseError(serde_json::Error),
    Mess(&'static str),
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Transaction {
    from: String,
    to: String,
    amount: u64,
}
impl From<std::io::Error> for TransactionError {
    fn from(e: std::io::Error) -> Self {
        TransactionError::LoadError(e)
    }
}

impl From<serde_json::Error> for TransactionError {
    fn from(e: serde_json::Error) -> Self {
        TransactionError::ParseError(e)
    }
}

impl From<&'static str> for TransactionError {
    fn from(e: &'static str) -> Self {
        TransactionError::Mess(e)
    }
}

pub fn all_transactions() {
    println!("Loading data!");
    let trxs = get_transactions_b("data/transactions.json").expect("Could not load transactions");
    for t in trxs {
        println!("{:?}", t);
    }
}

pub fn get_transaction_to(to_name: &str) -> Option<Transaction> {
    let trxs = get_transactions_b("data/transactions.json").ok()?;
    for t in trxs {
        if t.to == to_name {
            return Some(t);
        }
    }
    None
}

fn get_transactions(fname: &str) -> Result<Vec<Transaction>, String> {
    let s = match std::fs::read_to_string(fname) {
        Ok(r) => r,
        Err(e) => return Err(e.to_string()),
    };
    let t: Vec<Transaction> = match serde_json::from_str(&s) {
        Ok(v) => v,
        Err(e) => return Err(e.to_string()),
    };
    Ok(t)
}

fn get_transactions_b(fname: &str) -> Result<Vec<Transaction>, TransactionError> {
    /* std::fs::read_to_string(fname)
    .map_err(|e| e.into())
    .and_then(|loaded| serde_json::from_str(&loaded).map_err(|e| e.into()))
    */
    Ok(
        match serde_json::from_str(&match std::fs::read_to_string(fname) {
            Ok(v) => v,
            Err(e) => return Err(e.into()),
        }) {
            Ok(v) => v,
            Err(e) => return Err(e.into()),
        },
    )
    // Ok(serde_json::from_str(&td::fs::read_to_string(fname)?)?)
}
