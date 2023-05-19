# Contribution

[简体中文](./docs/translation/contribute/zh.md)

## I found a bug and can fix it. What should I do?

Thank you very much for taking the time to help us. 

You can start by creating an issue and then fork this repository. 

After making the necessary modifications to the program according to the instructions in the [Development](#development) below, you can submit a pull request to us.

## I want to add support for a new language to selien. What do I need to do?

Selien has already implemented the functionality to transform YAML files to AST. 

What you need to do is implement the generator and compiler functionalities for this new language.

**Generator**

You can see the existing generator implementations [here](./packages/core/src/generator/lang/).

When implementing a generator for a new language, don't forget to write tests as well! Selien uses [insta](https://github.com/mitsuhiko/insta) as the library for snapshot testing. Existing snapshots should not be easily modified unless necessary.

Also, don't forget to add the invocation of your new implementation in the `match` statement at the interface call site.

Compiler

You can see the existing compiler implementations [here](./packages/core/src/compiler/).

The compiler is different from the generator in that it doesn't have a unified interface for different languages. You need to separately implement functionalities like `importer` and `special` for each language.

For example, the `importer` in Golang is used to implement cross-file reuse of `$ref`.

And `special` is needed because Golang's reuse is different from TypeScript and other languages.

Go does not support importing directly into functions, so it requires specifying the package name when using imports. Like `<pkg-name>.TypeName`

`special` is where such logic is implemented.

The current (v0.1.x) design of the `compiler` interface is still relatively simple. If you have any good design ideas, you can also propose them in an issue.

# Development

Selien uses Husky to manage git hooks and PNPM to simplify various CLI operations, so you also need Node.js as the development environment.

## Requirements

1. nodejs (18+)
2. pnpm
3. rust

## how to dev

First, install Node.js, PNPM, and Rust. The official websites have corresponding installation tutorials, so we won't go into detail here.

Then clone this repository:

```bash
git clone git@github.com:March-mitsuki/selien.git
```

Navigate to the cloned folder:

```bash
cd selien
```

Install Husky using PNPM:

```bash
pnpm i && pnpm prepare
```

Now you can start development. Let's run a test first:

```
pnpm test
```

If all tests pass, it means everything is fine, and you can start development.

# about code

The main code here [packages/core](./packages/core/)

The other packages are intended for future release to npm using optionalDependencies.
