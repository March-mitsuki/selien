# spec

支持不同语言做不同定义, 保证各个语言中特殊的类型定义不会互相冲突

```yaml
selien-version: 0.1.0
definition:
  splitType:
    declaration: type-alias
    body:
      type: split
      ts: # selien 会把你的定义分发到不同语言中
        type: number
      go:
        type: string
  splitSide:
    declaration: type-alias
    body:
      type: split
      ts: # 也可以只定义单个语言, 此时其他语言会无视此项
        type: object
        properties:
          head:
            type: boolean
          body:
            type: string
```

# output

## typescript

```ts
export type SplitType = number;
export type SplitSide = {
  head: boolean;
  body: string;
};
```

## go

```go
type SplitType string
```
