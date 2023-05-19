# 贡献

## 我找到了一个bug, 并且能够修复它, 我应该怎么办?

非常感谢你百忙之中来帮助我们，你可以先创建一个 issue, 然后 fork 一下这个 repo, 根据下面 [开发环境](#开发环境) 的引导对程序进行修改之后, 通过 pull request 提交给我们!

## 我想为 selien 添加一个新的支持语言, 我需要做些什么?

selien 已经实现了把 yaml 文件 transform 到 AST 的工作。你需要做的是为这个新的语言实现 generator 和 compiler 功能。

generator 将 selien 的 AST 转换为特定语言的代码, 而 compiler 将他们与一些特殊的 metadata (例如 import 等) 一起生成到指定的文件。

**generator**

现有的 generator 实现你可以在 [这里看到](../../src/generator/lang/). 

当你为新语言实现一个 generator 时，别忘了连测试一起写！ selien 使用 [insta](https://github.com/mitsuhiko/insta) 作为快照测试的库。当不是必要更改时, 现有快照不应该被轻易改动。

最后也别忘了在接口调用处的 `match` 文处加上你新实现的调用.

**compiler**

现有的 compiler 实现你可以在 [这里看到](../../src/compiler/)

compiler 和 generator 有些不同, 虽然也是按照语言实现, 但并没有统一的大一统接口。你需要分别为每个语言实现`importer`和`special`等功能。

例如 golang 中的 `importer` 就是为了实现 `$ref` 的跨文件复用。

而 `special` 是因为 golang 的复用和 typescript 不一样, 必须写成`<imported-pkg>.TypeName`的形式, 所以需要一些特殊的逻辑。`special`就是干这种事的地方。

目前(v0.1.x)的 `compiler` 接口设计还比较简陋, 如果各位有什么好的设计思路也可以在 issue 中提出来。

# 开发环境

selien 使用 husky 来管理 git-hooks, 以及使用 pnpm 来简略化各种 cli 操作, 所以你也需要 nodejs 作为开发环境.

## require

1. nodejs (18+)
2. pnpm
3. rust

## how to dev

首先安装 nodejs 和 pnpm 以及 rust, 官网都有相应的安装教程, 这里就不赘述了。

然后 clone 这个 repo

```bash
git clone git@github.com:March-mitsuki/selien.git
```

之后进入 clone 下来的文件夹

```bash
cd selien
```

然后使用 pnpm 安装 husky

```bash
pnpm i && pnpm prepare
```

之后你就可以开始开发了, 让我们先跑个测试试一下

```
pnpm test
```

完全通过就代表没问题, 可以开始开发了
