use quote::Tokens;

pub trait VdomRenderable {
    fn render() {

    }
}

pub enum DiffUpdate<'a> {
    Create(&'a str),
}

pub fn gen_impl_template_syntax(source: &str) -> Tokens {
    quote! {
        // struct Asdf1;
        impl ::handlebars_vdom_derive::vdom::VdomRenderable for String {

        }
    }
}
