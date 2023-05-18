# todo

- [ ] golang 的 tag 支持, 目前会自动添加`json:"xxx"`的 tag, 之后想做到可以自定义(有必要吗？)
- [ ] 思考是否应该为 go 增加定义 interface 的功能
- [ ] 目前并未检测$ref的path是否合法, 之后考虑是否应该增加此功能
- [ ] 目前只接受`.yaml`后缀, 之后可以改成也接受`.yml`
- [ ] 尚未测试如果`output`路径设置的深度不一样会怎么样
  - [ ] 好像是还需要一个 mod-root, 来计算 mod-root 到 output-root之间的距离
- [ ] 编写 wasm fallback, 让 selien 在小众平台也可以进行工作
