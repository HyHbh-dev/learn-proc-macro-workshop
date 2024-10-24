// 该测试将查找是否存在名称正确的派生宏。现在
// 测试并不要求宏生成任何特定代码，因此
// 返回一个空的令牌流（TokenStream）就足够了。
//
// 在继续之前，让派生宏将宏输入解析为
// syn::DeriveInput 语法树。
//
// 点击 docs.rs 花一些时间探索 syn::DeriveInput 结构。
// 浏览文档中的字段，看看它是否与你的
// 宏可以使用哪些信息的预期。
//
//
// Resources:
//
//   - The Syn crate for parsing procedural macro input:
//     https://github.com/dtolnay/syn
//
//   - The DeriveInput syntax tree which represents input of a derive macro:
//     https://docs.rs/syn/2.0/syn/struct.DeriveInput.html
//
//   - An example of a derive macro implemented using Syn:
//     https://github.com/dtolnay/syn/tree/master/examples/heapsize

use derive_builder::Builder;

#[derive(Builder)]
pub struct Command {
    executable: String,
    args: Vec<String>,
    env: Vec<String>,
    current_dir: String,
}

fn main() {}
