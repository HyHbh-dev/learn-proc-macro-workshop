// 01-parse.rs
// use proc_macro::TokenStream;
// use syn::{self, DeriveInput};

// #[proc_macro_derive(Builder)]
// pub fn derive(input: TokenStream) -> TokenStream {
//     let _ = syn::parse_macro_input!(input as DeriveInput);

//     TokenStream::new()
// }

// 02-create-builder.rs
// use proc_macro::TokenStream;
// use quote::quote;
// use syn::{parse, parse_macro_input, DeriveInput};

// #[proc_macro_derive(Builder)]
// pub fn derive(input: TokenStream) -> TokenStream {
//     // 解析输入的 TokenStream 并将其转换为 DeriveInput
//     let x = parse_macro_input!(input as DeriveInput);
//     // 获取输入结构体的标识符
//     let name = &x.ident;
//     // 使用quote!宏生成impl块 生成的是proc_macro2 TokenStream
//     let expanded = quote! {
//         impl #name {
//             pub fn builder()-> CommandBuilder{
//                 CommandBuilder{
//                     executable: None,
//                     args: None,
//                     env: None,
//                     current_dir: None,
//                 }
//             }
//         }
//     };
//     TokenStream::from(expanded)
// }
