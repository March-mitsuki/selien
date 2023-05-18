# spec

```yaml
selien-version: 0.1.0
definition:
  normalObject:
    declaration: type-alias
    body:
      type: object
      properties:
        username:
          type: string
        password:
          type: any
  nestedObject:
    declaration: type-alias
    body:
      type: object
      properties:
        head:
          type: object
          properties:
            cmd:
              type: string
            stat:
              type: number
        body:
          type: number
```

# output

typescript:
```ts
export type NormalObject = {
   username: string;
   password: any
};
export type NestedObject = {
   head: {
      cmd: string;
      stat: number;
   };
   body: number;
};
```

go:
```go
type NormalObject struct {
   Username string `json:"username"`
   Password interface{} `json:"password"`
}
type NestedObject struct {
   Head struct {
      Cmd string `json:"cmd"`
      Stat int `json:"cmd"`
   } `json:"head"`
   Body int `json:"body"`
}
```