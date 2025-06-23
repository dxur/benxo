#!/bin/bash

sudo mongod --dbpath /var/lib/mongodb --bind_ip 127.0.0.1 --port 27017 --noauth --replSet rs0
