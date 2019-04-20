extern crate proc_macro;

use std::iter::FromIterator;

use quote::quote;
use quote::ToTokens;

use syn::parse_macro_input;
use syn::Data;
use syn::DeriveInput;
use syn::Fields;

use proc_macro2::Group;
use proc_macro2::Ident;
use proc_macro2::Span;
use proc_macro2::TokenStream;
use proc_macro2::TokenTree;

/// Search TokenStream for #[optional_builder(skip)]
/// Or {
///     Ident,
///     Group: {
///         ..
///         Ident: "skip"    
///     }
/// }
fn has_skip_attr(stream: &TokenStream) -> bool {
    let mut tokens: Vec<TokenTree> = stream.clone().into_iter().take(2).collect();

    if tokens.len() < 2 {
        return false;
    }

    let group = tokens.remove(1);
    let ident = tokens.remove(0);

    if let (TokenTree::Ident(ident), TokenTree::Group(group)) = (ident, group) {
        let is_optional_builder = ident == Ident::new("optional_builder", ident.span());
        let mut is_skip_attr = false;

        for group_item in group.stream() {
            if let TokenTree::Ident(ref ident) = group_item {
                if ident == &Ident::new("skip", ident.span()) {
                    is_skip_attr = true;
                }
            }
        }

        is_optional_builder && is_skip_attr
    } else {
        false
    }
}

/// Consume a group and return a new group with all inner
/// groups filtered to remove skip attributes, also register
/// which fields should be ignored because of those attributes.
fn group_remove_attr(fields_to_ignore: &mut Vec<Ident>, group: Group) -> Group {
    let mut tokens_whitelist: Vec<TokenTree> = Vec::new();
    let mut to_skip = false;

    for token in group.stream() {
        if let TokenTree::Group(ref inner_group) = token {
            // Don't insert skipped attributes
            if has_skip_attr(&inner_group.stream()) {
                // Remove last (should be #)
                tokens_whitelist.pop();
                // next ident should be added to skiplist
                to_skip = true;
                // Do not add group
                continue;
            }
        }

        if let TokenTree::Ident(id) = token.clone() {
            if to_skip {
                fields_to_ignore.push(id);
                to_skip = false;
            }
        }

        tokens_whitelist.push(token);
    }

    Group::new(group.delimiter(), TokenStream::from_iter(tokens_whitelist))
}

/// Consume a stream and return a new stream
/// with all used attributes removed, also return
/// a list of the field idents that were affected
/// by these attributes, since we only have an
/// skip attribute, this is a list of fields to ignore.
fn stream_remove_attr(stream: TokenStream) -> (Vec<Ident>, TokenStream) {
    let new_stream = stream.clone();
    let mut tokens_to_insert: Vec<TokenTree> = Vec::new();
    let mut fields_to_ignore: Vec<Ident> = Vec::new();

    for item in new_stream.into_iter() {
        if let TokenTree::Group(group) = item {
            tokens_to_insert.push(TokenTree::Group(group_remove_attr(
                &mut fields_to_ignore,
                group,
            )));
        } else {
            tokens_to_insert.push(item);
        }
    }

    (fields_to_ignore, TokenStream::from_iter(tokens_to_insert))
}

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
    let input_stream = TokenStream::from(input.clone());
    let (fields_to_ignore, filtered_stream) = stream_remove_attr(input_stream);

    let DeriveInput {
        ident: struct_name,
        data: derive_data,
        ..
    } = parse_macro_input!(input as DeriveInput);

    let mut builder_impls: TokenStream = TokenStream::new();

    if let Data::Struct(data_struct) = derive_data {
        if let Fields::Named(named_fields) = data_struct.fields {
            for field_name in named_fields.named {
                let mut stream = TokenStream::new();
                field_name.ty.to_tokens(&mut stream);

                let field_name_ident = field_name
                    .ident
                    .expect("optional_builder require named fields.");

                // Implement all non-ignored fields
                if !fields_to_ignore.contains(&field_name_ident) {
                    let field_impl_option = get_impl_for_field(stream, field_name_ident);

                    if let Some(field_impl) = field_impl_option {
                        builder_impls.extend(field_impl);
                    }
                }
            }
        }
    }

    if !builder_impls.is_empty() {
        let final_stream = quote! {
            #filtered_stream

            impl #struct_name {
                #builder_impls
            }
        };

        final_stream.into()
    } else {
        filtered_stream.into()
    }
}
