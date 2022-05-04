use crate::schema;
use crate::schema::SchemaType;
use heck::ToSnakeCase;
use itertools::Itertools;
use proc_macro2::TokenStream;
use quote::{quote, ToTokens};
use serde_json::Value;
use std::cmp::Ordering;
use std::collections::{BTreeMap, HashMap};

#[derive(Debug, Clone)]
pub(crate) struct KrostField {
    pub(crate) collection: bool,
    pub(crate) nullable: bool,
    pub(crate) field_name: String,
    pub(crate) type_name: String,
    pub(crate) version_added: Option<i16>,
    pub(crate) version_removed: Option<i16>,
    pub(crate) default: Option<KrostValue>,
    pub(crate) doc: Option<String>,
}

impl KrostField {
    pub(crate) fn from_schema(field: &schema::Field) -> Self {
        let collection = field.r#type.is_array();
        let nullable = field.nullable_versions.is_some();
        let field_name = field.name.to_snake_case();
        let type_name = field
            .r#type
            .to_string()
            .trim_start_matches("[]")
            .to_string();
        let version_added = field.versions.version_start();
        let version_removed = field.versions.version_end();
        let default = match &field.default {
            Some(Value::Bool(b)) => Some(KrostValue::Bool(*b)),
            Some(Value::Number(n)) => Some(KrostValue::Number(n.as_f64().unwrap())),
            Some(Value::String(s)) => Some(KrostValue::String(s.clone())),
            _ => None,
        };
        let doc = field.about.clone();
        Self {
            collection,
            nullable,
            field_name,
            type_name,
            version_added,
            version_removed,
            default,
            doc,
        }
    }

    fn field_type(&self) -> proc_macro2::TokenStream {
        let mut tokens = match self.type_name.as_str() {
            "bool" => quote! { krost::primitive::Bool },
            "byte" => quote! { krost::primitive::Int8 },
            "int8" => quote! { krost::primitive::Int8 },
            "int16" => quote! { krost::primitive::Int16 },
            "int32" => quote! { krost::primitive::Int32 },
            "int64" => quote! { krost::primitive::Int64 },
            "bytes" => quote! { Vec<u8> },
            "string" => quote! { krost::primitive::String },
            v => {
                let type_ident = to_ident(v);
                quote! { #type_ident }
            }
        };
        if self.collection {
            tokens = quote! { Vec<#tokens> };
        }
        if self.nullable {
            tokens = quote! { Option<#tokens> };
        }
        tokens
    }
}

impl ToTokens for KrostField {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let field_name = self.field_name.clone();
        let version_added = self.version_added;
        let field_name_ident = to_ident(&field_name);
        let field_type_ident = self.field_type();
        let version_added_ident = quote! { added = #version_added };
        let version_removed_ident = self.version_removed.map(|v| quote! {,removed = #v});
        let default_ident = self.default.clone().map(|v| quote! {,default = #v});
        let doc_ident = self.doc.clone().map(|v| quote! { #[doc = #v] });
        tokens.extend(quote! {
          #doc_ident
          #[kafka(#version_added_ident #version_removed_ident #default_ident)]
          pub #field_name_ident: #field_type_ident
        })
    }
}

#[derive(Debug, Clone)]
pub enum KrostValue {
    Bool(bool),
    Number(f64),
    String(String),
}

impl ToTokens for KrostValue {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        match self {
            KrostValue::Bool(b) => tokens.extend(quote! { #b }),
            KrostValue::Number(n) => tokens.extend(quote! { #n }),
            KrostValue::String(s) => tokens.extend(quote! { #s }),
        }
    }
}

#[derive(Debug, Clone)]
pub(crate) struct KrostStruct {
    pub(crate) struct_name: String,
    pub(crate) schema_type: SchemaType,
    pub(crate) api_key: Option<i16>,
    pub(crate) fields: Vec<KrostField>,
}

impl ToTokens for KrostStruct {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let full_struct_name = self.struct_name.to_owned();
        let struct_name_ident = to_ident(&full_struct_name);
        let struct_fields = &self.fields;
        tokens.extend(quote! {
            #[derive(Debug, PartialEq, Krost, Clone)]
            pub struct #struct_name_ident {
               #(#struct_fields),*
            }
        })
    }
}

#[derive(Debug, Clone)]
pub(crate) struct KrostSchema {
    pub(crate) name: String,
    pub(crate) r#type: schema::SchemaType,
    pub(crate) api_key: Option<i16>,
    pub(crate) fields: Vec<KrostField>,
    pub(crate) structs: Vec<KrostStruct>,
    pub(crate) version_added: Option<i16>,
    pub(crate) version_removed: Option<i16>,
}

impl ToTokens for KrostSchema {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let full_struct_name = self.name.clone();
        let struct_name_ident = to_ident(&full_struct_name);
        let struct_fields = &self.fields;
        let substructs = &self.structs;
        let version_added = self.version_added;
        let version_added_ident = quote! {,added = #version_added };
        let version_removed_ident = self.version_removed.map(|v| quote! {,removed = #v});

        let api_key = self.api_key;
        let api_key_ident = quote! {apikey = #api_key};

        tokens.extend(quote! {
            #[derive(Debug, PartialEq, Krost, Clone)]
            #[kafka(#api_key_ident #version_added_ident #version_removed_ident)]
            pub struct #struct_name_ident {
               #(#struct_fields),*
            }
            #(#substructs)*
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

pub(crate) fn render_module(
    tokens: &mut TokenStream,
    request: &KrostSchema,
    response: &KrostSchema,
) {
    tokens.extend(quote! {
        #request
        #response
    });
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

pub(crate) fn gen_api_key_file_contents(
    grouped_specs: &BTreeMap<i16, (KrostSchema, KrostSchema)>,
) -> TokenStream {
    let mut file_contents = TokenStream::new();
    let mut api_key_pairs: Vec<(String, i16)> = vec![];
    for (api_key, (request_spec, _response_spec)) in grouped_specs {
        api_key_pairs.push((request_spec.name.clone(), *api_key));
    }
    render_api_keys(&mut file_contents, api_key_pairs);
    file_contents
}

pub(crate) fn render_api_keys(tokens: &mut TokenStream, pairs: Vec<(String, i16)>) {
    let variants = pairs
        .into_iter()
        .map(|(name_string, key)| (to_ident(&name_string), key))
        .map(|(name, key)| quote! { #name = #key })
        .collect::<Vec<_>>();
    tokens.extend(quote! {
            #[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
            #[repr(i16)]
            pub enum ApiKey {
                #(#variants),*
            }
            impl Into<i16> for ApiKey {
                fn into(self) -> i16 {
                    self as i16
                }
            }
    });
}

fn gen_header_imports(file_contents: &mut TokenStream) {
    file_contents.extend(quote! { #![allow(dead_code)] });
    file_contents.extend(quote! { use krost_derive::Krost; });
    file_contents.extend(quote! { use krost::KrostType; });
    file_contents.extend(quote! { use from_variants::FromVariants; });
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
        #[derive(Debug, Clone, PartialEq, FromVariants)]
        pub enum RequestBody {
           #(#req_enum_variants),*
        }
    } });
    file_contents.extend(quote! { pub mod #response_module_ident {
        #(#response_contents)*

        #[allow(dead_code)]
        #[derive(Debug, Clone, PartialEq, FromVariants)]
        pub enum ResponseBody {
           #(#resp_enum_variants),*
        }
    } });

    file_contents
}
