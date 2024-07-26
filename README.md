# Yamanote: automate network setup

Simulate and test custom network setup for your (home) lab. Deploy it to your routers and servers.

## Idea

The goal is to create a hardware-independent setup for experimenting with network topology and tools. User will be able to describe the components (routers, switches, servers) of the designed network using code and configuration files. That description will be then used to simulate the behaviour of the network with QEMU and TAP interfaces.

In the next step, the set of deployment tools will help the user to create configuration files and scripts for real hardware network elements to deploy the design to the real lab network.

## Priorities

First things to get to work to achieve PoC stage
- Prepare API for describing network elements
- Set up virtual L2s that the API-described elements can attach to
- Use the API to create a setup of:
  - simulated WAN resource - a virtual machine running nginx
  - simulated home router - a virtual machine running openwrt
  - simulated home client device - a virtual machine sending HTTP requests
- Make them all controllable via an independent debug network interface

## Prerequisites
To run yamanote you need a Linux machine with docker and docker-compose. All the necessary tools will be downloaded during the development environment image build.

The startup script assumes that you have sudoless docker.

## Setup
To start development environment run:
```
$ . source.me
```

## License

This project is licensed under [GPLv3](LICENSE)
