#!/bin/bash

# Import common functions.
. ./common.sh --source-only

source_env

$BITCOIN_D -regtest -daemon
$BITCOIN_D -regtest -datadir=./alice -daemon
$BITCOIN_D -regtest -datadir=./bob -daemon
