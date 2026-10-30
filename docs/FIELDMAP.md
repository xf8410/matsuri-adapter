# FIELDMAP — hachimi_ura_plugin 3.28.4 (hlpatch)

> Source of truth: `/tmp/build3284/repo/hachimi_ura_plugin/src/lib.rs` (26 149 lines)
> Bind: `127.0.0.1:18765` (`lib.rs:7491`)
> Dispatch: `handle_http()` from `lib.rs:8114`; `if path == "..."` chain from `lib.rs:8250`.
> All key names are copied verbatim from `json!` / `format!(r#"..."#)` literals.

---

## 1. `/health` (alias `/`)

- **Dispatch:** `lib.rs:8256` — `path == "/" || path == "/health"`
- **Method:** GET, no params
- **Boot-safe:** YES (`lib.rs:8139`)
- **Shape:** flat JSON object
- **Keys:**
  - `status` — string, always `"ok"` (`lib.rs:8258`)
  - `version` — string, `PLUGIN_VERSION` = `env!("CARGO_PKG_VERSION")` (`lib.rs:22`, formatted at `lib.rs:8258`)
  - `endpoints` — array of strings, hard-coded list of all known route paths (`lib.rs:8258`)
- **Error sentinels:** none; always returns 200 with `{"status":"ok",...}`
- **ObscuredInt:** NO — static data only

---

## 2. `/summary`

- **Dispatch:** `lib.rs:8323` → calls `read_summary()` (`lib.rs:5126`)
- **Method:** GET, no params
- **Boot-safe:** NO (touches game memory)
- **DL_ALLOWED:** YES (`lib.rs:8206`)
- **Shape:** deeply nested JSON object
- **Handler:** `read_summary()` → `read_summary_inner()` → `read_summary_inner_impl()` (`lib.rs:5184`→`5216`)
- **Top-level keys** (from the format string at `lib.rs:7263-7317`):

| Key path | Type | Source line | Notes |
|---|---|---|---|
| `version` | string | `lib.rs:7265` | PLUGIN_VERSION |
| `year` | int | `lib.rs:7266` | Computed from turn |
| `turn` | int | `lib.rs:7267` | `cumulative_turn` |
| `raw_total_turn_num` | int | `lib.rs:7268` | ObscuredInt at sm_obj+0x44 (`lib.rs:5304`) |
| `ui_turn_semantics` | string | `lib.rs:7269` | Always `"countdown"` |
| `raw_field_mapping` | string | `lib.rs:7269` | `"verified_ramen_upstream_semantics"` |
| `month` | int | `lib.rs:7269` | Via `get_Month` getter (`lib.rs:5292`) |
| `half` | int | `lib.rs:7269` | Via `get_Half` getter (`lib.rs:5297`) |
| `scenario` | string | `lib.rs:7271` | e.g. `"Festa"`, `"Ramen"` (`lib.rs:5396-5413`) |
| `chara_id` | int | `lib.rs:7272` | ObscuredInt at chara_obj+0x24 (`lib.rs:5331`) |
| `stats.speed` | int | `lib.rs:7273` | ObscuredInt at chara_obj+0xF8 (`lib.rs:5279`) |
| `stats.stamina` | int | `lib.rs:7274` | ObscuredInt at chara_obj+0x10C (`lib.rs:5280`) |
| `stats.power` | int | `lib.rs:7275` | ObscuredInt at chara_obj+0x120 (`lib.rs:5281`) |
| `stats.guts` | int | `lib.rs:7276` | ObscuredInt at chara_obj+0x134 (`lib.rs:5282`) |
| `stats.wiz` | int | `lib.rs:7277` | ObscuredInt at chara_obj+0x148 (`lib.rs:5283`) |
| `stats.vital` | int | `lib.rs:7278` | ObscuredInt at chara_obj+0xD0 (`lib.rs:5285`) |
| `stats.max_vital` | int | `lib.rs:7279` | ObscuredInt at chara_obj+0xE4 (`lib.rs:5286`) |
| `stats.motivation` | string | `lib.rs:7280` | Decoded from ObscuredInt (`lib.rs:5388-5395`) |
| `stats.skill_point` | int | `lib.rs:7281` | ObscuredInt at chara_obj+0x2C0 (`lib.rs:5288`) |
| `stats.fan` | int | `lib.rs:7282` | ObscuredInt at chara_obj+0x3E4 (`lib.rs:5289`) |
| `max_stats.speed` | int | `lib.rs:7283` | ObscuredInt (`lib.rs:5334`) |
| `max_stats.stamina` | int | `lib.rs:7284` | ObscuredInt (`lib.rs:5335`) |
| `max_stats.power` | int | `lib.rs:7285` | ObscuredInt (`lib.rs:5336`) |
| `max_stats.guts` | int | `lib.rs:7286` | ObscuredInt (`lib.rs:5337`) |
| `max_stats.wiz` | int | `lib.rs:7287` | ObscuredInt (`lib.rs:5338`) |
| `proper.dist_short` | int | `lib.rs:7288` | ObscuredInt (`lib.rs:5350`) |
| `proper.dist_mile` | int | `lib.rs:7289` | ObscuredInt (`lib.rs:5351`) |
| `proper.dist_mid` | int | `lib.rs:7290` | ObscuredInt (`lib.rs:5352`) |
| `proper.dist_long` | int | `lib.rs:7291` | ObscuredInt (`lib.rs:5353`) |
| `proper.ground_turf` | int | `lib.rs:7292` | ObscuredInt (`lib.rs:5354`) |
| `proper.ground_dirt` | int | `lib.rs:7293` | ObscuredInt (`lib.rs:5355`) |
| `running_style` | int | `lib.rs:7294` | ObscuredInt (`lib.rs:5340`) |
| `scenario_progress` | int | `lib.rs:7295` | ObscuredInt (`lib.rs:5339`) |
| `training_event_type` | int | `lib.rs:7296` | ObscuredInt (`lib.rs:5341`) |
| `talent_level` | int | `lib.rs:7297` | ObscuredInt (`lib.rs:5344`) |
| `chara_grade` | int | `lib.rs:7298` | ObscuredInt (`lib.rs:5346`) |
| `difficulty` | int | `lib.rs:7299` | ObscuredInt (`lib.rs:5347`) |
| `fixed_turn_chara_seed` | int | `lib.rs:7300` | ObscuredInt at sm_obj+0x198 (`lib.rs:5360`) |
| `trainings` | array | `lib.rs:7301` | Per-command training data, see below |
| `support_cards` | array | `lib.rs:7302` | Equipped support cards |
| `evaluation` | array | `lib.rs:7303` | Partner bond evaluations |
| `training_levels` | array | `lib.rs:7304` | Per-command training levels |
| `buffs` | array | `lib.rs:7305` | Active buff effects |
| `chara_effect_ids` | array[int] | `lib.rs:7306` | Raw effect IDs |
| `skills.eval` | int | `lib.rs:7307` | Skill evaluation score |
| `skills.count` | int | `lib.rs:7308` | Learned skill count |
| `skills.list` | array | `lib.rs:7309` | Skill list |
| `skills.src` | string | `lib.rs:7310` | Source label |
| `ai` | object | `lib.rs:7311` | AI evaluation result |
| `ramen` | object\|absent | `lib.rs:7313` | Only present when `scenario_id==14` |
| `last_action` | object\|absent | `lib.rs:7314` | Only present when ACTION_STATE has data |
| `festa` | object\|absent | `lib.rs:7315-7316` | Only present when `scenario_id==15` |
| `scenario_mdb_name` | string\|absent | `lib.rs:7316` | From MDB text_data |
| `scenario_name_src` | string\|absent | `lib.rs:7316` | Source label |
| `team_data` | object\|absent | `lib.rs:7312` | Only present when `scenario_id==13` |

- **`trainings[0]`** keys (`lib.rs:6706-6707`):
  `name`, `name_src`, `command_id`, `cmd_type`, `cmd_class`, `is_enable`, `failure_rate`,
  `offs` (array of 3 ints), `heads`, `shining`, `partner_ids` (array),
  `partners` (array of objects), `gains` (object)

- **`trainings[].partners[0]`** keys (`lib.rs:6570-6571`):
  `partner_id`, `support_position`, `support_card_id`, `current_bond`, `is_shining`,
  `is_unique_active`, `bond_threshold`, `support_card_type`, `is_tips_event`,
  `partner_type`, `name`, `bond_gain`

- **`support_cards[0]`** keys (`lib.rs:6810-6812`):
  `position`, `support_card_id`, `limit_break_count`, `training_partner_state`,
  `chara_id`, `kizuna`, `exp`, `rental_type`

- **`evaluation[0]`** keys (`lib.rs:6847-6849`):
  `target_id`, `partner_id`, `evaluation`, `current_bond`

- **`festa`** object (only sid==15): produced by `read_festa_dataset()` (`lib.rs:4099`→`4383`),
  embedded as raw JSON string. See `/debug/festa_probe` for the identical structure..
  Top-level keys inside the `festa` object:
  `festa_info`, `acts`, `advice`, `checkpoints`, `gain_gauges`, `command_info`,
  `evaluation_info`, `ds_class`, `note`, `diag`

- **Error sentinels:**
  - `{"error":"sigsegv_cooldown","retry_after":N}` (`lib.rs:5134-5137`)
  - `{"error":"sigsegv_recovered","hint":"..."}` (`lib.rs:5158-5159`)
  - `{"error":"panic_caught","hint":"..."}` (`lib.rs:5171`)
  - `{"error":"api_null"}` (`lib.rs:5218`)
  - `{"error":"image_null"}` (`lib.rs:5222`)
  - `{"error":"no_wdm"}` (`lib.rs:5234`)
  - `{"error":"no_wdm_inst"}` (`lib.rs:5238`)
  - `{"error":"no_sm_class"}` (`lib.rs:5250`)
  - `{"error":"no_sm"}` (`lib.rs:5257`)
  - `{"error":"no_chara_class"}` (`lib.rs:5268`)
  - `{"error":"no_chara"}` (`lib.rs:5275`)

- **ObscuredInt:** YES — extensively. All stat values, bond values, command IDs, etc. are decoded via `read_obscured_int_at()` (XOR key^hidden, 20-byte stride 0x14 layout). The returned numbers are already decoded plaintext.

---

## 3. `/debug/turn_probe`

- **Dispatch:** `lib.rs:8325`
- **Method:** GET, no params
- **Boot-safe:** NO (calls `read_summary()`)
- **DL_ALLOWED:** NO (not in list at `lib.rs:8205-8240`)
- **Shape:** flat JSON object
- **Handler:** inline at `lib.rs:8325-8333`; calls `read_summary()` then `extract_json_int()`
- **Keys** (from `lib.rs:8328-8333`):

| Key | Type | Source | Notes |
|---|---|---|---|
| `status` | string | literal | Always `"ok"` |
| `raw_total_turn_num` | int | `extract_json_int(&s, "\"raw_total_turn_num\"")` (`lib.rs:8330`) | From /summary; -1 if missing |
| `ui_turn_semantics` | string | literal | Always `"countdown"` |
| `raw_field_mapping` | string | literal | Always `"unverified"` |
| `year` | null | literal | Always `null` |
| `month` | int | `extract_json_int(&s, "\"month\"")` (`lib.rs:8331`) | -1 if missing |
| `half` | int | `extract_json_int(&s, "\"half\"")` (`lib.rs:8332`) | -1 if missing |
| `derived_turn` | null | literal | Always `null` |

- **Error sentinels:** inherits `/summary` errors (sigsegv_cooldown, etc.)
- **ObscuredInt:** YES — re-exports decoded value from `/summary`

---

## 4. `/debug/gauge`

- **Dispatch:** `lib.rs:9258`
- **Method:** GET, no params
- **Boot-safe:** NO
- **DL_ALLOWED:** YES (`lib.rs:8229`)
- **Shape:** flat JSON object
- **Handler:** `debug_gauge()` (`lib.rs:21786`); sigsetjmp-protected (`lib.rs:9258-9271`)
- **Gate:** requires `scenario_id == 14` (Ramen); returns error otherwise (`lib.rs:21826-21828`)
- **Keys** (from `lib.rs:21889-21893`):

| Key | Type | Source | Notes |
|---|---|---|---|
| `version` | string | literal | `"3.22.91"` |
| `count` | int | `llen` | CommandFeelingInfoArray length |
| `elements` | array | `elems` | Per-element diagnostic |

- **`elements[0]`** keys (`lib.rs:21883-21886`):
  `idx` (int), `class` (string — element class name), `dict_info` (string)

- **Error sentinels:**
  - `{"error":"sigsegv_recovered","hint":"/debug/gauge hit native crash, game protected"}` (`lib.rs:9263`)
  - `{"error":"gauge_panic"}` (`lib.rs:9268`)
  - `{"error":"api_null"}` (`lib.rs:21788`)
  - `{"error":"image_null"}` (`lib.rs:21792`)
  - `{"error":"no_wdm"}` (`lib.rs:21800`)
  - `{"error":"no_wdm_inst"}` (`lib.rs:21804`)
  - `{"error":"no_sm"}` (`lib.rs:21813`)
  - `{"error":"no_chara"}` (`lib.rs:21822`)
  - `{"error":"not_ramen","sid":N}` (`lib.rs:21827`)
  - `{"error":"no_ramen_sc_obj"}` (`lib.rs:21832`)
  - `{"error":"no_ds"}` (`lib.rs:21837`)
  - `{"error":"no_CommandFeelingInfoArray_field"}` (`lib.rs:21844`)
  - `{"error":"list_null"}` (`lib.rs:21848`)
  - `{"error":"bad_len","len":N}` (`lib.rs:21853`)

- **ObscuredInt:** NO — reads class names and dict pointers only; no ObscuredInt decoding in this endpoint

---

## 5. `/debug/gauge2`

- **Dispatch:** `lib.rs:9272`
- **Method:** GET, no params
- **Boot-safe:** NO
- **DL_ALLOWED:** YES (`lib.rs:8230`)
- **Shape:** flat JSON object
- **Handler:** `debug_gauge2()` (`lib.rs:21898`); sigsetjmp-protected (`lib.rs:9272-9286`)
- **Gate:** requires `scenario_id == 14` (Ramen); returns error otherwise (`lib.rs:21937-21938`)
- **Keys** (from `lib.rs:22003-22006`):

| Key | Type | Source | Notes |
|---|---|---|---|
| `version` | string | literal | `"3.22.91"` |
| `arrays` | array | `results` | Per-field diagnostic |

- **`arrays[0]`** keys (`lib.rs:21997-21999`):
  `field` (string — field name), `count` (int — array length), `sample_classes` (array of strings — up to 3 element class names)

- **Scanned fields** (`lib.rs:21951-21958`):
  `CommandInfoArray`, `FeelingReduceTurnInfoArray`, `FeelingTurnInfoArray`,
  `FeelingInfoArray`, `CommandFeelingInfoArray`, `TrainingExecInfoArray`, `CheckPointInfoArray`

- **Error sentinels:**
  - `{"error":"sigsegv_recovered","hint":"/debug/gauge2 hit native crash, game protected"}` (`lib.rs:9277`)
  - Same chain as `/debug/gauge` for api/image/wdm/sm/chara/ds errors (`lib.rs:21899-21948`)
  - `{"error":"not_ramen","sid":N}` (`lib.rs:21938`)

- **ObscuredInt:** NO — class name discovery only

---

## 6. `/debug/training_partners`

- **Dispatch:** `lib.rs:8389` → calls `debug_training_partners()` (`lib.rs:20284`)
- **Method:** GET, no params
- **Boot-safe:** NO
- **DL_ALLOWED:** YES (`lib.rs:8224`)
- **Shape:** nested JSON object
- **Handler:** `debug_training_partners()` → `debug_training_partners_inner()` (`lib.rs:20175`); sigsetjmp-protected
- **Keys** (from `lib.rs:20274-20280`):

| Key | Type | Source | Notes |
|---|---|---|---|
| `ok` | bool | literal | `true` |
| `diagnostic_version` | int | literal | `1` |
| `read_only` | bool | literal | `true` |
| `notes` | array[string] | literal | Diagnostic notes |
| `commands_collection` | object | `commands_diag` | Collection diagnostic |
| `commands` | array | `cmd_parts` | Per-command details |
| `support_cards` | object | `sc_diag` | Support card collection diagnostic |
| `evaluation_list` | object | `eval_diag` | Evaluation list diagnostic |

- **`commands[0]`** keys (`lib.rs:20250-20255`):
  `index` (int), `command_id` (int — ObscuredInt decoded), `command_ptr` (string — hex address),
  `command_class` (string), `command_raw_hex` (string — hex dump of 0x70 bytes),
  `training_partners` (object — collection diagnostic), `tips_event_partners` (object — collection diagnostic)

- **Error sentinels:**
  - `{"error":"sigsegv_recovered","hint":"training partner diagnostic hit an invalid runtime pointer; game was protected"}` (`lib.rs:20289`)
  - `{"error":"panic_caught"}` (`lib.rs:20295`)
  - `{"error":"api_null"}` (`lib.rs:20177`)
  - `{"error":"image_null"}` (`lib.rs:20181`)
  - `{"error":"wdm_class_null"}` (`lib.rs:20186`)
  - `{"error":"wdm_null"}` (`lib.rs:20194`)
  - `{"error":"sm_null"}` (`lib.rs:20199`)

- **ObscuredInt:** YES — `command_id` at offset 36 is decoded via `read_obscured_int_at()` (`lib.rs:20230`)

---

## 7. `/scenario`

- **Dispatch:** `lib.rs:8349` → calls `read_scenario_detail()` (`lib.rs:1718`)
- **Method:** GET, no params
- **Boot-safe:** NO
- **DL_ALLOWED:** YES (`lib.rs:8207`)
- **Shape:** deeply nested JSON object; structure varies by `scenario_id`
- **Handler:** `read_scenario_detail()` builds `result_parts` vec and joins at `lib.rs:2351`
- **Common top-level keys** (from `lib.rs:1795-1827`):

| Key | Type | Source | Notes |
|---|---|---|---|
| `scenario_id` | int | `lib.rs:1796` | From `get_ScenarioId` |
| `scenario_class` | string | `lib.rs:1797` | e.g. `"WorkSingleModeScenarioFesta"` |
| `scenario_obj` | string | `lib.rs:1798` | Hex pointer |
| `dataset_obj` | string | `lib.rs:1805` | Hex pointer (if non-null) |
| `dataset_class` | string | `lib.rs:1827` | e.g. `"WorkSingleModeScenarioFestaDataSet"` |
| `dataset_values` | object | `lib.rs:1848` | ObscuredInt getter results |
| `dataset_arrays` | object | `lib.rs:1875` | Array getter lengths + pointers |
| `dataset_objects` | object | `lib.rs:2343` | Object-type getter pointers |

- **`dataset_values`** keys (from `lib.rs:1830-1848`):
  `get_TeamRank`, `get_HavingEnhancePoint`, `get_PredictEnhancePoint`,
  `get_BcRaceTrackId`, `get_DeckId`, `get_TeamSpLevelLimit`, `get_TeamUnionProgress`

- **`dataset_arrays`** keys (from `lib.rs:1852-1875`):
  `get_EnhanceGroupArray`, `get_CommandInfoArray`, `get_TeamMemberInfoArray`,
  `get_TeamReviewResultArray`, `get_BcRaceResultArray`, `get_CommandGainExpArray`
  Each value: `{"len":N,"ptr":"0x..."}`

- **Scenario 13 (Breeders) extra keys:**
  - `enhance_groups` — array (`lib.rs:1898`)
  - `command_info` — array (`lib.rs:2060`)
  Each command_info element: `CommandType`, `CommandId`, `RankUpPredict`, `CommandName`,
  `params_inc_dec` (array of `{TargetType, TargetTypeName, Value}`), `team_member_len`

- **Scenario 14 (Ramen) extra keys** (`lib.rs:2065-2326`):
  - `ramen_values` — object: `get_CheckPointPt`, `get_ExpectedCheckPointPt`, `get_SpecialFeelingNum`, `get_RecommendType`
  - `ramen_bools` — object: `get_IsGaugeGained`, `get_IsUrafEffectSelectEventChecked`, `get_IsNotGainSpecialFeeling`
  - `active_effects` — array: `{EffectCategory, EffectId, EffectValue}`
  - `uraf_effect` — object: `{UrafEffectType, UrafEffectState}`
  - `selected_region_ids` — array[int]
  - `all_selected_region_ids` — array[int]
  - `feeling_info` — array: `{FeelingIndex, FeelingId}`
  - `feeling_turn_info` — array: `{RemainTurn, FeelingId}`
  - `command_feeling_info` — array: `{CommandType, CommandId, FeelingId}`
  - `feeling_reduce_turn_info` — array: `{CommandType, CommandId}`

- **Error sentinels:**
  - `{"error":"api_null"}` (`lib.rs:1720`)
  - `{"error":"image_null"}` (`lib.rs:1725`)
  - `{"error":"no_wdm_class"}` (`lib.rs:1734`)
  - `{"error":"no_wdm_singleton"}` (`lib.rs:1739`)
  - `{"error":"no_single_mode"}` (`lib.rs:1749`)
  - `{"error":"no_chara"}` (`lib.rs:1759`)
  - `{"scenario_id":N,"error":"scenario_obj_null","hint":"getter_name_not_found"}` (`lib.rs:1767`)

- **ObscuredInt:** YES — all `dataset_values` use `call_getter_obscured_int()` (`lib.rs:1842`);
  Ramen arrays use `read_obscured_int_from_obj()` (`lib.rs:2126-2128`);
  Breeders params use plain Int32 (`lib.rs:2008-2016`)

---

## 8. `/debug/festa_probe`

- **Dispatch:** `lib.rs:8387` → calls `debug_festa_probe()` (`lib.rs:20415`)
- **Method:** GET, no params
- **Boot-safe:** NO
- **DL_ALLOWED:** YES (`lib.rs:8238`)
- **Shape:** nested JSON object
- **Handler:** `debug_festa_probe()` (`lib.rs:20415-20510`); calls `read_festa_dataset()` (`lib.rs:4099-4383`)
- **Top-level keys** (from `lib.rs:20506-20508`):

| Key | Type | Source | Notes |
|---|---|---|---|
| `ok` | bool | literal | `true` on success |
| `scenario_id_hint` | int | `sid` | From chara_obj+0x18 (`lib.rs:20460`) |
| `getter_key_used` | int | `getter_key` | Actual sid used for getter |
| `turn_hint` | int | `turn` | From chara_obj+0x24 (`lib.rs:20504`) |
| `festa` | object | `festa_data` | Full festa dataset probe |

- **`festa`** object keys (from `read_festa_dataset()` at `lib.rs:4099-4383`):

| Key path | Type | Source | Notes |
|---|---|---|---|
| `festa.festa_info` | object\|null | `lib.rs:4110-4128` | Summer camp state |
| `festa.festa_info.summer_camp_remain_turn` | int | `lib.rs:4114` | ObscuredInt at FestaInfo+0x10 |
| `festa.festa_info.max_act_level` | int | `lib.rs:4116` | ObscuredInt at FestaInfo+0x24 |
| `festa.festa_info.act_training_step` | int | `lib.rs:4118` | ObscuredInt at FestaInfo+0x38 |
| `festa.festa_info.elem_class` | string | `lib.rs:4127` | Runtime class name |
| `festa.acts` | object | `lib.rs:4184-4188` | Act info array |
| `festa.acts.len` | int | `lib.rs:4185` | Array length |
| `festa.acts.list` | array | `lib.rs:4186` | Per-act details |
| `festa.acts.list[0].act_type` | int | `lib.rs:4150` | ObscuredInt at ActInfo+0x10; range 1..=5 |
| `festa.acts.list[0].command_id` | int | `lib.rs:4164-4166` | Mapped from act_type via `single_mode_15_act_info` |
| `festa.acts.list[0].cid_src` | string | `lib.rs:4165` | `"single_mode_15_act_info"` or `"formula_5100_plus_act_type"` |
| `festa.acts.list[0].name` | string | `lib.rs:4168` | From MDB text_data category 55 |
| `festa.acts.list[0].name_src` | string | `lib.rs:4168` | `"mdb"` or `"mdb_miss"` |
| `festa.acts.list[0].gauge` | int | `lib.rs:4151` | ObscuredInt at ActInfo+0x24 |
| `festa.acts.list[0].need_gauge` | int | `lib.rs:4153` | ObscuredInt at ActInfo+0x4C |
| `festa.acts.list[0].level` | int | `lib.rs:4152` | ObscuredInt at ActInfo+0x38 |
| `festa.acts.list[0].state` | int | `lib.rs:4154` | ObscuredInt at ActInfo+0x60 |
| `festa.acts.list[0].advice_id` | int | `lib.rs:4155` | ObscuredInt at ActInfo+0x74 |
| `festa.acts.list[0].friend_outing` | int | `lib.rs:4156` | ObscuredInt at ActInfo+0x88 |
| `festa.acts.list[0].raw` | array | `lib.rs:4177` | Hex dump of first 0xA0 bytes |
| `festa.advice` | object | `lib.rs:4214-4218` | Advice info array |
| `festa.advice.len` | int | `lib.rs:4215` | Array length |
| `festa.advice.list` | array | `lib.rs:4216` | Per-advice details |
| `festa.advice.list[0].advice_id` | int | `lib.rs:4203` | ObscuredInt at AdviceInfo+0x10 |
| `festa.advice.list[0].fusion_advice_id` | int | `lib.rs:4206` | ObscuredInt at AdviceInfo+0x24 |
| `festa.checkpoints` | object | `lib.rs:4247-4251` | Checkpoint results |
| `festa.checkpoints.len` | int | `lib.rs:4248` | Array length |
| `festa.checkpoints.list` | array | `lib.rs:4249` | Per-checkpoint details |
| `festa.checkpoints.list[0].schedule_id` | int | `lib.rs:4233` | ObscuredInt at CheckPoint+0x10 |
| `festa.checkpoints.list[0].result_type` | int | `lib.rs:4239` | ObscuredInt at CheckPoint+0x24 |
| `festa.gain_gauges` | object | `lib.rs:4283-4287` | Command gain gauge array |
| `festa.gain_gauges.len` | int | `lib.rs:4284` | Array length |
| `festa.gain_gauges.list` | array | `lib.rs:4285` | Per-command gauge details |
| `festa.gain_gauges.list[0].command_type` | int | `lib.rs:4266` | ObscuredInt at GainGauge+0x10 |
| `festa.gain_gauges.list[0].predict_len` | int | `lib.rs:4273` | Inner array length |
| `festa.gain_gauges.list[0].predict_class` | string | `lib.rs:4278` | Inner array element class name |
| `festa.gain_gauges.list[0].predict` | array | `lib.rs:4279` | Raw hex dump of gauge leaves |
| `festa.command_info` | object | `lib.rs:4324-4329` | Command info array |
| `festa.command_info.len` | int | `lib.rs:4325` | Array length |
| `festa.command_info.list` | array | `lib.rs:4326` | Per-command details |
| `festa.command_info.list[0].command_type` | int | `lib.rs:4305` | ObscuredInt at CmdInfo+0x10 |
| `festa.command_info.list[0].command_id` | int | `lib.rs:4306` | ObscuredInt at CmdInfo+0x24 |
| `festa.command_info.list[0].params_len` | int | `lib.rs:4309` | ParamsIncDecInfoArray length |
| `festa.command_info.list[0].predict_len` | int | `lib.rs:4312` | PredictGainActGaugeArray length |
| `festa.command_info.list[0].is_upper_training_event_occurred` | int | `lib.rs:4314` | ObscuredInt at CmdInfo+0x48 |
| `festa.command_info.list[0].predict` | array | `lib.rs:4319` | Raw hex dump of gauge leaves |
| `festa.command_info.note` | string | `lib.rs:4327` | `"decoded"` or `"released_in_settlement"` |
| `festa.evaluation_info` | object | `lib.rs:4354-4358` | Evaluation info array |
| `festa.evaluation_info.len` | int | `lib.rs:4355` | Array length |
| `festa.evaluation_info.list` | array | `lib.rs:4356` | Per-evaluation details |
| `festa.evaluation_info.list[0].class` | string | `lib.rs:4344` | Element class name |
| `festa.evaluation_info.list[0].fields_hex` | array | `lib.rs:4346` | Hex dump of first 0x40 bytes |
| `festa.ds_class` | string | `lib.rs:4360-4362` | DataSet class name |
| `festa.note` | string | `lib.rs:4367-4373` | `"offsets_verified_runtime_chain_2026-10-26"` or `"layout_suspect_see_diag"` |
| `festa.diag` | array[string] | `lib.rs:4375-4381` | Diagnostic log messages |

- **Error sentinels:**
  - `{"error":"api_null"}` (`lib.rs:20417`)
  - `{"error":"image_null"}` (`lib.rs:20421`)
  - `{"error":"no_wdm"}` (`lib.rs:20430`)
  - `{"error":"no_wdm_inst"}` (`lib.rs:20434`)
  - `{"error":"no_sm"}` (`lib.rs:20444`)
  - `{"error":"no_chara"}` (`lib.rs:20455`)
  - `{"ok":false,"error":"scenario_obj_null","scenario_id_hint":N,"getter_key_used":N}` (`lib.rs:20473`)
  - `{"ok":false,"error":"festa_scenario_class_null"}` (`lib.rs:20481`)
  - `{"ok":false,"error":"dataset_null"}` (`lib.rs:20492`)
  - `{"ok":false,"error":"dataset_class_null"}` (`lib.rs:20497`)

- **ObscuredInt:** YES — extensively. All act/advice/checkpoint/command fields use `read_obscured_int_at()` with 20-byte stride 0x14 layout. The returned numbers are already decoded plaintext.

---

## 9. `/action/latest`

- **Dispatch:** `lib.rs:9170`
- **Method:** GET, no params
- **Boot-safe:** YES (`lib.rs:8162`)
- **DL_ALLOWED:** NO (not in list)
- **Shape:** flat JSON object
- **Handler:** inline at `lib.rs:9170-9193`; reads `ACTION_STATE` mutex (no IL2CPP calls)
- **Keys** (from `lib.rs:9190-9192`):

| Key | Type | Source | Notes |
|---|---|---|---|
| `sequence` | int | `state.sequence` (`lib.rs:9174`) | Monotonic counter |
| `raw_command_id` | int | `state.command_id` (`lib.rs:9174`) | Raw ID from training hook |
| `normalized_command_id` | int | `normalized` (`lib.rs:9176`) | Mapped to 101-106 range |
| `action` | string | `action` (`lib.rs:9176`) | `"Speed"`, `"Power"`, `"Guts"`, `"Stamina"`, `"Wiz"`, `"None"`, `"Unknown"` |
| `result_type` | int | `state.training_result` (`lib.rs:9174`) | Training result code |
| `source` | string | literal | Always `"training_hook"` |

- **Command ID mapping** (`lib.rs:9176-9189`):
  - 101/601 → Speed (normalized 101)
  - 102/603 → Power (normalized 102)
  - 103/604 → Guts (normalized 103)
  - 105/602 → Stamina (normalized 105)
  - 106/605 → Wiz (normalized 106)
  - -1 → None (normalized -1)
  - other → Unknown (normalized = raw_command_id)

- **Error sentinels:** none; always returns valid JSON even when `ACTION_STATE` is empty (returns `sequence=0, raw_command_id=-1, action="None"`)

- **ObscuredInt:** NO — reads from in-memory `ACTION_STATE` struct, not game objects

---

## 10. `/debug/params`

- **Dispatch:** `lib.rs:8357` → calls `debug_params_inc_dec()` (`lib.rs:13293`)
- **Method:** GET, no params
- **Boot-safe:** NO
- **DL_ALLOWED:** YES (`lib.rs:8221`)
- **Shape:** nested JSON object
- **Handler:** `debug_params_inc_dec()` (`lib.rs:13293-13552`)
- **Top-level keys** (from `lib.rs:13545-13551`):

| Key | Type | Source | Notes |
|---|---|---|---|
| `scenario_id` | int | `scenario_id` | From `get_ScenarioId` |
| `actual_elem_class` | string | `actual_elem_class_name` | Element class name |
| `elem_is_info_type` | bool | `elem_is_info_type` | true = SingleModeParamsIncDecInfo (plain Int32) |
| `items` | array | `debug_items` | Per-param details |

- **`items[0]`** keys (from `lib.rs:13538-13541`):

| Key | Type | Source | Notes |
|---|---|---|---|
| `cmd_idx` | int | `i` | Command index (0..2) |
| `param_idx` | int | `j` | Param index within command (0..2) |
| `actual_class` | string | `actual_elem_class_name` | Element class name |
| `elem_is_info_type` | bool | `elem_is_info_type` | Layout flag |
| `auto_tt` | int | `auto_tt` | Auto-detected TargetType |
| `auto_val` | int | `auto_val` | Auto-detected Value |
| `plain_tt` | int | `plain_tt` | Plain Int32 read at 0x10 |
| `plain_val` | int | `plain_val` | Plain Int32 read at 0x14 |
| `field_tt_xor` | int | `tt_decrypted` | ObscuredInt XOR decode at 0x10/0x14 |
| `field_val_xor` | int | `val_decrypted` | ObscuredInt XOR decode at 0x24/0x28 |
| `raw` | string | `hex_dump` | Hex dump of first 0x20 bytes |

- **Error sentinels:**
  - `{"error":"image_null"}` (`lib.rs:13296`)
  - `{"error":"wdm_class_null"}` (`lib.rs:13306`)
  - `{"error":"wdm_no_singleton"}` (`lib.rs:13310`)
  - `{"error":"sm_data_null"}` (`lib.rs:13315`)
  - `{"error":"chara_null"}` (`lib.rs:13333`)
  - `{"error":"scenario_obj_null"}` (`lib.rs:13339`)
  - `{"error":"unknown_scenario","id":N}` (`lib.rs:13358`)
  - `{"error":"scenario_class_null","name":"..."}` (`lib.rs:13362`)
  - `{"error":"dataset_null"}` (`lib.rs:13371`)
  - `{"error":"dataset_class_null","name":"..."}` (`lib.rs:13377`)
  - `{"error":"cmd_elem_class_null"}` (`lib.rs:13386`)
  - `{"error":"cmd_arr_null"}` (`lib.rs:13391`)
  - `{"error":"cmd_arr_empty"}` (`lib.rs:13398`)

- **ObscuredInt:** YES — reads both plain Int32 and ObscuredInt XOR-decoded values; `auto_tt`/`auto_val` select the correct layout based on element class name (`lib.rs:13522-13526`)

---

## Open Questions

### A. Is there a single route that aggregates matsuri/scenario-15 state (e.g. `/api/matsuri/state`)?

**NO.** Grep for `matsuri` and `api/matsuri` in lib.rs returns zero matches. The dispatch table at `lib.rs:8250-9947` and the 404 "available" list at `lib.rs:9944` contain no `/api/matsuri/*` route.

The closest aggregation is:
- `/summary` with `scenario_id==15` includes a `festa` object (`lib.rs:7020-7022`, `7217-7221`)
- `/debug/festa_probe` is dedicated to scenario 15 Festa state (`lib.rs:8387`, `20415`)
- `/scenario` returns full scenario detail including Festa DataSet when `scenario_id==15` (`lib.rs:8349`, `1718`)

### B. Which routes are declared in `DL_ALLOWED` that permit `?dl=1`?

From `lib.rs:8205-8240`:
```
/summary, /scenario, /data, /ramen, /debug/ramen_transition,
/api/sniff, /api/sniff/metadata, /api/sniff/diag,
/api/event/choices, /api/event/observations,
/debug/event_reward_targets, /debug/resource_meta_schema,
/debug/resource_meta_probe, /debug/resource_crypto_symbols,
/debug/all, /debug/params, /debug/cmdinfo, /debug/breeders,
/debug/training_partners, /debug/rameninfo, /debug/laststep,
/debug/storydata, /debug/ramenfields, /debug/gauge, /debug/gauge2,
/debug/ramengains, /debug/paramsincdec, /debug/training_seed,
/debug/unique_skills, /debug/hint_gain, /debug/sc_effect,
/debug/unique_detail, /debug/festa_probe, /classes
```

The `?dl=1` flag is parsed at `lib.rs:8241-8243`; when `dl_enabled=true`, the response is sent with `Content-Disposition: attachment` header.

### C. What is the exact 404 response body format, and does it enumerate available routes?

**YES.** From `lib.rs:9943-9946`:
```json
{
  "error": "not_found",
  "path": "<requested_path>",
  "available": ["/scan", "/data", "/status", "/health", "/scenario", ...]
}
```

The `available` array is a hard-coded list of ~100 route paths (`lib.rs:9944`). It does NOT include all routes (e.g. missing `/debug/turn_probe`, `/action/latest`, `/debug/gauge`, `/debug/gauge2`, `/debug/training_partners`, `/debug/festa_probe`, `/debug/params`).

---

## Summary

**Fully mapped routes (all fields documented with line numbers):**
1. `/health` — simple status endpoint
2. `/summary` — comprehensive game state (deeply nested)
3. `/debug/turn_probe` — turn number extraction from /summary
4. `/debug/gauge` — Ramen scenario CommandFeelingInfoArray diagnostic
5. `/debug/gauge2` — Ramen scenario array field scanner
6. `/debug/training_partners` — training partner diagnostic
7. `/scenario` — full scenario detail (varies by scenario_id)
8. `/debug/festa_probe` — Festa scenario 15 comprehensive probe
9. `/action/latest` — last training action from hook cache
10. `/debug/params` — ParamsIncDecInfoArray element layout diagnostic

**NOT FOUND fields:** None. All requested routes exist and are fully documented.

**Answers to extra questions:**
- **A:** No `/api/matsuri/state` route exists. Use `/summary` (with `festa` object when sid==15), `/debug/festa_probe`, or `/scenario`.
- **B:** 33 routes in `DL_ALLOWED` list (`lib.rs:8205-8240`) permit `?dl=1` for attachment download.
- **C:** 404 returns `{"error":"not_found","path":"...","available":[...]}` with a hard-coded list of ~100 routes (`lib.rs:9943-9946`).
