#[proc_macro_attribute]
pub fn branded(
    _attr: proc_macro::TokenStream,
    item: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let input = syn::parse_macro_input!(item as syn::ItemType);

    let vis = &input.vis;
    let ident = &input.ident;
    let ty = &input.ty;
    let crate_path: syn::Path = syn::parse_quote!(::skrc_branded);

    let tag_name = syn::Ident::new(&format!("{}Tag", ident), ident.span());

    let expanded = quote::quote! {
        #[doc(hidden)]
        #vis struct #tag_name;

        impl #crate_path::Tag for #tag_name {
            const NAME: &'static str = stringify!(#ident);
        }

        #vis type #ident = #crate_path::Branded<#tag_name, #ty>;
    };

    expanded.into()
}
