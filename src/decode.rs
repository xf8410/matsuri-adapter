//! 解密模块：把采集帧解析成结构化事件。
//! 输入格式见 `capture::FRAME_PREFIX`：`turn:<N> <kind>=<value>`

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GameEvent {
    pub turn: u32,
    pub kind: String,
    pub value: String,
}

/// 一行帧 → 事件。格式不符（缺前缀、缺空格、turn 不是数字、缺 `=`）返回 None。
pub fn decode_frame(line: &str) -> Option<GameEvent> {
    let rest = line.trim().strip_prefix(crate::capture::FRAME_PREFIX)?;
    let (turn_str, kv) = rest.split_once(' ')?;
    let turn = turn_str.parse::<u32>().ok()?;
    let (kind, value) = kv.split_once('=')?;
    if kind.is_empty() {
        return None;
    }
    Some(GameEvent {
        turn,
        kind: kind.to_string(),
        value: value.to_string(),
    })
}

pub fn decode(frames: Vec<String>) -> Vec<GameEvent> {
    frames.iter().filter_map(|l| decode_frame(l)).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn decodes_valid_frame() {
        assert_eq!(
            decode_frame("turn:23 gauge=42"),
            Some(GameEvent {
                turn: 23,
                kind: "gauge".into(),
                value: "42".into()
            })
        );
    }

    #[test]
    fn value_may_contain_equals() {
        let e = decode_frame("turn:1 note=a=b").unwrap();
        assert_eq!(e.value, "a=b");
    }

    #[test]
    fn skips_dirty_frames() {
        for bad in ["", "not-a-frame", "turn:x gauge=1", "turn:1", "turn:1 =9", "turn: 1 g=1"] {
            assert_eq!(decode_frame(bad), None, "should skip {bad:?}");
        }
    }

    #[test]
    fn decode_drops_dirty_and_keeps_good() {
        let kept = decode(vec![
            "turn:0 stage=Junior".into(),
            "junk".into(),
            "turn:67 gauge=5".into(),
        ]);
        assert_eq!(kept.len(), 2);
        assert_eq!(kept[1].turn, 67);
    }
}
