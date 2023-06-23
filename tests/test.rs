use std::net::IpAddr;

use led_strip_control::LedStrip;

#[test]
fn white() {
    let ip = IpAddr::V4("192.168.1.73".parse().unwrap());
    let mut led_strip = LedStrip::new(ip, 1, 100);
    // Set up any other necessary resources

    let mut data: Vec<u8> = Vec::new();

    for _ in 0..(170 * 3) {
        data.push(255);
    }

    led_strip.send(data);
}
