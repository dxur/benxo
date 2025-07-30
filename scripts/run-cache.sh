#!/usr/bin/env bash

getpath() {
  TARGET="$1"
  [ -z "$TARGET" ] && return 1

  cd "$(dirname "$TARGET")" || return 1
  echo "$(pwd -P)/$(basename "$TARGET")"
}

mkdir -p /tmp/varnish

varnishd -a :6080 -f $(getpath "./Cache.vcl") -s malloc,256M -s file,/tmp/varnish/cache.bin,1G -n /tmp/varnish -F
