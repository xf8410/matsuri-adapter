//! 适配模块：把解密事件适配成 DSH / Agora 可消费的统一格式。

use crate::decode::GameEvent;

pub fn adapt(events: &[GameEvent]) -> Vec<String> {
    events
        .iter()
        .map(|e| format!("turn={} {}={}", e.turn, e.kind, e.value))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{capture, decode};

    #[test]
    fn formats_event() {
        let e = decode::decode_frame("turn:1 gauge=42").unwrap();
        assert_eq!(adapt(&[e]), vec!["turn=1 gauge=42"]);
    }

    /// 三个模块串起来：脏帧被丢，好帧按约定格式出。
    #[test]
    fn pipeline_end_to_end() {
        let events = decode::decode(capture::capture());
        let out = adapt(&events);
        assert_eq!(
            out,
            vec!["turn=0 stage=Junior", "turn=23 gauge=42"],
            "capture 里的 not-a-frame 应被丢掉"
        );
    }
}
