extern crate globset;
extern crate handlebars_vdom_parse;
extern crate proc_macro;
#[macro_use]
extern crate quote;
extern crate syn;
extern crate walkdir;

use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;

use std::path::Path;
use proc_macro::TokenStream;
use std::env;
use walkdir::WalkDir;

use quote::Tokens;
use syn::MetaItem::NameValue;
use syn::Lit::Str;

mod vdom;

#[proc_macro_derive(HandlebarsTemplates, attributes(match_files))]
pub fn derive(input: TokenStream) -> TokenStream {
    let match_files = parse_derive_attr("match_files", &input)
        .expect("HandlebarsTemplates missing #[match_files = \"..\"] attribute.");

    let matcher = globset::Glob::new(&match_files)
        .expect("\"match_files\" attr provided to HandlebarsTemplates is not a valid glob pattern")
        .compile_matcher();

    let root = env::var("CARGO_MANIFEST_DIR").unwrap_or(".".into());
    let match_files = WalkDir::new(&root)
        .into_iter()
        .filter_map(|entry| entry.ok())
        .filter_map(|entry| {
            entry
                .path()
                // only perform glob match on relative path
                .strip_prefix(&root)
                .ok()
                .and_then(|p| p.to_str())
                .map(|p| p.to_string())
        })
        .filter(|path| matcher.is_match(path));

    let mut tokens = Tokens::new();

    for path in match_files {
        if let Ok(contents) = read_file(&Path::new(&root).join(&path)) {
            println!("{}", contents);
            tokens.append(vdom::gen_impl_template_syntax(&contents));
        } else {
            println!("WARN: unable to parse handlebars template: {}", &path);
        }
    }

    tokens
        .parse()
        .expect("HandlebarsTemplates failed to generate template impl syntax")
}

fn read_file(path: &Path) -> std::io::Result<String> {
    let file = File::open(path)?;
    let mut buf_reader = BufReader::new(file);
    let mut contents = String::new();
    buf_reader.read_to_string(&mut contents)?;

    Ok(contents)
}

// adapted from https://github.com/vulkano-rs/vulkano/blob/master/vulkano-shader-derive/src/lib.rs
fn parse_derive_attr(attr_name: &str, input: &TokenStream) -> Option<String> {
    if let Some(syn_item) = syn::parse_macro_input(&input.to_string()).ok() {
        let attr_value = syn_item.attrs.into_iter().next().map(|a| a.value);

        match attr_value {
            Some(NameValue(ref i, Str(ref val, _))) if i == attr_name => Some(val.clone()),
            _ => None,
        }
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
