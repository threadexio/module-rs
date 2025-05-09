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

    let mut impl_contents = proc_macro2::TokenStream::new();

    match fields {
        Fields::Unit => quote! {
            fn merge(self, _other: Self) -> Result<Self, ::module::Error> {
                Ok(Self)
            }

            fn merge_ref(&mut self, _other: Self) -> Result<(), ::module::Error> {
                Ok(())
            }
        },
        Fields::Unnamed(FieldsUnnamed { unnamed, .. }) => {
            let mut merge_fields: Punctuated<_, Token![,]> = Punctuated::new();
            let mut merge_ref_fields = proc_macro2::TokenStream::new();

            for field in unnamed.iter().enumerate().map(|(i, _)| Index::from(i)) {
                let name = field.to_token_stream().to_string();

                merge_fields.push(quote! {
                    self.#field.merge(other.#field).value(#name)?
                });

                merge_ref_fields.extend(quote! {
                    self.#field.merge_ref(other.#field).value(#name)?;
                });
            }

            quote! {
                fn merge(self, other: Self) -> Result<Self, ::module::Error> {
                    use ::module::error::Context as _;
                    Ok(Self(#merge_fields))
                }

                fn merge_ref(&mut self, other: Self) -> Result<(), ::module::Error> {
                    use ::module::error::Context as _;
                    #merge_ref_fields
                    Ok(())
                }
            }
        }
        Fields::Named(FieldsNamed { named, .. }) => {
            let mut merge_fields: Punctuated<_, Token![,]> = Punctuated::new();
            let mut merge_ref_fields = proc_macro2::TokenStream::new();

            for field in named.iter().map(|x| x.ident.clone().unwrap()) {
                let name = field.to_token_stream().to_string();

                merge_fields.push(quote! {
                    #field: self.#field.merge(other.#field).value(#name)?
                });

                merge_ref_fields.extend(quote! {
                    self.#field.merge_ref(other.#field).value(#name)?;
                });
            }

            quote! {
                fn merge(self, other: Self) -> Result<Self, ::module::Error> {
                    use ::module::error::Context as _;
                    Ok(Self { #merge_fields })
                }

                fn merge_ref(&mut self, other: Self) -> Result<(), ::module::Error> {
                    use ::module::error::Context as _;
                    #merge_ref_fields
                    Ok(())
                }
            }
        }
    }
    .to_tokens(&mut impl_contents);

    quote! {
        impl #generics ::module::Merge for #ident #generics
        #where_clause
        {
            #impl_contents
        }
    }
    .into()
}
