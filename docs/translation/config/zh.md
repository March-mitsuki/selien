# config

先看一个例子:

```yaml
spec:
  root: selien-spec
output:
  go: # 接受 go 或者 golang 关键字, 若同时定义则只生效最开始的定义
    mod_name: github.com/March-mitsuki/selien
    root: pckages/go
    output: packages/server/api/selien_spec
    tabsize: 4
  ts: # 接受 ts 或者 typescript 关键字, 若同时定义则只生效最开始的定义
    output: packages/web/interfaces/selien_spec
    tabsize: 2
```

## spec

对 selien 进行定义的地方, 目前只接受一个值 `root`

| Name     | Is required | Default | Type   | Description |
| -------- | ----------- | ------- | ------ | --- |
| root [1] | required    | -       | string | 指向你的 selien-root 文件夹的 path |

- root [1]
  - 可以是 **绝对路径** 或者是 _相对路径_. 也可以是 **文件夹** 或者直接指向一个 _具体的文件_
  - 当为一个文件夹的时候, selien 会在给定的文件夹中寻找 `selien.config.yaml` 文件.
  - 当给到文件路径的时候, selien 会直接使用那个文件.
  - 当给到相对路径时, selien 会在当前的 working directory 进行上述工作.


## output

### go

| Name       | Is required | Default | Type   | Description |
| ---------- | ----------- | ------- | ------ | --- |
| mod_name   | required    | -       | string | go.mod 中 module 的名字 |
| [1] root   | required    | -       | string |一个指向你go project 的 root 文件夹的 path |
| [2] output | required    | -       | string | 输出文件位置 |
| tabsize    | optional    | 4       | number | 缩进时使用的 tabsize |

- root [1], output [2]
  - 都接受一个 **绝对路径** 或者 _相对路径_
  - 当为 _相对路径_ 时, selien 会以当前的 working directory 为基准点


### typescript

| Name       | Is required | Default | Type   | Description |
| ---------- | ----------- | ------- | ------ | --- |
| [1] output | required    | -       | string | 输出文件位置 |
| tabsize    | optional    | 4       | number | 缩进时使用的 tabsize |

- output [1]
  - 都接受一个 **绝对路径** 或者 _相对路径_
  - 当为 _相对路径_ 时, selien 会以当前的 working directory 为基准点