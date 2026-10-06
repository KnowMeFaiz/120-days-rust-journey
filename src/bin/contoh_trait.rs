struct Wallet {
    nama: String,
    balance: f64,
}

struct NFT {
    nama: String,
    rarity: u8,
}
trait Describe {
    fn describe(&self) -> String;
}

impl Describe for Wallet {
    fn describe(&self) -> String {
        format!("Wallet '{}' - balance: {:.2}", self.nama, self.balance)
    }
}
impl Describe for NFT {
    fn describe(&self) -> String {
        format!("NFT '{}' - rarity: {}/100", self.nama, self.rarity)
    }
}

fn main() {

    let aset: Vec<Box<dyn Describe>> = vec![
    Box::new(Wallet { nama: String::from("dompetku"), balance: 200.0 }),
    Box::new(NFT { nama: String::from("dompetku"), rarity: 22 }),
];

for item in &aset {
    println!("{}", item.describe());
}
}
