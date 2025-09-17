/*

    Utilità FLSM(Vecchio deprecato)

*/
use crate::utility::conv;

/*

    Restituisce il numero di
    host disponibili e i bit necessari

*/

pub fn host_necessary(host_reali: i32) -> (u32, u32) {
    let host_tot: f64 = host_reali as f64 + 2.00;
    let bit = host_tot.log2().ceil() as u32;
    let avv_host = 2u32.pow(bit);
    (bit,avv_host)
}

/*

    Passi il cidr e passi i bit necessari
    resituisce la subnet disponibile

*/
pub fn subnet_available(cidr: u32, bit_nece: u32) -> u32 {
    2u32.pow(bit_nece)
}

/*
    Calcola quanti host per sottorete rimangono
*/
pub fn host_per_subnet(cidr: u32, bit_nece: u32) -> u32 {
    let host_bits = 32 - (cidr + bit_nece);
    if host_bits == 0 {
        0 // non ci sono host disponibili
    } else {
        2u32.pow(host_bits) - 2
    }
}

/*

    Passa i bit necessari
    Restituisce il prefix-id

*/

pub fn get_prefix_id(bit_nece: u32) -> u32 {
    32 as u32 - bit_nece
}


/*

    Passi ip, prefix e subnet necessarie
    Restituisce il numero di NET-ID di ogni sottorete

*/

pub fn net_id_for_subnet(ip: &str, prefix: i32, sub_nec: i32) -> Vec<String>{
    let block_size: u32 = 2u32.pow(32 - prefix as u32);
    let ip_base: u32 = conv::ipv4_to_int(ip);

    let mut net_ids = Vec::new();
    for i in 0..sub_nec {
        let net_id = conv::int_to_ipv4(ip_base + (i as u32) * block_size);
        net_ids.push(format!("{}", net_id));
    }
    net_ids
}

/*

    Ottieni il broadcast passando il net id e prefix

*/


pub fn broadcast_from_netid(net_id: &str, prefix: u32) -> String{
    let net_int: u32 = conv::ipv4_to_int( net_id);
    let host_bits = 32 - prefix;

    let wildcard = if host_bits == 0 {
        0
    }else {
        (1u32 << host_bits) - 1
    };

    let brodcast_int = net_int | wildcard;
    conv::int_to_ipv4(brodcast_int).to_string()
}


pub fn gateway_from_netid(net_id: &str, prefix: u32) -> String{
    let net_int: u32 = conv::ipv4_to_int( net_id);
    let host_bits = 32 - prefix;

    let wildcard = if host_bits == 0 {
        0
    }else {
        (1u32 << host_bits) - 1
    };

    let brodcast_int = net_int + 1;
    conv::int_to_ipv4(brodcast_int).to_string()
}


/*

    Dando gli host ottieni il numero di host sprecati

*/

pub fn hosts_wasted(host:  i32) -> i32{
    let host_tot: f64 = host as f64 + 3.00;
    let bit = host_tot.log2().ceil() as u32;
    let host_avi = 2u32.pow(bit) as i32;

    host_avi - (host_tot as i32)
}


/*

    Dando informazioni ottieni net-id di una rete

*/

pub fn select_host(net_id: &str, prefix: u32, subnet_req: u32, host_req: u32) -> String{
    let net_int: u32 = conv::ipv4_to_int( net_id);
    let block_size = 2u32.pow(32 - prefix);
    let subnet_base = net_int + subnet_req * block_size;

    let ip_net = subnet_base + host_req;

    conv::int_to_ipv4(ip_net).to_string()
}
