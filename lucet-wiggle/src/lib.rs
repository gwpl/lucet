extern crate proc_macro;
use proc_macro::TokenStream;
use syn::parse_macro_input;

#[proc_macro]
pub fn from_witx(args: TokenStream) -> TokenStream {
    let config = parse_macro_input!(args as lucet_wiggle_generate::Config);
    let doc = witx::load(&config.wiggle.witx.paths).expect("loading witx");

    let mut ts = wiggle_generate::generate(&doc, &config.wiggle);
    ts.extend(lucet_wiggle_generate::generate(&doc, &config));

    TokenStream::from(ts)
}