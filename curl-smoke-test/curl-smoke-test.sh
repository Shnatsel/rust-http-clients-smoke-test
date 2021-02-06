#!/bin/sh

tempfile="$(mktemp)"
curl --connect-timeout 10 --speed-time 30 -s -v --location "$1" > "$tempfile" 2>&1
curl_exit_code="$?"
# only retain the first 16k of the output
head -c 16384 "$tempfile"
rm "$tempfile"
exit "$curl_exit_code"