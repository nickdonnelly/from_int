extern crate proc_macro;
extern crate syn;
#[macro_use] extern crate quote;

use proc_macro::TokenStream;
use syn::DeriveInput;

/// The trait that you derive to get `YourEnum::from_int(i32)`.
trait FromInt {
    fn from_int(i: i32) -> Self;
}

#[proc_macro_derive(FromInt)]
pub fn from_int(input: TokenStream) -> TokenStream
{
    let syntax_tree = syn::parse::<DeriveInput>(input).unwrap();
    let gen = impl_from_int(&syntax_tree);

    gen.into()
}

fn impl_from_int(syntax_tree: &syn::DeriveInput) -> quote::Tokens
{
    if let syn::Data::Enum(ref _enum) = syntax_tree.data {
        let name = &syntax_tree.ident;
        let variants = &_enum.variants;

        let match_int = match_int(&name, variants);

        quote! {
            impl FromInt for #name {
                fn from_int(i: i32) -> Self
                {
                    match i {
                        #(#match_int)*
                        _ => unreachable!()
                    }
                }
            }
        }

    } else {
        panic!("from_int: FromInt can only be implemented on enums!");
    }
}

fn match_int(
    name: &syn::Ident, 
    variants: &syn::punctuated::Punctuated<syn::Variant, syn::token::Comma>)
    -> Vec<quote::Tokens>
{
    use quote::ToTokens;
    
    // Our final result
    let mut result_tokens: Vec<quote::Tokens> = Vec::new();

    for (_, variant) in variants.iter().enumerate() {
        let ident = &variant.ident; // get the identifier of this enum type
        match &variant.discriminant {
            &Some((_, syn::Expr::Lit(ref lit))) => { // extract inner data
                match lit.lit {
                    syn::Lit::Int(ref _lit) => {
                        let _lit_tokens = _lit.into_tokens();
                        let t = quote!{
                            #_lit_tokens => #name::#ident,
                        };
                        result_tokens.push(t);
                    },
                    
                    _ => panic!("from_int: Enum variants must be literal integers!")
                }
            },
            _ => panic!("from_int: Enum variants must be literal integers!")
        }
    }

    result_tokens
}

