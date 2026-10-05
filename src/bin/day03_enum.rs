enum Status {
    Aktif,
    Dibekukan,
    Ditutup { alasan: String },
}

fn cek(status: &Status) -> String {
    match status {
        Status::Aktif => String::from("Dompet Aktif"),
        Status::Dibekukan => String::from("Dompet Dibekukan"),
        Status::Ditutup { alasan } => format!("Ditutup: {}", alasan),
    }
}

fn main() {

    let a = Status::Aktif;
    let b = Status::Ditutup { alasan: String::from("rugpull") };
    let c = Status::Dibekukan;

    println!("{}", cek(&a));
    println!("{}", cek(&b));
    println!("{}", cek(&c));
}