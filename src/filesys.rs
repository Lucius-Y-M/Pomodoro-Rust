
use std::fs::File;
use std::io::{Write, Result};

pub fn save_file(mut file_name: String) -> Result<()> {


    let mut file = File::create({
        file_name.push_str("ini");
        file_name
    })?;

    // write INI content
    writeln!(file, "[General]")?;
    writeln!(file, "")?;

    Ok(())
}

