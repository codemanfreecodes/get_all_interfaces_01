#ComingSoon
# Check All Interfaces on Device

A Rust crate for checking and listing all network interfaces on a device.

## Overview

This Rust crate provides a simple and straightforward way to retrieve a list of all network interfaces available on a device. It abstracts the platform-specific details and offers a unified interface for cross-platform compatibility.

## Features

- Retrieve a list of all network interfaces on the device.
- Access information about each network interface, including name, MAC address, IP addresses, and more.
- Cross-platform compatibility (Linux, Windows, macOS, etc.).

## Usage

To use this crate in your Rust project, add this in your `main.rc`:

```
extern crate get_all_interfaces;

fn main() {
        let interface = get_all_interfaces::select_network_interface();

        match interface {
                Some(iface) => println!("Selected interface: {}", iface.name),
                None => println!("No network interface selected."),
                }
}

```

