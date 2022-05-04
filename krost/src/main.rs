pub mod primitive;
pub mod render;
pub mod schema;
pub mod util;

use crate::render::{KrostField, KrostSchema, KrostStruct};
use crate::schema::SchemaType;
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

pub struct DecodeContext<D> {
    pub decoder: D,
    pub version: i16,
}

pub struct EncodeContext<E> {
    pub encoder: E,
    pub version: i16,
}

impl<E> EncodeContext<E> {
    pub fn new(encoder: E, version: i16) -> Self {
        EncodeContext { encoder, version }
    }
    pub fn into_inner(self) -> E {
        self.encoder
    }
}

impl<D> DecodeContext<D> {
    pub fn new(decoder: D, version: i16) -> Self {
        DecodeContext { decoder, version }
    }
    pub fn into_inner(self) -> D {
        self.decoder
    }
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
    fn decode<D: io::Read>(ctx: &mut DecodeContext<D>) -> Result<Self, KrostError>;
    fn encode<E: io::Write>(&self, ctx: &mut EncodeContext<E>) -> Result<usize, KrostError>;
}

fn expand_fields(
    api_key: Option<i16>,
    schema_type: SchemaType,
    structs: &mut Vec<KrostStruct>,
    fields: &[schema::Field],
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
                let substruct_fields = expand_fields(api_key, schema_type, structs, subfields);
                let substruct = KrostStruct {
                    struct_name: substruct_name,
                    schema_type,
                    api_key,
                    fields: substruct_fields,
                };
                structs.push(substruct);
                converted_fields.push(field_spec)
            }
        }
    }
    converted_fields
}

fn expand_schema(schema: schema::Schema) -> KrostSchema {
    let name = schema.name;
    let mut structs = vec![];

    let root_fields = expand_fields(schema.api_key, schema.r#type, &mut structs, &schema.fields);
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
    let buf = buf.join(
        PathBuf::from_str("clients/src/main/resources/common/message")
            .expect("could not form path from schema path"),
    );
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
    let path = Path::new("/Users/iamazy/Documents/GitHub/kafka");
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
        // fs::write(API_KEY_TARGET, api_key_file_contents.to_string()).expect("Unable to write file");
        println!("generated modules");
    }
}