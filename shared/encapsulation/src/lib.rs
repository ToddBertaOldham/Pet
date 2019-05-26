// *************************************************************************
// lib.rs
// Copyright 2019 Todd Berta-Oldham
// This code is made available under the MIT License.
// *************************************************************************

#![feature(bind_by_move_pattern_guards)]

extern crate proc_macro;

use quote::quote;

#[proc_macro_derive(GetterSetters, attributes(field_access))]
pub fn getter_setters(token_stream: proc_macro::TokenStream) -> proc_macro::TokenStream {
    generate_impl(token_stream, "field_access", |context| {
        let mut function_tokens = proc_macro2::TokenStream::new();

        let field_identifier = &context.field_identifier;
        let field_type = context.field_type;

        let mut name = None;
        let mut get = true;
        let mut set = false;
        let mut borrow_self = true;

        for property in context.properties.iter() {
            let property_name = property.ident.to_string();
            match property_name.as_ref() {
                "get" => {
                    match &property.lit {
                        syn::Lit::Bool(lit) => get = lit.value,
                        _ => panic!("The attribute property \"get\" on \"{}\" should have a boolean value.", field_identifier)
                    };
                },
                "set" => {
                    match &property.lit {
                        syn::Lit::Bool(lit) => set = lit.value,
                        _ => panic!("The attribute property \"set\" on \"{}\" should have a boolean value.", field_identifier)
                    };                
                },
                "borrow_self" => {
                    match &property.lit {
                        syn::Lit::Bool(lit) => borrow_self = lit.value,
                        _ => panic!("The attribute property \"borrow_self\" on \"{}\" should have a boolean value.", field_identifier)
                    };                
                },
                "name" => {
                    match &property.lit {
                        syn::Lit::Str(lit) => name = Some(syn::Ident::new(lit.value().as_ref(), proc_macro2::Span::call_site())),
                        _ => panic!("The attribute property \"name\" on \"{}\" should have a string value.", field_identifier)
                    }         
                },
                _ => panic!("Unknown attribute property \"{}\" on \"{}\".", property_name, field_identifier)
            };
        }

        let function_name = {
            if let Some(ref ident) = name {
                ident
            }
            else {
                match context.field_identifier {
                    FieldIdentitifer::Name(name) => name,
                    FieldIdentitifer::Index(_) => panic!("The name attribute property must be specified for tuple structs.")
                }
            }
        };

        if get {
            let implementation = {
                if borrow_self {
                    quote! {
                        pub fn #function_name (&self) -> #field_type {
                            self.#field_identifier
                        }
                    }
                }
                else {
                    quote! {
                        pub fn #function_name (self) -> #field_type {
                            self.#field_identifier
                        }
                    }
                }
            };

            function_tokens.extend(implementation);
        }

        if set {
            let implementation = quote! {
                pub fn set_#function_name (&mut self, value : #field_type) {
                    self.#field_identifier = value;
                }
            };

            function_tokens.extend(implementation);
        }

        function_tokens
    })
}

#[proc_macro_derive(BitGetterSetters, attributes(bit_access))]
pub fn bit_getter_setters(token_stream: proc_macro::TokenStream) -> proc_macro::TokenStream {
    generate_impl(token_stream, "bit_access", |context| {
        let mut function_tokens = proc_macro2::TokenStream::new();

        function_tokens
    })
}

fn generate_impl(token_stream : proc_macro::TokenStream, attribute_name : &str, function_generator : fn(&FunctionGeneratorContext) -> proc_macro2::TokenStream) -> proc_macro::TokenStream {
    let input = syn::parse_macro_input!(token_stream as syn::DeriveInput);
    
    if let syn::Data::Struct(syn::DataStruct { ref fields, ..}) = input.data {
        let mut function_tokens = proc_macro2::TokenStream::new();

        match fields {
            syn::Fields::Named(named_list) => { 
                for field in named_list.named.iter() {
                    let result = generate_field_functions(&field, None, attribute_name, function_generator);
                    function_tokens.extend(result);
                }
            },
            syn::Fields::Unnamed(unnamed_list) => { 
                for i in 0..unnamed_list.unnamed.len() {
                    let field = &unnamed_list.unnamed[i];

                    let index_token = syn::Index { 
                        index : i as u32,
                        span :  proc_macro2::Span::call_site()
                    };

                    let result = generate_field_functions(&field, Some(&index_token), attribute_name, function_generator);
                    function_tokens.extend(result);
                }
            },
            syn::Fields::Unit => { }
        };

        let struct_name = input.ident;
        let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();

        let implementation = quote! {
            impl #impl_generics #struct_name #ty_generics #where_clause {
                #function_tokens
            }
        };

        implementation.into()
    }
    else {
        panic!("Can only generate for structs.")
    }
}

fn generate_field_functions(field : &syn::Field, field_index : Option<&syn::Index>, attribute_name : &str, function_generator : fn(&FunctionGeneratorContext) -> proc_macro2::TokenStream) -> proc_macro2::TokenStream {
    let mut function_tokens = proc_macro2::TokenStream::new();

    for attribute_meta in field.attrs.iter().filter_map(|a| {
        match a.parse_meta() {
            Ok(meta) if meta.name() == attribute_name => Some(meta),
            _ => None
        }
    }) {
        let id = {
            if let Some(index) = field_index {
                FieldIdentitifer::Index(index)
            }
            else {
                match field.ident {
                    Some(ref ident) => FieldIdentitifer::Name(ident),
                    None => unreachable!()
                }
            }
        };

        let mut context = FunctionGeneratorContext {
            field_identifier : id,
            field_type : &field.ty,
            properties : Vec::new()
        };

        match attribute_meta {
            syn::Meta::List(ref list) => {
                for nested in list.nested.iter() {
                    if let syn::NestedMeta::Meta(inner) = nested {
                        match inner {
                            syn::Meta::List(_) => panic!("Invalid attribute synax on \"{}\".", &context.field_identifier),
                            syn::Meta::NameValue(name_value) => context.properties.push(&name_value),
                            syn::Meta::Word(_) => { }
                        };
                    }
                    else {
                        panic!("Invalid attribute synax on \"{}\".", &context.field_identifier);
                    }
                }
            },
            syn::Meta::NameValue(_) => panic!("Invalid attribute synax on \"{}\".", &context.field_identifier),
            syn::Meta::Word(_) => { }
        };

        context.check_for_duplicate_properties();

        function_tokens.extend((function_generator)(&context));
    }

    function_tokens
}

enum FieldIdentitifer<'a> {
    Name(&'a syn::Ident),
    Index( &'a syn::Index)
}

impl<'a> std::fmt::Display for FieldIdentitifer<'a> {
    fn fmt(&self, f : &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        match self {
            FieldIdentitifer::Name(name) => std::fmt::Display::fmt(&name.to_string(), f),
            FieldIdentitifer::Index(index) => std::fmt::Display::fmt(&index.index, f)
        }
    }
}

impl<'a> quote::ToTokens for FieldIdentitifer<'a> {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        match self {
            FieldIdentitifer::Name(name) => quote::ToTokens::to_tokens(name, tokens),
            FieldIdentitifer::Index(index) => quote::ToTokens::to_tokens(index, tokens)
        }
    }
}

struct FunctionGeneratorContext<'a> {
    field_identifier : FieldIdentitifer<'a>,
    field_type : &'a syn::Type,
    properties : Vec<&'a syn::MetaNameValue>
}

impl<'a> FunctionGeneratorContext<'a> {
    fn check_for_duplicate_properties(&self) {
        for a in self.properties.iter() {
            let mut found = false;
            for b in self.properties.iter() {
                if a.ident == b.ident {
                    if found {
                        panic!("Attribute property \"{}\" specified multiple times on \"{}\".", a.ident, self.field_identifier);
                    }
                    found = true;               
                }
            }
        }
    }
}