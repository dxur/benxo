#!/usr/bin/env bash

sudo -v || exit 1

( while true; do sudo -v; sleep 60; done ) &
KEEPALIVE_PID=$!

trap "kill $KEEPALIVE_PID" EXIT

process-compose up --keep-project --no-server
