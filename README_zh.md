# Polkaeos

- [Polkaeos](#polkaeos)
  - [1. 介绍](#1-%e4%bb%8b%e7%bb%8d)
  - [2. Getting Start](#2-getting-start)
  - [3. TODOs](#3-todos)

## 1. 介绍

`Polkaeos`是一种无信任损失的polkadot-EOSIO跨链桥实现，其设计方案可以参见：

- zh : [Polkaeos: 一种无信任损失的polkadot-EOSIO跨链桥实现](https://github.com/polkaeos/polkaeos/blob/demo/desgin_zh.md)
- en : [Polkaeos: A Polkadot-EOSIO Cross-Chain Bridge Implementation without Trust Loss](https://github.com/polkaeos/polkaeos/blob/demo/desgin.md)

## 2. Getting Start

目前`Polkaeos`仅仅是一个原型实现，为了简化开发，目前的实现仅仅是用来验证`Polkaeos`的可行性。

`Polkaeos`需要一个EOSIO轻节点实现以辅助同步EOSIO网络，由于EOSIO生态在rust实现上支持不足，所以目前的实现是采用基于golang编写的简单的轻节点与EOSIO网络连接，通过zeromq与基于substrate的polkaeos节点共享EOSIO区块信息。

**安装libzmq**

OSX:

```bash
brew install libsodium zeromq czmq
```

**编译EOSIO轻节点**

首先需要配置golang编译环境.

编译：

```bash
git clone https://github.com/fanyang1988/eos-light-node.git
cd eos-light-node
go mod vendor
go build
```

**启动EOSIO测试网**

由于目前EOSIO测试网连接并不稳定，且不易用，所以需要启动测试网来进行中继测试。

> 当前EOSIO版本（v1.8.x）启动测试网脚本不可用，需要使用修改过的版本

编译并安装eosio.cdt

```bash
git clone https://github.com/EOSIO/eosio.cdt.git
cd eosio.cdt
git submodule update --init --recursive
./build.sh
sudo ./install.sh
```

编译eos

```bash
git clone https://github.com/fanyang1988/eos.git -b fix-bios-boot-tutorial
cd eos
git submodule update --init --recursive
cd scripts
./eosio_build.sh -s EOS -y
./eosio_install.sh
```

编译eosio系统合约

```bash
git clone https://github.com/EOSIO/eosio.contracts.git
cd eosio.contracts
./build.sh
```

基于EOS项目中的脚本启动测试网, 脚本基于python3, 需要安装numpy

```bash
cd eos
cd tutorials/bios-boot-tutorial
./bios-boot-tutorial.py --symbol EOS -a --contracts-dir /path/to/eosio.contracts/build/contracts
```

启动时间较长, 需要等待.

**编译Polkaeos**

目前的Polkaeos为了简化编码,对substrate进行了一部分硬编码,使用node-template启动节点:

这里的构建方式与substrate相同, 所以略去init部分

```bash
git clone https://github.com/polkaeos/polkaeos.git
cd polkaeos
cargo build
cd node-template/
./scripts/build.sh
cargo build
```

**启动**

```bash
./target/debug/node-template --dev --base-path /tmp/tmppath
```

此时可以接收EOSIO区块.

## 3. TODOs

Polkaeos目前仅仅处于原型demo状态,仅仅展示Polkaeos的原理,其完整实现与当前将会完全不同,因此Polkaeos还需要大量的工作:

首先完成在Polkaeos节点原生的EOSIO区块同步与验证,在Demo中为了简化开发,使用的是外部独立进程的验证.
由于目前EOSIO生态对rust支持较差,而EOSIO中使用了大量其独立设计的基础数据结构与算法,所以Polkaeos需要基于rust独立完成相关的工作.

其次是Polkaeos作为一条中继桥的链的开发工作,当前demo的实现中打破了很多substrate的抽象,在正式的Polkaeos中,将完全不存在这些实现,为此需要进行一部分工作.

最后Polkaeos需要很多EOSIO侧的合约开发工作,尤其是中继矿工节点的支持,由于这部分比较独立,所以开发量并不会很大.

在Polkaeos未来短期的开发规划中(三个月左右), 将会完成上述的第一三点,并完成一条类似[ChainX](https://github.com/chainx-org/ChainX)的主要面向Token类状态中继的桥实现.

在Polkaeos的中期规划中,主要针对以下三个方向:

- 增强Polkaeos的模块化以使得Polkaeos可以简单的嵌入到有需要和EOSIO链建立互操作的平行链中.
- 更好的支持任意类型状态与事件的映射.
- 基于EOSIO技术实现一条可以作为计算链嵌入Polkadot平行链的链,在此之上进行特化使得中继的过程更加去中心化和廉价.
