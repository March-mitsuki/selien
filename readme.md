# selien, a ssot interface specification

[简体中文](./docs/readme/zh.md)

**selien = connective(English) + 連携(日本語) + lien(français) + 连接(中文)**

ssot, [Single source of truth](https://en.wikipedia.org/wiki/Single_source_of_truth).

A tool for automatically generating type definitions from a **single source** file. Your great companion for cross-language development.

Inspire by [openAPI](https://www.openapis.org/).

- **What it does**
  - Helps you generate type definitions across languages by writing YAML files according to conventions.
  - Improves the quality of your cross-language code by following the rules of a single trusted source.
- **What it is not**
  - It is not a replacement for openAPI or similar tools like gRPC.
  - It doesn't handle as many things as openAPI does. It only focuses on generating type definitions, and you are responsible for the rest of the logic.
- **Currently supported languages**
  - typescript
  - go

# How to use

To explain the syntax, we first need to explain our file structure. Before you start writing your first selien file, you need to specify a folder as the `root` folder. All type definitions start from here.

For example, if you have an `my_project` folder as your project root directory, you can create a folder called `selien-spec `inside it and create YAML files inside that folder. Alternatively, you can continue creating folders below.

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

Then, you need to create a file named `selien.config.yaml` in your `<project-root>` folder and write your settings inside it. If we continue with the folder structure mentioned above:

```yaml
spec:
  root: selien-spec # Points to your selien root folder. When it is a folder, selien will look for the selien.config.yaml file in that folder. When given a file path, selien will use that file directly. When given a relative path, selien will perform the above steps in the current working directory.
output:
  go: # Accepts go or golang. If both are defined, only the first definition will take effect.
    modName: github.com/March-mitsuki/selien # Your golang mod name
    output: packages/go/selien_spec # The output folder. When it is a relative path, selien will start from the current working directory.
    tabsize: 4 # The default tabsize for golang is 4.
  ts: # Accepts ts or typescript. If both are defined, only the first definition will take effect.
    output: packages/ts/selien_spec
    tabsize: 2 # The default tabsize for typescript is 2.
```

- selien will automatically interpret the paths specified in the folder and maintain their structure.
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

Selien will generate the code for the language(s) you defined in `selien.config.yaml` into the specified folder. For example, based on the settings we defined above, we will get:
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

That's it! It's quite simple, isn't it? If you want to learn more about the syntax, you can take a look at the [expamle folder](./example/spec/)

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

# contribute
see [contribute](./contribution.md)
