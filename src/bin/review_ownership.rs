fn main() {
    let a = String::from("Faiz");
    ambil(&a);
    println!("{}", a);
}

fn ambil(s: &String) {
    println!("{}", s)
}
