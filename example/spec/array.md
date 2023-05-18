# spec

```yaml
selien-version: 0.1.0
definition:
  normalArray:
    declaration: type-alias
    body:
      type: array
      items:
        type: string
  nestedArray:
    declaration: type-alias
    body:
      type: array
      items:
        type: array
        items:
          type: number
  objectArray:
    declaration: type-alias
    body:
      type: array
      items:
        type: object
        properties:
          stat:
            type: boolean
```

# output

typescript:
```ts
export type NormalArray = string[];
export type NestedArray = number[][];
export type ObjectArray = {
  stat: boolean;
}[];
```

go:
```go
type NormalArray []string
type NestedArray [][]int
type ObjectArray []struct {
  Stat bool `json:"stat"`
}
```
