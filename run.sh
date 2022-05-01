#!/bin/sh

bin=./target/release/knapsack
message='tihomirov timofey mihaylovich'

printf "message: %s\n\n" "$message"

$bin keys
printf "\n"

private_key=$(awk 'NR == 1' private.key)
a=$(awk 'NR == 2'  private.key)
n=$(awk 'NR == 3'  private.key)
public_key=$(cat public.key)

printf "%12s %s\n" "private key:" "$private_key"
printf "%12s %d\n" "a:" "$a"
printf "%12s %d\n" "n:" "$n"
printf "%12s %s\n\n" "public key:" "$public_key"

encrypted=$(echo "$message" | $bin encrypt)
printf "encrypted: %s\n\n" "$encrypted"

decrypted=$(echo "$encrypted" | $bin decrypt)
printf "decrypted: %s\n" "$decrypted"
