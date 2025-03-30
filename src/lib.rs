use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::{DeriveInput, Field, Ident, ItemStruct, ItemTrait, parse_macro_input};

#[proc_macro_attribute]
pub fn create_enum(attr: TokenStream, item: TokenStream) -> TokenStream {
    let trait_name = parse_macro_input!(attr as syn::Path); // You can customize attribute parsing here if needed
    let item = parse_macro_input!(item as ItemStruct);

    // Extract the struct name
    let struct_name = &item.ident;

    // Get the struct fields
    let fields = item
        .fields
        .iter()
        .map(|field| field.ident.clone().expect("must use named fields"))
        .collect::<Vec<_>>();

    let types = item
        .fields
        .iter()
        .map(|field| field.ty.clone())
        .collect::<Vec<_>>();

    // // Define the enum using the field names
    let enum_name = format_ident!("{}Enum", struct_name);
    let discriminants_name = format_ident!("{}EnumDiscriminants", struct_name);

    let clones = types
        .iter()
        .zip(fields.clone())
        .map(|(t, f)| {
            quote! {
                #discriminants_name::#t => #enum_name::#t(self.#f.clone())
            }
        })
        .collect::<Vec<_>>();

    let to_traits = types
        .iter()
        .zip(fields.clone())
        .map(|(t, f)| {
            quote! {
                #discriminants_name::#t => &self.#f
            }
        })
        .collect::<Vec<_>>();

    let to_traits_mut = types
        .iter()
        .zip(fields.clone())
        .map(|(t, f)| {
            quote! {
                #discriminants_name::#t => &mut self.#f
            }
        })
        .collect::<Vec<_>>();

    // Generate the methods for the enum
    let methods = quote! {
        impl #struct_name {
            pub fn clone_to_enum(&self, discriminant: #discriminants_name) -> #enum_name {
                match discriminant {
                    #( #clones ),*
                }
            }

            pub fn to_trait(&self, discriminant: #discriminants_name) -> &dyn #trait_name {
                match discriminant {
                    #( #to_traits ),*

                }
            }

            pub fn to_trait_mut(&mut self, discriminant: #discriminants_name) -> &mut dyn #trait_name {
                match discriminant {
                    #( #to_traits_mut ),*
                }
            }
        }
    };

    // Create the enum with associated methods
    let generated = quote! {
        #item

        #[enum_dispatch(#trait_name)]
        #[derive(Debug, bevy_reflect::Reflect, strum::EnumDiscriminants, Clone)]
        #[strum_discriminants(
            derive(Reflect, Hash, strum::EnumIter),
            reflect(Debug, Hash, PartialEq),
            vis(pub)
        )]
        pub enum #enum_name {
            #( #types ),*
        }

        #methods
    };

    generated.into()
}
