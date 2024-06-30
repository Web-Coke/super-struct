use crate::quote;
use crate::HashSet;
use crate::Spanned;

fn is_one_type(types: &Vec<syn::Type>) -> bool {
    if types.len() == 1 || types.len() == 0 {
        return true;
    }
    let mut hash = HashSet::<String>::new();
    types.iter().for_each(|t| {
        hash.insert(t.span().source_text().unwrap_or("None".to_string()));
    });
    return hash.len() == 1;
}

pub fn fn_keys(struct_iden: &Vec<syn::Ident>) -> proc_macro2::TokenStream {
    let doc = r#"
获取`struct`的所有字段名(keys)

gets all field names(keys) for the `struct`
# Examples
```
#[derive(Rustdict)]
struct Test {
    a: u16,
    b: u8,
}
let test = Test {
    a: 0u16,
    b: 0u8,
};
println!("{:?}", test.keys());
```
```
--> ["a", "b"]
```
"#;
    quote! {
        #[doc = #doc]
        fn keys(&self) -> &'static[&'static str] {
            &[#(stringify!(#struct_iden)),*]
        }
    }
}

pub fn fn_values(
    struct_iden: &Vec<syn::Ident>,
    struct_type: &Vec<syn::Type>,
) -> proc_macro2::TokenStream {
    let doc = r#"
获取`struct`里的所有值 返回类型为`Vec<T>`

get all values in `struct` Return type `Vec<T>`

```
#[derive(Rustdict)]
struct Test {
    a: u8,
    b: u8,
}
let test = Test {
    a: 1u8,
    b: 2u8,
};
println!("{:?}", Test.values());
```
```
--> [1, 2]
```

如果`struct`的字段类型不一致则返回类型为`Vec<&dyn Any>`

If the field type of the `struct` is inconsistent, the return type is `Vec<&dyn Any>`

```
#[derive(Rustdict)]
struct Test {
    a: u16,
    b: u8,
}
let test = Test {
    a: 1u16,
    b: 2u8,
};
println!("{:?}", Test.values());
```
```
--> [Any { .. }, Any { .. }]
```
"#;
    if is_one_type(struct_type) {
        let struct_type = &struct_type[0];
        return quote! {
            #[doc = #doc]
            fn values(&self) -> Vec<&#struct_type> {
                vec![#(&self.#struct_iden),*]
            }
        };
    }
    quote! {
        #[doc = #doc]
        fn values(&self) -> Vec<&dyn ::core::any::Any> {
            vec![#(&self.#struct_iden),*]
        }
    }
}

pub fn fn_get(
    struct_iden: &Vec<syn::Ident>,
    struct_type: &Vec<syn::Type>,
) -> proc_macro2::TokenStream {
    let doc = r#"
根据`struct`的字段名(key)获取值

gets the value based on the field name(key) of the `struct`
# Examples
```
#[derive(Rustdict)]
struct Test {
    a: u8,
    b: u8,
}
let test = Test {
    a: 1u8,
    b: 2u8,
};
assert_eq!(&test.a, test.get("a"));
assert_eq!(&test.b, test.get("b"));
```

如果`struct`的字段类型不一致则返回类型为`Any`

If the field type of the `struct` is inconsistent, the return type is `Any`

```
#[derive(Rustdict)]
struct Test {
    a: u16,
    b: u8,
}
let test = Test {
    a: 1u16,
    b: 2u8,
};
assert_eq!(Some(&test.a), test.get("a").downcast_ref::<u16>());
assert_eq!(Some(&test.b), test.get("b").downcast_ref::<u8>());
```
"#;
    if is_one_type(struct_type) {
        let struct_type = &struct_type[0];
        return quote! {
            #[doc = #doc]
            fn get(&self, key: &'static str) -> &#struct_type {
                match key {
                    #(stringify!(#struct_iden) => &self.#struct_iden,)*
                    _ => panic!("不存在的key")
                }
            }
        };
    }
    quote! {
        #[doc = #doc]
        fn get(&self, key: &'static str) -> &dyn ::core::any::Any {
            match key {
                #(stringify!(#struct_iden) => &self.#struct_iden,)*
                _ => panic!("不存在的key")
            }
        }
    }
}

pub fn fn_set(
    struct_iden: &Vec<syn::Ident>,
    struct_type: &Vec<syn::Type>,
) -> proc_macro2::TokenStream {
    let doc = r#"
根据`struct`的字段名(key)分配值

Assign value based on the name(key) of the `struct` field

```
#[derive(Rustdict)]
struct Test {
    a: u8,
    b: u8,
}
let mut test = Test {
    a: 0u8,
    b: 0u8,
};
test.set("a", 5u8);
test.b = 5u8;
assert_eq!(test.a, 5u8);
assert_eq!(test.b, 5u8);
```

如果`struct`的字段类型不一致需要这样写

If the type of the `struct` field is inconsistent, you need to write like this

```
#[derive(Rustdict)]
struct Test {
    a: u16,
    b: u8,
}
let mut test = Test {
    a: 0u16,
    b: 0u8,
};
test.set("a", &5u16);
test.set("b", &5u8);
assert_eq!(test.a, 5u16);
assert_eq!(test.b, 5u8);
```
"#;
    if is_one_type(struct_type) {
        let struct_type = &struct_type[0];
        return quote! {
            #[doc = #doc]
            fn set(&mut self, key: &'static str, value: #struct_type) {
                match key {
                    #(stringify!(#struct_iden) => {
                        self.#struct_iden = value;
                     },)*
                    _ => panic!("不存在的key")
                }
            }
        };
    }
    quote! {
        #[doc = #doc]
        fn set(&mut self, key: &'static str, value: &dyn ::core::any::Any) {
            match key {
                #(stringify!(#struct_iden) => {
                    if let Some(value) = value.downcast_ref::<#struct_type>(){
                        self.#struct_iden.clone_from(value);
                    }
                },)*
                _ => panic!("不存在的key")
            }
        }
    }
}

pub fn fn_index(
    struct_name: &syn::Ident,
    struct_iden: &Vec<syn::Ident>,
    struct_type: &Vec<syn::Type>,
) -> proc_macro2::TokenStream {
    let doc = r#"
如果`struct`里的字段类型都一致则语法糖生效

If the fields in `struct` are of the same type, then syntactic sugar takes effect

```
#[derive(Rustdict)]
struct Test {
    a: u8,
    b: u8,
}
let mut test = Test {
    a: 0u8,
    b: 0u8,
};
for I in test.keys(){
    test[I] = 1u8;
}
println!("{:?}", test);
test["b] = 2u8;
println!("{:?}", test);
```
```
--> [1, 1]
--> [1, 2]
```
"#;
    if is_one_type(struct_type) {
        let struct_type = &struct_type[0];
        return quote! {
            #[doc = #doc]
            impl core::ops::Index<&'static str> for #struct_name {
                type Output = #struct_type;

                fn index(&self, index: &'static str) -> &Self::Output {
                    match index {
                        #(stringify!(#struct_iden) => &self.#struct_iden,)*
                        _ => panic!("不存在的key"),
                    }
                }
            }
            impl core::ops::IndexMut<&'static str> for #struct_name {
                fn index_mut(&mut self, index: &'static str) -> &mut Self::Output {
                    match index {
                        #(stringify!(#struct_iden) => &mut self.#struct_iden,)*
                        _ => panic!("不存在的key"),
                    }
                }
            }
        };
    }
    quote! {}
}
