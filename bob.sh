#!/bin/bash

# Import common functions.
. ./common.sh --source-only

source_env

$BITCOIN_CLI -regtest -datadir=./bob $*
