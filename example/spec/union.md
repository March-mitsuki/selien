# spec

Union type will be `interface{}` in golang.

```yaml
selien-version: 0.1.0
definition:
  unionType:
    declaration: type-alias
    body:
      type: union
      types:
        - type: number
        - type: string
```

# output

typescript:
```ts
export type UnionType = number | string;
```

go:
```go
type UnionType interface{}
```
