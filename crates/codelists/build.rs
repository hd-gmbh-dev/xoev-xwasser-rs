use std::io::Write;
use std::path::{Path, PathBuf};

#[path = "src/cl/mod.rs"]
mod cl;

use cl::model::{CodeList, DataSource};

use raxb::de::from_str;
use raxb::quick_xml::events::Event;

#[derive(serde::Serialize)]
pub struct Data<T> {
    items: Vec<T>,
}

impl<T> Default for Data<T> {
    fn default() -> Self {
        Self { items: Vec::new() }
    }
}

pub struct DataSet<T> {
    file: std::fs::File,
    data: Data<T>,
}

impl<T> DataSet<T> {
    pub fn insert(&mut self, entry: T) {
        self.data.items.push(entry);
    }
}

impl<T> DataSet<T>
where
    T: serde::ser::Serialize,
{
    pub fn save(&mut self) -> anyhow::Result<()> {
        self.file.write_all(&bson::to_vec(&self.data)?)?;
        Ok(())
    }
}

pub struct DataSetFactory {
    data_set_path: PathBuf,
}

impl DataSetFactory {
    pub fn new<P: AsRef<Path>>(data_set_path: P) -> anyhow::Result<Self> {
        if !data_set_path.as_ref().exists() {
            std::fs::create_dir_all(data_set_path.as_ref())?;
        }
        Ok(Self {
            data_set_path: data_set_path.as_ref().to_path_buf(),
        })
    }

    pub fn create<T>(&self, items: Vec<T>) -> anyhow::Result<DataSet<T>>
    where
        T: DataSource,
    {
        let file_path = self.data_set_path.join(T::name());
        if file_path.exists() {
            std::fs::remove_file(&file_path)?;
        }
        Ok(DataSet {
            file: std::fs::File::create(&file_path)?,
            data: Data { items },
        })
    }
}

fn write_codelist_docs(s: &mut String, codelist: &CodeList) -> anyhow::Result<()> {
    use std::fmt::Write;

    let field_ids = codelist
        .header
        .fields
        .iter()
        .enumerate()
        .map(|(id, v)| format!("`{}` ({id})", v.id.as_ref()))
        .collect::<Vec<String>>();

    writeln!(
        s,
        "## {}",
        codelist.header.identification.long_name.as_ref()
    )?;
    writeln!(s)?;
    writeln!(s, "{}", codelist.header.description.codelist_description)?;
    writeln!(s)?;
    writeln!(s, "| | |")?;
    writeln!(s, "| -- | -- |")?;
    writeln!(
        s,
        "| short name | {} |",
        codelist.header.identification.short_name.as_ref()
    )?;
    writeln!(
        s,
        "| canonical uri | `{}` |",
        codelist.header.identification.canonical_uri.as_ref()
    )?;
    writeln!(
        s,
        "| canonical version uri | {} |",
        codelist
            .header
            .identification
            .canonical_version_uri
            .as_ref()
    )?;
    writeln!(s)?;
    writeln!(s, "| Field | {} |", field_ids.join(" | "))?;
    write!(s, "|")?;
    for _ in 0..field_ids.len() {
        write!(s, " -- |")?;
    }
    writeln!(s, " -- |")?;
    write!(s, "| type ")?;
    for field in codelist.header.fields.iter() {
        write!(s, "| {:?} ", field.field_type)?;
    }
    writeln!(s, "|")?;
    write!(s, "| Usage ")?;
    for field in codelist.header.fields.iter() {
        write!(s, "| {:?} ", field.usage)?;
    }
    writeln!(s, "|")?;
    write!(s, "| Lang ")?;
    for field in codelist.header.fields.iter() {
        write!(s, "| {:?} ", field.lang)?;
    }
    writeln!(s, "|")?;

    writeln!(s)?;

    Ok(())
}

fn write_readme(items: &[CodeList], version: &str) -> anyhow::Result<String> {
    let mut content = format!("# XWasser codelists - version `{version}`\n\n");

    for item in items {
        write_codelist_docs(&mut content, item)?;
    }

    Ok(content)
}

fn main() -> anyhow::Result<()> {
    let public_out = Path::new("./public");
    let data_dir = Path::new("./data");
    let versions = std::fs::read_dir(data_dir)?;
    for e in versions {
        let version = e?.file_name();
        let version_dir = data_dir.join(&version);
        let json_dir = public_out.join(&version);
        let xml_files = std::fs::read_dir(&version_dir)?;
        let mut items = Vec::new();
        let mut json_content = Vec::new();
        std::fs::create_dir_all(&json_dir)?;
        let json_file = json_dir.join("codelist.json");
        let readme_file = json_dir.join("README.md");
        for xml_file in xml_files {
            let xml_file = xml_file?;
            eprintln!("process {}", xml_file.path().display());
            if xml_file
                .path()
                .extension()
                .map(|ext| ext.to_ascii_lowercase().to_string_lossy().as_ref() == "xml")
                .unwrap_or(false)
            {
                let xml = std::fs::read(xml_file.path())?;
                let mut rdr =
                    raxb::quick_xml::reader::Reader::from_reader(std::io::Cursor::new(xml.clone()));
                let mut buf = Vec::new();
                let mut s = String::default();
                let mut is_valid_utf_8 = true;
                loop {
                    match rdr.read_event_into(&mut buf) {
                        Ok(Event::Decl(decl)) => match decl.encoding() {
                            Some(Ok(encoding)) => {
                                if encoding.as_ref() == b"ISO-8859-1" {
                                    let encoder = encoding_rs::Encoding::for_label(b"iso-8859-1")
                                        .ok_or(anyhow::anyhow!(
                                        "unknown encoding ISO_8859_1"
                                    ))?;
                                    let (result, valid_to) = encoder.decode_with_bom_removal(&xml);
                                    eprintln!("{valid_to}");
                                    s = result.replace("ISO-8859-1", "UTF-8");
                                    std::fs::write(xml_file.path(), s.as_bytes())?;
                                    is_valid_utf_8 = false
                                }
                            }
                            _ => break,
                        },
                        Err(err) => {
                            eprintln!("{err:#?}");
                            std::process::exit(1);
                        }
                        _ => {
                            break;
                        }
                    }
                }
                if is_valid_utf_8 {
                    s = String::from_utf8(rdr.into_inner().into_inner())?;
                }
                let result: cl::parser::input::CodeList =
                    from_str(&s).expect("unable to deserialize xml");
                let parsed: CodeList = result.into();
                if !parsed.values.is_empty() {
                    let json_obj = serde_json::to_string(&parsed)?;
                    json_content.push(json_obj);
                    items.push(parsed);
                }
            }
        }
        std::fs::write(
            &readme_file,
            &write_readme(&items, version.to_str().unwrap())?,
        )?;
        DataSetFactory::new(
            PathBuf::from(std::env::var("OUT_DIR").expect("OUT_DIR variable"))
                .join("data")
                .join(version),
        )?
        .create::<CodeList>(items)?
        .save()?;
        std::fs::write(
            &json_file,
            format!("[\n  {}\n]", json_content.join(",\n  ")),
        )?;
    }
    Ok(())
}
