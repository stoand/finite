#[macro_use]
extern crate handlebars_vdom_derive;

#[derive(HandlebarsTemplates)]
#[match_files = "src/**/*.hbs"]
struct TemplatesDummy;

fn main() {}
