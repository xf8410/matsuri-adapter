# 剧本 15 逻辑抽取 · 采集单（照做版）

目标：把「拉人头 / 额外加成 buff / 赛后 buff / 上下层加成 / 设备槽增长 / 训练成败判定」这六件事，
从**代码和表**里问出来，而不是靠人脑猜。

适用对象：hlpatch 3.28.4 插件（`127.0.0.1:18765`）。所有命令都是只读 GET，除了 S3 的
`invoke_static`（见红线）。

---

## S0 前置（一次性）

**A. 手机本地跑**（Termux 里）：

```bash
curl -s "http://127.0.0.1:18765/health" | head -c 300
```

**B. 电脑跑**（推荐，方便存档）：

```bash
adb forward tcp:18765 tcp:18765
curl -s "http://127.0.0.1:18765/health"
```

看到 `{"status":"ok","version":"3.28.4","endpoints":[...]}` 才算通。
`connection refused` = 游戏没开 / 插件没注入。

**建一个工作目录**，每步都落文件，别在浏览器里复制（浏览器有复制上限，这也是插件
为什么专门做了 `*_dl` 下载路由）：

```
s15/
  00_health.json
  10_classes_*.json
  20_methods_*.json
  30_oracle_*.json
  40_asm_*.json
  50_table_*.json
```

---

## S1 先确认类名活着（关键，别跳）

**现网类名 ≠ 旧快照类名。** 手上那份 2026-10-30 的方法快照里 Ramen（剧本14）一族有 226 个类，
但 `Festa` 一个都没有 —— 说明它是剧本 14 时期的客户端。所以第一步必须在**你这版**里重新找。

```bash
for k in Festa ScenarioFesta SummerOuting Outing FailureRate BuffEffect PotentialIncDec; do
  curl -s "http://127.0.0.1:18765/il2cpp/classes?keyword=$k" \
    -o "s15/10_classes_$k.json"
  printf "%-18s %s hits\n" "$k" "$(grep -o '"name"' "s15/10_classes_$k.json" | wc -l)"
done
```

- 路由：`/il2cpp/classes?keyword=<K>`（`lib.rs:9617` → `il2cpp_search_classes` `lib.rs:22632`）
- 匹配方式：把 **`命名空间.类名` 拼起来小写**做子串匹配（`lib.rs:22690`），扫全量 ~3 万个类，
  **可能要几秒**。所以 `keyword=Festa` 也可能命中的是某个命名空间，不是类。
- 每条命中长这样（`lib.rs:22703`）：`{"ns":"...","name":"...","fields":N,"singleton":true|false}`
  ⇒ **`singleton:true` 才能用 S3 的 `/il2cpp/call` 和 `/il2cpp/dump`**；false 的类只能读汇编或先拿对象指针。
- 报错形状：`{"error":"image_null"}`（游戏没进单局）、`{"error":"class_enum_api_not_found"}`

**判据**：`Festa` 一族命中 > 0 ⇒ 剧本 15 的类可查，继续。
命中 = 0 ⇒ 别硬猜，换关键词再扫一轮：`Act`、`Gauge`、`Advis`、`Rehears`、`Matsuri`、`Grand`、`Taikai`。
还是 0 ⇒ 现网可能用了日文罗马字，改从**内存链**反查：`/debug/festa_probe` 的
`festa.acts.list[].raw` 里带类名，`/debug/gauge2` 专门扫 DataSet 数组字段的元素类名（`lib.rs:9272`）。

---

## S2 六个机制 → 锚点 → 命令

每个机制都是同一套三步：**找类 → 列方法 → 要么当黑盒问它（oracle），要么读汇编**。

先列方法（`/il2cpp/methods?name=<类>`，`lib.rs:9625`，返回
`{"ok":true,"requested":...,"found":<真名>,"method_count":N,"methods":[...]}`）：

```bash
curl -s "http://127.0.0.1:18765/il2cpp/methods?name=SingleModeTrainingFailureRateService" \
  -o s15/20_methods_FailureRate.json
```

| # | 机制 | 首选锚点（旧快照实证存在） | 换关键词兜底 | 想问出的东西 |
| --- | --- | --- | --- | --- |
| 1 | 训练成败 | `SingleModeTrainingFailureRateService`、`IsExistTrainingWithInFailureRateCondition` | `Failure`、`FailRate` | 失败率公式的**输入项**和阈值表；有没有保底 |
| 2 | 拉人头（夏训/外出） | `SummerOutingButtonAction`、`DialogSingleModeSummerOutingConfirm` | `Outing`、`Summer`、`Guest` | 人数/等级增长是查表还是累加；`friend_outing` 从哪来 |
| 3 | 额外加成 buff | `SingleMode10BuffEffect`、`MasterSingleMode10BuffEffect` | `Buff`、`Effect`、`Debuff` | 同类 buff 是取 max 还是累加；作用在哪个乘区 |
| 4 | 赛后 buff | `PartsSingleModeScenarioLegendBuffEffectAmountListItem` | `Settlement`、`Reward`、`Result` | 结算后写回的是**数值**还是**effect id** |
| 5 | 上/下层加成 | `ArcPotentialIncDec`（同族命名法） | `IncDec`、`Factor`、`Gain`、`Upper`、`Lower` | 乘法顺序；哪些项并进同一个「合计」 |
| 6 | 设备槽增长 | — | `Slot`、`Equip`、`Max.*Count`、`Unlock` | 槽位数由什么驱动：等级 / checkpoint / 表 |

内存侧已有的现成把手（不用再找）：
`friend_outing` = `FriendOutingGainActLevel`（元素偏移 `0x88`，`lib.rs:4179`）、
`gauge`/`need_gauge`/`level`/`state`/`advice_id`（偏移 `0x24/0x4C/0x38/0x60/0x74`）、
`festa_info` 三字段 `summer_camp_remain_turn`/`max_act_level`/`act_training_step`（`lib.rs:4123`）、
`checkpoints.list[]` 的 `schedule_id`/`result_type`（`lib.rs:4248`）。

「设备槽」这个词我不确定你指哪个，两种读法都给：**(a)** 演目/装备位随 `max_act_level` 解锁；
**(b)** 支援卡槽位。先各扫一次 `Slot` 和 `Equip`，看哪边有 `WorkSingleMode*` 族的类再定。

---

## S3 oracle：让游戏自己算（首选，省掉读汇编）

插件能直接调用方法，这是最省人的一步 —— 不用读 ARM64，问它就行。

**零参 getter**（最安全）：

```bash
curl -s "http://127.0.0.1:18765/il2cpp/call?class=<类短名>&method=<getter名>"
```

（`lib.rs:9601` → `il2cpp_call_method`，在单例上调，返回运行时值）

**带参静态方法**：

```bash
curl -s "http://127.0.0.1:18765/il2cpp/invoke_static?class=<类>&method=<方法>&n=2&p0=47&p1=3"
```

（`lib.rs:9579` → `il2cpp_invoke_static_method` `lib.rs:17786`）

硬约束（源码实证，不是建议）：

- **参数只接受 `System.Int32`**，最多 5 个（`p0..p4`）。任何别的类型直接拒：
  `{"ok":false,"error":"unsupported_parameter_type","index":0,"type_name":"System.Single","supported":["System.Int32"]}`（`lib.rs:17840`）
  ⇒ float 参数、对象参数、枚举参数的方法**都调不了**。这类只能走 S4 读汇编。
- `param_count` 越界 → `{"error":"param_count_out_of_range"}`；找不到方法 →
  `{"error":"method_not_found","class":...,"method":...,"params":N}`（`lib.rs:17820`）

### ⚠ 红线：这里没有白名单

我读了实现：`runtime_state_gate_required` 这个「闸门」**只对写死的一个方法生效**
（`SingleModeRamenAPI.SendUrafEffectApply`，`lib.rs:17844`）。除此之外**没有任何拦截** ——
只要参数都是 Int32，方法就真的执行。

所以：

- **只调名字像纯计算/查询的**：`Get*`、`Calc*`、`Check*`、`Is*`、`*Rate`、`*Amount`
- **绝不调**：`Send*`、`Set*`、`Apply*`、`Add*`、`Remove*`、`Update*`、`Save*`、`Start*`、`End*`、`Exec*`
- 拿不准就不调，改用 S4。真机上一旦调了写状态的方法，可能直接改局面、发请求、甚至被服务端判异常。

### 用法：单变量扫描

失败率这种「输入 → 概率」的关系，用 oracle 扫比差分实验快两个数量级：

```bash
for v in 1 2 3 4 5; do
  curl -s "http://127.0.0.1:18765/il2cpp/invoke_static?class=<Svc>&method=<GetRate>&n=1&p0=$v" \
    -o "s15/30_oracle_rate_lv$v.json"
done
```

一次扫一个变量，其余固定。步长要**跨过可疑阈值**（比如体力 1..max_vital 全扫），
不要只测两个点 —— 分段函数两点看不出来。

---

## S4 读汇编（兜底，只在 oracle 调不动时用）

按类名+方法名直接反汇编，不需要先拿地址：

```bash
curl -s "http://127.0.0.1:18765/il2cpp/disassemble_dl?class=<类>&method=<方法>&bytes=4096" \
  -o s15/40_asm_<方法>.json
```

- 路由：`lib.rs:9663`（普通）/ `lib.rs:9629`（`_dl` 下载版，**手机上用这个**）
- **陷阱**：`bytes` 上限 4096，而且 `bytes>4096` 或 `bytes=0` 时**静默回落到 2048**（`lib.rs:24041-24046`）。
  想要 4096 就老实写 4096，别写 8192 还以为拿到了长的。
- 方法体超过 4096 字节时，先用 `/il2cpp/methods` 确认方法存在，再按地址分段取：
  `/il2cpp/disassemble_addr?addr=<十六进制>&bytes=<N>`（`lib.rs:9657`，`_dl` 版 `lib.rs:9636`），
  地址从 `/il2cpp/dump_all_methods?letter=<L>` 里拿（`lib.rs:9559`，**带 `addr` 字段**；
  下载版 `lib.rs:9540`；按字母分组是防手机卡死）
- 类名用**短名**（不含命名空间）：查找走 `find_class_by_short_name`（`lib.rs:24057`）
- 重载方法只取**第一个同名匹配** ⇒ 同名多签名时结果可能不是你要的那个，先数一下
  `methods` 里同名出现几次

读的时候只找三样东西，其余不管：
`bl`（调了谁 → 子函数）、`fmul`/`fadd`/`fdiv`（乘区顺序）、`mov w?, #imm` 里的**魔数常量**
（阈值、倍率、千分比）。常量拿到后回 S5 去表里对号。

---

## S5 表侧：数字几乎都在 master.mdb 里

代码告诉你**怎么算**，表告诉你**算成多少**。两边都要。

```bash
# 找表
curl -s "http://127.0.0.1:18765/mdb/search?keyword=<关键词>" -o s15/50_mdb_search.json
# 拿表结构（列名 + 类型）
curl -s "http://127.0.0.1:18765/mdb/schema?name=<表名>" -o s15/50_schema_<表名>.json
# 一条 curl 拿全表 JSON（batch 默认 500，可加 &batch=&offset=）
curl -s "http://127.0.0.1:18765/debug/download_table?name=<表名>" -o s15/50_table_<表名>.json
```

已验证存在的相关表：`single_mode_training_effect`（剧本相关行，剧本 15 的 56 行**还缺**）、
`single_mode_15_act_info`（`act_type → command_id` 映射，插件自己就用它，`lib.rs:2164`）。

`/debug/download_table` 的返回行数有预期值，拿到先数一遍：5 / 5 / 66 / 10 / 153 / 6 / 20 / 4 / 9 / 10 / 3。
对不上就是分页或表名错了，别继续往下推。

---

## S6 最后一步：确认这版真的开着

代码里有 ≠ 这版生效。三件事各花不到两分钟：

1. **进一局剧本 15**，在机制真的触发的那一刻前后各抓一次 `/debug/festa_probe`，
   diff 两帧 —— 只有**真的在变的字段**才是这版在用的逻辑。
2. 查 `/summary` 的 `scenario` 和 `/debug/festa_probe` 的 `scenario_id_hint` / `getter_key_used`
   是否都是 15（不一致说明 getter 回退过，读到的可能是别的剧本对象）。
3. 你那个 P0（Senior 第三档排练值）就属于这类：**分档条件代码里直接写着**，
   但「这一档现在开没开」只能打一局看。

---

## 交付判据（每个机制算「问完了」的标准）

一个机制算挖到底，要同时满足：

- [ ] 能说出**输入项**（哪些字段/表列参与）
- [ ] 能说出**组合方式**（加进同一个合计 / 各自成乘区 / 取 max）
- [ ] 能说出**常量来源**（写死在代码里的数，还是表里的列）
- [ ] 至少一个 oracle 采样点或一帧真机数据能对上公式
- [ ] 说得出**没验证的部分**（明确标 unverified，不要混进结论）

四条都对上、第五条写清楚了，才进模拟器。

---

## 回收

跑完把 `s15/` 打包。用 Workbench 的**文件投递**（手机文件 → zip 分卷 → 私有仓
`xf8410/bestsoccer-gala`）传上来最省事，我从那边取回直接分析。
或者只贴 `10_classes_*.json` 的命中数 + 你打算先挖哪个机制，我来排下一步顺序。
