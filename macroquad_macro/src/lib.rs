extern crate proc_macro;

use proc_macro::{Ident, TokenStream, TokenTree};

use std::env;

fn async_main(item: TokenStream) -> TokenStream {
    let mut modified = TokenStream::new();
    let mut source = item.into_iter();

    if let TokenTree::Ident(ident) = source.next().unwrap() {
        assert_eq!(format!("{}", ident), "fn");

        modified.extend(std::iter::once(TokenTree::Ident(Ident::new(
            "async",
            ident.span(),
        ))));

        modified.extend(std::iter::once(TokenTree::Ident(ident)));
    } else {
        panic!("[macroquad::main] is allowed only for functions");
    }

    if let TokenTree::Ident(ident) = source.next().unwrap() {
        assert_eq!(format!("{}", ident), "main");

        modified.extend(std::iter::once(TokenTree::Ident(Ident::new(
            "amain",
            ident.span(),
        ))));
    } else {
        panic!("[macroquad::main] expecting main function");
    }
    modified.extend(source);

    modified
}

fn sync_main(item: TokenStream) -> TokenStream {
    let mut modified = TokenStream::new();
    let mut source = item.into_iter();

    if let TokenTree::Ident(ident) = source.next().unwrap() {
        assert_eq!(format!("{}", ident), "fn");

        modified.extend(std::iter::once(TokenTree::Ident(ident)));
    } else {
        panic!("[macroquad::main] is allowed only for functions");
    }

    if let TokenTree::Ident(ident) = source.next().unwrap() {
        assert_eq!(format!("{}", ident), "main");

        modified.extend(std::iter::once(TokenTree::Ident(Ident::new(
            "amain",
            ident.span(),
        ))));
    } else {
        panic!("[macroquad::main] expecting main function");
    }
    modified.extend(source);

    modified
}

#[proc_macro_attribute]
pub fn main(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let mut res = TokenStream::new();

    let mut wasm: TokenStream = format!("#[cfg(target_arch = \"wasm32\")]").parse().unwrap();
    let mut notwasm: TokenStream = format!("#[cfg(not(target_arch = \"wasm32\"))]").parse().unwrap();

    res.extend(wasm);
    res.extend(async_main(item.clone()));
    res.extend(notwasm);
    res.extend(sync_main(item));

    let mut prelude: TokenStream = format!(
        "
    #[cfg(not(target_arch = \"wasm32\"))]
    fn main() {{
        macroquad::Window::new_sync(\"MACROQUAD\");
        amain();
    }}
    #[cfg(target_arch = \"wasm32\")]
    fn main() {{
        macroquad::Window::new_async(\"MACROQUAD\", amain());
    }}

    "
    )
    .parse()
    .unwrap();

    res.extend(prelude);
    res
}
