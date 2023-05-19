# config

[简体中文](./translation/config/zh.md)

Let's look at an example first:

```yaml
spec:
  root: selien-spec
output:
  go: # Accepts go or golang keyword, if both defined, only the first definition will take effect
    mod_name: github.com/March-mitsuki/selien
    root: pckages/go
    output: packages/server/api/selien_spec
    tabsize: 4
  ts: # Accepts ts or typescript keyword, if both defined, only the first definition will take effect
    output: packages/web/interfaces/selien_spec
    tabsize: 2
```

## spec

The place where selien is defined, currently only accepts one value `root`

| Name     | Is required | Default | Type   | Description |
| -------- | ----------- | ------- | ------ | --- |
| root [1] | required    | -       | string | The path pointing to your selien-root folder |

- root [1]
  - It could be either **absolute path** or _relative path_. It can be either a **folder** or directly point to a _specific file_
  - If it is a folder, selien will look for the `selien.config.yaml` file in the given folder.
  - If it is a file path, selien will directly use that file.
  - If it is a relative path, selien will perform the above tasks in the current working directory.

## output

### go

| Name       | Is required | Default | Type   | Description |
| ---------- | ----------- | ------- | ------ | --- |
| mod_name   | required    | -       | string | The name of the module in go.mod |
| [1] root   | required    | -       | string | A path pointing to the root folder of your go project |
| [2] output | required    | -       | string | Output file location |
| tabsize    | optional    | 4       | number | Tab size to be used when indenting |

- root [1], output [2]
  - Both accept either an **absolute path** or a _relative path_
  - When it's a _relative path_, selien will use the current working directory as a reference point

### typescript

| Name       | Is required | Default | Type   | Description |
| ---------- | ----------- | ------- | ------ | --- |
| [1] output | required    | -       | string | Output file location |
| tabsize    | optional    | 4       | number | Tab size to be used when indenting |

- output [1]
  - Accepts either an **absolute path** or a _relative path_
  - When it's a _relative path_, selien will use the current working directory as a reference point
