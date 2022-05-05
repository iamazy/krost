pub mod primitive;
pub mod record;
pub mod render;
pub mod schema;
pub mod util;

use crate::render::{KrostField, KrostSchema, KrostStruct};
use crate::schema::{SchemaType, Versions};
use std::io::BufRead;
use std::path::{Path, PathBuf};
use std::str::FromStr;
use std::{fs, io};
use thiserror::Error;

pub trait Krost: KrostType {
    fn version_added() -> i16;
    fn version_removed() -> Option<i16>;
    fn apikey() -> i16;
}

#[derive(Error, Debug)]
pub enum KrostError {
    #[error("Cannot read/write data: {0}")]
    IO(#[from] std::io::Error),

    #[error("Overflow converting integer: {0}")]
    Overflow(#[from] std::num::TryFromIntError),

    #[error("Malformed data: {0}")]
    Malformed(#[from] Box<dyn std::error::Error + Send + Sync>),

    #[error("Invalid version: min={min:?}, max={max:?}, version={version:?}")]
    InvalidVersion {
        min: i16,
        max: Option<i16>,
        version: i16,
    },
}

pub trait KrostType: Sized {
    fn decode<D: io::Read>(buf: &mut D) -> Result<Self, KrostError>;
    fn encode<E: io::Write>(&self, buf: &mut E) -> Result<usize, KrostError>;
}

fn expand_fields(
    api_key: Option<i16>,
    schema_type: SchemaType,
    structs: &mut Vec<KrostStruct>,
    fields: &[schema::Field],
    flexible_versions: Option<i16>,
) -> Vec<KrostField> {
    let mut converted_fields = vec![];
    for field in fields {
        let field_spec = KrostField::from_schema(field, flexible_versions);
        match &field.fields {
            None => {
                // This is a leaf field, we don't need to do anything besides add it to the vec
                converted_fields.push(field_spec);
            }
            Some(subfields) => {
                // There are subfields for this schema, creating a new struct is necessary.
                let substruct_name = field_spec.type_name.clone();
                let mut substruct_fields = expand_fields(api_key, schema_type, structs, subfields, flexible_versions);
                if let Some(start) = flexible_versions {
                    substruct_fields.push(KrostField::tagged_fields(start));
                }
                let substruct = KrostStruct {
                    struct_name: substruct_name,
                    schema_type,
                    api_key,
                    fields: substruct_fields,
                };
                structs.push(substruct);
                converted_fields.push(field_spec);
            }
        }
    }
    converted_fields
}

fn expand_schema(schema: schema::Schema) -> KrostSchema {
    let name = schema.name;
    let mut structs = vec![];

    let flexible_versions = if let Some(Versions::Since(start) | Versions::Exact(start) | Versions::Range(start, _)) = schema.flexible_versions {
        Some(start)
    } else {
        None
    };

    let mut root_fields = expand_fields(schema.api_key, schema.r#type, &mut structs, &schema.fields, flexible_versions);
    if let Some(start) = flexible_versions {
        root_fields.push(KrostField::tagged_fields(start));
    }
    KrostSchema {
        name,
        r#type: schema.r#type,
        api_key: schema.api_key,
        fields: root_fields,
        version_added: schema.valid_versions.version_start(),
        version_removed: schema.valid_versions.version_end(),
        structs,
    }
}

fn parse_schema_file(path: &Path) -> Result<schema::Schema, KrostError> {
    let file = fs::File::open(path).expect("could not open schema file");
    let reader = io::BufReader::new(file);
    let file_contents: String = reader
        .lines()
        .map(|line| line.expect("invalid line"))
        .filter(|line| !line.contains("//"))
        .collect::<Vec<String>>()
        .join("");
    let mut schema_deserializer = serde_json::Deserializer::from_str(&file_contents);
    let schema: schema::Schema = serde::Deserialize::deserialize(&mut schema_deserializer)
        .map_err(|e| KrostError::Malformed(Box::new(e)))?;
    Ok(schema)
}

fn collect_paths(path: &Path) -> Vec<PathBuf> {
    let buf = path.to_path_buf();
    println!(
        "collecting schema files in {}",
        buf.to_str().expect("could not convert path to string")
    );
    std::fs::read_dir(&buf)
        .expect("could not read directory")
        .filter(|p| p.is_ok())
        .map(|p| p.unwrap())
        .filter(|p| p.file_name().to_str().unwrap().contains("json"))
        .filter(|p| {
            p.file_name().to_str().unwrap().contains("Request")
                | p.file_name().to_str().unwrap().contains("Response")
        })
        .filter(|p| !p.file_name().to_str().unwrap().contains("RequestHeader"))
        .filter(|p| !p.file_name().to_str().unwrap().contains("ResponseHeader"))
        .map(|p| p.path())
        .collect()
}

fn main() {
    let path = Path::new("message");
    let paths = collect_paths(path);
    let mut root_specs = vec![];
    for path in &paths[..] {
        let schema = parse_schema_file(path)
            .unwrap_or_else(|e| panic!("could not parse schema file {:?}, e: {:?}", path, e));
        let root_rpc_spec = expand_schema(schema);
        root_specs.push(root_rpc_spec);
    }

    let grouped_specs = render::group_schema_specs(root_specs);
    let api_file_contents = render::gen_api_file_contents(&grouped_specs);

    if let Ok(file) = syn::parse_file(&api_file_contents.to_string()) {
        let pretty = prettyplease::unparse(&file);
        fs::write("krost/tests/krost.rs", pretty).expect("Unable to write file");
        println!("generated modules");
    }
}
