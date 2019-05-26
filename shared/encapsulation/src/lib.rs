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

        let custom_function_name = context.custom_function_name();

        let function_name = {
            if let Some(ref ident) = custom_function_name {
                ident
            }
            else {
                match context.field_identifier {
                    FieldIdentitifer::Name(name) => name,
                    FieldIdentitifer::Index(_) => panic!("A name must be specified for tuple structs.")
                }
            }
        };

        let field_identifier = &context.field_identifier;
        let field_type = context.field_type;

        if context.get() {
            let implementation = quote! {
                pub const fn #function_name (&self) -> #field_type {
                    self.#field_identifier
                }
            };

            function_tokens.extend(implementation);
        }

        if context.set() {
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

fn generate_impl(token_stream : proc_macro::TokenStream, attribute_name : &str, function_generator : fn(FunctionGeneratorContext) -> proc_macro2::TokenStream) -> proc_macro::TokenStream {
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
        panic!("Only valid on structs.")
    }
}

fn generate_field_functions(field : &syn::Field, field_index : Option<&syn::Index>, attribute_name : &str, function_generator : fn(FunctionGeneratorContext) -> proc_macro2::TokenStream) -> proc_macro2::TokenStream {
    let mut function_tokens = proc_macro2::TokenStream::new();

    for attribute_meta in field.attrs.iter().filter_map(|a| {
        match a.parse_meta() {
            Ok(meta) if meta.name() == attribute_name => Some(meta),
            _ => None
        }
    }) {
        fn invalid_attribute_synax() {
            panic!("Invalid attribute synax. Use #[attribute(property = value, ) form.");
        }

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
                            syn::Meta::List(_) => invalid_attribute_synax(),
                            syn::Meta::NameValue(name_value) => context.properties.push(&name_value),
                            syn::Meta::Word(_) => { }
                        };
                    }
                    else {
                        invalid_attribute_synax();
                    }
                }
            },
            syn::Meta::NameValue(_) => invalid_attribute_synax(),
            syn::Meta::Word(_) => { }
        };

        function_tokens.extend((function_generator)(context));
    }

    function_tokens
}

enum FieldIdentitifer<'a> {
    Name(&'a syn::Ident),
    Index( &'a syn::Index)
}

impl<'a> quote::ToTokens for FieldIdentitifer<'a> {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        match self {
            FieldIdentitifer::Name(name) => quote::ToTokens::to_tokens(name, tokens),
            FieldIdentitifer::Index(index) => quote::ToTokens::to_tokens(index, tokens)
        }
    }
}

//TODO panic! over invalid properties.

struct FunctionGeneratorContext<'a> {
    field_identifier : FieldIdentitifer<'a>,
    field_type : &'a syn::Type,
    properties : Vec<&'a syn::MetaNameValue>
}

impl<'a> FunctionGeneratorContext<'a> {
    fn custom_function_name(&self) -> Option<syn::Ident> {
        if let Some(ref custom_name) = self.read_string_property("name") {
            Some(syn::Ident::new(custom_name, proc_macro2::Span::call_site()))
        }
        else {
            None
        }
    }

    fn get(&self) -> bool {
        match self.read_bool_property("get") {
            Some(value) => value,
            None => true
        }
    }

    fn set(&self) -> bool {
        match self.read_bool_property("set") {
            Some(value) => value,
            None => false
        }    
    }

    fn clone(&self) -> bool {
        match self.read_bool_property("clone") {
            Some(value) => value,
            None => false
        }    
    }

    fn read_bool_property(&self, name : &str) -> Option<bool> {
        if let Some(get_value) = self.read_property(name) {
            match get_value {
                syn::Lit::Bool(outer) => Some(outer.value),
                _ => panic!("The property \"{}\" should have a boolean value.", name)
            }
        }
        else {
            None
        }
    }

    fn read_string_property(&self, name : &str) -> Option<String> {
        if let Some(get_value) = self.read_property(name) {
            match get_value {
                syn::Lit::Str(outer) => Some(outer.value()),
                _ => panic!("The property \"{}\" should have a string value.", name)
            }
        }
        else {
            None
        }
    }

    fn read_property(&self, name : &str) -> Option<&syn::Lit> {
        self.properties.iter().find_map(|m| {
            if m.ident == name {
                Some(&m.lit)
            }
            else {
                None
            }
        })
    }
}