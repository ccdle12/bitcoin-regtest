#!/bin/bash

# Import common functions.
. ./common.sh --source-only

source_env

cd $BITCOIN_PROJ_PATH 
make clean && ./autogen.sh && ./configure --without-gui && make
