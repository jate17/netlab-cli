/*

SEZIONE NOIOSA


*/



use crate::run::helpers;
use crate::run::aesthetics;
use crate::utility::flsm;

pub fn hostnec() {
    helpers::clear();
    println!("Inserisci il numero di host reali");
    let host_real = helpers::read_input("=>") ;
    let cov: i32 = match host_real.trim().parse() {
        Ok(n) => n,
        Err(_) => {
            println!("Errore numero non valido");
            return;
        }
    };

    let ope = flsm::host_necessary(cov);
    println!("[*] Host necessari: {}\n[**] Bit Necessari: {}", ope.1, ope.0 );

    helpers::continue_run();

}


pub fn subnetavv(){
    helpers::clear();
    println!("Inserisci CIDR:");
    let cidr = helpers::read_input("=>") ;
    let cidr_conv: u32 = match cidr.trim().parse() {
        Ok(n) => n,
        Err(_) => {
            println!("Errore numero non valido");
            return;
        }
    };
    println!("Inserisci i bit necessari: ");
    let bit = helpers::read_input("=>") ;
    let bit_conv: u32 = match bit.trim().parse() {
        Ok(n) => n,
        Err(_) => {
            println!("Errore numero non valido");
            return;
        }
    };

    let ope = flsm::subnet_available(cidr_conv, bit_conv);
    println!("[*] Subnet Disponibili: {}\n", ope );

    helpers::continue_run();


}


pub fn hostxsub(){
    helpers::clear();
    println!("Inserisci CIDR:");
    let cidr = helpers::read_input("=>") ;
    let cidr_conv: u32 = match cidr.trim().parse() {
        Ok(n) => n,
        Err(_) => {
            println!("Errore numero non valido");
            return;
        }
    };
    println!("Inserisci i bit necessari: ");
    let bit = helpers::read_input("=>") ;
    let bit_conv: u32 = match bit.trim().parse() {
        Ok(n) => n,
        Err(_) => {
            println!("Errore numero non valido");
            return;
        }
    };

    let ope = flsm::host_per_subnet(cidr_conv, bit_conv);
    println!("[*] Subnet Disponibili: {}\n", ope );

    helpers::continue_run();


}


pub fn prefix_id() {
    helpers::clear();
    println!("Inserisci il numero di bit:");
    let host_real = helpers::read_input("=>") ;
    let cov: u32 = match host_real.trim().parse() {
        Ok(n) => n,
        Err(_) => {
            println!("Errore numero non valido");
            return;
        }
    };

    let ope = flsm::get_prefix_id(cov);
    println!("[*] Prefix-ID: {}\n", ope);
    helpers::continue_run();
}






pub fn netidsubnet(){
    helpers::clear();
    println!("Inserisci IP:");
    let ip = helpers::read_input("=>") ;
    let ip_conv: &str = ip.as_str();

    println!("Inserisci Prefix-ID: ");
    let pid = helpers::read_input("=>") ;
    let pid_conv: i32 = match pid.trim().parse() {
        Ok(n) => n,
        Err(_) => {
            println!("Errore numero non valido");
            return;
        }
    };

    println!("Inserisci Subnet Necessarie: ");
    let sub = helpers::read_input("=>") ;
    let sub_conv: i32 = match sub.trim().parse() {
        Ok(n) => n,
        Err(_) => {
            println!("Errore numero non valido");
            return;
        }
    };

    let ope = flsm::net_id_for_subnet(ip_conv, pid_conv,sub_conv);
    println!("\n[*] Subnet Disponibili:\n");

    let mut count: u8 = 1;

    for items in ope {
        println!("[{}] => {}",count, items );
        count += 1;
    }
    println!("\n\n");


    helpers::continue_run();
}



pub fn broadcast_netid(){
    helpers::clear();
    println!("Inserisci Net-ID:");
    let boradcast = helpers::read_input("=>") ;
    let bc_conv: &str = boradcast.as_str();
    println!("Inserisci Prefix-ID: ");
    let pid = helpers::read_input("=>") ;
    let pid_conv: u32 = match pid.trim().parse() {
        Ok(n) => n,
        Err(_) => {
            println!("Errore numero non valido");
            return;
        }
    };

    let ope = flsm::broadcast_from_netid(bc_conv, pid_conv);
    println!("[*] Subnet Disponibili: {}\n", ope );

    helpers::continue_run();
}


pub fn gateway_netid(){
    helpers::clear();
    println!("Inserisci Net-ID:");
    let boradcast = helpers::read_input("=>") ;
    let bc_conv: &str = boradcast.as_str();
    println!("Inserisci Prefix-ID: ");
    let pid = helpers::read_input("=>") ;
    let pid_conv: u32 = match pid.trim().parse() {
        Ok(n) => n,
        Err(_) => {
            println!("Errore numero non valido");
            return;
        }
    };

    let ope = flsm::gateway_from_netid(bc_conv, pid_conv);
    println!("[*] Subnet Disponibili: {}\n", ope );

    helpers::continue_run();
}



pub fn hostsprecati(){
    helpers::clear();
    println!("Inserisci Numero di Host: ");
    let host = helpers::read_input("=>") ;
    let host_conv: i32 = match host.trim().parse() {
        Ok(n) => n,
        Err(_) => {
            println!("Errore numero non valido");
            return;
        }
    };

    let ope = flsm::hosts_wasted(host_conv);
    println!("[*] Subnet Disponibili: {}\n", ope );

    helpers::continue_run();
}



pub fn findhost(){
    helpers::clear();
    println!("Inserisci Net-ID:");
    let ip = helpers::read_input("=>") ;
    let ip_conv: &str = ip.as_str();

    println!("Inserisci Prefix-ID: ");
    let pid = helpers::read_input("=>") ;
    let pid_conv: u32 = match pid.trim().parse() {
        Ok(n) => n,
        Err(_) => {
            println!("Errore numero non valido");
            return;
        }
    };

    println!("Inserisci Subnet Necessarie: ");
    let sub = helpers::read_input("=>") ;
    let sub_conv: u32 = match sub.trim().parse() {
        Ok(n) => n,
        Err(_) => {
            println!("Errore numero non valido");
            return;
        }
    };

    println!("Inserisci Host Necessarie: ");
    let host = helpers::read_input("=>") ;
    let host_conv: u32 = match host.trim().parse() {
        Ok(n) => n,
        Err(_) => {
            println!("Errore numero non valido");
            return;
        }
    };

    let ope = flsm::select_host(ip_conv, pid_conv,sub_conv, host_conv);
    println!("\n[*] Subnet Disponibili: {} \n", ope);

    helpers::continue_run();
}

