mod hsv;

fn main() {
    println!("{:x}", hsv::hvs2_rgbu8(1200, 100, 100));
    println!("{:x}", hsv::hvs2_rgbu32(1200, 100, 100));
}
