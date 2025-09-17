mod controller;
mod utility;
mod run;

fn main() {
    //controller::run();


    let prefix: u32 = 26;

    let mask: u32 = if prefix == 0 {
        0
    } else {
        (!0u32) << (32 - prefix)
    };

    let wildcard = u32::MAX - mask;
    let oct1 = (wildcard >> 24) & 255;
    let oct2 = (wildcard >> 16) & 255;
    let oct3 = (wildcard >> 8) & 255;
    let oct4 = wildcard  & 255;


    println!("{}.{}.{}.{}", oct1, oct2, oct3, oct4 );

}


