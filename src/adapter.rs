//! 适配模块：Frame → 行式文本（一行一个事实，`key=v` 空格分隔）。
//!
//! 为什么要这一层：上游端点的 JSON 很深、键名很啰嗦，直接丢给模型或脚本既费 token
//! 又容易被改字段名打挂。这一层把结构压平，字段顺序固定，缺值一律显式留 `-1`，
//! 这样两帧之间 diff 就能直接看出「哪一格变了」。

use crate::decode::{Act, Advice, Checkpoint, Frame};

pub fn adapt(f: &Frame) -> Vec<String> {
    let mut out = Vec::new();
    out.push(format!(
        "frame complete={} missing=[{}] notes=[{}]",
        f.complete,
        f.missing.join("; "),
        f.notes.join("; ")
    ));
    out.push(format!(
        "turn value={} raw_total={} month={} half={} semantics={}",
        f.turn, f.raw_total_turn_num, f.month, f.half, or_dash(&f.turn_semantics)
    ));
    out.push(format!(
        "scenario name={} id_hint={} turn_hint={}",
        or_dash(&f.scenario),
        f.scenario_id_hint,
        f.turn_hint
    ));
    out.push(format!(
        "camp remain_turn={} max_act_level={} act_step={}",
        f.camp_remain_turn, f.max_act_level, f.act_training_step
    ));
    for a in &f.acts {
        out.push(act_line(a));
    }
    for (i, a) in f.advice.iter().enumerate() {
        out.push(advice_line(i, a));
    }
    for (i, c) in f.checkpoints.iter().enumerate() {
        out.push(checkpoint_line(i, c));
    }
    out.push(format!(
        "stats speed={} stamina={} power={} guts={} wiz={} vital={}/{} skill_point={} fan={} motivation={}",
        f.stats.speed,
        f.stats.stamina,
        f.stats.power,
        f.stats.guts,
        f.stats.wiz,
        f.stats.vital,
        f.stats.max_vital,
        f.stats.skill_point,
        f.stats.fan,
        or_dash(&f.stats.motivation)
    ));
    match &f.action {
        Some(a) => out.push(format!(
            "action seq={} raw={} normalized={} name={} result_type={} source={}",
            a.sequence, a.raw_command_id, a.normalized_command_id, a.action, a.result_type, a.source
        )),
        None => out.push("action none".to_string()),
    }
    out
}

fn act_line(a: &Act) -> String {
    format!(
        "act cid={} type={} name={} name_src={} cid_src={} gauge={}/{} level={} state={} advice={} outing={}",
        a.command_id,
        a.act_type,
        or_dash(&a.name),
        or_dash(&a.name_src),
        or_dash(&a.cid_src),
        a.gauge,
        a.need_gauge,
        a.level,
        a.state,
        a.advice_id,
        a.friend_outing
    )
}

fn advice_line(i: usize, a: &Advice) -> String {
    format!("advice idx={} id={} fusion={}", i, a.advice_id, a.fusion_advice_id)
}

fn checkpoint_line(i: usize, c: &Checkpoint) -> String {
    format!(
        "checkpoint idx={} schedule_id={} result_type={}",
        i, c.schedule_id, c.result_type
    )
}

fn or_dash(s: &str) -> String {
    if s.is_empty() {
        "-".to_string()
    } else {
        s.to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::capture;
    use crate::decode;

    fn frame_from(bodies: [(&str, &str); 3]) -> Frame {
        let raw: Vec<(String, Result<String, String>)> = bodies
            .iter()
            .map(|(n, b)| (n.to_string(), Ok(b.to_string())))
            .collect();
        decode::decode(&raw)
    }

    const FESTA: &str = r#"{"ok":true,"scenario_id_hint":15,"getter_key_used":15,"turn_hint":47,
      "festa":{"festa_info":{"summer_camp_remain_turn":2,"max_act_level":5,"act_training_step":3,"elem_class":"X"},
      "acts":{"len":1,"list":[{"act_type":1,"command_id":5101,"cid_src":"single_mode_15_act_info","name":"短距離","name_src":"mdb","gauge":8,"need_gauge":12,"level":2,"state":1,"advice_id":3,"friend_outing":1,"raw":{}}]},
      "advice":{"len":1,"list":[{"advice_id":7,"fusion_advice_id":0}]},
      "checkpoints":{"len":1,"list":[{"schedule_id":23,"result_type":2}]}}}"#;

    const SUMMARY: &str = r#"{"turn":47,"raw_total_turn_num":47,"month":3,"half":2,
      "ui_turn_semantics":"countdown","scenario":"festa",
      "stats":{"speed":412,"stamina":388,"power":501,"guts":300,"wiz":255,"vital":9,"max_vital":64,"motivation":"GREAT","skill_point":12,"fan":92378}}"#;

    const ACTION: &str = r#"{"sequence":12,"raw_command_id":5103,"normalized_command_id":5103,"action":"Unknown","result_type":0,"source":"training_hook"}"#;

    #[test]
    fn renders_one_line_per_fact() {
        let lines = adapt(&frame_from([
            ("festa", FESTA),
            ("summary", SUMMARY),
            ("action", ACTION),
        ]));
        assert!(lines[0].starts_with("frame complete=true missing=[]"), "{}", lines[0]);
        assert!(lines.iter().any(|l| l == "turn value=47 raw_total=47 month=3 half=2 semantics=countdown"));
        assert!(lines
            .iter()
            .any(|l| l == "act cid=5101 type=1 name=短距離 name_src=mdb cid_src=single_mode_15_act_info gauge=8/12 level=2 state=1 advice=3 outing=1"));
        assert!(lines.iter().any(|l| l == "advice idx=0 id=7 fusion=0"));
        assert!(lines.iter().any(|l| l == "checkpoint idx=0 schedule_id=23 result_type=2"));
        assert!(lines
            .iter()
            .any(|l| l == "stats speed=412 stamina=388 power=501 guts=300 wiz=255 vital=9/64 skill_point=12 fan=92378 motivation=GREAT"));
        assert!(lines
            .iter()
            .any(|l| l == "action seq=12 raw=5103 normalized=5103 name=Unknown result_type=0 source=training_hook"));
    }

    #[test]
    fn empty_output_when_plugin_is_down_but_nothing_is_fabricated() {
        let raw = capture::SNAPSHOT_ROUTES
            .iter()
            .map(|(n, _)| (n.to_string(), Err("connect refused".to_string())))
            .collect::<Vec<_>>();
        let lines = adapt(&decode::decode(&raw));
        assert!(lines[0].starts_with("frame complete=false missing=[festa:"));
        assert!(lines.iter().any(|l| l.starts_with("turn value=-1")));
        assert!(lines.iter().any(|l| l == "action none"));
        assert!(!lines.iter().any(|l| l.starts_with("act cid=")));
    }

    #[test]
    fn line_count_grows_with_acts() {
        let base = adapt(&frame_from([
            ("festa", r#"{"ok":true,"scenario_id_hint":15,"getter_key_used":15,"turn_hint":1,"festa":{}}"#),
            ("summary", SUMMARY),
            ("action", ACTION),
        ]));
        let with_acts = adapt(&frame_from([("festa", FESTA), ("summary", SUMMARY), ("action", ACTION)]));
        assert_eq!(base.len() + 3, with_acts.len(), "acts/advice/checkpoint 各多一行");
        // festa 是空对象：键在、内容空 → 不算缺，只演目数为 0。
        assert!(base.iter().any(|l| l.starts_with("frame complete=true missing=[] notes=[]")));
        assert!(base.iter().any(|l| l.starts_with("camp remain_turn=-1")));
    }
}
