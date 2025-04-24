extern crate proc_macro;
extern crate syn;
#[macro_use]
extern crate quote;

use proc_macro::TokenStream;

fn impl_display(ast: &syn::DeriveInput) -> quote::Tokens {
    let name = &ast.ident;
    quote! {
        impl std::fmt::Display for #name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                let cp = CodePrinter::new();
                let cp = self.pretty_fmt(cp)?;
                write!(f, "{}", cp)
            }
        }
    }
}

#[proc_macro_derive(DisplayCode)]
pub fn display_code_derive(input: TokenStream) -> TokenStream {
    // Construct a string representation of the type definition
    let s = input.to_string();

    // Parse the string representation
    let ast = syn::parse_derive_input(&s).unwrap();

    // Build the impl
    let gen = impl_display(&ast);

    // Return the generated impl
    gen.parse().unwrap()
}
