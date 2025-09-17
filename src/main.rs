mod controller;
mod utility;
mod run;

/*
    let ip = conv::str_to_bin_octets("10.0.5.130");
    let subnet = conv::str_to_bin_octets("255.255.255.192");


    println!("{:?}",ip );


    let f = bcast_bits(ip, subnet);
    let g = conv::to_int_vec(f);
    println!("{:?}", g);


    let d = splitnet(23);
    println!("{:?}",d);
    let j = find_host(d);
    println!("{:?}", j);


    let cl = class_from_ip_first_octet();
    println!("{:?}",cl);




    let ip: &str = "130.100.0.0";
    let prefix: i32 = 22;        // <-- era 16, deve essere 22
    let sub_neces: i32 = 3;
    let ips = flsm::net_id_for_subnet(ip, prefix, sub_neces);
    println!("{:?}",ips );
    for i in 0..3{
        let ipb = flsm::gateway_from_netid(&ips[i],prefix as u32);
        println!("{:?}",ipb );
    }


    let host = flsm::select_host(ip, prefix, 63, 260);
    println!("{:?}",host );
*/

fn main() {
    controller::run();




}


