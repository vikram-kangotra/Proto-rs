use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, ItemStruct, Fields};

#[proc_macro_attribute]
pub fn generate_ast(attr: TokenStream, item: TokenStream) -> TokenStream {

    let args = parse_macro_input!(attr as syn::AttributeArgs);

    if args.len() != 2 {
        panic!("You must provide exactly two identifiers.");
    }

    let impl_trait = match &args[0] {
        syn::NestedMeta::Meta(syn::Meta::Path(path)) => path.get_ident().unwrap(),
        _ => panic!("You must provide two identifiers."),
    };

    let visitor_ident = match &args[1] {
        syn::NestedMeta::Meta(syn::Meta::Path(path)) => path.get_ident().unwrap(),
        _ => panic!("You must provide two identifiers."),
    };

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

            impl #impl_trait for #item_name {
                fn accept(&self, visitor: &mut dyn #visitor_ident) -> f64 {
                    visitor.#visit(self)
                }
            }

        };

        return output.into()
    }

    TokenStream::new()
}
