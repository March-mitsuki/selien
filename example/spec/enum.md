# spec

```yaml
selien-version: 0.1.0
definition:
  numberEnum:
    declaration: enum
    type: number
    members:
      - one: 1
      - two: 2
  stringEnum:
    declaration: enum
    type: string
    members:
      - h: Hello
      - w: World
```

# output

typescript:
```ts
export enum NumberEnum {
  one = 1,
  two = 2,
};
export enum StringEnum {
  h = "Hello",
  w = "World",
};
```

go:
```go
type numberEnum int
const (
  one numberEnum = 1
  two numberEnum = 2
)

type stringEnum string
const (
  h stringEnum = "Hello"
  w stringEnum = "World"
)
```
