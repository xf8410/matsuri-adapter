# matsuri-adapter

把手机上那个 SO 插件（`127.0.0.1:18765`）吐出来的深 JSON，压成一行一个事实的文本帧，
给 DSH / Workbench / 脚本 / 模型消费。三个模块各管一段，互不越界：

| 模块 | 文件 | 只负责 | 明确不负责 |
| --- | --- | --- | --- |
| 采集 | `src/capture.rs` | 发 GET、拿文本、报「为什么没拿到」 | 看懂任何字段 |
| 解码 | `src/decode.rs` | JSON → `Frame`（键名全部抄自插件源码） | 换算回合语义、猜缺值 |
| 适配 | `src/adapter.rs` | `Frame` → 固定顺序的 `key=value` 行 | 加派生指标、加判断 |

## 一帧 = 三条只读 GET

| 名字 | 路由 | 提供 |
| --- | --- | --- |
| `festa` | `/debug/festa_probe` | 五演目 gauge/level/state、advice、checkpoints、`summer_camp_remain_turn`、`max_act_level`、`act_training_step` |
| `summary` | `/summary` | `turn`/`raw_total_turn_num`/`month`/`half`、五维 `stats`、`scenario` |
| `action` | `/action/latest` | 最近一次下发命令（`raw_command_id`、`normalized_command_id`、`result_type`） |

刻意不取 `/debug/turn_probe`：它内部就是 `read_summary()`（`lib.rs:8327`），字段和
`/summary` 重复，多调一次等于多扫一遍游戏内存。

`/summary` 里其实已经带了 `last_action`（`lib.rs:7254`），但它没有 `result_type`；
`/action/latest` 便宜（只读一个 mutex，不碰 IL2CPP），所以还是单独取。

`/api/matsuri/state` 这类聚合端点**不存在**（插件源码里没有 `matsuri` 字样），所以聚合
放在本仓库这一侧做。

## 跑

设备上或 `adb forward` 之后本机跑都行：

```bash
adb forward tcp:18765 tcp:18765     # 笔记本访问手机插件
cargo run                           # 出一帧
cargo test                          # 12 个测试，全部离线，不碰真机
```

退出码：`0` = 三条路由全通；`1` = 有缺，第一行会写 `frame complete=false missing=[...]`。

## 输出长这样

```
frame complete=true missing=[] notes=[]
turn value=47 raw_total=47 month=3 half=2 semantics=countdown
scenario name=festa id_hint=15 turn_hint=47
camp remain_turn=2 max_act_level=5 act_step=3
act cid=5101 type=1 name=短距離 name_src=mdb cid_src=single_mode_15_act_info gauge=8/12 level=2 state=1 advice=3 outing=1
advice idx=0 id=7 fusion=0
checkpoint idx=0 schedule_id=23 result_type=2
stats speed=412 stamina=388 power=501 guts=300 wiz=255 vital=9/64 skill_point=12 fan=92378 motivation=GREAT
action seq=12 raw=5103 normalized=5103 name=Unknown result_type=0 source=training_hook
```

两条硬约定：

1. **缺值一律显式 `-1`**（和插件自己 `unwrap_or(-1)` 同口径），不静默变 0。
   「没有」和「是 0」必须长得不一样。
2. **不换算回合**。`semantics` 原样带出插件声明（`countdown` /
   `verified_ramen_upstream_semantics`），字段映射未验证的部分留给上游，不在这里编。

`action name=Unknown` 是正常现象：`/action/latest` 的映射表只认 101–106 / 601–605
（`lib.rs:9177-9188`），祭典的 5101–5105 不在表里，会落到 `Unknown` + 原值。

## 键名来源

不猜字段。每个键都能对到插件源码行号：

- `/debug/festa_probe` 顶层 `lib.rs:20507`，失败形状 `lib.rs:20474`
- `festa_info` `lib.rs:4123`；`acts.list[]` `lib.rs:4179` + 外层 `4185`
- `advice.list[]` `lib.rs:4209` + `4215`；`checkpoints.list[]` `lib.rs:4248`
- `/summary` `lib.rs:7264`；`/action/latest` `lib.rs:9191`

## Token 政策

- 仓库里、工作流里**都不许出现 personal access token**。
- `.github/workflows/ci.yml` 只用 GitHub 每次运行自动签发的临时 `GITHUB_TOKEN`，
  且 `permissions: contents: read`（只读，什么都改不了）。
- 真要密钥：建到仓库的 Secrets 里，工作流用 `secrets.XXX` 引用，永远不要写进文件。
- 本工具全程只走 `127.0.0.1` loopback，不带任何鉴权头。
