# Regtest Helper

This project helps run regtest.

There are certain assumptions... 

* You have Bitcoin already cloned on your machine
* Update the .env file with the path of your Bitcoin folder

## Compiling Bitcoin

* After updating `.env` with the path to your Bitcoin folder
* Run `./install.sh`

## Setting up the regtest

* `./setup-nodes.sh` will set up the network with `alice` + `bob`

* run  `$ bitcoin-cli -regtest getpeerinfo`

## Resetting the network

* `reset-network.sh` will reset the regtest chain state to 0.
* `setup-nodes.sh` to run the nodes again.

## Generate address for alice

```
$ ./alice.sh getnewaddress
$ 2N3St5nSMEKJ4PNq5eNAzjPvJjYQigHyhVg
```

* Check balance

```
$ ./alice.sh getnewaddress
```

* Generate 200 blocks

```
$ ./alice.sh generatetoaddress 200 2N3St5nSMEKJ4PNq5eNAzjPvJjYQigHyhVg 
```

* Generate an address for Bob

```
$ ./bob.sh getnewaddress
$ 2NDMYM2i9cTj2nd9z7cXhY5q9uMfYsvz41w
```

* Send coins from Alice to Bob

Returns a tx hash
```
$ ./alice.sh sendtoaddress 2NDMYM2i9cTj2nd9z7cXhY5q9uMfYsvz41w 10
$ ca5883432e0c6e6660c47707acd66d17b95ef2eda3d7a2890d9ebbca3c8767f4
```

* Check Bobs list of unspent transactions

We are looking for 0 confirmation transactions.

```
$ ./bob.sh listunspent 0
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
