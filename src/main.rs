mod adapter;
mod capture;
mod decode;

/// 抓一帧 → 解码 → 压平输出。
/// 退出码：0 = 三条路由全通；1 = 有缺（`frame complete=false`），方便脚本判断。
fn main() {
    let frame = decode::decode(&capture::snapshot());
    for line in adapter::adapt(&frame) {
        println!("{line}");
    }
    if !frame.complete {
        std::process::exit(1);
    }
}
