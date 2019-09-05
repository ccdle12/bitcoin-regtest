#!/bin/bash

# Import common functions.
. ./common.sh --source-only

source_env

# cd ./alice && rm -rf ./regtest/
# cd ../bob && rm -rf ./regtest/
# cd ../

rm -rf alice/regtest/
rm -rf bob/regtest/

./alice.sh stop
./bob.sh stop
