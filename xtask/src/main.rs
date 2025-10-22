use std::{
    fs::{create_dir_all, remove_dir_all},
    io::{self, Cursor, Read},
    path::Path,
    process::ExitCode,
};

use raxb::quick_xml::events::Event;

mod flags {
    xflags::xflags! {
        cmd x-task {
            default cmd set-version {
                required --version version: String
            }
        }
    }
}

impl flags::XTask {
    fn process(self) -> Result<(), Box<dyn std::error::Error>> {
        match self.subcommand {
            flags::XTaskCmd::SetVersion(flags::SetVersion { version }) => {
                set_version(&version)?;
                fetch_codelists(&version)?;
            }
        }
        Ok(())
    }
}

fn fetch_codelists(version: &str) -> Result<(), Box<dyn std::error::Error>> {
    let version = version
        .split('+')
        .nth(1)
        .expect("version of XWasser after + sign");
    let file_path_version = format!("v{}", version.replace('.', "_"));
    let url = format!("https://www.xrepository.de/api/version_standard/urn:xoev-de:lgl:standard:xwasser_{version}/genutzteAktuelleCodelisten");
    println!("Fetching codelists from {}", url);
    let response = reqwest::blocking::get(url)?.error_for_status()?.bytes()?;
    let zip_data = Cursor::new(response);

    let mut zip_file = zip::ZipArchive::new(zip_data)?;
    let target_dir = Path::new("crates/codelists/data").join(file_path_version.to_uppercase());
    if target_dir.exists() {
        remove_dir_all(&target_dir)?;
    }
    if !target_dir.exists() {
        create_dir_all(&target_dir)?;
    }
    for i in 0..zip_file.len() {
        let mut file = zip_file.by_index(i)?;
        let filename = target_dir.join(file.name());
        println!("Extracting {}", filename.display());
        if matches!(filename.extension(), Some(ext) if ext == "xml") {
            let mut data = Vec::new();
            file.read_to_end(&mut data)?;
            let mut rdr = raxb::quick_xml::reader::Reader::from_reader(std::io::Cursor::new(&data));
            let mut buf = Vec::new();
            let saved = loop {
                match rdr.read_event_into(&mut buf) {
                    Ok(Event::Decl(decl)) => match decl.encoding() {
                        Some(Ok(encoding)) => {
                            if encoding.as_ref() == b"ISO-8859-1" {
                                let encoder = encoding_rs::Encoding::for_label(b"iso-8859-1")
                                    .ok_or(String::from("unknown encoding ISO_8859_1"))?;
                                let (result, _) = encoder.decode_with_bom_removal(&data);
                                eprintln!(
                                    "Converted encoding from ISO-8859-1 to UTF-8 for {}",
                                    filename.display()
                                );
                                let s = result.replace("ISO-8859-1", "UTF-8");
                                std::fs::write(&filename, s.as_bytes())?;
                                break true;
                            }
                        }
                        _ => break false,
                    },
                    Err(err) => {
                        eprintln!("{err:#?}");
                        std::process::exit(1);
                    }
                    _ => {
                        break false;
                    }
                }
            };
            if !saved {
                std::fs::write(&filename, data)?;
            }
        } else {
            let mut output = std::fs::File::create(filename)?;
            io::copy(&mut file, &mut output)?;
        }
    }

    Ok(())
}

fn main() -> ExitCode {
    let flags = flags::XTask::from_env_or_exit();
    match flags.process() {
        Ok(_) => ExitCode::SUCCESS,
        Err(err) => {
            eprintln!("Error: {}", err);
            ExitCode::FAILURE
        }
    }
}

fn set_version(version: &str) -> Result<(), String> {
    println!("Setting version to {version}");
    if std::process::Command::new("cargo")
        .arg("set-version")
        .arg("--workspace")
        .arg(version)
        .status()
        .expect("Failed to set version using `cargo set-version`")
        .success()
    {
        println!("Version set successfully");
    } else {
        println!("Failed to set version");
        return Err("Failed to set version".to_string());
    }
    Ok(())
}
