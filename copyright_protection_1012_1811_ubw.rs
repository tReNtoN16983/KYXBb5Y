{
  "hash": "example_hash",
  "owner": "example_owner",
  "validity_period": "2023-2033"
}
```
"]
    #[doc = "### 状态码
- `202 Accepted`: 注册成功
"]
    #[doc = "/register"]
    // 版权检查接口文档
    #[doc = "检查版权信息"]
    #[doc = "
这个接口允许用户检查特定内容的版权状态。传入的内容哈希值将用于检索版权信息。
"]
    #[doc = "### 输入
- `hash`: 文本内容的哈希值
"]
    #[doc = "### 输出
- 包含版权信息的列表
"]
    #[doc = "### 示例
```json
{
  "hash": "example_hash"
}