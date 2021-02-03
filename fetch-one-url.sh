#!/bin/sh

set -e

binary_to_invoke="$1"
url="$2"

timeout --kill-after=70s 60s "$binary_to_invoke" "$url" 2>&1 > "$url"
exit_code="$?"
echo "Exit code: $exit_code" >> "$url"
if ["$exit_code" -ne 0]; then
    mv "$url" 'success'/"$url"
else
    mv "$url" 'failure'/"$url"
fi
