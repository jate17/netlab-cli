
use std::process;

use std::io::{self, Write};


use crate::utility::flsm;
use crate::utility::conv;
use crate::run::helpers;
use crate::run::aesthetics;
use crate::run::func;



enum MState{
    Main,
    Flsm,
    Exit,
    HostNec,
    SubnetAvv,
    GetPrefix,
    NetIdSubnet,
    BroadcastNetID,
    GatewayNetID,
    HostXSub,
    HostWasted,
    FindHost,
}





pub fn run(){
    let mut state = MState::Main;
    helpers::clear();
    loop{
        match state{
            /*MAIN MENU CONTROLLER*/
            MState::Main => {
                aesthetics::banner();
                let input = helpers::read_input("=> ");
                match input.as_str(){
                    "exit" => state = MState::Exit,
                    "0" => state = MState::Flsm,
                     _      => println!("Opzione non valida"),
                }
            }
            /* MENU FLSM*/
            MState::Flsm => {
                aesthetics::flsm_banner();
                let input = helpers::read_input("=> ");
                match input.as_str() {
                    "exit" => state = MState::Main,
                    "0" => state = MState::HostNec,
                    "1" => state = MState::SubnetAvv,
                    "2" => state = MState::HostXSub,
                    "3" => state = MState::GetPrefix,
                    "4" => state = MState::NetIdSubnet,
                    "5" => state = MState::BroadcastNetID,
                    "6" => state = MState::GatewayNetID,
                    "7" => state = MState::HostWasted,
                    "8" => state = MState::FindHost,
                    _      => println!("Opzione non valida"),
                }
            }

            /* EXIT */
            MState::Exit => {
                println!("Bye Dear User");
                process::exit(0);
            }

            /*FUNCTION CONTROLLER FLSM*/
            MState::HostNec => {
                func::hostnec();
                state = MState::Flsm;
            }
            MState::SubnetAvv => {
                func::subnetavv();
                state = MState::Flsm;
            }
            MState::HostXSub => {
                func::hostxsub();
                state = MState::Flsm;
            }
            MState::GetPrefix => {
                func::prefix_id();
                state = MState::Flsm;
            }
            MState::NetIdSubnet => {
                func::netidsubnet();
                state = MState::Flsm;
            }
            MState::BroadcastNetID => {
                func::broadcast_netid();
                state = MState::Flsm;
            }
            MState::GatewayNetID => {
                func::gateway_netid();
                state = MState::Flsm;
            }
            MState::HostWasted => {
                func::hostsprecati();
                state = MState::Flsm;
            }
            MState::FindHost => {
                func::findhost();
                state = MState::Flsm;
            }
        }
    }
    }


