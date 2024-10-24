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
    // 解析输入的 TokenStream 并将其转换为 DeriveInput
    let ast = parse_macro_input!(input as DeriveInput);
    match do_expand(&ast) {
        Ok(token_stream) => token_stream.into(),
        Err(e) => e.to_compile_error().into(),
    }
}

// 获取AST节点名称转换为{}Builder
fn do_expand(ast: &syn::DeriveInput) -> syn::Result<proc_macro2::TokenStream> {
    eprintln!("Struct name: {:#?}", ast.span());

    let struct_name_ident = &ast.ident;
    let struct_name_literal = struct_name_ident.to_string();
    let builder_name_literal = format!("{}Builder", struct_name_literal);
    //  span作用为获取报错源代码位置
    let builder_name_ident = syn::Ident::new(&builder_name_literal, struct_name_literal.span());

    let builder_struct_field_def = generate_builder_struct_fields_def(ast)?;
    let builder_struct_init = generate_builder_struct_factory_init_clauses(ast)?;

    let ret = quote::quote! {
        // TODO
        pub struct #builder_name_ident {
            #builder_struct_field_def
        }

        impl #struct_name_ident{
            pub fn builder()->#builder_name_ident{
                // TODO
                #builder_name_ident{
                    #(#builder_struct_init,)*
                }
            }
        }
    };
    Ok(ret)
}

// 获取struct 中的fields

fn get_fields_from_derive_input(
    ast: &syn::DeriveInput,
) -> Result<&syn::punctuated::Punctuated<syn::Field, syn::token::Comma>, syn::Error> {
    if let syn::Data::Struct(syn::DataStruct {
        fields: syn::Fields::Named(syn::FieldsNamed { ref named, .. }),
        ..
    }) = ast.data
    {
        Ok(named)
    } else {
        Err(syn::Error::new_spanned(
            ast,
            "Expected a struct with named fields",
        ))
    }
}

fn generate_builder_struct_fields_def(
    ast: &syn::DeriveInput,
) -> syn::Result<proc_macro2::TokenStream> {
    // 获取字段
    let fields = get_fields_from_derive_input(ast)?;
    // 获取fields名字和类型并生成Vec
    let idents: Vec<_> = fields.iter().map(|f| &f.ident).collect();
    let types: Vec<_> = fields.iter().map(|f| &f.ty).collect();
    let fields_def = quote::quote! {
        #(pub #idents:Option<#types>,)*
    };
    Ok(fields_def)
}

// 初始化builder方法构造函数
fn generate_builder_struct_factory_init_clauses(
    ast: &syn::DeriveInput,
) -> syn::Result<Vec<proc_macro2::TokenStream>> {
    let fields = get_fields_from_derive_input(ast)?;
    let init_cluase: Vec<_> = fields
        .iter()
        .map(|f| {
            let ident = &f.ident;
            quote::quote! {
                #ident:None
            }
        })
        .collect();

    Ok(init_cluase)
}
