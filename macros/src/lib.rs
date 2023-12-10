use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, ItemStruct};

#[proc_macro_derive(Expr)]
pub fn expr_derive(item: TokenStream) -> TokenStream {

    let item = parse_macro_input!(item as ItemStruct);

    let item_name = &item.ident;
    let lifetime = match &item.generics.lifetimes().next() {
        Some(lifetime) => Some(lifetime.lifetime.clone()),
        None => None,
    };

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

        impl<'ctx> Expr<'ctx> for #item_name<#lifetime> {
            fn accept(&self, visitor: &mut dyn Visitor<'ctx>) -> IntValue<'ctx> {
                visitor.#visit(self)
            }
        }

    };

    return output.into();
}
