#!/bin/bash

# . ./.env

# $BITCOIN_PROJ_PATH/src/bitcoin-cli -regtest -datadir=./alice $*

# Import common functions.
. ./common.sh --source-only

source_env

$BITCOIN_CLI -regtest -datadir=./alice $*
