# matsuri-adapter

三模块模板：

| 模块 | 文件 | 作用 |
|---|---|---|
| 采集 | `src/capture.rs` | 从本机 SO 插件（`127.0.0.1:18765`）抓事件帧 |
| 解密 | `src/decode.rs` | 帧 → 结构化事件 |
| 适配 | `src/adapter.rs` | 事件 → DSH / Agora 统一格式 |

## Token 约定（重要）

- 本仓库公开，**任何 personal token / PAT 都不许进仓库、进工作流**。
- CI（`.github/workflows/ci.yml`）只使用 GitHub 自动签发的临时 `GITHUB_TOKEN`，且无需个人凭据。
- 真需要密钥时放 **repo secrets**（Settings → Secrets and variables），在 yml 里用 `secrets.XXX` 引用，不要把值写进工作流文件。

## 本地运行

```bash
cargo run
cargo test
```

## 推送（沙箱里没 gh 时，在你手机上执行）

```bash
git init && git add -A && git commit -m "init: capture/decode/adapter scaffold"
git remote add origin git@github.com:<你的用户名>/matsuri-adapter.git
git push -u origin main
```
