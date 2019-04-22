extern crate proc_macro;

use std::iter::FromIterator;

use quote::quote;
use quote::ToTokens;

use syn::parse_macro_input;
use syn::Data;
use syn::DeriveInput;
use syn::Fields;

use proc_macro2::Ident;
use proc_macro2::Span;
use proc_macro2::TokenStream;
use proc_macro2::TokenTree;

/// Add a builder implementation for a specific field
fn get_impl_for_field(stream: TokenStream, field_name_ident: Ident) -> Option<TokenStream> {
    let mut tokens: Vec<TokenTree> = stream.into_iter().collect();
    let mut output_stream = TokenStream::new();

    // Option<> is enough for at least 3 tokens
    // if we have less, something is wrong and
    // we skip.
    if tokens.len() <= 2 {
        return None;
    };

    let first_token = tokens.remove(0);

    if let TokenTree::Ident(first_ident) = first_token {
        // Assert that first identifier is an Option
        if first_ident == Ident::new("Option", Span::call_site()) {
            // Remove the first < from Option<*>
            tokens.remove(0);
            // Remove the last > from Option<*>
            tokens.pop();

            // The type inside the option
            let inner_type = TokenStream::from_iter(tokens.into_iter());
            let with_func_name =
                Ident::new(&format!("with_{}", field_name_ident), Span::call_site());
            let without_func_name =
                Ident::new(&format!("without_{}", field_name_ident), Span::call_site());

            let new_func = quote! {
                pub fn #with_func_name<IN: Into<#inner_type>>(mut self, inner: IN) -> Self {
                    self.#field_name_ident = Some(inner.into());

                    self
                }

                pub fn #without_func_name(mut self) -> Self {
                    self.#field_name_ident = None;

                    self
                }
            };

            output_stream.extend(new_func);
        }
    }

    if output_stream.is_empty() {
        None
    } else {
        Some(output_stream)
    }
}

/// Check if an attribute is #[optional_builder(skip)]
fn is_optional_builder_skip(attribute: syn::Attribute) -> bool {
    if let Some(segment) = attribute.path.segments.first() {
        let ident = segment.value().ident.clone();

        if ident == Ident::new("optional_builder", ident.span()) {
            let tokens: Vec<TokenTree> = attribute.clone().tts.into_iter().collect();

            // Our token has only one element, a group "(skip)"
            if tokens.len() > 1 {
                return false;
            }

            if let TokenTree::Group(ref group) = tokens[0] {
                let group_tokens: Vec<TokenTree> = group.clone().stream().into_iter().collect();

                if group_tokens.len() > 1 {
                    return false;
                }

                if let TokenTree::Ident(ref ident) = group_tokens[0] {
                    return ident == &Ident::new("skip", ident.span());
                }
            }
        }
    }

    false
}

/// The optional_builder procedural macro
/// adds an implementation of two functions
/// for every option guarded field:
///
/// with_#field_name and without_#fieldname,
/// responsible for both injecting data into
/// field and removing it.
#[proc_macro_attribute]
pub fn optional_builder(
    _: proc_macro::TokenStream,
    input: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let mut derive_input = parse_macro_input!(input as DeriveInput);
    let mut builder_impls: TokenStream = TokenStream::new();

    if let Data::Struct(ref mut data_struct) = derive_input.data {
        if let Fields::Named(ref mut named_fields) = data_struct.fields {
            for field in named_fields.named.iter_mut() {
                let mut to_remove: Vec<usize> = Vec::new();

                for (i, attribute) in field.attrs.iter().enumerate() {
                    if is_optional_builder_skip(attribute.clone()) {
                        to_remove.push(i);
                    }
                }

                if !to_remove.is_empty(){
                    to_remove.sort();

                    for (idx_counter, idx) in to_remove.iter().enumerate() {
                        field.attrs.remove(idx - idx_counter);
                    }
                } else {
                    let mut stream = TokenStream::new();
                    field.ty.to_tokens(&mut stream);

                    let field_name_ident = field
                        .ident
                        .clone()
                        .expect("optional_builder require named fields.");

                    let field_impl_option = get_impl_for_field(stream, field_name_ident);

                    if let Some(field_impl) = field_impl_option {
                        builder_impls.extend(field_impl);
                    }
                }
            }
        }
    }

    let mut input_stream = TokenStream::new();
    derive_input.to_tokens(&mut input_stream);

    if !builder_impls.is_empty() {
        let struct_name = derive_input.ident.clone();

        let (impl_generics, ty_generics, where_clause) = derive_input.generics.split_for_impl();

        let final_stream = quote! {
            #input_stream

            impl #impl_generics #struct_name #ty_generics
            #where_clause
            {
                #builder_impls
            }
        };

        final_stream.into()
    } else {
        input_stream.into()
    }
}
