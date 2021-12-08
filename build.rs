//! This build script generates the KANA_TO_ROMANJI phf::Map
//! from the kana_to_romanji.csv file at the project root.
use std::fs::File;
use std::io::{BufWriter, Write};
use std::path::Path;

type BuildResult = Result<(), Box<dyn std::error::Error>>;


fn main() {
    generate_kana_to_romanji(
        Path::new("kana_to_romanji.csv"),
        &Path::new("src").join("kana_to_romanji_codegen.rs")
    ).expect("failed to generate KANA_TO_ROMANJI");
}

fn generate_kana_to_romanji(csv_path: &Path, out_path: &Path) -> BuildResult {
    let mut reader = csv::Reader::from_path(csv_path)?;
    let mut file = BufWriter::new(File::create(out_path)?);

    let mut phf_map = phf_codegen::Map::new();
    for line in reader.records() {
        let record = line?;
        assert_eq!(record.len(), 3);

        let (kana, romanji) = (record[0].to_string(), record[1].to_string());
        let alt_romanji = {
            if record[2].is_empty() {
                None
            } else {
                Some(record[2].to_string())
            }
        };
        phf_map.entry(
            kana.clone(),
            &format!(
                "KanaTranslation {{ kana: {:?}, romanji: {:?}, alt_romanji: {:?} }}",
                kana,
                romanji,
                alt_romanji
            )
        );
    }
    writeln!(
        &mut file,
        "static KANA_TO_ROMANJI: phf::Map<&'static str, KanaTranslation> = {};\n",
        phf_map.build()
    )?;
    Ok(())
}
