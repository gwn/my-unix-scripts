#!/bin/sh

# lightweight REST client
# Define $rshost and optionally $rstoken (for protected endpoints) to start!
# $rstoken is the value of the "Authorization" header. If you use a different
# header for this, change the script!

if test -z "$rshost"; then
    echo '$rshost must be defined in the environment'
    exit 1
fi

if test -z $(which jq); then
    echo 'Please install jq'
    exit 2
fi

path=$1; shift
curlParams="$@"

response=$(
    curl -i $rshost$path \
        -H "Authorization: $rstoken" \
        -H 'Content-type: application/json' \
        "$curlParams"
)

statusCodeFirstChar=$(echo $response | head -n 1 | cut -d' ' -f2 | cut -c 1)

sepLineNo=$(echo "$response" | tr -d '\r' | grep -n ^$ | cut -d: -f1)
# empty line that separates the HTTP Headers from the body

body=$(echo "$response" | tail -n +$(expr $sepLineNo + 1))

if test $statusCodeFirstChar -eq 2; then
    if test -t 1; then
        echo "$body" | jq -C . | less -RSFX
    else
        echo "$body" | jq .
    fi
else
    echo "$response"
fi
