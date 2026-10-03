fn main () {
    println!("=== Program dimulai");
    
    sapa_pemain();

    let saldo = 5000;

    cek_status_sultan( saldo);

    println!("=== Program selesai");

}

fn sapa_pemain() {
    println!("halo rakyat web3.");

}

fn cek_status_sultan(uang: i32) {
    if uang >= 1000 {
        println!("Status: Anda adalah seorang sultan karna uang ada {}", uang);
    } else {
        println!("Status: anda kurang uang, anda bukan sultan karna {}", uang);
    }
}