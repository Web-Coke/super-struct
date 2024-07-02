#![doc = include_str!("../README.md")]
//! 根据`struct`自动生成以下实现
//!
//! The following impl are automatically generated based on `struct`
//! ```
//! impl Self {
//!     fn keys() -> Vec<&str>;
//!     fn values(&self) -> Vec<&T>;
//!     fn values(&self) -> Vec<&dyn Any>;
//!     fn get(&self, key: &str) -> &T;
//!     fn get(&self, key: &str) -> &dyn Any;
//!     fn set(&mut self, key: &str, value: T);
//!     fn set(&mut self, key: &str, value: &dyn Any);
//! }
//! ```
//!
//! 当`struct`中类型一致时实现
//!
//! impl when the type is consistent in `struct`
//! ```
//! impl Index<&str> for Self {
//!     type Output = T;
//!     fn index(&self, index: &str) -> &Self::Output;
//! }
//! impl IndexMut<&str> for Self {
//!     fn index_mut(&mut self, index: &str) -> &mut Self::Output;
//! }
//! ```

extern crate proc_macro;
extern crate proc_macro2;
extern crate quote;
extern crate syn;

mod rsdict;
// mod rstruct;

use std::collections::HashSet;

use proc_macro::TokenStream;
use quote::quote;
use syn::spanned::Spanned;

use crate::rsdict::*;

#[proc_macro_derive(Rustdict)]
pub fn derive_rustdict(input: TokenStream) -> TokenStream {
    let struct_item = syn::parse::<syn::ItemStruct>(input.clone()).unwrap();
    let struct_name = &struct_item.ident;
    let struct_gene = &struct_item.generics;
    let struct_where = &struct_item.generics.where_clause;
    let struct_iden = &struct_item
        .fields
        .iter()
        .map(|f| f.ident.clone().unwrap())
        .collect::<Vec<_>>();
    let struct_type = &struct_item
        .fields
        .iter()
        .map(|f| f.ty.clone())
        .collect::<Vec<_>>();

    if struct_type.len() == 0 {
        return TokenStream::new();
    }

    let keys = fn_keys(struct_iden);
    let values = fn_values(struct_iden, struct_type);
    let get = fn_get(struct_iden, struct_type);
    let set = fn_set(struct_iden, struct_type);
    let index = fn_index(
        struct_name,
        struct_iden,
        struct_type,
        struct_gene,
        struct_where,
    );

    let expanded = quote! {
        impl #struct_gene #struct_name #struct_gene
        #struct_where
        {
            #keys
            #values
            #get
            #set
        }
        #index
    };
    TokenStream::from(expanded)
}
