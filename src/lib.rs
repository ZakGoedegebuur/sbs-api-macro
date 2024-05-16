use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Data, DeriveInput};

#[proc_macro_derive(Serialize)]
pub fn derive_serialize(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = input.ident;

    let out = match input.data {
        Data::Struct(s) => {
            let fields = s.fields.into_iter().map(|field| field.ident.unwrap());
            quote! {
                impl ::serialize_internal::Serialize for #name {
                    fn serialize(&self, sbi: &mut ::serialize_internal::SBI) {
                        #(
                            Serialize::serialize(&self.#fields, sbi);
                        )*
                    }
                }
            }
        },
        _ => todo!()
    };

    out.into()
}

#[proc_macro_derive(DeSerialize)]
pub fn derive_deserialize(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = input.ident;

    let out = match input.data {
        Data::Struct(s) => {
            let fields = s.fields.clone().into_iter().map(|field| field.ident.unwrap());
            let types = s.fields.clone().into_iter().map(|field| field.ty);
            quote! {
                impl ::serialize_internal::DeSerialize for #name {
                    fn deserialize(sbi: &mut SBI, offset: &mut usize) -> Result<Self, ()> where Self: Sized {
                        Ok(Self {
                            #(  
                                #fields: #types ::deserialize(sbi, offset)?,
                            )*
                        })
                    }
                }
            }
        },
        _ => todo!()
    };

    out.into()
}