# VPN Client Implemented in Rust

A minimal (for now) VPN client written in rust.


## Architechture
The VPN client consists of two services:

    1) A daemon process which will run in the users background
    2) The CLI binary used to interact with the daemon

When installing `rust-vpn`, a network interface by the name of `rust-vpn-interface` will be added to the computer. 
This network interface will be the one this program will use for networking.

The reason for choosing this architechture is due to the simplicity of interacting with a binary
in order to talk with the daemon.

#### CLI
The CLI consists of 2 simple commands, `on` and `off`.

`on` will check wether the daemon process is alive and if the network interface `rust-vpn-interface` is enabled.
If not, it will run the daemon process and enable the network interface.

`off` kills the daemon and disables the network interface.

#### Daemon
Upon running, the daemon will connect to the VPN server through socket.
The daemon simply listens to the network interface `rust-vpn-interface`, and
encrypts packets that need to be sent, and sends them to the VPN server.
It also decrypts packets that are received from the VPN server.


## How To Use
In order to use the vpn client, type `rust-vpn on` in order to start running the VPN daemon.

To turn it off, type `rust-vpn off` in order to stop running the VPN daemon
