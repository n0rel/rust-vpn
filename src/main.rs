use clap::{Parser, ValueEnum};
use log::{debug, info};


#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Toggle the VPN
    toggle: Toggle,

}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum, Debug)]
enum Toggle {
    /// Turn the VPN on
    On,
    /// Turn the VPN off
    Off
}


/// Main function
fn main() {
    pretty_env_logger::init();

    debug!("Running main, parsing args...");
    let args = Args::parse();
    parse_arguments(&args);
    
}


/// Deals with user inputted arguments
fn parse_arguments(arguments: &Args) {
    debug!("Parsing arguments...");
    
    match arguments.toggle {
        Toggle::On => enable_vpn(),
        Toggle::Off => disable_vpn(),
    }
}


/// Enables the VPN
fn enable_vpn() {
    info!("Enabling VPN...");
}

fn disable_vpn() {
    info!("Disabling VPN...");
}
