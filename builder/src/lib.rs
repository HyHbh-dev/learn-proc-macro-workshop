// 01-parse.rs
// use proc_macro::TokenStream;
// use syn::{self, DeriveInput};

// #[proc_macro_derive(Builder)]
// pub fn derive(input: TokenStream) -> TokenStream {
//     let _ = syn::parse_macro_input!(input as DeriveInput);

//     TokenStream::new()
// }

// 02-create-builder.rs
use proc_macro::TokenStream;
use syn::{parse_macro_input, spanned::Spanned, DeriveInput};

#[proc_macro_derive(Builder)]
pub fn derive(input: TokenStream) -> TokenStream {
    // 获取输入结构体转为 AST
    let ast = parse_macro_input!(input as DeriveInput);
    match do_expand(&ast) {
        Ok(expanded) => expanded.into(),
        Err(e) => e.to_compile_error().into(),
    }
}

fn do_expand(ast: &syn::DeriveInput) -> syn::Result<proc_macro2::TokenStream> {
    let struct_ident = &ast.ident;
    let struct_ident_literal = struct_ident.to_string();
    let builder_ident = syn::Ident::new(
        &format!("{}Builder", struct_ident_literal),
        struct_ident.span(),
    );

    // 生成结构体字段定义和初始化代码
    // pub struct CommandBuilder {
    //         executable: Option<String>,
    //         args: Option<Vec<String>>,
    //         env: Option<Vec<String>>,
    //         current_dir: Option<String>,
    //     }
    let struct_item = generate_builder_struct_fields_def(ast)?;
    //     impl Command {
    //         pub fn builder() -> CommandBuilder {
    //             CommandBuilder {
    //                 executable: None,
    //                 args: None,
    //                 env: None,
    //                 current_dir: None,
    //             }
    //         }
    //     }
    let struct_init = generate_builder_struct_fields_init(ast)?;

    let res = quote::quote! {
        pub struct #builder_ident {
            #struct_item
        };

        impl #struct_ident {
            pub fn builder() -> #builder_ident {
                #builder_ident {
                    #(#struct_init),*
                }
            }
        }
    };
    Ok(res)
}

// 获取结构体内部数据
fn get_struct_fields(
    ast: &syn::DeriveInput,
) -> Result<&syn::punctuated::Punctuated<syn::Field, syn::token::Comma>, syn::Error> {
    if let syn::Data::Struct(syn::DataStruct {
        fields: syn::Fields::Named(syn::FieldsNamed { ref named, .. }),
        ..
    }) = &ast.data
    {
        Ok(named)
    } else {
        Err(syn::Error::new(
            ast.span(),
            "Builder derive only supports structs with named fields",
        ))
    }
}

// 生成结构体字段定义
fn generate_builder_struct_fields_def(
    ast: &syn::DeriveInput,
) -> syn::Result<proc_macro2::TokenStream> {
    let fields = get_struct_fields(ast)?;
    let ident: Vec<_> = fields.iter().map(|f| &f.ident).collect();
    let ty: Vec<_> = fields.iter().map(|f| &f.ty).collect();
    let ret = quote::quote! {
        #(#ident: Option<#ty>,)*
    };
    Ok(ret)
}

// 设置结构体字段初始化为None
fn generate_builder_struct_fields_init(
    ast: &syn::DeriveInput,
) -> syn::Result<Vec<proc_macro2::TokenStream>> {
    let fields = get_struct_fields(ast)?;
    let init_clause: Vec<_> = fields
        .iter()
        .map(|f| {
            let ident = &f.ident;
            quote::quote! {
                #ident: None
            }
        })
        .collect();

    Ok(init_clause)
}
