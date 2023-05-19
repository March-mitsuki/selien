# selien, a ssot interface specification

[简体中文](./docs/translation/readme/zh.md)

**selien = connective(English) + 連携(日本語) + lien(français) + 连接(中文)**

ssot, [Single source of truth](https://en.wikipedia.org/wiki/Single_source_of_truth).

A tool for automatically generating type definitions from a **single source** file. Your great companion for cross-language development.

Inspire by [openAPI](https://www.openapis.org/).

- **What selien does**
  - Helps you generate type definitions across languages by writing YAML files according to conventions.
  - Improves the quality of your cross-language code by following the rules of a single trusted source.
- **What selien is not**
  - Selien is not a replacement for openAPI or similar tools like gRPC.
  - Selien doesn't handle as many things as openAPI does. It only focuses on generating type definitions, and you are responsible for the other logic.
- **Currently supported languages**
  - typescript
  - go

# How to use

To explain the syntax, we first need to explain our file structure. Before you start writing your first selien file, you need to specify a directory as the `selien-root` directory. All type definitions start from here.

For example, if you have an `my_project` directory as your project root directory, you can create a directory called `selien-spec `inside it and create YAML files inside that directory. Alternatively, you can continue creating directorys below.

For example:

```sh
.
└── my_project # project root
    ├── src/
    │   └── ...your_other_file
    ├── selien-spec/ # selien root
    │   ├── shared.yaml # Type definitions to be directly shared
    │   ├── rest/  # Type definition files for REST API
    │   │   └── api.yaml
    │   └── websocket/ # Type definition files for WebSocket
    │       └── common.yaml
    └── ...your_other_file
```

Then, you need to create a file named `selien.config.yaml` in your `<project-root>` directory and write your settings inside it. If we continue with the directory structure mentioned above:

For more information about `selien.config.yaml`, see [here]

```yaml
spec:
  root: selien-spec # Path to your selien root directory. When it is a directory, selien will look for the selien.config.yaml file in that directory. When given a file path, selien will use that file directly. When given a relative path, selien will perform the above steps in the current working directory.
output:
  go: # Accepts go or golang. If both are defined, only the first definition will take effect.
    mod_name: github.com/March-mitsuki/selien # Your golang mod name
    mod_root: pckages/go # The root directory of golang project. Selien use this to calculate path.
    output: packages/go/selien_spec # The output directory. When it is a relative path, selien will start from the current working directory.
  ts: # Accepts ts or typescript. If both are defined, only the first definition will take effect.
    output: packages/ts/selien_spec
```

- selien will automatically interpret the paths specified in the directory and keep their structure.
  - For example, when generating TypeScript code with selien, all the types defined in `<selien-root>/rest/api.yaml`will be generated into `<ts-output>/rest/api.ts`.

Now, let's finally look at the syntax. We'll start with a simple example.

```yaml
# For example, this file exists in <selien-root>/shared.yaml

selien-version: 0.1.0 # First, we need to define the version of selien.
definition: # Each selien file must start with a definition.
  customString: # This is the name of the type. You can define it freely. Selien will automatically capitalize the first letter.
    declaration: type-alias # Currently supports two declarations: type-alias and enum.
    body: # A declaration must have a sibling body.
      type: string # Defines the type of the type. See examples and the supported types below.
```

Then run the CLI command:

```bash
selien gen
```

Selien will generate the code for the language(s) you defined in `selien.config.yaml` into the specified directory. For example, based on the settings we defined above, we will get:
```ts
// <working-directory>/packages/ts/selien_spec/shared.ts

export type CustomString = string;

```
and
```go
// <working-directory>/packages/go/selien_spec/shared.go

// Since it's directly under selien_spec, the package is selien_spec.
package selien_spec

type CustomString string
```

That's it! It's quite simple, isn't it? If you want to learn more about the syntax, you can take a look at the [expamle directory](./example/spec/)

# Currently supported types

- type-alias
  - number
    - go -> int
    - ts -> number
  - string
    - go -> string
    - ts -> string
  - boolean
    - go -> bool
    - ts -> boolean
  - any
    - go -> interface{}
    - ts -> any
  - object
    - go -> struct
    - ts -> type alias object
  - array
    - go -> slice
    - ts -> array
  - number literal
    - go -> int
    - ts -> number literal
  - string literal
    - go -> string
    - ts -> string literal
  - $ref
    - Supports reusing previously defined types. See [example](./example/spec/ref.md)
  - split
    - Supports separating type definitions for different languages. See [example](./example/spec/split.md)
- enum
  - go -> a special type and const block
  - ts -> enum

# install

As of now (v0.1.x), there are two ways to install Selien onto your host.
1. (Recommended) Build from the source code.
2. Download the binary file from GitHub release and add the path yourself. (Not recommended as there may be some electronic signature issues that could cause errors.)

This guide will only cover how to build from the source code.

**Building from the source code:**

First, you need to have `rust` installed. If you haven't done so, please follow the official installation steps, see [here](https://www.rust-lang.org/)

You can check if you've already installed `rust` by running the command below:
```sh
rustup --version
```

Next, you need to clone this repository and enter the directory:
```sh
git clone https://github.com/March-mitsuki/selien.git && cd selien
```

Then run the following command to build Selien from the source code. The binary file will be located in the `~/.selien/bin` directory after that.
```sh
cd packages/core && cargo build --bin selien --release && rm -rf ~/.selien && mkdir ~/.selien/bin && mv target/release/selien ~/.selien/bin && echo 'Selien is installed to ~/.selien/bin'
```

**If you are a Windows user**
Please run the _command below_ in `powershell` to build Selien. The binary file will be in the `~/.selien/bin` directory thereafter.
```powershell
cd packages/core; if ($?) { cargo build --bin selien --release }; Remove-Item -Path ~/.selien -Recurse -ErrorAction Ignore; mkdir ~/.selien/bin; if ($?) { mv target/release/selien.exe ~/.selien/bin }
```

# contribution
see [contribution](./contribution.md)
