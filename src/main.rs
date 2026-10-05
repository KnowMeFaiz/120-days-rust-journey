struct Wallet {
    nama: String,
    balance: f64,
}

trait Describe {
    fn describe(&self) -> String;
}

impl Describe for Wallet {
    fn describe(&self) -> String {
        format!("Wallet '{}' - balance: {:.2}", self.nama, self.balance)
    }
}

fn main() {
    let aset: Vec<Box<dyn Describe>> = vec![Box::new(Wallet {
        nama: String::from("Dompet gw"),
        balance: 99.5,
    })];

    for item in &aset {
        println!("{}", item.describe());
    }
}
