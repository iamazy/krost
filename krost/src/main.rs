pub mod record;
pub mod render;
pub mod schema;
pub mod types;
pub mod util;

use crate::render::{KrostField, KrostSchema, KrostStruct};
use crate::schema::Versions;
use std::io::BufRead;
use std::path::{Path, PathBuf};
use std::{fs, io};
use thiserror::Error;

pub trait Krost: KrostType {
    fn version_added() -> Option<i16>;
    fn version_removed() -> Option<i16>;
    fn apikey() -> Option<i16>;
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
        min: Option<i16>,
        max: Option<i16>,
        version: i16,
    },
}

pub trait KrostType: Sized {
    fn decode<D: io::Read>(buf: &mut D, version: i16) -> Result<Self, KrostError>;
    fn encode<E: io::Write>(&self, buf: &mut E, version: i16) -> Result<usize, KrostError>;
}

fn expand_struct(
    api_key: Option<i16>,
    structs: &mut Vec<KrostStruct>,
    common_structs: &[schema::Struct],
    flexible_versions: Versions,
) {
    for common_struct in common_structs {
        let substruct_name = common_struct.name.clone();
        let mut substruct_fields =
            expand_fields(api_key, structs, &common_struct.fields, flexible_versions);
        if !matches!(flexible_versions, Versions::None) {
            substruct_fields.push(KrostField::tagged_fields(flexible_versions));
        }
        let substruct = KrostStruct {
            struct_name: substruct_name,
            fields: substruct_fields,
        };
        structs.push(substruct);
    }
}

fn expand_fields(
    api_key: Option<i16>,
    structs: &mut Vec<KrostStruct>,
    fields: &[schema::Field],
    flexible_versions: Versions,
) -> Vec<KrostField> {
    let mut converted_fields = vec![];
    for field in fields {
        let field_spec = KrostField::from_schema(field);
        match &field.fields {
            None => {
                // This is a leaf field, we don't need to do anything besides add it to the vec
                converted_fields.push(field_spec);
            }
            Some(subfields) => {
                // There are subfields for this schema, creating a new struct is necessary.
                let substruct_name = field_spec.type_name.clone();
                let mut substruct_fields =
                    expand_fields(api_key, structs, subfields, flexible_versions);
                if !matches!(flexible_versions, Versions::None) {
                    substruct_fields.push(KrostField::tagged_fields(flexible_versions));
                }
                let substruct = KrostStruct {
                    struct_name: substruct_name,
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
    expand_struct(
        schema.api_key,
        &mut structs,
        &schema.common_structs,
        schema.flexible_versions,
    );
    let mut root_fields = expand_fields(
        schema.api_key,
        &mut structs,
        &schema.fields,
        schema.flexible_versions,
    );
    if !matches!(schema.flexible_versions, Versions::None) {
        root_fields.push(KrostField::tagged_fields(schema.flexible_versions));
    }
    KrostSchema {
        name,
        r#type: schema.r#type,
        api_key: schema.api_key,
        fields: root_fields,
        versions: schema.valid_versions,
        structs,
        flexible_versions: schema.flexible_versions,
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
        .map(|p| p.path())
        .collect()
}

fn main() {
    let path = Path::new("message");
    let paths = collect_paths(path);
    let mut root_specs = vec![];
    let mut header_specs = vec![];
    for path in &paths[..] {
        let schema = parse_schema_file(path)
            .unwrap_or_else(|e| panic!("could not parse schema file {:?}, e: {:?}", path, e));
        let root_rpc_spec = expand_schema(schema);
        if root_rpc_spec.r#type == schema::SchemaType::Header {
            header_specs.push(root_rpc_spec);
        } else {
            root_specs.push(root_rpc_spec);
        }
    }

    let headers_specs = render::gen_header_contents(header_specs);
    let grouped_specs = render::group_schema_specs(root_specs);
    let mut api_file_contents = render::gen_api_file_contents(&grouped_specs);
    api_file_contents.extend(headers_specs);

    if let Ok(file) = syn::parse_file(&api_file_contents.to_string()) {
        let pretty = prettyplease::unparse(&file);
        fs::write("krost/tests/krost.rs", pretty).expect("Unable to write file");
        println!("generated modules");
    }
}
