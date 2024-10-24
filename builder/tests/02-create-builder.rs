// 让宏为生成器状态生成一个结构，并生成一个 `builder` 结构。
// 函数，用于创建一个空的构建器实例。
//
// 作为快速入门，请尝试生成以下代码（但要确保类型
// 名称与调用者输入的名称一致）。
//
//     impl Command {
//         pub fn builder() {}
//     }
//
// 此时测试应该会通过，因为它并没有对
// 因此，`()` 作为生成器类型和其他类型一样好。
//
// 在继续之前，还需要生成宏：
//
//     pub struct CommandBuilder {
//         executable: Option<String>,
//         args: Option<Vec<String>>,
//         env: Option<Vec<String>>,
//         current_dir: Option<String>,
//     }
//
// and in the `builder` function:
//
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
//
//
// Resources:
//
//   - The Quote crate for putting together output from a macro:
//     https://github.com/dtolnay/quote
//
//   - Joining together the type name + "Builder" to make the builder's name:
//     https://docs.rs/syn/2.0/syn/struct.Ident.html

use derive_builder::Builder;

pub struct CommandBuilder {
    executable: Option<String>,
    args: Option<Vec<String>>,
    env: Option<Vec<String>>,
    current_dir: Option<String>,
}

#[derive(Builder)]
pub struct Command {
    executable: String,
    args: Vec<String>,
    env: Vec<String>,
    current_dir: String,
}

fn main() {
    let builder = Command::builder();

    let _ = builder;
}
