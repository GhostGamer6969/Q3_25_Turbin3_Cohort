#[test]
fn base58_to_wallet() {
    // println!("Enter your name:");
    // let stdin = io::stdin();
    let base58 = "";
    let wallet = bs58::decode(base58).into_vec().unwrap();
    println!("{:?}", wallet);
}
#[test]
fn wallet_to_base58() {
    let wallet: Vec<u8> = vec![];
    let base58 = bs58::encode(wallet).into_string();
    println!("{:?}", base58);
}
