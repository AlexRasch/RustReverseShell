pub fn ip_str_to_u32(ip: &str) -> u32 {
    let mut octets = [0u8; 4];
    let mut parts = ip.as_bytes().split(|&b| b == b'.');

    for i in 0..4 {
        if let Some(part) = parts.next() {
            let mut num = 0u8;
            for &b in part {
                if b < b'0' || b > b'9' {
                    return 0; // Ogiltig IP, returnera 0.0.0.0
                }
                num = num * 10 + (b - b'0');
            }
            octets[i] = num;
        }
    }

    u32::from_ne_bytes(octets)
}