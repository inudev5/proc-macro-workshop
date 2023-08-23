use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Data, DeriveInput, Ident};
#[proc_macro_derive(Builder)]
pub fn derive(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);
    let name = &ast.ident;
    let fields = if let Data::Struct(syn::DataStruct {
        fields: syn::Fields::Named(syn::FieldsNamed { ref named, .. }),
        ..
    }) = ast.data
    {
        named
    } else {
        unimplemented!();
    };
    let field_name = fields.iter().map(|f| &f.ident);
    let bname = format!("{}Builder", name);
    let bident = Ident::new(&bname, name.span());
    let optionized = fields.iter().map(|f| {
        let name = &f.ident;
        let ty = &f.ty;
        quote! {#name: std::option::Option<#ty>}
    });
    let nones = fields.iter().map(|f| {
        let name = &f.ident;
        quote! {#name:std::option::Option::None}
    });
    let methods = fields.iter().map(|f| {
        let name = &f.ident;
        let ty = &f.ty;
        quote! {
            pub fn #name(&mut self, value:#ty)->&mut Self{
                self.#name = Some(value);
                self
            }
        }
    });
    let build_fields = fields.iter().map(|f| {
        let name = &f.ident;
        quote! {#name:self.#name.clone().ok_or(concat!(stringify!(#name)," is not set"))?}
    });
    quote!(
        struct #bident{
            #(#optionized,)*
        }
        impl #bident{
            #(#methods)*
            pub fn build(&mut self)->Result<#name, Box<dyn std::error::Error>>{
                Ok(#name{
                        #(#build_fields),*
                    }
                )
            }
        }
        impl #name{
            fn builder()-> #bident{
                #bident{
                    #(#nones),*
                }
            }
        }
    )
    .into()
}
