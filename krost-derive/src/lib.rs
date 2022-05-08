use darling::{self, FromDeriveInput, FromField};
use nom::branch::alt;
use nom::bytes::complete::{tag, tag_no_case};
use nom::character::complete::i16 as nom_i16;
use nom::combinator::{all_consuming, map};
use nom::sequence::{pair, tuple};
use nom::IResult;
use proc_macro::TokenStream;
use proc_macro2::Ident;
use quote::{quote, ToTokens};
use syn::{self, parse_macro_input, Attribute};

#[proc_macro_derive(Message, attributes(kafka))]
pub fn derive(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as syn::DeriveInput);
    gen(ast).into()
}

fn gen(ast: syn::DeriveInput) -> proc_macro2::TokenStream {
    let opts: KrostOpts = match KrostOpts::from_derive_input(&ast) {
        Ok(val) => val,
        Err(err) => {
            return err.write_errors();
        }
    };
    opts.render()
}

#[derive(Debug, Clone, FromField)]
#[darling(attributes(kafka), forward_attrs(doc, cfg, allow))]
struct Field {
    ident: Option<Ident>,
    attrs: Vec<Attribute>,
    vis: syn::Visibility,
    ty: syn::Type,
    #[darling(default)]
    apikey: Option<i16>,
    #[darling(default)]
    versions: String,
    #[darling(default)]
    flexible: Option<String>,
    #[darling(default)]
    tagged: Option<String>,
    #[darling(default)]
    tag: Option<i32>,
    #[darling(default)]
    nullable: Option<String>,
    #[darling(default)]
    default: Option<String>,
}

#[derive(Debug, Clone, FromDeriveInput)]
#[darling(attributes(kafka), supports(struct_named))]
struct KrostOpts {
    ident: Ident,
    attrs: Vec<Attribute>,
    #[darling(default)]
    apikey: Option<i16>,
    #[darling(default)]
    versions: String,
    #[darling(default)]
    flexible: Option<String>,
    #[darling(default)]
    tagged: Option<String>,
    #[darling(default)]
    tag: Option<i32>,
    #[darling(default)]
    nullable: Option<String>,
    #[darling(default)]
    ignorable: bool,
    data: darling::ast::Data<darling::util::Ignored, Field>,
}

fn parse_default_block(input: String) -> syn::Block {
    syn::parse_str(&format!("{{{}}}", input))
        .map_err(|e| format!("{}", e))
        .expect("invalid default value")
}

impl KrostOpts {
    fn versions_impls(&self) -> proc_macro2::TokenStream {
        fn option_quote<T: ToTokens>(opt: Option<T>) -> proc_macro2::TokenStream {
            match opt {
                Some(key) => quote!( Some(#key)),
                None => quote!(None),
            }
        }

        let apikey = option_quote(self.apikey);
        let (version_start, version_end) = versions(&self.versions);
        let version_start = option_quote(version_start);
        let version_end = option_quote(version_end);
        quote! {
            fn version_added() -> Option<i16> {
                return #version_start;
            }
            fn version_removed() -> Option<i16> {
                return #version_end;
            }
            fn apikey() -> Option<i16> {
                return #apikey
            }
        }
    }

    fn extract_fields(&self) -> darling::ast::Fields<Field> {
        self.data
            .clone()
            .take_struct()
            .expect("expected struct fields only")
    }

    fn default_impls(&self) -> proc_macro2::TokenStream {
        if self.data.is_enum() {
            panic!("enum derivations are unsupported");
        } else {
            let fields = self.extract_fields();
            let mut tokens = proc_macro2::TokenStream::new();
            if fields.fields.is_empty() {
                return quote! { Self {} };
            }
            let mut field_set = Vec::new();

            for field in fields.fields.into_iter() {
                let id = field.ident.expect("only named fields are supported");
                let default_expr = field
                    .default
                    .map(|v| parse_default_block(v).into_token_stream())
                    .unwrap_or(quote! {::std::default::Default::default()});
                tokens.extend(quote! {
                    let #id = #default_expr;
                });
                field_set.push(id);
            }
            tokens.extend(quote! {
               Self { #(#field_set),* }
            });
            tokens
        }
    }

    fn decode_impls(&self) -> proc_macro2::TokenStream {
        if self.data.is_enum() {
            panic!("enum derivations are unsupported");
        } else {
            let fields = self.extract_fields();
            let mut tokens = proc_macro2::TokenStream::new();
            if fields.fields.is_empty() {
                return quote! { Ok(Self {}) };
            }
            let mut field_set = Vec::new();

            for field in fields.fields.into_iter() {
                let id = field.ident.expect("only named fields are supported");
                let ty = field.ty;
                let versions = versions(&field.versions);
                let added_version_check = versions.0.map(|added_v| quote! { version >= #added_v });
                let removed_version_check = versions
                    .1
                    .map(|removed_v| quote! { && version <= #removed_v});
                let versions_check = quote! { #added_version_check #removed_version_check };
                let default_expr = field
                    .default
                    .map(|v| parse_default_block(v).into_token_stream())
                    .unwrap_or(quote! {::std::default::Default::default()});

                tokens.extend(quote! {
                let #id = if #versions_check {
                             <#ty as krost::KrostType>::decode(buf, version)?
                          } else {
                             #default_expr
                          };
                });
                field_set.push(id);
            }
            tokens.extend(quote! {
               Ok(Self { #(#field_set),* })
            });
            tokens
        }
    }

    fn encode_impls(&self) -> proc_macro2::TokenStream {
        if self.data.is_enum() {
            panic!("enum derivations are unsupported");
        } else {
            let fields = self.extract_fields();
            let mut tokens = proc_macro2::TokenStream::new();
            if fields.fields.is_empty() {
                return quote! { Ok(0) };
            }
            let mut field_set = Vec::new();

            tokens.extend(quote! {
                let mut len = 0;
            });
            for field in fields.fields.into_iter() {
                let id = field.ident.expect("only named fields are supported");
                let versions = versions(&field.versions);
                let added_version_check = versions.0.map(|added_v| quote! { version >= #added_v });
                let removed_version_check = versions
                    .1
                    .map(|removed_v| quote! { && version <= #removed_v});
                let versions_check = quote! { #added_version_check #removed_version_check };
                tokens.extend(quote! {
                    if #versions_check {
                       len += self.#id.encode(buf, version)?;
                    }
                });
                field_set.push(id);
            }
            tokens.extend(quote! {
               Ok(len)
            });
            tokens
        }
    }

    fn render(&self) -> proc_macro2::TokenStream {
        let ident = &self.ident;
        let default_impls = self.default_impls();
        let version_impls = self.versions_impls();
        let decode_impls = self.decode_impls();
        let encode_impls = self.encode_impls();
        quote! {
            impl krost::Krost for #ident {
                #version_impls
            }
            impl ::std::default::Default for #ident {
                fn default() -> Self {
                  #default_impls
                }
            }
            impl krost::KrostType for #ident {
                fn decode<D: ::std::io::Read>(buf: &mut D, version: i16) -> Result<Self, krost::KrostError> {
                   #decode_impls
                }
                fn encode<E: ::std::io::Write>(&self, buf: &mut E, version: i16) -> Result<usize, krost::KrostError> {
                   #encode_impls
                }
            }
        }
    }
}

fn versions(input: &str) -> (Option<i16>, Option<i16>) {
    if let Ok((_, (start, end))) = all_consuming(alt((
        versions_none,
        versions_range,
        versions_since,
        versions_exact,
    )))(input)
    {
        (start, end)
    } else {
        (None, None)
    }
}

fn versions_none(input: &str) -> IResult<&str, (Option<i16>, Option<i16>)> {
    map(tag_no_case("none"), |_| (None, None))(input)
}

fn versions_exact(input: &str) -> IResult<&str, (Option<i16>, Option<i16>)> {
    map(nom_i16, |num| (Some(num), None))(input)
}

fn versions_since(input: &str) -> IResult<&str, (Option<i16>, Option<i16>)> {
    map(pair(nom_i16, tag("+")), |(num, _)| (Some(num), None))(input)
}

fn versions_range(input: &str) -> IResult<&str, (Option<i16>, Option<i16>)> {
    map(tuple((nom_i16, tag("-"), nom_i16)), |(start, _, end)| {
        (Some(start), Some(end))
    })(input)
}
