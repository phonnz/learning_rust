use serde_derive::*;

#[derive(Deserialize, Serialize, Debug)]
pub struct Transaction {
    from: String,
    to: String,
    amount: u64,
}

pub fn file_loader() {
    println!("Loading data!");
    let trxs = get_transactions("data/transactions.json").expect("Could not load transactions");
    for t in trxs {
        println!("{:?}", t);
    }
}

fn get_transactions(fname: &str) -> Result<Vec<Transaction>, String> {
    //    Ok(Vec::new())
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
