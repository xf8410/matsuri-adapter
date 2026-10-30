//! 采集模块：从本机 SO 插件（同 UID，loopback）抓事件帧。
//! 端点默认 127.0.0.1:18765，不暴露外网、不带任何 token。
//!
//! 帧格式（三个模块共用这一份约定）：`turn:<N> <kind>=<value>`
//! 例如 `turn:23 gauge=42`。

pub const FRAME_PREFIX: &str = "turn:";

pub fn capture() -> Vec<String> {
    // TODO: 换成真实抓取，例如 GET http://127.0.0.1:18765/summary 后按上面的格式转帧
    vec![
        format!("{FRAME_PREFIX}0 stage=Junior"),
        format!("{FRAME_PREFIX}23 gauge=42"),
        "not-a-frame".to_string(), // 故意留一条脏数据，验证下游会跳过
    ]
}
