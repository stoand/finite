#[macro_use]
extern crate quote;
extern crate proc_macro;
extern crate walkdir;

use std::path::Path;
use proc_macro::TokenStream;
use std::env;
use walkdir::WalkDir;

#[proc_macro_derive(HandlebarsTemplates)]
pub fn derive(_input: TokenStream) -> TokenStream {
    
    let root = env::var("CARGO_MANIFEST_DIR").unwrap_or(".".into());
    let files = WalkDir::new(root).into_iter().filter_map(|e| e.ok());

    for entry in  {
        println!("{}", entry.unwrap().path().display());
    }

    let source = quote! {
        // impl From<Asdf> for A {

        // }
    };

    source.parse().unwrap()
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
