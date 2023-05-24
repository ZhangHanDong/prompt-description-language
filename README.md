# Prompt Description Language (V0.1.1 POC)

## Description

PDL (Prompt Description Language) format provides an extensible way to describe the behavior and characteristics of prompts. Inspired by `json/yaml/toml/markdown` designs, PDL aims to minimize the token count compared to `json/yaml/toml/markdown`.

## Basic Syntax Description

The basic rules of the PDL structure are as follows:

- `{}` represents a structure, and `.` is also used to express hierarchical structure.
-  `@` represents a reference to a specified structure field.
- `import` signifies the importation of the structure referenced by `@`.
- In `Key: Value` key-value pairs, the Key generally does not need to be enclosed in quotes (unless it contains other special characters, in which case single or double quotes can be used). Value can take the following forms:
    - `Key: "value"`
    - `Key: ["v1", "v2"]`
    - `Key: {k1: 'v1', k2: 'v2', k3: 'v3', ...}`
    - `Key: {'1/3': 'v1', '2/3': 'v2', '3/3': 'v3'}`
    - `Key: "<Value>"`
    - `Key: "<Value>" / None`
- `Key [ v1, v2, ...]`, used to define a sequence.


## Usage Case`

Example：[Mr.Trans](https://github.com/Illumine-Labs/Mr.trans)

## TODO

Despite its advantages in design, PDL still has some potential drawbacks or limitations:

- Complexity: The syntax of PDL can be relatively complex, especially for those unfamiliar with programming or scripting languages. This may make it difficult to create and modify PDL files, especially for complex model behaviors.
- Lack of documentation and tutorials: Since PDL is a specialized language that is not widely used, there may be a lack of sufficient tutorials and documentation, which can increase the difficulty for new users to learn and use it.
- Scalability and flexibility: Although PDL is designed to be an extensible language, it may still encounter situations where it cannot accommodate certain specific needs, especially when dealing with uncommon or unusual AI model behaviors.
- Tool support: Due to PDL not being a widely adopted language, there may be a lack of corresponding development and debugging tools, which can impact development efficiency and quality.
- Readability and maintainability: If a PDL file becomes too large or its structure becomes overly complex, it can affect its readability and maintainability. This is particularly true when lacking proper documentation and comments, which can make subsequent maintenance work difficult.

Based on actual use cases and requirements, there may be additional specific challenges and issues that need to be addressed.
