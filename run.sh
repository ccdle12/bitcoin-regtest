#!/bin/bash


. ./.env

$BITCOIN_PROJ_PATH/src/bitcoind -regtest -daemon
$BITCOIN_PROJ_PATH/src/bitcoind -regtest -datadir=./alice -daemon
$BITCOIN_PROJ_PATH/src/bitcoind -regtest -datadir=./bob -daemon


