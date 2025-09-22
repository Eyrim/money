use lib_money::html;
use proc_macro::{self, TokenStream};
use quote::quote;
use syn::{parse_macro_input, DeriveInput, Attribute};

#[proc_macro_derive(HTMLElement, attributes(html))]
pub fn derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let html_attr = get_html_attrs(input.attrs);
}

struct HTMLAttribute {
    pub class_name: String,
}

impl HTMLAttribute {
    pub fn from_attribute(attr: Attribute) -> Result<Self> {
        let args = attr.parse_args().unwrap();
    }
}

fn get_html_attrs(attrs: Vec<Attribute>) -> HTMLAttribute {
    let html_attrs: Vec<Attribute> = attrs.iter()
        .filter(|a| a.meta.path().is_ident("html"))
        .collect::<Vec<Attribute>>();

    if html_attrs.len() == 0 { panic!("Cannot have zero HTML attributes!") }
    if html_attrs.len() > 1 { panic!("Cannot have more than 1 HTML attibutes!") }

    HTMLAttribute::from_attribute(html_attrs.get(0).unwrap())
}