

/*

Ottiene da un ip il primo ottetto restituisce u8

*/
fn first_oct(s: &str) -> u8{
    let g: Vec<&str> = s.split('.').collect();
    g[0].parse::<u8>().expect("Errore nella conversione")

}

/*

Ottenimento della classfull

*/
pub fn classfull(ip: &str) -> char {
    let o1: u8 = first_oct(ip);
    match o1 {
        1..=126 => 'A',
        127     => 'L',      // loopback
        128..=191 => 'B',
        192..=223 => 'C',
        224..=239 => 'D',    // multicast
        240..=255 => 'E',
        _ => 'R',            // 0 e altri riservati
    }
}



/*

AND -> Per NETID
*/

pub fn netid(ip: Vec<Vec<u8>>, subnet: Vec<Vec<u8>>) -> Vec<Vec<u8>> {
    ip.iter()
        .zip(subnet.iter())
        .map(|(a, b)| {
            a.iter()
             .zip(b.iter())
             .map(|(x, y)| x & y) // AND bit a bit
             .collect()
        })
        .collect()
}

/*
OR NOT -> x il broadcast

*/

pub fn bcast_bits(ip_bits: Vec<Vec<u8>>, mask_bits: Vec<Vec<u8>>) -> Vec<Vec<u8>> {
    ip_bits
        .iter()
        .zip(mask_bits.iter())
        .map(|(oct_ip, oct_m)| {
            oct_ip
                .iter()
                .zip(oct_m.iter())
                .map(|(bit_ip, bit_m)| bit_ip | (1 - *bit_m)) // OR con NOT “booleano” della mask
                .collect()
        })
        .collect()
}






/*

    Passando n ti dice il numero di host disponibili

*/

pub fn find_host(n: u32) -> u32{
    2u32.pow(n) - 2
}


/*
Ritorna i bit dando il numero di sottoreti che si vuole usare
*/
pub fn splitnet(n: u32) -> u32 {
    assert!(n >= 1, "n deve essere >= 1");
    if n <= 1 { 0 } else { 32 - (n - 1).leading_zeros() }
}
