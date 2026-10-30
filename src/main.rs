mod adapter;
mod capture;
mod decode;

fn main() {
    let frames = capture::capture();
    let events = decode::decode(frames);
    for line in adapter::adapt(&events) {
        println!("{line}");
    }
}
