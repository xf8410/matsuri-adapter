//! 采集模块：从本机 SO 插件（同 UID，127.0.0.1:18765）抓原始 JSON。
//!
//! 这里只负责「拿到文本」，不解释任何字段——解释全在 decode.rs。
//! 每条路由单独返回 Result：一条挂了不影响其它条，这样下游能区分
//! 「插件没起来」和「这个字段现在没有值」。

use std::io::{Read, Write};
use std::net::TcpStream;
use std::time::Duration;

pub const PLUGIN_ADDR: &str = "127.0.0.1:18765";
const CONNECT_TIMEOUT: Duration = Duration::from_secs(3);
const IO_TIMEOUT: Duration = Duration::from_secs(20);
const MAX_BODY_BYTES: u64 = 8 * 1024 * 1024;

/// 一帧快照 = 这三条只读 GET。
///
/// 刻意不取 `/debug/turn_probe`：它内部就是 `read_summary()`（lib.rs:8327），
/// 字段在 `/summary` 里已经全有，重复调用等于多扫一次游戏内存。
pub const SNAPSHOT_ROUTES: [(&str, &str); 3] = [
    ("festa", "/debug/festa_probe"),
    ("summary", "/summary"),
    ("action", "/action/latest"),
];

/// (路由名, 结果)。Err 里是「为什么没拿到」，不是游戏数据。
pub type Raw = Vec<(String, Result<String, String>)>;

pub fn snapshot() -> Raw {
    SNAPSHOT_ROUTES
        .iter()
        .map(|(name, path)| (name.to_string(), get(path)))
        .collect()
}

/// 一个极简 HTTP/1.1 GET。不引第三方 client：整个 crate 只要 serde_json 一个依赖，
/// 方便在设备上或离线环境里直接 cargo build。
pub fn get(path: &str) -> Result<String, String> {
    let mut sock = TcpStream::connect_timeout(
        &PLUGIN_ADDR
            .parse()
            .map_err(|e| format!("bad addr {PLUGIN_ADDR}: {e}"))?,
        CONNECT_TIMEOUT,
    )
    .map_err(|e| format!("connect {PLUGIN_ADDR} failed ({e}) — 插件没起来 / 没做 adb forward"))?;
    sock.set_read_timeout(Some(IO_TIMEOUT))
        .map_err(|e| e.to_string())?;
    sock.set_write_timeout(Some(IO_TIMEOUT))
        .map_err(|e| e.to_string())?;

    let req = format!(
        "GET {path} HTTP/1.1\r\nHost: {PLUGIN_ADDR}\r\nAccept: application/json\r\nConnection: close\r\n\r\n"
    );
    sock.write_all(req.as_bytes()).map_err(|e| format!("write: {e}"))?;
    sock.flush().map_err(|e| format!("flush: {e}"))?;

    // Connection: close，所以读到 EOF 就是全文。上限防炸。
    let mut buf: Vec<u8> = Vec::new();
    sock.take(MAX_BODY_BYTES + 1)
        .read_to_end(&mut buf)
        .map_err(|e| format!("read: {e}"))?;
    if buf.len() as u64 > MAX_BODY_BYTES {
        return Err(format!("{path}: body over {MAX_BODY_BYTES} bytes"));
    }

    let split = find_header_end(&buf).ok_or("response has no \\r\\n\\r\\n")?;
    let head = String::from_utf8_lossy(&buf[..split]).to_string();
    let mut body: Vec<u8> = buf[split + 4..].to_vec();

    let status_line = head.lines().next().unwrap_or("").to_string();
    if !status_line.contains(" 200") {
        return Err(format!("{path}: {status_line}"));
    }
    if header_has(&head, "transfer-encoding", "chunked") {
        body = dechunk(&body)?;
    }
    Ok(String::from_utf8_lossy(&body).to_string())
}

fn find_header_end(buf: &[u8]) -> Option<usize> {
    buf.windows(4).position(|w| w == b"\r\n\r\n")
}

fn header_has(head: &str, key: &str, value: &str) -> bool {
    head.lines().skip(1).any(|l| match l.split_once(':') {
        Some((k, v)) => {
            k.trim().eq_ignore_ascii_case(key) && v.trim().to_ascii_lowercase().contains(value)
        }
        None => false,
    })
}

/// 分块传输解码。按字节切，避免在非 UTF-8 边界上切片 panic。
fn dechunk(src: &[u8]) -> Result<Vec<u8>, String> {
    const CRLF: &[u8] = b"\r\n";
    let mut out: Vec<u8> = Vec::new();
    let mut rest = src;
    loop {
        let at = rest
            .windows(CRLF.len())
            .position(|w| w == CRLF)
            .ok_or("chunk: no size line")?;
        let size_hex = String::from_utf8_lossy(&rest[..at]).to_string();
        let size_hex = size_hex.split(';').next().unwrap_or("0").trim().to_string();
        let size = usize::from_str_radix(&size_hex, 16).map_err(|e| format!("chunk size {size_hex}: {e}"))?;
        if size == 0 {
            return Ok(out);
        }
        let body_start = at + CRLF.len();
        let need = body_start + size + CRLF.len();
        if rest.len() < need {
            return Err("chunk: truncated".to_string());
        }
        out.extend_from_slice(&rest[body_start..body_start + size]);
        rest = &rest[body_start + size + CRLF.len()..];
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn dechunks_a_two_chunk_body() {
        let src = b"5\r\nhello\r\n6\r\n world\r\n0\r\n\r\n";
        assert_eq!(dechunk(src).unwrap(), b"hello world".to_vec());
    }

    #[test]
    fn dechunk_rejects_truncated_chunk() {
        assert!(dechunk(b"5\r\nhe\r\n0\r\n\r\n").is_err());
    }

    #[test]
    fn header_lookup_is_case_insensitive() {
        let head = "HTTP/1.1 200 OK\r\nTransfer-Encoding: Chunked\r\n";
        assert!(header_has(head, "transfer-encoding", "chunked"));
        assert!(!header_has(head, "content-type", "chunked"));
    }

    #[test]
    fn routes_are_the_three_readonly_ones() {
        assert_eq!(
            SNAPSHOT_ROUTES.map(|(_, p)| p),
            ["/debug/festa_probe", "/summary", "/action/latest"]
        );
    }
}
