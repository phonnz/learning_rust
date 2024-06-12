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
    //Err("No transactions".to_string())
    //    Ok(Vec::new())
    let s = std::fs::read_to_string(fname).unwrap();
    let t: Vec<Transaction> = serde_json::from_str(&s).unwrap();
    Ok(t)
}
