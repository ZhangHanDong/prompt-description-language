# PDL Compiler[POC]

This crate is used for:

- 解析 PDL 格式文件，用于验证配置文件格式正确性（TODO）。
- Rust 序列化反序列化支持（TODO）。
- 编译：
    - 处理 PDL 格式为最优 tokens 数。
    - 支持模块化（TODO）

## 使用:

> cargo run -- -i ./template -f ai.pdl -o mr_life -d ./output

or

> ./pdl_compiler -i ./template -f ai.pdl -o mr_life -d ./output

## Prompt Tokens 优化原则：

1. 尽可能减少 Prompt 中的符号使用。对于 GPT 来说，配对的符号 `[]/{}/()/""/''`算做一个token。空格和换行也会占token。
2. 使用英文描述 Prompt，这也是推荐 PDL 的原因。PDL 优于 `Json/yaml/toml/markdown`。
2. 尽可能采用 PDL 编写原则。