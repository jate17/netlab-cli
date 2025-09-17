use std::net::Ipv4Addr;
/*

    Medoti di conversione

*/



pub fn ipv4_to_int(ip: &str) -> u32 {
    let addr: Ipv4Addr = ip.parse().expect("IP non valido");
    let octets = addr.octets(); // [u8; 4]
    ((octets[0] as u32) << 24)
        | ((octets[1] as u32) << 16)
        | ((octets[2] as u32) << 8)
        | (octets[3] as u32)
}

pub fn int_to_ipv4(num: u32) -> String {
    let o1 = ((num >> 24) & 0xFF) as u8;
    let o2 = ((num >> 16) & 0xFF) as u8;
    let o3 = ((num >> 8) & 0xFF) as u8;
    let o4 = (num & 0xFF) as u8;
    Ipv4Addr::new(o1, o2, o3, o4).to_string()
}


pub fn to_bin8(n: u8) -> Vec<u8> {
    (0..8).rev().map(|i| ((n >> i) & 1)).collect()
}



pub fn str_to_bin_octets(s: &str) -> Vec<Vec<u8>> {
    s.split('.')
        .map(|part| part.parse::<u8>().unwrap())
        .map(to_bin8)
        .collect()
}


pub fn to_int_vec(bits: Vec<Vec<u8>>) -> Vec<u32> {
    bits.into_iter()
        .map(|octet| {
            octet.into_iter().fold(0, |acc, bit| (acc << 1) | bit as u32)
        })
        .collect()
}


