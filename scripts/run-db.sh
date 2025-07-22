#!/usr/bin/env bash

mkdir -p /tmp/mongodb

mongod --dbpath /tmp/mongodb --bind_ip 127.0.0.1 --port 27017 --noauth --replSet rs0
