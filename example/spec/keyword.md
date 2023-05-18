# spec

```yaml
selien-version: 0.1.0
definition:
  customString:
    declaration: type-alias
    body:
      type: string
  customNumber:
    declaration: type-alias
    body:
      type: number
  customBoolean:
    declaration:  type-alias
    body:
      type: boolean
```

# output

typescript:
```ts
export type CustomString = string;
export type CustomNumber = number;
export type CustomBoolean = boolean;
```

go:
```go
type CustomeString string
type CustomeNumber int
type CustomeBoolean bool
```
