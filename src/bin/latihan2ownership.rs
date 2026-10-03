fn main () {
    let dompet_a = String::from("NFT kera goblok");

    println!("awalnya, Dompet A berisi: {}", dompet_a);

    let dompet_b = &dompet_a;

    println!("Setelah Transfer, Dompet B berisi: {}", dompet_b);

    println!("coba cek dompet A: {}", dompet_a);
}

//testing wakatime