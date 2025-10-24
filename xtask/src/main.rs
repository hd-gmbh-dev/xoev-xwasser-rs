use std::{
    fs::{create_dir_all, remove_dir_all},
    io::{self, Cursor, Read},
    path::Path,
    process::{Command, ExitCode},
};

use raxb::quick_xml::events::Event;

mod flags {
    xflags::xflags! {
        cmd x-task {
            default cmd set-version {
                required --version version: String
            }
            cmd get-version {}
            cmd patch {}
            cmd release {
                optional --no-tag
                optional --no-push
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
            flags::XTaskCmd::GetVersion(_) => {
                print!("{}", current_version()?);
            }
            flags::XTaskCmd::Patch(_) => {
                patch_version(&get_meta_version()?)?;
            }
            flags::XTaskCmd::Release(flags::Release { no_tag, no_push }) => {
                release(no_tag, no_push)?;
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

fn current_version() -> Result<String, String> {
    let output = Command::new("cargo")
        .arg("pkgid")
        .output()
        .expect("Failed to get current version using `cargo version`");

    if output.status.success() {
        let version =
            String::from_utf8(output.stdout).map_err(|_| "Failed to parse version".to_string())?;
        version
            .trim()
            .split('@')
            .nth(1)
            .map(str::to_string)
            .ok_or_else(|| format!("Failed to parse version: {version}"))
    } else {
        Err("Failed to get current version".to_string())
    }
}

fn get_meta_version() -> Result<String, String> {
    current_version().and_then(|version| {
        version
            .split('+')
            .nth(1)
            .map(str::to_string)
            .ok_or_else(|| format!("Could not get metainfo from {version}"))
    })
}

fn set_version(version: &str) -> Result<(), String> {
    println!("Setting version to {version}");
    if Command::new("cargo")
        .arg("set-version")
        .arg("--workspace")
        .arg(version)
        .status()
        .expect("Failed to set version using `cargo set-version`")
        .success()
    {
        set_package_json_versions(version)?;
        println!("Version set successfully");
    } else {
        println!("Failed to set version");
        return Err("Failed to set version".to_string());
    }
    Ok(())
}

fn patch_version(metadata: &str) -> Result<(), String> {
    println!("Patching version");
    if Command::new("cargo")
        .arg("set-version")
        .arg("--workspace")
        .arg("--bump")
        .arg("patch")
        .arg("--metadata")
        .arg(metadata)
        .status()
        .expect("Failed to set version using `cargo set-version`")
        .success()
    {
        let version = current_version()?;
        set_package_json_versions(&version)?;
        println!("Version set successfully");
    } else {
        println!("Failed to set version");
        return Err("Failed to set version".to_string());
    }
    Ok(())
}

fn set_package_json_versions(version: &str) -> Result<(), String> {
    for json_file in ["package.json", "package.tmp.json", "package.tmp.web.json"] {
        set_json_version(json_file, version)?;
    }
    Ok(())
}

fn set_json_version(json_file: &str, new_version: &str) -> Result<(), String> {
    println!("Setting version in {json_file} to {new_version}");
    use json_spanned_value::Spanned;
    #[derive(serde::Deserialize)]
    struct PackageJson {
        version: Spanned<String>,
    }
    let mut json_data = std::fs::read_to_string(json_file)
        .map_err(|err| format!("Failed to read {json_file}: {err}"))?;

    let PackageJson { version } = json_spanned_value::from_str(&json_data)
        .map_err(|err| format!("Failed to parse {json_file}: {err}"))?;

    let range = version.start() + 1..version.end() - 1;
    json_data.replace_range(range, new_version);

    std::fs::write(json_file, json_data)
        .map_err(|err| format!("Failed to write {json_file}: {err}"))?;
    Ok(())
}

fn release(no_tag: bool, no_push: bool) -> Result<(), Box<dyn std::error::Error>> {
    let version = current_version()?;
    cargo_update()?;
    cargo_build()?;
    git_add()?;
    git_commit(&format!("release {version}"))?;
    if !no_tag {
        git_tag(&format!("v{}", version))?;
    }
    if !no_push {
        git_push()?;
        if !no_tag {
            git_push_tag()?;
        }
    }
    Ok(())
}

fn git_add() -> Result<(), String> {
    let status = std::process::Command::new("git")
        .arg("add")
        .arg(".")
        .status()
        .expect("Failed to add changes using `git add`")
        .success();
    if status {
        println!("Changes added successfully");
    } else {
        println!("Failed to add changes");
        return Err("Failed to add changes".to_string());
    }
    Ok(())
}

fn cargo_update() -> Result<(), String> {
    let status = std::process::Command::new("cargo")
        .arg("update")
        .status()
        .expect("Failed to update dependencies using `cargo update`")
        .success();
    if status {
        println!("Dependencies updated successfully");
    } else {
        println!("Failed to update dependencies");
        return Err("Failed to update dependencies".to_string());
    }
    Ok(())
}

fn cargo_build() -> Result<(), String> {
    let status = std::process::Command::new("cargo")
        .arg("build")
        .status()
        .expect("Failed to build using `cargo build`")
        .success();
    if status {
        println!("Build successful");
    } else {
        println!("Build failed");
        return Err("Build failed".to_string());
    }
    Ok(())
}

fn git_commit(message: &str) -> Result<(), String> {
    let status = std::process::Command::new("git")
        .arg("commit")
        .arg("-m")
        .arg(message)
        .status()
        .expect("Failed to commit changes using `git commit`")
        .success();
    if status {
        println!("Changes committed successfully");
    } else {
        println!("Failed to commit changes");
        return Err("Failed to commit changes".to_string());
    }
    Ok(())
}

fn git_tag(tag: &str) -> Result<(), String> {
    let status = std::process::Command::new("git")
        .arg("tag")
        .arg(tag)
        .status()
        .expect("Failed to tag changes using `git tag`")
        .success();
    if status {
        println!("Changes tagged successfully");
    } else {
        println!("Failed to tag changes");
        return Err("Failed to tag changes".to_string());
    }
    Ok(())
}

fn git_push() -> Result<(), String> {
    let status = std::process::Command::new("git")
        .arg("push")
        .status()
        .expect("Failed to push changes using `git push`")
        .success();
    if status {
        println!("Changes pushed successfully");
    } else {
        println!("Failed to push changes");
        return Err("Failed to push changes".to_string());
    }
    Ok(())
}

fn git_push_tag() -> Result<(), String> {
    let status = std::process::Command::new("git")
        .arg("push")
        .arg("--tag")
        .status()
        .expect("Failed to push changes using `git push --tags`")
        .success();
    if status {
        println!("Changes pushed successfully");
    } else {
        println!("Failed to push changes");
        return Err("Failed to push changes".to_string());
    }
    Ok(())
}
