use av_protocol_converter::*;
use std::io;

/// Set Up
///
/// Initial setup process for the program.

fn set_up() {
    println!("\nSet up an output connection first:\n1. DMXCallee (UDMXDevice)\nChoose an output: ");
    let mut input = String::new();
    let _ = io::stdin().read_line(&mut input);
    println!("{}", input);
    match input.as_str().trim() {
        "1"=> {
            let dmx_device= set_up_dmx_callee();
            println!("\nSet up an input connection second:\n1. ArtNet\nChoose an input: ");
            let mut second_input = String::new();
            let _ = io::stdin().read_line(&mut second_input);
            match second_input.as_str().trim() {
                "1"=> {
                    get_artnet_info(dmx_device);
                },
                _ => set_up(),
            }
        },
        _ => set_up(),
    }

}

/// Get ArtNet Info
///
/// Used to collect the ArtNet information, and then runs the ArtNet listener.
///
/// ArtNet only supports DMXCallee outputs as of the current version.
fn get_artnet_info(dmx_callee :impl DMXCallee) {
    println!("\nSet an IP to listen on:");
    let mut ip = String::new();
    let _ = io::stdin().read_line(&mut ip);
    accept_artnet_and_callback(ip.as_str().trim(), 6454, dmx_callee);
}

/// DMXCallee
///
/// Called when a DMXCallee needs to be the recipient of an input.

fn set_up_dmx_callee() -> impl DMXCallee {
    println!("\nSelect a DMX Callee:\n1. UDMXDevice\nEnter here: ");
    let mut input = String::new();
    let _ = io::stdin().read_line(&mut input);
    match input.as_str().trim() {
        "1"=> return UDMXDevice::new(),
        &_ => {return set_up_dmx_callee()}
    }
}

fn main() {
    set_up();
}