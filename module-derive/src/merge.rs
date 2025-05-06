use proc_macro::TokenStream;
use proc_macro2::Span;
use quote::{ToTokens, quote};
use syn::{
    Data, DataStruct, DeriveInput, Fields, FieldsNamed, FieldsUnnamed, GenericParam, Ident, Index,
    Path, PathSegment, PredicateType, Token, TraitBound, TypeParamBound, TypePath, WherePredicate,
    punctuated::Punctuated,
};

pub fn merge(item: TokenStream) -> TokenStream {
    let DeriveInput {
        ident,
        generics,
        data,
        ..
    } = syn::parse_macro_input!(item as syn::DeriveInput);

    let Data::Struct(DataStruct { fields, .. }) = data else {
        panic!("Merge can only be derived on structs");
    };

    let extra_bounds: Punctuated<WherePredicate, Token![,]> = generics
        .params
        .iter()
        .filter_map(|x| match x {
            GenericParam::Type(x) => Some(x),
            _ => None,
        })
        .map(|x| {
            WherePredicate::Type(PredicateType {
                lifetimes: None,
                bounded_ty: syn::Type::Path(TypePath {
                    qself: None,
                    path: Path {
                        leading_colon: None,
                        segments: [PathSegment::from(x.ident.clone())].into_iter().collect(),
                    },
                }),
                colon_token: Token![:]([Span::call_site()]),
                bounds: {
                    [TypeParamBound::Trait(TraitBound {
                        paren_token: None,
                        modifier: syn::TraitBoundModifier::None,
                        lifetimes: None,
                        path: Path {
                            leading_colon: Some(Token![::]([Span::call_site(), Span::call_site()])),
                            segments: [
                                PathSegment::from(Ident::new("module", Span::call_site())),
                                PathSegment::from(Ident::new("Merge", Span::call_site())),
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
            x.predicates.extend(extra_bounds);
            x
        }
        None => syn::WhereClause {
            where_token: Token![where](Span::call_site()),
            predicates: extra_bounds,
        },
    };

    let mut output = quote! {
        impl #generics ::module::Merge for #ident #generics
        #where_clause
    };

    match fields {
        Fields::Unit => quote! {
            {
                fn merge(self, other: Self) -> Result<Self, ::module::Error> {
                    Ok(Self)
                }
            }
        },
        Fields::Unnamed(FieldsUnnamed { unnamed, .. }) => {
            let fields: Punctuated<proc_macro2::TokenStream, Token![,]> = unnamed
                .iter()
                .enumerate()
                .map(|(i, _)| Index::from(i))
                .map(|i| {
                    quote! {
                        self.#i.merge(other.#i)?
                    }
                })
                .collect();

            quote! {
                {
                    fn merge(self, other: Self) -> Result<Self, ::module::Error> {
                        Ok(Self(#fields))
                    }
                }
            }
        }
        Fields::Named(FieldsNamed { named, .. }) => {
            let fields: Punctuated<proc_macro2::TokenStream, Token![,]> = named
                .iter()
                .map(|x| x.ident.clone().unwrap())
                .map(|x| {
                    quote! {
                        #x: self.#x.merge(other.#x)?
                    }
                })
                .collect();

            quote! {
                {
                    fn merge(self, other: Self) -> Result<Self, ::module::Error> {
                        Ok(Self { #fields })
                    }
                }
            }
        }
    }
    .to_tokens(&mut output);

    output.into()
}
