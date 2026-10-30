//! 解码模块：原始 JSON → 一帧结构化状态。
//!
//! 所有键名都抄自插件源码，不猜：
//!   `/debug/festa_probe` → lib.rs:20507（顶层）/ 4123（festa_info）/ 4179+4185（acts）
//!                            / 4209+4215（advice）/ 4248（checkpoints）
//!   `/summary`           → lib.rs:7264
//!   `/action/latest`     → lib.rs:9191
//!
//! 约定：拿不到的整数一律留 `UNKNOWN`（-1），和插件自己 `unwrap_or(-1)` 的口径一致。
//! 解码器绝不编数字——「没有」必须长得和「0」不一样。

use serde_json::Value;

/// 「没拿到」的整数哨兵。
pub const UNKNOWN: i64 = -1;

#[derive(Debug, Clone, PartialEq, Default)]
pub struct Act {
    pub act_type: i64,
    pub command_id: i64,
    pub cid_src: String,
    pub name: String,
    pub name_src: String,
    pub gauge: i64,
    pub need_gauge: i64,
    pub level: i64,
    pub state: i64,
    pub advice_id: i64,
    pub friend_outing: i64,
}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct Advice {
    pub advice_id: i64,
    pub fusion_advice_id: i64,
}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct Checkpoint {
    pub schedule_id: i64,
    pub result_type: i64,
}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct Stats {
    pub speed: i64,
    pub stamina: i64,
    pub power: i64,
    pub guts: i64,
    pub wiz: i64,
    pub vital: i64,
    pub max_vital: i64,
    pub skill_point: i64,
    pub fan: i64,
    /// 插件里 motivation 是字符串（`"motivation":"{}"`，lib.rs:7264），保持原样。
    pub motivation: String,
}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct Action {
    pub sequence: i64,
    pub raw_command_id: i64,
    pub normalized_command_id: i64,
    pub action: String,
    pub result_type: i64,
    pub source: String,
}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct Frame {
    /// 三条路由全部拿到且解析成功。
    pub complete: bool,
    /// 没拿到 / 解析失败的路由名。
    pub missing: Vec<String>,
    /// 插件自己报出来的 ok:false / warning，原样留着。
    pub notes: Vec<String>,

    pub turn: i64,
    pub raw_total_turn_num: i64,
    pub month: i64,
    pub half: i64,
    /// 回合语义由插件声明（countdown / verified_...），本工具不做换算。
    pub turn_semantics: String,

    pub scenario: String,
    pub scenario_id_hint: i64,
    pub turn_hint: i64,

    pub camp_remain_turn: i64,
    pub max_act_level: i64,
    pub act_training_step: i64,

    pub acts: Vec<Act>,
    pub advice: Vec<Advice>,
    pub checkpoints: Vec<Checkpoint>,
    pub stats: Stats,
    pub action: Option<Action>,
}

pub fn decode(raw: &[(String, Result<String, String>)]) -> Frame {
    let mut f = Frame {
        complete: true,
        turn: UNKNOWN,
        raw_total_turn_num: UNKNOWN,
        month: UNKNOWN,
        half: UNKNOWN,
        scenario_id_hint: UNKNOWN,
        turn_hint: UNKNOWN,
        camp_remain_turn: UNKNOWN,
        max_act_level: UNKNOWN,
        act_training_step: UNKNOWN,
        ..Default::default()
    };

    for (name, body) in raw {
        let body = match body {
            Ok(b) => b.clone(),
            Err(e) => {
                f.missing.push(format!("{name}: {e}"));
                continue;
            }
        };
        let v: Value = match serde_json::from_str(&body) {
            Ok(v) => v,
            Err(e) => {
                f.missing.push(format!("{name}: bad json ({e})"));
                continue;
            }
        };
        match name.as_str() {
            "festa" => fill_festa(&mut f, &v),
            "summary" => fill_summary(&mut f, &v),
            "action" => f.action = Some(fill_action(&v)),
            other => f.notes.push(format!("unmapped route {other}")),
        }
    }

    f.complete = f.missing.is_empty();
    f
}

fn fill_festa(f: &mut Frame, v: &Value) {
    if bool_field(v, "ok") != Some(true) {
        // 插件的失败形状（lib.rs:20474）里根本没有 festa 键，
        // 报一次就返回，别再补一条「没有 festa」的重复噪音。
        f.notes.push(format!(
            "festa_probe: {}",
            str_field(v, "error").unwrap_or_else(|| "ok!=true".to_string())
        ));
        return;
    }
    f.scenario_id_hint = int_field(v, "scenario_id_hint").unwrap_or(UNKNOWN);
    f.turn_hint = int_field(v, "turn_hint").unwrap_or(UNKNOWN);

    let festa = match v.get("festa") {
        Some(o) if o.is_object() => o,
        _ => {
            f.notes.push("festa: no festa object".to_string());
            return;
        }
    };

    if let Some(info) = festa.get("festa_info") {
        f.camp_remain_turn = int_field(info, "summer_camp_remain_turn").unwrap_or(UNKNOWN);
        f.max_act_level = int_field(info, "max_act_level").unwrap_or(UNKNOWN);
        f.act_training_step = int_field(info, "act_training_step").unwrap_or(UNKNOWN);
    }

    for el in list_of(festa, "acts") {
        f.acts.push(Act {
            act_type: int_field(el, "act_type").unwrap_or(UNKNOWN),
            command_id: int_field(el, "command_id").unwrap_or(UNKNOWN),
            cid_src: str_field(el, "cid_src").unwrap_or_default(),
            name: str_field(el, "name").unwrap_or_default(),
            name_src: str_field(el, "name_src").unwrap_or_default(),
            gauge: int_field(el, "gauge").unwrap_or(UNKNOWN),
            need_gauge: int_field(el, "need_gauge").unwrap_or(UNKNOWN),
            level: int_field(el, "level").unwrap_or(UNKNOWN),
            state: int_field(el, "state").unwrap_or(UNKNOWN),
            advice_id: int_field(el, "advice_id").unwrap_or(UNKNOWN),
            friend_outing: int_field(el, "friend_outing").unwrap_or(UNKNOWN),
        });
    }
    for el in list_of(festa, "advice") {
        f.advice.push(Advice {
            advice_id: int_field(el, "advice_id").unwrap_or(UNKNOWN),
            fusion_advice_id: int_field(el, "fusion_advice_id").unwrap_or(UNKNOWN),
        });
    }
    for el in list_of(festa, "checkpoints") {
        f.checkpoints.push(Checkpoint {
            schedule_id: int_field(el, "schedule_id").unwrap_or(UNKNOWN),
            result_type: int_field(el, "result_type").unwrap_or(UNKNOWN),
        });
    }
}

fn fill_summary(f: &mut Frame, v: &Value) {
    f.turn = int_field(v, "turn").unwrap_or(UNKNOWN);
    f.raw_total_turn_num = int_field(v, "raw_total_turn_num").unwrap_or(UNKNOWN);
    f.month = int_field(v, "month").unwrap_or(UNKNOWN);
    f.half = int_field(v, "half").unwrap_or(UNKNOWN);
    f.turn_semantics = str_field(v, "ui_turn_semantics").unwrap_or_default();
    f.scenario = str_field(v, "scenario").unwrap_or_default();
    if let Some(w) = str_field(v, "warning") {
        f.notes.push(format!("summary: {w}"));
    }
    if let Some(s) = v.get("stats") {
        f.stats = Stats {
            speed: int_field(s, "speed").unwrap_or(UNKNOWN),
            stamina: int_field(s, "stamina").unwrap_or(UNKNOWN),
            power: int_field(s, "power").unwrap_or(UNKNOWN),
            guts: int_field(s, "guts").unwrap_or(UNKNOWN),
            wiz: int_field(s, "wiz").unwrap_or(UNKNOWN),
            vital: int_field(s, "vital").unwrap_or(UNKNOWN),
            max_vital: int_field(s, "max_vital").unwrap_or(UNKNOWN),
            skill_point: int_field(s, "skill_point").unwrap_or(UNKNOWN),
            fan: int_field(s, "fan").unwrap_or(UNKNOWN),
            motivation: str_field(s, "motivation").unwrap_or_default(),
        };
    }
}

fn fill_action(v: &Value) -> Action {
    Action {
        sequence: int_field(v, "sequence").unwrap_or(UNKNOWN),
        raw_command_id: int_field(v, "raw_command_id").unwrap_or(UNKNOWN),
        normalized_command_id: int_field(v, "normalized_command_id").unwrap_or(UNKNOWN),
        action: str_field(v, "action").unwrap_or_default(),
        result_type: int_field(v, "result_type").unwrap_or(UNKNOWN),
        source: str_field(v, "source").unwrap_or_default(),
    }
}

/// `festa.<key>.list`（外层是 `{"len":N,"list":[...]}`，lib.rs:4185/4215/4248）。
fn list_of<'a>(obj: &'a Value, key: &str) -> Vec<&'a Value> {
    obj.get(key)
        .and_then(|w| w.get("list"))
        .and_then(Value::as_array)
        .map(|a| a.iter().collect())
        .unwrap_or_default()
}

fn int_field(v: &Value, key: &str) -> Option<i64> {
    v.get(key).and_then(Value::as_i64)
}

fn str_field(v: &Value, key: &str) -> Option<String> {
    v.get(key).and_then(Value::as_str).map(|s| s.to_string())
}

fn bool_field(v: &Value, key: &str) -> Option<bool> {
    v.get(key).and_then(Value::as_bool)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn ok(name: &str, body: &str) -> (String, Result<String, String>) {
        (name.to_string(), Ok(body.to_string()))
    }

    fn dead(name: &str) -> (String, Result<String, String>) {
        (name.to_string(), Err("connect refused".to_string()))
    }

    const FESTA: &str = r#"{
      "ok": true, "scenario_id_hint": 15, "getter_key_used": 15, "turn_hint": 47,
      "festa": {
        "festa_info": {"summer_camp_remain_turn": 2, "max_act_level": 5, "act_training_step": 3, "elem_class": "WorkSingleModeScenarioFestaInfo"},
        "acts": {"len": 2, "list": [
          {"act_type":1,"command_id":5101,"cid_src":"single_mode_15_act_info","name":"短距離","name_src":"mdb","gauge":8,"need_gauge":12,"level":2,"state":1,"advice_id":3,"friend_outing":1,"raw":{}},
          {"act_type":2,"command_id":5102,"cid_src":"formula_5100_plus_act_type","name":"マイル","name_src":"mdb_miss","gauge":0,"need_gauge":10,"level":0,"state":0,"advice_id":0,"friend_outing":0,"raw":{}}
        ]},
        "advice": {"len":1,"list":[{"advice_id":7,"fusion_advice_id":0}]},
        "checkpoints": {"len":1,"list":[{"schedule_id":23,"result_type":2}]}
      }
    }"#;

    const SUMMARY: &str = r#"{
      "version":"3.28.4","year":2,"turn":47,"raw_total_turn_num":47,"ui_turn_semantics":"countdown",
      "raw_field_mapping":"verified_ramen_upstream_semantics","month":3,"half":2,"scenario":"festa","chara_id":1001,
      "stats":{"speed":412,"stamina":388,"power":501,"guts":300,"wiz":255,"vital":9,"max_vital":64,"motivation":"GREAT","skill_point":12,"fan":92378}
    }"#;

    const ACTION: &str = r#"{"sequence":12,"raw_command_id":5103,"normalized_command_id":5103,"action":"Unknown","result_type":0,"source":"training_hook"}"#;

    #[test]
    fn decodes_a_full_frame() {
        let f = decode(&[ok("festa", FESTA), ok("summary", SUMMARY), ok("action", ACTION)]);
        assert!(f.complete, "missing={:?}", f.missing);
        assert_eq!(f.turn, 47);
        assert_eq!(f.month, 3);
        assert_eq!(f.turn_semantics, "countdown");
        assert_eq!(f.camp_remain_turn, 2);
        assert_eq!(f.max_act_level, 5);
        assert_eq!(f.acts.len(), 2);
        assert_eq!(
            (f.acts[0].gauge, f.acts[0].need_gauge, f.acts[0].level),
            (8, 12, 2)
        );
        assert_eq!(f.acts[1].command_id, 5102);
        assert_eq!(f.acts[1].name_src, "mdb_miss");
        assert_eq!(f.advice, vec![Advice { advice_id: 7, fusion_advice_id: 0 }]);
        assert_eq!(f.stats.fan, 92378);
        assert_eq!(f.stats.motivation, "GREAT");
        assert_eq!(f.action.clone().map(|a| (a.raw_command_id, a.action)), Some((5103, "Unknown".to_string())));
    }

    #[test]
    fn plugin_level_failure_becomes_a_note_not_a_panic() {
        let f = decode(&[
            ok("festa", r#"{"ok":false,"error":"scenario_obj_null","scenario_id_hint":15,"getter_key_used":0}"#),
            ok("summary", SUMMARY),
            ok("action", ACTION),
        ]);
        assert!(f.complete, "ok:false 是数据状态，不是采集失败");
        assert_eq!(f.notes, vec!["festa_probe: scenario_obj_null".to_string()]);
        assert!(f.acts.is_empty(), "没有 acts 就是没有，不许造");
        assert_eq!(f.camp_remain_turn, UNKNOWN);
    }

    #[test]
    fn all_routes_down_is_marked_incomplete() {
        let f = decode(&[dead("festa"), dead("summary"), dead("action")]);
        assert!(!f.complete);
        assert_eq!(f.missing.len(), 3);
        assert_eq!(f.turn, UNKNOWN);
        assert!(f.action.is_none());
    }

    #[test]
    fn one_dead_route_still_yields_a_partial_frame() {
        let f = decode(&[dead("festa"), ok("summary", SUMMARY), ok("action", ACTION)]);
        assert!(!f.complete);
        assert_eq!(f.missing.len(), 1);
        assert_eq!(f.stats.speed, 412);
        assert_eq!(f.acts.len(), 0);
    }

    #[test]
    fn unparsable_body_is_reported_as_missing() {
        let f = decode(&[ok("festa", "<html>502</html>"), ok("summary", SUMMARY), ok("action", ACTION)]);
        assert!(!f.complete);
        assert!(f.missing[0].starts_with("festa: bad json"));
    }
}
