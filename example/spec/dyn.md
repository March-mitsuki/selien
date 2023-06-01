# spec

`$dyn` 能够让你直接引用外部包的一些定义, `$dyn` 常常和 `split` 一起使用, 因为大概率你每个语言中的包名字都不一样

例如这个例子我们在 ts 中引用了 `react` 包中的 `CSSProperties`, 然后在 go 中引用了在 `selien/api/types` 中自定义的一个 `Css` 类型

```yaml
selien-version: 0.1.0
definition:
  dynamicHead: # 创建一个 dynmicHead 作为两个 split 的共同部分
    declaration: type-alias
    body:
      type: object
      properties:
        token:
          type: string
  refDynamic:
    declaration: type-alias
    body:
      type: split
      ts:
        type: object
        properties:
          head:
            type: $ref
            uri: "#/definition/dynamicHead"
          body:
            type: object
            properties:
              css:
                type: $dyn
                name: "CSSProperties"
                from: "react"
      go:
        type: object
        properties:
          head:
            type: $ref
            uri: "#/definition/dynamicHead"
          body:
            type: object
            properties:
              css:
                type: $dyn
                name: "css"
                from: "selien/api/types"
```

# output

typescript:
```ts
// 你可以看到, 在 ts 中, selien 只是复制粘贴你的 $dyn 到代码中, 不做其他任何操作
import { CSSProperties } from "react";

export type DynamicHead = {
  token: string;
};
export type RefDynamic = {
  head: DynamicHead;
  body: {
    css: CSSProperties;
  };
};
```

go:
```go
import (
  "selien/api/types"
)

type DynamicHead struct {
  Token string `json:"token"`
}
type RefDynamic struct {
  Head DynamicHead `json:"head"`
  Body struct {
    // 在 go 中, selien 会自动将首字母大写 (如果你没有在selien中大写它的话), 以保持代码符合 go 的语法
    Css types.Css `json:"css"`
  } `json:"body"`
}
```