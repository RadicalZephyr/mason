extern crate proc_macro;

use mason::generate_builder as _generate_builder;

#[proc_macro_attribute]
pub fn generate_builder(
    attr: proc_macro::TokenStream,
    input: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    proc_macro::TokenStream::from(_generate_builder(attr, input))
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
