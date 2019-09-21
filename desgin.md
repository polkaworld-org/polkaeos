# Polkaeos: A Polkadot-EOSIO Cross-Chain Bridge Implementation without Trust Loss

Fan Yang fanyang.coder@gmail.com

> As I have less time to make a totel english version of polkaros doc, so most of this doc was make by google translate and some changes, it will make lots of mistake

## 0. Preface

Cross-Chain Bridge is one of the most important part in Polkadot. In Polkadot whitepaper proposed a design to implement cross-chain from polkadot to ETH or Bitcoin. In this document propose the `Polkaeos` which is a cross-chain implementation between polkadot and EOSIO without trust loss. At present, EOSIO is one of the main technologies of block chain, so it is of great value to build a cross-chain bridge between Polkadot and EOSIO.

In order to run DAPP with high performance and low latency, EOSIO has made a lot of design choices. On the one hand, this makes EOSIO landing quickly, but on the other hand, it also brings a great degree of centralization problems. At the same time, the features of EOSIO make it very difficult to cross-chain with it: EOSIO node deployment is expensive. Comparing with other public chains, the computing in EOSIO is very large. All of this will make a bridge just designed like polkadot-ETH cannot do work well in polkadot-EOSIO.

Polkaeos achieves a cross-chain Bridge Based on the features and functions of Polkadot and EOSIO. The following is the specific design scheme.

## 1. Design goals

As a bridge chain that implement interoperates before two independent blockchain systems, Polkaeos should be decentralized as much as possible, and its trustworthy assumption should not add new hypotheses on the trust hypothesis of the two blockchains. .

At the same time, in Polkaeos, nodes as different roles should not have too high threshold, including the least value of the mortgage token, the stability of the node, the performance and the network bandwidth requirements, etc, which can avoid the centralization of the real structure of the Polkaeos.  The appropriate threshold here should be to allow participants of Polkaeos and EOSIO, that is, to have certain Polkaeos tokens and EOSIO core tokens, and to have EOSIO accounts with deployment contract capabilities. The requirements for nodes should be close to just have network links and a computer with average performancer, and its network bandwidth requirements should be able to meet the ability to simultaneously accept and send Polkaeos and EOSIO real-time block information.

Since both Polkaeos and EOSIO are public chain with the support to Turing complete contracts, the above requirements should be the minimum requirements for any relay node in a polkadot-EOSIO bridge which without witness nodes.

From the perspective of Polkadot, Polkaeos will be designed as a stand-alone module instead of a single parallel chain, which will allow the parallel chain of requirements in polkadot to quickly interoperate with the EOSIO chain. For this reason, the cross-chain implementation of Polkaeos should be consensus-independent and should not occupy excessive computational and storage resources on the chain.

## 2. From EOSIO to polkadot

The first thing Polkaeos needs to solve is to make a trust mapping from EOSIO status to Polkaeos.

The state mapping scheme from ETH to polkadot is given in the polkadot white paper, but for EOSIO to polkadot this scheme needs to be improved and specialized for EOSIO features.

First, the state mapping from EOSIO to polkadot in Polkaeos needs to be strictly guaranteed to be credible, so solutions based on a small number of verifiers and phishers are not very effective, and also note that the number of EOSIO blocks is relatively high. There are a lot of blocks, which means that once a few verifiers do evil, even if the phishers find that the chain is penalized, it will be difficult to re-synchronize the state with EOSIO.

Second, EOSIO is different from the chain that uses the PoW consensus mechanism. Its block will be final, which is, the irreversible block in EOSIO. For Polkaeos, to ensure the trust of the block, its nodes can only Synchronous irreversible blocks, and the decision to irreversible needs to be verified based on the EOSIO consensus mechanism, that is, all nodes participating in the verification need to complete the EOSIO consensus. If there is no targeted technology, this will raise the threshold for participation.

Third, EOSIO provides a rich set of contract functions, and Polkaeos can more easily implement certain assertions in the relay process based on EOSIO contracts.

The above means that Polkaeos needs to do a lot of work to better support a trust state mapping from EOSIO to polkadot.

### 2.3 EOSIO light node

One of the important tasks of Polkaeos is to implement the EOSIO light node. Here, in order to ensure the trust of the chain, the EOSIO light node needs to be as more as lightweight.

In the EOSIO network, a full node has very high requirements for machine performance, network, operation and maintenance, which means that even a small number of verifiers and phishers are required to start the full node, which is also unrealistic. The entire bridge is not cost-effective, so Polkaeos needs a usable EOSIO light node. The EOSIO white paper describes that the light node is technically feasible, but there is currently no available implementation in this area, so Polkaeos needs to implement a EOSIO light node by itself.

One of the main reasons for EOSIO's high demand for nodes is that even though its memory (RAM) state is very large and not verified in the block, any node that wants to get any state needs to replay the EOSIO block completely. Although the node can be restored by the snapshot function, but for a node that is synchronized, the recovery time is long if there is a failure. At the same time, due to the busy transactions in the EOSIO chain -- the peak time can reach 4000 TPS, and the synchronous real-time EOSIO block will also have higher requirements on the machine.

Based on the above considerations, the EOSIO light node implemented by Polkaeos will not process every transaction, but only verify the validity of the block header and confirm that its block is irreversible.

Not processing every transaction means that most of the work of the light node can be done in parallel, and synchronization can also be started from any block height. In the current implementation of the Polkaeos prototype, the light node can complete the verification with only a small amount of computing resources.

Not processing each transaction allows the light node to take up very little resources, but it also makes the light node unable to provide the memory (RAM) state of EOSIO, while in EOSIO almost all states are in memory, some operations (Action in EOSIO) also needs to be triggered by the execution of the transaction, which requires some assertion mechanism to ensure that Polkaeos can obtain the state of EOSIO only based on the block data. In Polkaeos, the EOSIO state assertion contract and state static assertion contract are realized. 

### 2.4 EOSIO State Assertion Contract

Polkaeos maps the state of EOSIO, which only need to determine the value of a certain state at a certain block height, or can determine the value of a certain state within the interval based on the transaction within a block height interval.

To this end, Polkaeos proposed a state assertion contract and a state static assertion contract, through which the two purposes are achieved.

The state assertion contract is very simple. The contract is only to judge whether the value submitted by the action is consistent with the current memory value. As long as the action is successful and the block containing this action(in a transaction) reaches an irreversible state, the assertion can be considered valid.

The state assertion contract can adapt to any arbitrary EOSIO state structure. To ensure trusted, this contract account should be a black hole account, the contract Hash of its constract code should be confirmed to unchanged, so that unless there are more than 2/3 of block producer(super node) in the EOSIO chain tampers with the contract itself or its results, otherwise the contract can be considered credible.

Even if more than two-thirds of the block producer(super node) in the EOSIO chain tamper with the contract, Polkaeos can detect contract hash changes or find a fork of chain, which can turn off the relay process to ensure Polkaeos security.

### 2.5 EOSIO Status Static Assertion Contract

In addition to determining the value of a state at a certain block height, Polkaeos also needs to determine the value of a state within the interval based on transactions within a block height interval.

For any state, we only need to know the value of its initial state and all state transitions to get the current state. For the state on EOSIO, the contract can be used to make the state not be changed by the behavior outside the transaction in the block (Mainly inline action), then this state can be defined as "static state", which means that if a state can be confirmed as a static state, then the state is determined to be within the interval according to the transaction within a block height interval Value.

After several features of EOSIO v1.8 are activated, it is easy to ensure that a state is static, as long as the state limits its changing behavior and does not accept the inline action (this also includes the event call that is about to be abolished Action -- the underlying implementation of the two is actually the same) make these behaviors.

The inability to use the inline action may limit the functionality of some contracts. This can be a simple bypass, that is, using a secondary acknowledgment mechanism, by storing two associated states, and performing a confirmatory synchronization through a synchronous action.

State assertion contracts and state static assertion contracts consume very little resources, except for their own RAM consumption, which does not add extra RAM consumption. In addition, these two contracts are hardly calculated, so CPU consumption is also very small, NET consumption and To confirm the size of the state, for larger states, such as some important files in RAM, we can use the method of asserting its hash.

State assertion contracts and state static assertion contracts allow Polkaeos to complete state mapping from EOSIO to polkadot based on block data only, with minimal resource consumption. In fact, this can be considered zero resource consumption in EOSIO.  The above two contracts actually divide the state in EOSIO into two categories: one is "static confirmation" and the other is "calculation confirmation". The state of static confirmation can be mapped not only to Polkadot by Polkaeos. It can also be used by many other applications and cross-chain scenarios.

### 2.6 EOSIO Block Cropping

Through the above mechanism, the state mapping from EOSIO to polkadot can be well realized, and the requirements of the participants are very low, but the participating nodes still need to synchronize the complete EOSIO real-time data, which is relatively large, although if it is to be guaranteed The credibility of the cross-chain process, then all EOSIO block header data must be verified, but in some specific cases, we can make some specializations to optimize.

The EOSIO side can use specific implementations to tailor and filter blocks, such as defining some "critical" blocks in a block. All transactions involving cross-chain or static states can be packaged into only these critical blocks. The key block can be selected to embed the verification information of its last key block, which is equivalent to filtering out the chain in EOSIO. The above process can be completely based on contract implementation, but the chain needs to give certain contract API support. To confirm the critical blocks, EOSIO is currently working on adding these features after v1.8.

After filtering, the nodes in Polkaeos only need to synchronize the contents of the EOSIO block header and the key block, which will greatly reduce the amount of data that needs to be synchronized.

For some chains based on EOSIO technology, this effect can also be achieved through the modification of the chain itself. Relative to the contract implementation, the changes based on the chain itself will be more effective.

## 3. From polkadot to EOSIO

EOSIO is based on the DPOS mechanism, so it is difficult to complete the state mapping from polkadot to EOSIO through the same mechanism described above. After the trusted state mapping from EOSIO to polkadot, Polkaeos can check whether the state mapping from polkadot to EOSIO is trustworthy. This state mapping from polkadot to EOSIO can be done by any individual.

### 3.1 Polkaeos state mapping proof

We can think of the state map from polkadot to EOSIO as two associated state transitions:

For example, if the $p$ state in polkadot corresponds to the $p'$ state in EOSIO, then the mapping process can be seen as the following two transformations: $\vec p$ and $\vec p'$ :

$$p = s \implies p' = s$$

Another common token transfer process from polkadot to EOSIO can be recorded as the following two transformations:

$$a\overset{n,a^T}{\to}N \implies N'\overset{n}{\to}a^T$$

Obviously, the two transformations are one-to-one correspondence. In fact, when need to map state to EOSIO,it only need to change the state of polkadot to get the EOSIO state transformation, and calculate its hash, push the hash to EOSIO, and then execute it on the EOSIO side. The result can into the polladot side by statically asserting the contract during the transformation. This process is called the Polkaeos state mapping proof.

The Polkaeos state map proof  will contain a stream id information and the block height, and adds a check code. Although the message size is increased to some extent, it will significantly prevent the erroneous operation caused by the synchronization problem when the operation is performed.

### 3.2 Relay "miners"

Polkaeos state mapping proof can be done by any node participating in Polkaeos. All that is required is a small portion of the deposit. These nodes or individuals are called relay “miners”, after each time the miner completes the certificate, he will get a fee, and each proof of waiting for delivery can be set an extra fee to complete faster.

Since the mapping process only involves the hash value, the consumption of each mapping is actually fixed, which means that the cost of the miner is fixed, so there can be a large number of miners in Polkaeos. so for the miners, It is not required to guarantee stability, and at the same time, in designing the contract for transmitting information, it will considered that the misuse of network, synchronization and calculation amount may prevent the miner from being punished for unexpected errors. Only when a miner maliciously delivers a mapping that does not exist at all is punished.

The miner needs to pay the deposit at Polkaeos. The miner cannot perform other status mapping certificates until the status map certification confirmation is completed. This interval is actually fixed and can be completed by the contract on both sides. It should be noted that if the miner pays more than the deposit, it can actually be allowed to complete multiple state mapping certificates at one time, but this is equivalent to the fact that some individuals register several miners, so they can be specially designed.

In order to ensure that the miners are credible, and in order to ensure that the lower limit of the miners' deposit is not too high to limit the participation of users, the value of the token involved in each state mapping should have an upper limit. Most state mappings have certain value and can be defined as miners. The maximum value that can be obtained by do evil, obviously the miner's deposit needs to be greater than this value, so in order to ensure that the miner's deposit limit is not too high to limit user participation, the relay contract can limit this maximum value, more typical the process of simply setting a single upper limit can achieve this.

Because the miners are involved in the behavior of the two chains, they need to confirm their status on both chains. The registration process is as follows:

- First, a miner should temporarily mortgage the Token on EOSIO to complete the registration on the EOSIO side.
- After the state is mapped to Polkaeos, the miner deposit is mortgaged at Polkaeos and the mapping authority is obtained.
- At this point its first state mapping is to be the proves its miner's deposit. This state mapping proves that an unlock block height will be set for its temporarily mortgage token on EOSIO side.
- (Optional) Before this block height, other miners can as a fisherman to check if the miner has forged the deposit status. If so, the new miner's deposit on the EOSIO side can be obtained by another status mapping.
- (Optional) If the miner does not initiate another certificate at Polkaeos to mortgage his miner's deposit, it is considered to be in a forged state and other miners can obtain the deposit on the EOSIO side as a fisherman.
- If there is no miner to prove the status of the forged deposit, it can be considered that the miner has completed its status confirmation, at which point the miner can understand the temporary deposit on the EOSIO side after locking the height.

The slightly more complicated process above is to ensure that the miner's registration process is not interfered by other miners. In particular, the first miner of Polkaeos can complete the initial state mapping by itself, although as there are no other miners at this time. The freeze on the temporary deposit of the miners at EOSIO will be longer.

Since the mapping from EOSIO to polkadot is credible, the miner registration process should begin on the EOSIO side, and each walk is protected by a chain mechanism on both sides.

The above design ensures that the miners are completely decentralized and market-oriented. The miners cannot be monopolized and have no threshold. When the relay channel is congested with the development of Polkaeos, new miners will inevitably join to obtain the formalities fees, when the trunk channel of Polkaeos is idle, the miners can also quit according to their own circumstances.


## 4. Application

A large number of applications can be realized through the cross-chain bridge between the EOSIO chain. EOSIO makes many compromises to carry high-performance and low-latency DAPP, which makes EOSIO show a centralization trend to some extent. through Polkaeos, The polkadot ecosystem can guarantee decentralization and high performance and low latency DAPP applications under certain scenarios.

On the other hand, EOSIO-based chains can be used to map their states to polkadot-based parallel chains by Polkaeos, thereby enhancing their degree of decentralization.

Here are a few possible applications based on the Polkaeos:

### 4.1 Value transfer channel between Polkadot and EOSIO chain

The most typical application is the value transfer channel between Polkadot and the EOSIO chain, which is to transfer the two-way pass to the peer, which can support some of the token-based applications, such as decentralized exchanges, Defi applications.

### 4.2 Cross-chain contract

Based on Polkaeos, cross-chain smart contracts can be implemented. Both Polkadot and EOSIO support complex smart contracts. Through the cross-chain mechanism, one DAPP can be run on two or more heterogeneous chains at the same time.

For homogeneous parallel chains, such cross-chain contracts are of little significance, but for heterogeneous chains, especially in EOSIO, implementing cross-chain contracts allows contracts or DAPPs to choose to run on these two chains based on their needs. 

For example, some important settlements (such as generating cryptographically secure random numbers) can be done in the parallel chain of Polkadot, and some high computational logic can be run in EOSIO.

### 4.3 Computational subchain of Polkadot parallel chain

Based on Polkaeos, a computational subchain based on the EOSIO implementation can be added to the parallel chain of Polkadot, similar to the usual Layer 2 architecture.

The EOSIO chain, which is a computational sub-chain, can be deeply modified and customized to incorporate itself into the governance mechanism of the parallel chain, so that the Polkaeos solution can be simplified, so that the cross-chain process can be completed more quickly.

### 4.4 EOSIO Status Proof

EOSIO-based chains can also be further improved through Polkaeos. Chains based on EOSIO tend to be strongly centralized. The state of these chains can be tampered in some cases. Based on Polkaeos, EOSIO users can map their status to The Polkadot parallel chain acts as a deposit to prevent anomalies in certain situations.

## 5. Summary

Polkadot and EOSIO are both important blockchain technologies at the moment. The interoperability between the two can bring more functionality to each other, thus supporting more complex decentralized applications.

In addition to implementing the polkadot-EOSIO cross-chain bridge, the Polkaeos scheme can also be referenced to implement the cross-chain mechanism between the high-performance chains.
