use std::{error::Error, path::PathBuf, str::FromStr};

use regex::Regex;

fn main() -> Result<(), Box<dyn Error>> {
    let re = Regex::new(r"(day_\d+).txt")?;

    let out_dir = std::env::var("OUT_DIR").unwrap();
    let path = PathBuf::from_str(&out_dir)?;

    std::fs::create_dir_all(path.join("src/bin"))?;

    let dir = std::fs::read_dir("inputs")?;
    for input in dir {
        let input = input?;
        let input = input.file_name();
        let s = input.to_string_lossy();

        if let Some(input) = re.captures(&s) {
            let day = input.get(1).unwrap().as_str();

            std::fs::copy(
                format!("inputs/{}.txt", day),
                path.join(format!("src/bin/{}.rs.txt", day)),
            )?;
        }
    }

    Ok(())
}
