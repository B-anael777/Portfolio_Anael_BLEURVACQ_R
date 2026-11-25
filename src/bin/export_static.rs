#[path = "../page.rs"]
mod page;

use std::{env, fs, path::Path};

const ASSETS: &[&str] = &[
    "1000046943.mp4",
    "aquarium.jpeg",
    "BLEURVACQ-Anael-SIO1-rapport de stage.pdf",
    "CV-BLEURVACQ_Anaël.pdf",
    "email.png",
    "Logo.NDS.jpg",
    "logo_SA.png",
    "photo.png.png",
    "photo_de_fond.jpg",
    "Procédure 1.docx",
    "Procédure 2.docx",
    "Procédure 3.docx",
    "Sph.png",
    "sphero_bolt.jpeg",
];

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let output = env::args().nth(1).unwrap_or_else(|| "public".to_string());
    let output = Path::new(&output);
    fs::create_dir_all(output)?;
    fs::write(output.join("index.html"), page::INDEX_HTML)?;

    for asset in ASSETS {
        fs::copy(asset, output.join(asset))?;
    }

    println!("Export statique généré dans {}", output.display());
    Ok(())
}
