# selien, a ssot-type-specification

**selien = connective(English) + 連携(日本語) + lien(français) + 连接(中文)**

ssot, [Single source of truth](https://en.wikipedia.org/wiki/Single_source_of_truth).

从 **单一来源** 的 yaml 文件自动生成 type definition 的工具。你跨语言开发的好伙伴。

Inspire by [openAPI](https://www.openapis.org/).

- **它能帮你做什么**
  - 帮助你跨语言的生成类型定义, 你只需要按照约定编写 yaml 文件
  - 遵循单一可信任来源的规则来提高你跨语言编写代码的品质
- **它不是什么**
  - 它不是 openApi 的代替品, 也不会成为 openApi 或者 gPRC 这类东西的代替品
  - 它不像 openApi 一样管这么多, 它只负责生成类型定义, 其余的逻辑你需要自己负责
- **目前支持的语言**
  - typescript
    - 在 typescript 中, 我们更建议使用 type alias 而不是 interface。因为 interface 会在不经意间被扩张而导致危险。
  - go

# 如何使用

要解释语法, 首先我们需要解释我们的文件姐结构。在开始写你的第一个 selien 文件之前, 我们需要指定一个文件夹作为`selien-root`文件夹。所有的类型定义都从这里开始。

例如你有一个`src`文件夹作为你的主目录, 你可以在其下面建立一个叫做`selien-spec`的文件夹, 然后在里面创建 yaml 文件。或者继续在下面建立文件夹。

例如:

```sh
.
└── my_project # project root
    ├── src/
    │   └── ...your_other_file
    ├── selien-spec/ # selien root
    │   ├── shared.yaml # 想要直接共享的类型定义
    │   ├── rest/  # 用来存放关于 rest api 的类型定义文件
    │   │   └── api.yaml
    │   └── websocket/ # 用来存放关于 websocket 的类型定义文件
    │       └── common.yaml
    └── ...your_other_file
```

然后你需要在你的`<project-root>`文件夹下面, 创建一个叫做`selien.config.yaml`的文件, 并且在里面编写你的设置, 如果我们继续用上面介绍的文件夹结构:

```yaml
spec:
  root: selien-spec # 指向你的 selien root 文件夹, 当为一个文件夹的时候, selien 会在给定的文件夹中寻找 selien.config.yaml 文件. 当给到文件路径的时候, selien 会直接使用那个文件. 当给到相对路径时, selien 会在当前的 working directory 进行上述工作.
output:
  go: # 接受 go 或者 golang, 若同时定义则只生效最开始的定义
    modName: github.com/March-mitsuki/selien # 你 golang 的 mod name
    output: packages/go/selien_spec # 想要输出的文件夹, 当为一个相对路径的时候, selien 会从当前的 working directory 开始算起.
  ts: # 接受 ts 或者 typescript, 若同时定义则只生效最开始的定义
    output: packages/ts/selien_spec
```

- selien 会自动解释你指定的文件夹下的路径并保持其结构。
  - 例如使用 selien 生成 typescript 代码的情况下, 在`<selien-root>/rest/api.yaml`内的所有定义的类型, 会被生成到`<selien-output>/rest/api.ts`

然后我们终于可以来看语法部分了, 我们直接看一个简单的例子。

```yaml
# 例如此文件存在于 <selien-root>/shared.yaml

selien-version: 0.1.0 # 首先需要定义 selien 的版本
definition: # 每个 selien 文件一定首先要有一个 definition
  customString: # 这是一个类型的名字, 你可以随意定义. selien 会自动把首字母大写.
    declaration: type-alias # 目前支持2个 declaration, type-alias 和 enum.
    body: # declaration 必须有一个兄弟级别的 body
      type: string # 定义类型的类型, 详见 example 与下文的支持类型
```

然后运行 cli 命令:

```bash
selien gen
```

selien 就会自动生成你在 `selien.config.yaml` 中定义过的语言的代码到指定的文件夹了。例如我们上面定义的设定的话，我们会得到:
```ts
// <working-directory>/packages/ts/selien_spec/shared.ts

export type CustomString = string;

```
以及
```go
// <working-directory>/packages/go/selien_spec/shared.go

// 因为是直接在 selien_spec 下面的文件, 所以 package 为 selien_spec
package selien_spec

type CustomString string

```

就是这样, 是不是非常简单? 如果你还想知道更多的语法可以看看[expamle文件夹](../../../example/spec/)

# 目前支持的类型

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
    - 支持复用已经定义过的类型, 详见 [example](../../../example/spec/ref.md)
  - split
    - 支持分开定义不同语言中的类型, 详见 [example](../../../example/spec/split.md)
- enum
  - go -> a special type and const block
  - ts -> enum

# 安装

截止目前为止 (v0.1.x), 你有两种方式来安装 selien 到你的主机。
1. (推荐) 自行从源代码构建
2. 从 github release 下载二进制文件, 并且自己添加 path. (不推荐, 因为可能会有一些电子签名问题导致报错.)
   
这里仅介绍从源代码进行构建的方式。

**从源代码构建:**

首先你需要安装有 `rust`, 如果你还没有安装, 请遵循官方的安装步骤进行安装, 详情看 [这里](https://www.rust-lang.org/)

检查自己是否已经安装了 `rust` 可以运行下面这个命令:
```sh
rustup --version
```

然后你需要 clone 这个仓库 然后进入文件夹:
```sh
git clone https://github.com/March-mitsuki/selien.git && cd selien
```

然后运行下面这行命令从源代码构建 selien, 之后成果物的二进制文件就会在 `~/.selien/bin` 这个文件夹里了
```sh
cd packages/core && cargo build --bin selien --release && rm -rf ~/.selien && mkdir ~/.selien/bin && mv target/release/selien ~/.selien/bin && echo 'Selien is install to ~/.selien/bin'
```

**如果你是windows用户**
那么请在 `powershell` 中运行 _下面这行_ 命令来构建 selien, 之后成果物的二进制文件就会在 `~/.selien/bin` 这个文件夹里了
```powershell
cd packages/core; if ($?) { cargo build --bin selien --release }; Remove-Item -Path ~/.selien -Recurse -ErrorAction Ignore; mkdir ~/.selien/bin; if ($?) { mv target/release/selien.exe ~/.selien/bin }
```

# 贡献

看 [这里](../contribute/zh.md)
