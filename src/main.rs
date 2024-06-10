use std::time::{SystemTime, UNIX_EPOCH};

fn nombre_aleatoire() -> u32 {
    let since_epoch = SystemTime::now().duration_since(UNIX_EPOCH)
        .expect("Le temps a reculé");
    let nanos = since_epoch.as_nanos();
    (nanos % 21) as u32
}

fn main() {
    let aleatoire = nombre_aleatoire();

    if aleatoire >= 18 {
        println!("Réussite critique");
    }else if aleatoire <= 2 {
        println!("Echec critique");
    } else {
        println!("Coup normal");
    }
}