extern crate proc_macro;

use std::iter::FromIterator;

use quote::quote;
use quote::ToTokens;

use syn::Data;
use syn::Fields;
use syn::DeriveInput;
use syn::parse_macro_input;

use proc_macro2::Span;
use proc_macro2::Group;
use proc_macro2::Ident;
use proc_macro2::TokenTree;
use proc_macro2::TokenStream;

/// Search TokenStream for #[optional_builder(skip)]
/// Or {
///     Ident,
///     Group: {
///         ..
///         Ident: "skip"    
///     }
/// }
fn has_skip_attr(stream: &TokenStream) -> bool {
    let mut found_attr_header = false;

    for item in stream.clone().into_iter() {
        if let TokenTree::Ident(ref ident) = item {
            if ident == &Ident::new("optional_builder", ident.span()) {
                found_attr_header = true;
            }
        }

        if let TokenTree::Group(ref group) = item {
            for group_item in group.clone().stream() {
                if let TokenTree::Ident(ref ident) = group_item {
                    if ident == &Ident::new("skip", ident.span()) {
                        return found_attr_header;
                    }
                }
            }
        }
    }

    false
}

/// Return the group with all inner groups that contains
/// skip removed, and annotate the ident's contained after
fn group_remove_attr(fields_to_ignore: &mut Vec<Ident>, group: Group) -> Group {
    let mut skip_list: Vec<usize> = Vec::new();
    let mut ignore_list: Vec<usize> = Vec::new();

    // Mutable borrow of skip_list to detect
    // attributes
    let out_stream: Vec<(usize, TokenTree)> = group
        .stream()
        .into_iter()
        .enumerate()
        .inspect(|(i, token)| {
            if let TokenTree::Group(ref grep) = token {
                if has_skip_attr(&grep.stream()) {
                    skip_list.push(*i);
                    skip_list.push(*i - 1);
                    ignore_list.push(*i + 1);
                }
            }
        })
        .collect();

    // Immutable borrow of skip_list to skip these
    // elements on iterator.
    let out_stream: Vec<TokenTree> = out_stream
        .into_iter()
        .filter(|(i, _)| !skip_list.clone().contains(i))
        .map(|(i, token)| {
            if ignore_list.contains(&i) {
                if let TokenTree::Ident(ident) = token.clone() {
                    fields_to_ignore.push(ident);
                }
            }

            token
        })
        .collect();

    Group::new(group.delimiter(), TokenStream::from_iter(out_stream))
}

/// Consume a stream and return a new stream
/// with all attributes removed, also return
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

/// The Optional Builder Macro adds an method
/// for every Option field on a struct for
/// optional definition.
///
/// e.g.
///
/// Struct S {
///   foo: Option<A>
/// }
///
/// Would generate
///
/// impl S {
///     pub fn with_foo(inner: A) -> Self {
///         self.foo = Some(inner);
///     
///         self
///     }
/// 
///     pub fn without_foo() -> Self {
///         self.foo = None;
/// 
///         self
///     }
///     
/// }
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
