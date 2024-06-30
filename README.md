# 💟💟💟小奕💟💟💟

# Docs

[![github][github]](https://github.com/Web-Coke/super-struct)&ensp;[![crates-io][crates-io]](https://crates.io/crates/super-struct)&ensp;[![docs-rs][docs-rs]](https://docs.rs/super-struct)

`super-struct` 旨在帮助开发者像 `Python` 的 `dict` 那样使用 `Rust` 的 `struct`

The `super-struct` is designed to help developers use the `struct` of `Rust` in the same way that `dict` of `Python`

目前这个库的大部分功能已经实现

Most of the functionality of this library has already been implemented

还有一部分功能正在积极开发中

There are also some features that are under active development

---

* [X] 获取 `struct` 中的字段名(keys)
* [X] 获取 `struct` 中的值
* [X] 根据 `struct` 的字段名称(key)获取值
* [X] 根据 `struct` 的字段名称(key)设置值
* [ ] 动态向 `struct` 中添加字段(key)(这个应该很难做到)

---

* [X] Get field names (keys) in `struct`
* [X] Get the value in `struct`
* [X] Gets the value based on the field name (key) of `struct`
* [X] Set the value based on the field name (key) of `struct`
* [ ] Dynamically add a field (key) to the `struct` (this should be hard to do)

---

如果你有更好的提议或需求欢迎提issue

If you have a better proposal or need, please feel free to raise an issue

# Example

在Cargo.toml中添加

Add in Cargo.toml

```toml
[dependencies]
super-struct = "1.0"

```

`struct` 里的类型都一致的情况下

`struct` is the case where the types are the same
```rust
use super_struct::*;

#[derive(Debug, Rustdict)]
struct Test {
    name: String,
    country: String,
    language: String,
}

fn main() {
    let mut test = Test {
        name: "WebChang".to_string(),
        country: "China".to_string(),
        language: "Mandarin".to_string(),
    };
    println!("{:?}", test.keys());
    // ["name", "country", "language"]

    for i in test.keys() {
        test[i] = "Hello".to_string();
        if i == &"language" {
            test[i] = "Rust".to_string()
        }
    }
    println!("{:?}", test);
    // Test { name: "Hello", country: "Hello", language: "Rust" }

    test.set("country", "中国".to_string());
    println!("{:?}", test.values());
    // ["Hello", "中国", "Rust"]

    test["country"] = test.get("language").clone();
    println!("{:?}", test.values());
    //["Hello", "Rust", "Rust"]
}
```

如果 `struct` 里的类型不一致则 `Self[key]` 的语法糖不可用

If the types in `struct` are inconsistent, the syntactic sugar for `Self[key]` is not available
```rust
use super_struct::*;

#[derive(Debug, Rustdict)]
struct Test {
    name: String,
    country: String,
    language: String,
    age: u8,
}

fn main() {
    let mut test = Test {
        name: "WebChang".to_string(),
        country: "China".to_string(),
        language: "Mandarin".to_string(),
        age: 24u8,
    };
    println!("{:?}", test.keys());
    // ["name", "country", "language", "age"]
    println!("{:?}", test.values());
    // [Any { .. }, Any { .. }, Any { .. }, Any { .. }]

    for i in test.keys() {
        test.set(i, &"Hello".to_string());
        // 如果类型不一致则什么都不会发生
        // If the types are inconsistent, nothing happens
        // if i == &"age" {
        //     test.set(i, &25u8)
        // }
    }
    println!("{:?}", test);
    // Test { name: "Hello", country: "Hello", language: "Hello", age: 24 }

    test.set("age", &25u8);
    println!("{:?}", test);
    // Test { name: "Hello", country: "Hello", language: "Hello", age: 25 }

    println!(
        "{:?}",
        test.get("language").downcast_ref::<String>().unwrap()
    );
    // "Hello"
}
```

[github]: https://img.shields.io/badge/github-8da0cb?style=for-the-badge&labelColor=555555&logo=github
[crates-io]: https://img.shields.io/badge/crates.io-fc8d62?style=for-the-badge&labelColor=555555&logo=rust
[docs-rs]: https://img.shields.io/badge/docs.rs-66c2a5?style=for-the-badge&labelColor=555555&logo=docs.rs
