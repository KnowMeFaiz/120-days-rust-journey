fn main () {
    let mut saldo_uang = 1000;
    let harga_nft = 2000;

    println!("Saldo kamu saat ini: {}", saldo_uang);
    println!("kamu mau beli nft seharga: {}", harga_nft);

    if saldo_uang >= harga_nft {
        println!("hore transaksi kamu berhasil beli nft ini");

        saldo_uang = saldo_uang - harga_nft;
        println!("sisa saldo kamu sekarang: {}", saldo_uang);
    }    else {
            println!("transaksi kamu gagal, skill issue");
        }
    
}