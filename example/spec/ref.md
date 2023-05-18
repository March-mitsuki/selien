# spec

### 首先, 关于 `$ref` 的使用, 需要遵循一些规则:

- 只能引用 selien 中进行了的定义, 无法引用外部的定义
- 引用同一文件中的定义
  - 以`#`号开头, 表示当前文件
  - 例如引用同一文件中的`user`definition
    - `uri: "#/definition/user"`
- 引用别处文件中的定义
  - 以`#`号分割, #号往前为 `path/to/ref/file`, #号往后为引用定义在引用文件中的位置, 与引用同一文件时写法相同
  - #号往前可以使用 **绝对路径** 或者 _相对路径_
    - 当使用 **绝对路径** 时, 这个路径必须是从 `<selien-root>` 开始到你要引用的文件的路径
    - 当使用 _相对路径_ 的时候, 路径则必须是从 `该文件` 到引用文件的相对位置


例如我们的文件夹结构是这样的, 其中`selien-spec`是我们在 config 中定义的 selien-root 文件夹
```sh
.
└── my_project/
    └── selien-spec/
        ├── ping.yaml
        └── rest/
            ├── user/
            │   └── uesr.yaml
            └── logging.yaml
```

例如里面的内容是这样的:

```yaml
# selien-spec/ping.yaml

selien-version: 0.1.0
definition:
  pingReq:
    declaration: type-alias
    body:
      type: any
```

```yaml
# selien-spec/rest/user/user.yaml

selien-version: 0.1.0
definition:
  registerReq:
    declaration: type-alias
    body:
      type: object
      properties:
         email: string
         password: string
```

然后我们要在 `logging.yaml` 中去复用这两个文件中的定义

```yaml
# selien-spec/rest/logging.yaml

selien-version: 0.1.0
definition:
  loggingReq:
    declaration: type-alias
    body:
      type: any
  refSelf:
    declaration: type-alias # $ref 只能够在 type-alias内使用
    body:
      type: $ref
      uri: "#/definition/loggingReq" # 复用此文件中的 loggingReq
  refOuter:
    declaration: type-alias
    body:
      type: $ref
      uri: "../ping#/definition/pingReq" # 复用 ping.yaml 中的 pingReq, 使用相对路径, 注意文件名不包含.yaml, 直接为ping
  refInner:
    declaration: type-alias
    body:
      type: $ref
      uri: "/rest/user/user#definition/registerReq" # 复用 user.yaml 中的 registerReq, 使用绝对路. 注意此路径是从 <selien-root> 开始算起的绝对路径
```

# output

## typescript

```ts
// <ts-output>/ping.ts

export type PingReq = any;
```

```ts
// <ts-output>/rest/user/user.ts

export type RegisterReq = {
   email: string;
   password: string;
};
```

```ts
// <ts-output>/rest/logging.ts

import { PingReq } from "../ping";
import { RegisterReq } from "./user/user";

export type LoggingReq = any;
export type RefSelf = LoggingReq;
export type RefOuter = PingReq;
export type RefInner = RegisterReq;
```

## go

这里假定你在 config 中正确配置了 go 的输出

```go
// <go-output>/ping.go
package <go-output>

type PingReq interface{}
```

```go
// <go-output>/rest/user/user.go
package user

type RegisterReq struct {
   Email string `json:"email"`
   Password string `json:"password"`
}
```

```go
// <go-output>/rest/logging.ts
package rest

// selien 会自动根据你的配置帮你处理 import 路径
// 所以这让跨文件引用变的非常简单
import (
   "<go-mod>/<go-output>"
   "<go-mod>/<go-output>/rest/user"
)

type LoggingReq interface{}
type RefSelf LoggingReq
type RefOuter <go-output>.PingReq;
type RefInner user.RegisterReq;
```
