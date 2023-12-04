use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, ItemStruct, Fields};

#[proc_macro_attribute]
pub fn generate_ast(_attr: TokenStream, item: TokenStream) -> TokenStream {

    let item = parse_macro_input!(item as ItemStruct);

    let item_name = &item.ident;

    if let Fields::Named(fields) = &item.fields {
        let field_params = fields
            .named
            .iter()
            .map(|field| {
                let field_name = &field.ident;
                let field_type = &field.ty;
                quote! {
                    #field_name: #field_type
                }
            })
        .collect::<Vec<_>>();

        let field_names = fields
            .named
            .iter()
            .map(|field| {
                let field_name = &field.ident;
                quote! {
                    #field_name
                }
            })
        .collect::<Vec<_>>();

        let camel_str = format!("{}", item_name.to_string());
        let snake_str = camel_str.chars().fold(String::new(), |mut acc, c| {
            if c.is_ascii_uppercase() {
                if !acc.is_empty() {
                    acc.push('_');
                }
                acc.push(c.to_ascii_lowercase());
            } else {
                acc.push(c);
            }
            acc
        });

        let visit = format!("visit_{}", snake_str);
        let visit = syn::Ident::new(&visit, item_name.span());

        let output = quote! {

            #item

            impl #item_name {
                pub fn new(#(#field_params),*) -> #item_name {
                    #item_name {
                        #(#field_names),*
                    }
                }
            }

            impl Expr for #item_name {
                fn accept(&self, visitor: &mut dyn Visitor) -> f64 {
                    visitor.#visit(self)
                }
            }

        };

        return output.into()
    }

    TokenStream::new()
}
