//! # Docs
//!
//! [![github][github]](https://github.com/Web-Coke/super-struct)&ensp;[![crates-io][crates-io]](https://crates.io/crates/super-struct)&ensp;[![docs-rs][docs-rs]](crate)
//!
//! [github]: https://img.shields.io/badge/github-8da0cb?style=for-the-badge&labelColor=555555&logo=github
//! [crates-io]: https://img.shields.io/badge/crates.io-fc8d62?style=for-the-badge&labelColor=555555&logo=rust
//! [docs-rs]: https://img.shields.io/badge/docs.rs-66c2a5?style=for-the-badge&labelColor=555555&logo=docs.rs
//! 
//! `super-struct` 旨在帮助开发者像 `Python` 的 `dict` 那样使用 `Rust` 的 `struct`
//! 
//! The `super-struct` is designed to help developers use the `struct` of `Rust` in the same way that `dict` of `Python`
//!
//! 目前这个库的大部分功能已经实现
//! 
//! Most of the functionality of this library has already been implemented
//! 
//! 还有一部分功能正在积极开发中
//! 
//! There are also some features that are under active development
//!
//! ---
//!
//! * [X] 获取 `struct` 中的字段名(keys)
//! * [X] 获取 `struct` 中的值
//! * [X] 根据 `struct` 的字段名称(key)获取值
//! * [X] 根据 `struct` 的字段名称(key)设置值
//! * [ ] 动态向 `struct` 中添加字段(key)(这个应该很难做到)
//!
//! ---
//!
//! * [X] Get field names (keys) in `struct`
//! * [X] Get the value in `struct`
//! * [X] Gets the value based on the field name (key) of `struct`
//! * [X] Set the value based on the field name (key) of `struct`
//! * [ ] Dynamically add a field (key) to the `struct` (this should be hard to do)
//!
//! ---
//!
//! 如果你有更好的提议或需求欢迎提issue
//! 
//! If you have a better proposal or need, please feel free to raise an issue
//! 
//! ---
//!
//! # Example
//!
//! 在Cargo.toml中添加
//! 
//! Add in Cargo.toml
//!
//! ```toml
//! [dependencies]
//! super-struct = "1.0"
//!
//! ```
//!
//! `struct` 里的类型都一致的情况下
//! 
//! `struct` is the case where the types are the same
//! ```rust
//! use super_struct::*;
//!
//! #[derive(Debug, Rustdict)]
//! struct Test {
//!     name: String,
//!     country: String,
//!     language: String,
//! }
//!
//! fn main() {
//!     let mut test = Test {
//!         name: "WebChang".to_string(),
//!         country: "China".to_string(),
//!         language: "Mandarin".to_string(),
//!     };
//!     println!("{:?}", test.keys());
//!     // ["name", "country", "language"]
//!
//!     for i in test.keys() {
//!         test[i] = "Hello".to_string();
//!         if i == &"language" {
//!             test[i] = "Rust".to_string()
//!         }
//!     }
//!     println!("{:?}", test);
//!     // Test { name: "Hello", country: "Hello", language: "Rust" }
//!
//!     test.set("country", "中国".to_string());
//!     println!("{:?}", test.values());
//!     // ["Hello", "中国", "Rust"]
//!
//!     test["country"] = test.get("language").clone();
//!     println!("{:?}", test.values());
//!     //["Hello", "Rust", "Rust"]
//! }
//! ```
//!
//! 如果 `struct` 里的类型不一致则 `Self[key]` 的语法糖不可用
//! 
//! If the types in `struct` are inconsistent, the syntactic sugar for `Self[key]` is not available
//! ```rust
//! use super_struct::*;
//!
//! #[derive(Debug, Rustdict)]
//! struct Test {
//!     name: String,
//!     country: String,
//!     language: String,
//!     age: u8,
//! }
//!
//! fn main() {
//!     let mut test = Test {
//!         name: "WebChang".to_string(),
//!         country: "China".to_string(),
//!         language: "Mandarin".to_string(),
//!         age: 24u8,
//!     };
//!     println!("{:?}", test.keys());
//!     // ["name", "country", "language", "age"]
//!     println!("{:?}", test.values());
//!     // [Any { .. }, Any { .. }, Any { .. }, Any { .. }]
//!
//!     for i in test.keys() {
//!         test.set(i, &"Hello".to_string());
//!         // 如果类型不一致则什么都不会发生
//!         // If the types are inconsistent, nothing happens
//!         // if i == &"age" {
//!         //     test.set(i, &25u8)
//!         // }
//!     }
//!     println!("{:?}", test);
//!     // Test { name: "Hello", country: "Hello", language: "Hello", age: 24 }
//!
//!     test.set("age", &25u8);
//!     println!("{:?}", test);
//!     // Test { name: "Hello", country: "Hello", language: "Hello", age: 25 }
//!
//!     println!(
//!         "{:?}",
//!         test.get("language").downcast_ref::<String>().unwrap()
//!     );
//!     // "Hello"
//! }
//! ```

extern crate proc_macro;
extern crate proc_macro2;
extern crate syn;
extern crate quote;

mod rsdict;
// mod rstruct;

use std::collections::HashSet;

use proc_macro::TokenStream;
use syn::spanned::Spanned;
use quote::quote;

use crate::rsdict::*;

// 根据`struct`自动生成以下实现
// The following impl are automatically generated based on `struct`
// ```
// impl Self {
//     fn keys(&self) -> &[&str];
//     fn values(&self) -> Vec<&T>;
//     fn values(&self) -> Vec<&dyn Any>;
//     fn get(&self, key: &str) -> &T;
//     fn get(&self, key: &str) -> &dyn Any;
//     fn set(&mut self, key: &str, value: T);
//     fn set(&mut self, key: &str, value: &dyn Any);
// }
// ```
// 
// 当`struct`中类型一致时实现
// impl when the type is consistent in `struct`
// ```
// impl Index<&str> for Self {
//     type Output = T;
//     fn index(&self, index: &'static str) -> &Self::Output;
// }
// impl IndexMut<&str> for Self {
//     fn index_mut(&mut self, index: &'static str) -> &mut Self::Output;
// }
// ```
#[proc_macro_derive(Rustdict)]
pub fn derive_rustdict(input: TokenStream) -> TokenStream {
    let rust_ast = syn::parse::<syn::ItemStruct>(input.clone()).unwrap();
    let struct_name = &rust_ast.ident;
    let struct_iden = &rust_ast
        .fields
        .iter()
        .map(|f| f.ident.clone().unwrap())
        .collect::<Vec<_>>();
    let struct_type = &rust_ast
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
    let index = fn_index(struct_name, struct_iden, struct_type);

    let expanded = quote! {
        impl #struct_name {
            #keys
            #values
            #get
            #set
        }
        #index
    };
    TokenStream::from(expanded)
}
