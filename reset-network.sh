#!/bin/bash

# Import common functions.
. ./common.sh --source-only

source_env

rm -rf alice/regtest/
rm -rf bob/regtest/

./alice.sh stop
./bob.sh stop
