#!/bin/bash

# Source the env file to use the env variable.
. ./.env

cd $BITCOIN_PROJ_PATH 
make clean && ./autogen.sh && ./configure --without-gui && make
