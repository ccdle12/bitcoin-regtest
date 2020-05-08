# Regtest Helper

This project helps run regtest.

There are certain assumptions... 

* You have Bitcoin already cloned on your machine
* You have already compiled Bitcoin
* [Linux guide to compiling Bitcoin](https://gist.github.com/jonatack/9f57d398656433de5a5e04d5e0e4447e)

## Build the CLI

- Build the CLI:

```
$ cd ./cli && cargo build
```

- Move the binary to preferred location (OPTIONAL)

## Set environment variables

```
BITCOIN_PROJ_PATH=<path-to-bitcoin-folder>
REGTEST_ALICE=<path-to-alice-folder>
REGTEST_BOB=<path-to-bob-folder>
```

## Setting up the regtest

- `$ regtest-cli setupnodes`

## Resetting the network

- `$ regtest resetnetwork`
- `$ regtest setupnodes`

## RPC Commands

- [List of Bitcoin RPC Calls](https://en.bitcoin.it/wiki/Original_Bitcoin_client/API_calls_list)

```
$ regtest-cli alice --rpc getnewaddress
$ 2N3St5nSMEKJ4PNq5eNAzjPvJjYQigHyhVg
```

- Check balance

```
$ regtest-cli alice --rpc getbalance
```

- Generate 200 blocks

```
$ regtest-cli alice --rpc generatetoaddress 200 2N3St5nSMEKJ4PNq5eNAzjPvJjYQigHyhVg 
```

- Generate an address for Bob

```
$ regtest-cli bob --rpc getnewaddress
$ 2NDMYM2i9cTj2nd9z7cXhY5q9uMfYsvz41w
```

- Send coins from Alice to Bob

```
$ regtest-cli alice --rpc sendtoaddress 2NDMYM2i9cTj2nd9z7cXhY5q9uMfYsvz41w 10
$ ca5883432e0c6e6660c47707acd66d17b95ef2eda3d7a2890d9ebbca3c8767f4
```

* Check Bobs list of unspent transactions

We are looking for 0 confirmation transactions.

```
$ regtest-cli bob --rpc listunspent 0
$ [
  {
    "txid": "ca5883432e0c6e6660c47707acd66d17b95ef2eda3d7a2890d9ebbca3c8767f4",
    "vout": 1,
    "address": "2NDMYM2i9cTj2nd9z7cXhY5q9uMfYsvz41w",
    "label": "",
    "redeemScript": "0014564473116e43f69ec7814aafe44230748860a53c",
    "scriptPubKey": "a914dc9455ef0ae6e66538047ecde60cd2e5185770a287",
    "amount": 10.00000000,
    "confirmations": 0,
    "spendable": true,
    "solvable": true,
    "desc": "sh(wpkh([1f702f1c/0'/0'/0']03c54d523849c41b153b97d68f61d0f852216ffd20bfccbb9c215342d8f757514e))#5wwaxhwa",
    "safe": false
  }
]
```
