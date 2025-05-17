use proc_macro2::{Span, TokenStream};
use quote::{ToTokens, quote};
use syn::Token;
use syn::parse::{Parse, ParseStream, Parser};
use syn::punctuated::Punctuated;

pub fn merge(item: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = syn::parse_macro_input!(item as syn::DeriveInput);

    let merge = Merge::new(input);
    merge.to_token_stream().into()
}

struct Merge {
    name: syn::Ident,
    generics: syn::Generics,
    fields: Fields,
}

impl Merge {
    pub fn new(input: syn::DeriveInput) -> Self {
        let syn::Data::Struct(syn::DataStruct { fields, .. }) = input.data else {
            panic!("Merge can only be derived on structs");
        };

        let name = input.ident;
        let generics = input.generics;
        let fields = Fields::new(fields);

        Self {
            name,
            generics,
            fields,
        }
    }

    fn make_impl_header(&self) -> TokenStream {
        let Self { generics, name, .. } = self;

        let extra_predicates: Punctuated<syn::WherePredicate, Token![,]> = generics
            .params
            .iter()
            .filter_map(|x| match x {
                syn::GenericParam::Type(x) => Some(x),
                _ => None,
            })
            .map(|x| {
                syn::WherePredicate::Type(syn::PredicateType {
                    lifetimes: None,
                    bounded_ty: syn::Type::Path(syn::TypePath {
                        qself: None,
                        path: syn::Path {
                            leading_colon: None,
                            segments: [syn::PathSegment::from(x.ident.clone())]
                                .into_iter()
                                .collect(),
                        },
                    }),
                    colon_token: Token![:]([Span::call_site()]),
                    bounds: {
                        [syn::TypeParamBound::Trait(syn::TraitBound {
                            paren_token: None,
                            modifier: syn::TraitBoundModifier::None,
                            lifetimes: None,
                            path: syn::Path {
                                leading_colon: Some(Token![::]([
                                    Span::call_site(),
                                    Span::call_site(),
                                ])),
                                segments: [
                                    syn::PathSegment::from(syn::Ident::new(
                                        "module",
                                        Span::call_site(),
                                    )),
                                    syn::PathSegment::from(syn::Ident::new(
                                        "Merge",
                                        Span::call_site(),
                                    )),
                                ]
                                .into_iter()
                                .collect(),
                            },
                        })]
                        .into_iter()
                        .collect()
                    },
                })
            })
            .collect();

        let where_clause = match generics.where_clause.clone() {
            Some(mut x) => {
                x.predicates.extend(extra_predicates);
                x
            }
            None => syn::WhereClause {
                where_token: Token![where](Span::call_site()),
                predicates: extra_predicates,
            },
        };

        quote! {
            impl #generics ::module::Merge for #name #generics
            #where_clause
        }
    }

    fn make_impl_body(&self) -> TokenStream {
        let Some(fields) = self.fields.as_fields() else {
            return quote! {
                fn merge(self, _: Self) -> ::core::result::Result<Self, ::module::Error> {
                    Ok(Self)
                }

                fn merge_ref(&mut self, _: Self) -> ::core::result::Result<(), ::module::Error> {
                    Ok(())
                }
            };
        };

        let mut merge_fields = TokenStream::new();
        let mut merge_ref_fields = TokenStream::new();

        for field in fields {
            let name = &field.name;

            if field.attributes.skip {
                merge_fields.extend(quote! {
                    #name: self.#name,
                });

                continue;
            }

            let value = match field.attributes.rename {
                Some(ref x) => x.clone(),
                None => syn::Expr::Lit(syn::ExprLit {
                    attrs: Vec::new(),
                    lit: syn::Lit::Str(syn::LitStr::new(
                        &field.name.to_token_stream().to_string(),
                        field.name.span(),
                    )),
                }),
            };

            let merge_base_path = field.attributes.with.clone().unwrap_or_else(|| syn::Path {
                leading_colon: Some(Token![::](Span::call_site())),
                segments: [
                    syn::PathSegment {
                        ident: syn::Ident::new("module", Span::call_site()),
                        arguments: syn::PathArguments::None,
                    },
                    syn::PathSegment {
                        ident: syn::Ident::new("Merge", Span::call_site()),
                        arguments: syn::PathArguments::None,
                    },
                ]
                .into_iter()
                .collect(),
            });

            merge_fields.extend(quote! {
                #name: #merge_base_path::merge(self.#name, _other.#name).value(#value)?,
            });

            merge_ref_fields.extend(quote! {
                #merge_base_path::merge_ref(&mut self.#name, _other.#name).value(#value)?;
            });
        }

        quote! {
            fn merge(self, _other: Self) -> ::core::result::Result<Self, ::module::Error> {
                use ::module::Context as _;
                Ok(Self { #merge_fields })
            }

            fn merge_ref(&mut self, _other: Self) -> ::core::result::Result<(), ::module::Error> {
                use ::module::Context as _;
                #merge_ref_fields
                Ok(())
            }
        }
    }
}

impl ToTokens for Merge {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let header = self.make_impl_header();
        let body = self.make_impl_body();

        let x = quote! {
            #header { #body }
        };

        // panic!("{x}")
        x.to_tokens(tokens)
    }
}

enum Fields {
    Unit,
    Fields(Vec<Field>),
}

impl Fields {
    pub fn new(fields: syn::Fields) -> Self {
        let from_iter = |iter: syn::punctuated::Iter<'_, syn::Field>| {
            let x = iter
                .into_iter()
                .cloned()
                .enumerate()
                .map(|(i, field)| (syn::Index::from(i), field))
                .map(|(i, field)| Field::new(i, field))
                .collect();

            Self::Fields(x)
        };

        match fields {
            syn::Fields::Unit => Self::Unit,
            syn::Fields::Unnamed(x) => from_iter(x.unnamed.iter()),
            syn::Fields::Named(x) => from_iter(x.named.iter()),
        }
    }

    pub fn as_fields(&self) -> Option<&[Field]> {
        match self {
            Self::Unit => None,
            Self::Fields(x) => Some(x),
        }
    }
}

struct Field {
    attributes: Attributes,
    name: FieldName,
}

impl Field {
    pub fn new(i: syn::Index, field: syn::Field) -> Self {
        let attributes = Attributes::new(field.attrs);

        let name = match field.ident {
            Some(x) => FieldName::Named(x),
            None => FieldName::Unnamed(i),
        };

        Self { attributes, name }
    }
}

struct Attributes {
    rename: Option<syn::Expr>,
    skip: bool,
    with: Option<syn::Path>,
}

impl Attributes {
    pub fn new(attrs: Vec<syn::Attribute>) -> Self {
        let mut rename = None;
        let mut skip = false;
        let mut with = None;

        for attr in attrs {
            let syn::Meta::List(meta) = attr.meta else {
                continue;
            };

            if meta.path.get_ident().is_none_or(|x| x != "merge") {
                continue;
            }

            let parsed_attrs: Vec<parse::Attribute> =
                Parser::parse2(parse::Attributes::parse_terminated, meta.tokens)
                    .unwrap()
                    .into_iter()
                    .collect();

            for parsed_attr in parsed_attrs {
                match parsed_attr {
                    parse::Attribute::Rename(x) => rename = Some(x.name),
                    parse::Attribute::Skip(_) => skip = true,
                    parse::Attribute::With(x) => with = Some(x.path),
                    parse::Attribute::Unknown => {}
                }
            }
        }

        Self { rename, skip, with }
    }
}

enum FieldName {
    Named(syn::Ident),
    Unnamed(syn::Index),
}

impl FieldName {
    pub fn span(&self) -> Span {
        match self {
            Self::Named(x) => x.span(),
            Self::Unnamed(x) => x.span,
        }
    }
}

impl ToTokens for FieldName {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        match self {
            Self::Named(x) => x.to_tokens(tokens),
            Self::Unnamed(x) => x.to_tokens(tokens),
        }
    }
}

#[allow(dead_code)]
mod parse {
    use super::*;

    pub struct Rename {
        pub rename: kw::rename,
        pub equals: Token![=],
        pub name: syn::Expr,
    }

    impl Parse for Rename {
        fn parse(input: ParseStream) -> syn::Result<Self> {
            let rename = input.parse()?;
            let equals = input.parse()?;
            let name = input.parse()?;

            Ok(Self {
                rename,
                equals,
                name,
            })
        }
    }

    pub struct Skip {
        pub skip: kw::skip,
    }

    impl Parse for Skip {
        fn parse(input: ParseStream) -> syn::Result<Self> {
            let skip = input.parse()?;

            Ok(Self { skip })
        }
    }

    pub struct With {
        pub with: kw::with,
        pub equals: Token![=],
        pub path: syn::Path,
    }

    impl Parse for With {
        fn parse(input: ParseStream) -> syn::Result<Self> {
            let with = input.parse()?;
            let equals = input.parse()?;
            let path = input.parse()?;

            Ok(Self { with, equals, path })
        }
    }

    pub enum Attribute {
        Rename(Rename),
        Skip(Skip),
        With(With),
        Unknown,
    }

    impl Parse for Attribute {
        fn parse(input: ParseStream) -> syn::Result<Self> {
            let lookahead = input.lookahead1();

            if lookahead.peek(kw::rename) {
                let x = Rename::parse(input)?;
                Ok(Self::Rename(x))
            } else if lookahead.peek(kw::skip) {
                let x = Skip::parse(input)?;
                Ok(Self::Skip(x))
            } else if lookahead.peek(kw::with) {
                let x = With::parse(input)?;
                Ok(Self::With(x))
            } else {
                Ok(Self::Unknown)
            }
        }
    }

    pub type Attributes = Punctuated<Attribute, Token![,]>;

    mod kw {
        syn::custom_keyword!(rename);
        syn::custom_keyword!(skip);
        syn::custom_keyword!(with);
    }
}
