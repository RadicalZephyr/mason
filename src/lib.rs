use proc_macro2;

pub fn generate_builder(
    attr: impl Into<proc_macro2::TokenStream>,
    input: impl Into<proc_macro2::TokenStream>,
) -> proc_macro2::TokenStream {
    let _attr = attr.into();
    let input = input.into();

    input
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
