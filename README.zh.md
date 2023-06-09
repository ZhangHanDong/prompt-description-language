# 提示描述语言（Prompt Description Language） (V0.1.1 POC)

> 概念验证阶段

## 说明

PDL (Prompt Description Language) 格式提供了一种可扩展的方式来描述 Prompt 的行为和特性。参考 `json/yaml/toml/markdown`设计，相比于 `json/yaml/toml/markdown`，可以最大化节省 Token 数量。

PDL 尝试作为一种 GPT 可直接理解和识别的描述语言，专门用于描述成体系的模版化规范化的 Prompt，或可作为某种 Prompt 引擎的描述语言。

然而，PDL 也具备通用性，可用于一般的编程实践中，但是需要完善配套工具和文档。

## 语法基本描述

PDL 结构的基本规则如下：

- `{}` 代表一个结构，`.` 也用于表示层级结构。
- `@` 表示对指定结构字段的引用。
- `import` 表示导入由 `@` 引用的结构。
- 在 `Key: Value` 键值对中，键一般不需要用引号包围（除非它包含其他特殊字符，这种情况下可以使用单引号或双引号）。值可以采取以下形式：
    - `Key: "value"`
    - `Key: ["v1", "v2"]`
    - `Key: {k1: 'v1', k2: 'v2', k3: 'v3', ...}`
    - `Key: {'1/3': 'v1', '2/3': 'v2', '3/3': 'v3'}`
    - `Key: "<Value>"`
    - `Key: "<Value>" / None`
- `Key [ v1, v2, ...]`，用于定义一个序列。


## 案例

示例参考：[一个 Prompt 将 ChatGPT 打造为学习和翻译助手 ：Mr.Trans](https://github.com/Illumine-Labs/Mr.trans)


## PDL 编写原则：

为了优化 Prompt Tokens 长度在编写 PDL 时应该采用以下原则：

1. **精简语言**：尽可能地使用简洁明了的语言。避免冗余和复杂的表达方式。例如，你可以将 "我想知道如何优化 Prompt Tokens 长度" 简化为 "如何优化 Prompt Tokens 长度"。
2. **避免过多的上下文**：虽然上下文信息对于 AI 理解问题很重要，但是过多的上下文信息可能会导致 Prompt Tokens 过长。因此，你应该只提供足够的上下文信息，避免不必要的细节。
3. **使用关键词**：尽可能地使用关键词，而不是长句。例如，你可以使用 "Prompt Tokens 优化"，而不是 "我想知道如何优化 Prompt Tokens"。
4. **分解复杂问题**：如果你的问题很复杂，你可以尝试将它分解成几个简单的问题。这样，你可以分别对每个问题进行提问，而不是在一个 prompt 中包含所有的问题。
5. **实验和迭代**：通过实验和迭代，你可以找到最有效的 prompt 长度。你可以尝试使用不同长度的 prompt，看看哪个长度的 prompt 能得到最好的结果。

## TODO

功能：

- 编辑器高亮支持
- PDL Compiler
- 文档完善

尽管它在设计上具有一定的优点，但仍然存在一些可能的缺陷或者限制：

- 复杂性：PDL 的语法可能相对复杂，尤其是对于那些不熟悉编程或脚本语言的人来说。这可能使得创建和修改 PDL 文件变得困难，尤其是对于复杂的模型行为。
- 文档和教程的缺乏：由于 PDL 是一种专门的、不广泛使用的语言，可能缺乏足够的教程和文档，这会增加新用户学习和使用它的难度。
- 可扩展性和灵活性：虽然 PDL 被设计成一种可扩展的语言，但它可能仍然存在无法适应某些特定需求的情况，尤其是在面对一些特殊的、不常见的 AI 模型行为时。
- 工具支持：由于 PDL 不是一种广泛使用的语言，可能缺乏相应的开发和调试工具，这可能会影响开发效率和质量。
- 可读性和可维护性：如果 PDL 文件的规模过大或者结构过于复杂，可能会影响其可读性和可维护性。尤其是在没有良好的文档和注释的情况下，可能会导致后续维护工作困难。

根据实际的使用场景和需求，可能会有更多特定的挑战和问题需要解决。


