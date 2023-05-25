use std::io;

fn main() {
    // Meminta input nama dari pengguna
    println!("Masukkan nama Anda:");
    let mut nama = String::new();
    io::stdin().read_line(&mut nama).expect("Gagal membaca baris");

    // Meminta input usia dari pengguna
    println!("Masukkan usia Anda:");
    let mut usia = String::new();
    io::stdin().read_line(&mut usia).expect("Gagal membaca baris");

    // Mengonversi input usia ke tipe data integer
    let usia: u32 = match usia.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Usia yang dimasukkan tidak valid");
            return;
        }
    };

    // Mencetak pesan sapaan dan informasi usia
    println!("Halo, {}!", nama.trim());
    println!("Anda berusia {} tahun.", usia);
}
