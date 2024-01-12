use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, ItemStruct};

fn common_derive(item: TokenStream, trait_name: &str, returns: bool) -> TokenStream {
    let item = parse_macro_input!(item as ItemStruct);

    let item_name = &item.ident;
    let lifetime = match &item.generics.lifetimes().next() {
        Some(lifetime) => Some(lifetime.lifetime.clone()),
        None => None,
    };

    let field_names = item.fields.iter().map(|field| field.ident.as_ref().unwrap()).collect::<Vec<_>>();
    let field_types = item.fields.iter().map(|field| &field.ty).collect::<Vec<_>>();

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

    let visit = syn::Ident::new(&format!("visit_{}", snake_str), item_name.span());
    let visitor = syn::Ident::new(&format!("{}Visitor", trait_name), item_name.span());
    let trait_name = syn::Ident::new(trait_name, item_name.span());
    
    let return_value = if returns {
        quote! { -> Value<'ctx> }
    } else {
        quote! {}
    };
    
    let output = quote! {
        impl<'ctx> #trait_name<'ctx> for #item_name<#lifetime> {
            fn accept(&self, visitor: &mut dyn #visitor<'ctx>) #return_value {
                visitor.#visit(self)
            }
        }
    
        impl<'ctx> #item_name<#lifetime> {
            pub fn new(#(#field_names: #field_types),*) -> Self {
                Self {
                    #(
                        #field_names,
                    )*
                }
            }
        }
    };

    return output.into();

}

#[proc_macro_derive(Expr)]
pub fn expr_derive(item: TokenStream) -> TokenStream {
    return common_derive(item, "Expr", true);
}

#[proc_macro_derive(Stmt)]
pub fn stmt_derive(item: TokenStream) -> TokenStream {
    return common_derive(item, "Stmt", false);
}
