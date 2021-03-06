use crate::schema::Versions;
use crate::{schema, KrostError};
use heck::ToSnakeCase;
use itertools::Itertools;
use proc_macro2::TokenStream;
use quote::{quote, ToTokens};
use std::cmp::Ordering;
use std::collections::{BTreeMap, HashMap};
use std::io::BufRead;
use std::path::{Path, PathBuf};
use std::{fs, io};

#[derive(Debug, Clone)]
pub(crate) struct KrostSchema {
    pub(crate) name: String,
    pub(crate) r#type: schema::SchemaType,
    pub(crate) api_key: Option<i16>,
    pub(crate) fields: Vec<KrostField>,
    pub(crate) structs: Vec<KrostStruct>,
    pub(crate) versions: Versions,
    pub(crate) flexible_versions: Versions,
}

impl ToTokens for KrostSchema {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let full_struct_name = self.name.clone();
        let struct_name_ident = to_ident(&full_struct_name);
        let struct_fields = &self.fields;
        let substructs = &self.structs;
        let versions = self.versions.to_string();
        let versions_ident = quote! { versions = #versions, };
        let flexible_versions = match self.flexible_versions {
            Versions::None => None,
            versions => {
                let v = versions.to_string();
                Some(quote! { flexible = #v})
            }
        };
        let api_key = self.api_key;
        let api_key_ident = if api_key.is_some() {
            quote! {apikey = #api_key, }
        } else {
            TokenStream::new()
        };

        tokens.extend(quote! {
            #[derive(Debug, PartialEq, krost_derive::Message, Clone)]
            #[kafka(#api_key_ident #versions_ident #flexible_versions)]
            pub struct #struct_name_ident {
               #(#struct_fields),*
            }
            #(#substructs)*
        })
    }
}

#[derive(Debug, Clone)]
pub(crate) struct KrostStruct {
    pub(crate) struct_name: String,
    pub(crate) fields: Vec<KrostField>,
}

impl ToTokens for KrostStruct {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let full_struct_name = self.struct_name.to_owned();
        let struct_name_ident = to_ident(&full_struct_name);
        let struct_fields = &self.fields;
        tokens.extend(quote! {
            #[derive(Debug, PartialEq, krost_derive::Message, Clone)]
            pub struct #struct_name_ident {
               #(#struct_fields),*
            }
        })
    }
}

#[derive(Debug, Clone)]
pub(crate) struct KrostField {
    pub(crate) collection: bool,
    pub(crate) nullable_versions: Option<Versions>,
    pub(crate) field_name: String,
    pub(crate) type_name: String,
    pub(crate) versions: Versions,
    pub(crate) tagged_versions: Option<Versions>,
    pub(crate) tag: Option<i32>,
    pub(crate) default: Option<String>,
    pub(crate) doc: Option<String>,
}

impl KrostField {
    pub(crate) fn from_schema(field: &schema::Field) -> Self {
        let collection = field.r#type.is_array();
        let field_name = field.name.to_snake_case();
        let type_name = field
            .r#type
            .to_string()
            .trim_start_matches("[]")
            .to_string();
        let default = match field.default.clone() {
            Some(default) => default.as_str().map(|s| s.to_string()),
            None => None,
        };

        let doc = field.about.clone();
        Self {
            collection,
            nullable_versions: field.nullable_versions,
            field_name,
            type_name,
            versions: field.versions,
            tagged_versions: field.tagged_versions,
            tag: field.tag,
            default,
            doc,
        }
    }

    pub(crate) fn tagged_fields(flexible_versions: Versions) -> Self {
        Self {
            collection: false,
            nullable_versions: None,
            field_name: "_tagged_fields".to_string(),
            type_name: "tagged_fields".to_string(),
            tagged_versions: None,
            versions: flexible_versions,
            tag: None,
            default: None,
            doc: Some("The tagged fields.".to_string()),
        }
    }

    fn field_type(&self) -> proc_macro2::TokenStream {
        let mut tokens = match self.type_name.as_str() {
            "bool" => quote! { bool },
            "byte" => quote! { i8 },
            "int8" => quote! { i8 },
            "int16" => quote! { i16 },
            "int32" => quote! { i32 },
            "int64" => quote! { i64 },
            "uint16" => quote! { u16 },
            "float64" => quote! { f64 },
            "bytes" => quote! { Vec<u8> },
            "string" => quote! { String },
            "uuid" => quote! { krost::types::Uuid },
            "records" => quote! { krost::record::RecordBatch },
            "tagged_fields" => quote! { krost::types::TaggedFields },
            v => {
                let type_ident = to_ident(v);
                quote! { #type_ident }
            }
        };
        if self.collection {
            tokens = quote! { Vec<#tokens> };
        }
        if self.nullable_versions.is_some() {
            tokens = quote! { Option<#tokens> };
        }
        tokens
    }
}

impl ToTokens for KrostField {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let field_name_ident = to_ident(&self.field_name);
        let field_type_ident = self.field_type();
        let versions = self.versions.to_string();
        let versions_ident = quote! { versions = #versions, };
        let tagged_versions_ident = self.tagged_versions.as_ref().map(|v| {
            let v = v.to_string();
            quote! {tagged = #v, }
        });
        let nullable_versions_ident = self.nullable_versions.as_ref().map(|v| {
            let v = v.to_string();
            quote! {nullable = #v, }
        });
        let tag_ident = self.tag.map(|v| quote! {tag = #v, });
        let default_ident = self
            .default
            .clone()
            .map(|v| if v == "null" { "None".to_string() } else { v })
            .map(|v| quote! {default = #v, });
        let doc_ident = self.doc.clone().map(|v| quote! { #[doc = #v] });
        tokens.extend(quote! {
          #doc_ident
          #[kafka(#versions_ident #tagged_versions_ident #tag_ident #nullable_versions_ident #default_ident)]
          pub #field_name_ident: #field_type_ident
        })
    }
}

fn to_ident(s: &str) -> syn::Ident {
    let ident = syn::Ident::new(s, proc_macro2::Span::call_site());
    match s {
        // From https://doc.rust-lang.org/reference/keywords.html
        "abstract" | "alignof" | "as" | "become" | "box" | "break" | "const" | "continue"
        | "crate" | "do" | "else" | "enum" | "extern" | "false" | "final" | "fn" | "for" | "if"
        | "impl" | "in" | "let" | "loop" | "macro" | "match" | "mod" | "move" | "mut"
        | "offsetof" | "override" | "priv" | "proc" | "pub" | "pure" | "ref" | "return"
        | "Self" | "self" | "sizeof" | "static" | "struct" | "super" | "trait" | "true"
        | "type" | "typeof" | "unsafe" | "unsized" | "use" | "virtual" | "where" | "while"
        | "yield" => {
            let raw_string = format!("r#{}", s);
            let mut raw_ident: syn::Ident = syn::parse_str(&raw_string).unwrap();
            raw_ident.set_span(ident.span());
            raw_ident
        }
        _ => ident,
    }
}

pub(crate) fn group_schema_specs(
    specs: Vec<KrostSchema>,
) -> BTreeMap<i16, (KrostSchema, KrostSchema)> {
    let intermediate: HashMap<i16, Vec<KrostSchema>> = specs
        .into_iter()
        .filter(|spec| spec.api_key.is_some())
        .filter(|spec| {
            spec.r#type == schema::SchemaType::Request
                || spec.r#type == schema::SchemaType::Response
        })
        .map(|spec| (spec.api_key.unwrap(), spec))
        .into_group_map();
    for (api_key, specs) in &intermediate {
        if specs.len() != 2 {
            panic!(
                "{}",
                format!(
                    "did not expect more than 2 request types for api key {}",
                    api_key
                )
            );
        }
    }
    intermediate
        .into_iter()
        .map(|(api_key, mut specs)| {
            specs.sort_by(|s1, _| {
                if s1.r#type == schema::SchemaType::Request {
                    Ordering::Less
                } else {
                    Ordering::Greater
                }
            });
            (api_key, (specs.remove(0), specs.remove(0)))
        })
        .collect::<BTreeMap<i16, (KrostSchema, KrostSchema)>>()
}

fn gen_header_imports(file_contents: &mut TokenStream) {
    file_contents.extend(quote! { #![allow(dead_code)] });
}

pub(crate) fn gen_header_contents(headers: Vec<KrostSchema>) -> TokenStream {
    let mut header_contents = TokenStream::new();
    let header_module_ident = to_ident("header");

    header_contents.extend(quote! { pub mod #header_module_ident {
        #(#headers)*
    }});
    header_contents
}

pub(crate) fn gen_api_file_contents(
    grouped_specs: &BTreeMap<i16, (KrostSchema, KrostSchema)>,
) -> TokenStream {
    let mut file_contents = TokenStream::new();
    gen_header_imports(&mut file_contents);

    let mut request_contents = Vec::with_capacity(grouped_specs.len());
    let mut response_contents = Vec::with_capacity(grouped_specs.len());

    let mut req_enum_variants = vec![];
    let mut resp_enum_variants = vec![];

    for (request_spec, response_spec) in grouped_specs.values() {
        let module_name = to_ident(
            &request_spec
                .name
                .trim_end_matches("Request")
                .to_snake_case(),
        );

        let req = to_ident(&request_spec.name);
        req_enum_variants.push(quote! { #req(#module_name::#req) });
        request_contents.push(quote! { pub mod #module_name {
            #request_spec
        }});

        let resp = to_ident(&response_spec.name);
        resp_enum_variants.push(quote! { #resp(#module_name::#resp) });
        response_contents.push(quote! { pub mod #module_name {
            #response_spec
        }});
    }

    let request_module_ident = to_ident("request");
    let response_module_ident = to_ident("response");
    file_contents.extend(quote! { pub mod #request_module_ident {
        #(#request_contents)*

        #[allow(dead_code)]
        #[derive(Debug, Clone, PartialEq)]
        pub enum RequestBody {
           #(#req_enum_variants),*
        }
    } });
    file_contents.extend(quote! { pub mod #response_module_ident {
        #(#response_contents)*

        #[allow(dead_code)]
        #[derive(Debug, Clone, PartialEq)]
        pub enum ResponseBody {
           #(#resp_enum_variants),*
        }
    } });

    file_contents
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

pub fn build(input: &str, output: &str) {
    let path = Path::new(input);
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

    let headers_specs = gen_header_contents(header_specs);
    let grouped_specs = group_schema_specs(root_specs);
    let mut api_file_contents = gen_api_file_contents(&grouped_specs);
    api_file_contents.extend(headers_specs);

    if let Ok(file) = syn::parse_file(&api_file_contents.to_string()) {
        let pretty = prettyplease::unparse(&file);
        fs::write(output, pretty).expect("Unable to write file");
        println!("generated modules");
    }
}
