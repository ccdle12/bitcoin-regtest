#!/bin/bash

. ./.env

$BITCOIN_PROJ_PATH/src/bitcoin-cli -regtest -datadir=./alice $*
