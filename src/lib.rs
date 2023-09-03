extern crate pnet;

use pnet::datalink::interfaces;
use std::io::{self, Write};

/// Represents a network interface.
pub struct NetworkInterface {
    pub name: String,
}

/// Returns a vector of network interfaces.
pub fn list_network_interfaces() -> Vec<NetworkInterface> {
    let network_interfaces = interfaces();
    network_interfaces
        .iter()
        .map(|iface| NetworkInterface {
            name: iface.name.clone(),
        })
        .collect()
}

/// Runs the user interface for selecting a network interface.
pub fn select_network_interface() -> Option<NetworkInterface> {
    let interface_names = list_network_interfaces();

    if interface_names.is_empty() {
        println!("No network interfaces found.");
        return None;
    }

    println!("Available network interfaces:");
    for (index, name) in interface_names.iter().enumerate() {
        println!("{}: {}", index, name.name);
    }

    let selected_interface_index = get_user_input("Enter the index of the interface you want to use: ");

    match selected_interface_index.parse::<usize>() {
        Ok(index) if index < interface_names.len() => Some(interface_names[index].clone()),
        _ => {
            println!("Invalid index. Please choose a valid interface.");
            None
        }
    }
}

fn get_user_input(prompt: &str) -> String {
    print!("{}", prompt);
    io::stdout().flush().expect("Failed to flush stdout.");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input.");
    input.trim().to_string()
}
