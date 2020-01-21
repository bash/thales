#!/usr/bin/env bash

set -e

zip -0 thales.zip `find . \( -name 'thales*.gc*' \) -print`
grcov thales.zip \
       -t lcov \
       --llvm \
       --branch \
       --ignore-not-existing \
       --ignore '/*' \
       > lcov.info
