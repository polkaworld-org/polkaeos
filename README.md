# Polkaeos

- [Introduction](#1-introduction)
- [Getting Start](#2-getting-start)
- [TODOs](#3-todos)

[中文](https://github.com/polkaeos/polkaeos/blob/demo/README_zh.md)

## 1. Introduction

`Polkaeos` is a polkadot-EOSIO cross-chain bridge implementation with no loss of trust. The design scheme can be seen in:

 [Polkaeos: A Polkadot-EOSIO Cross-Chain Bridge Implementation without Trust Loss](https://github.com/polkaeos/polkaeos/blob/demo/desgin.md)

## 2. Getting Start

Currently `Polkaeos` is just a prototype implementation. To simplify development, the current implementation is only used to verify the feasibility of `Polkaeos`.

`Polkaeos` requires an EOSIO light node implementation to assist in synchronizing the EOSIO network. Since the EOSIO ecosystem is under-supported by the rust implementation, the current implementation uses a simple light node based on golang to connect with the EOSIO network, The polkaeos node shares EOSIO block information through zeromq.

**Install libzmq**

OSX:

```bash
Brew install libsodium zeromq czmq
```

** Compile EOSIO Light Node**

First need to configure the golang build environment.

Compile:

```bash
git clone https://github.com/fanyang1988/eos-light-node.git
cd eos-light-node
go mod vendor
go build
```

**Start EOSIO test network**

Since the current EOSIO test network connection is unstable and difficult to use, it is necessary to start the test network for relay testing.

> The current EOSIO version (v1.8.x) startup test network script is not available, you need to use the modified version

**Compile and install eosio.cdt**

```bash
git clone https://github.com/EOSIO/eosio.cdt.git
cd eosio.cdt
git submodule update --init --recursive
./build.sh
sudo ./install.sh
```

**Compile eos**

```bash
git clone https://github.com/fanyang1988/eos.git -b fix-bios-boot-tutorial
cd eos
git submodule update --init --recursive
cd scripts
./eosio_build.sh -s EOS -y
./eosio_install.sh
```

**Compile the eosio system contract**

```bash
git clone https://github.com/EOSIO/eosio.contracts.git
cd eosio.contracts
./build.sh
```

Start the test network based on the script in the EOS project. The script is based on python3 and needs to install numpy.

```bash
cd eos
cd tutorials/bios-boot-tutorial
./bios-boot-tutorial.py --symbol EOS -a --contracts-dir /path/to/eosio.contracts/build/contracts
```

The startup time is longer and needs to wait.

**Compile Polkaeos**

In order to simplify the coding, the current Polkaeos partially hardcodes the substrate and uses the node-template to start the node:

The build method here is the same as the substitute, so the init part is omitted.

```bash
git clone https://github.com/polkaeos/polkaeos.git
cd polkaeos
cargo build
cd node-template/
./scripts/build.sh
cargo build
```

**start up**

```bash
./target/debug/node-template --dev --base-path /tmp/tmppath
```

At this point you can receive the EOSIO block.

## 3. TODOs

Polkaeos is currently only in the prototype demo state, only showing the principle of Polkaeos, its complete implementation will be completely different from the current one, so Polkaeos still needs a lot of work:

Firstly, We need to completed the EOSIO block synchronization and verification in the Polkaeos node. In order to simplify the development in the Demo, it used a external independent process to verification.
Since the current EOSIO ecosystem has poor support for rust, and EOSIO uses a large number of independent design of basic data structures and algorithms, Polkaeos needs to perform related work independently based on rust.

The second is the development of Polkaeos as a chain of relay bridges. The implementation of the current demo breaks the abstraction of many substitutes. In the official Polkaeos, these implementations will not exist at all, and some work is needed for this.

Finally, Polkaeos needs a lot of contract development work on the EOSIO side, especially the support to the relay miner node. Since this part is relatively independent, the development amount will not be very large.

In the future short-term development plan of Polkaeos (about three months), the first and three points mentioned above will be completed and a Bridge implementation of Token class state relay similar to [ChainX](https://github.com/chainx-org/ChainX) will be completed.

In the medium-term planning of Polkaeos, it focuses on the following three directions:

- Enhance the modularity of Polkaeos so that Polkaeos can be easily embedded into parallel chains that need to be interoperable with the EOSIO chain.
- Better support for mapping of any type of state and event.
- Implementing a chain based on EOSIO which can be embedded as a computational chain for parallel chain of Polkadot. Specialization on top of this makes the relay process more decentralized and cheaper.
