# config

```yaml
spec:
  root: selien-spec # 指向你的 selien root 文件夹, 当为一个文件夹的时候, selien 会在给定的文件夹中寻找 selien.config.yaml 文件. 当给到文件路径的时候, selien 会直接使用那个文件. 当给到相对路径时, selien 会在当前的 working directory 进行上述工作.
output:
  go: # 接受 go 或者 golang, 若同时定义则只生效最开始的定义
    modName: github.com/March-mitsuki/selien # 你 golang 的 mod name
    modRoot: packages/go # go mod 的根文件夹, 用于计算从根文件夹到 output 文件夹的距离
    output: packages/go/api/selien_spec # 想要输出的文件夹, 当为一个相对路径的时候, selien 会从当前的 working directory 开始算起.
    tabsize: 4 # golang 的默认 tabsize 为 4
  ts: # 接受 ts 或者 typescript, 若同时定义则只生效最开始的定义
    output: packages/ts/interfaces/selien_spec
    tabsize: 2 # typescript 的默认 tabsize 为 2
```