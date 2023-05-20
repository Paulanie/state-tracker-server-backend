#!/usr/bin/env bash

set -euo pipefail

VERSION=${1:-latest}

docker login -u "pravez"
docker build --rm --no-cache -t pravez/statetracker:$VERSION .
docker push pravez/statetracker:$VERSION