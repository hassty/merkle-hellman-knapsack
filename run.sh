#!/bin/sh

bin=./target/release/knapsack
message='tihomirov timofey mihaylovich'
keyfile=knapsack.keys

printf "message: %s\n\n" "$message"

$bin keys
printf "\n"

private_key=$(awk 'NR == 1' $keyfile)
a=$(awk 'NR == 2' $keyfile)
n=$(awk 'NR == 3' $keyfile)
public_key=$(awk 'NR == 4' $keyfile)

printf "%12s %s\n" "private key:" "$private_key"
printf "%12s %d\n" "a:" "$a"
printf "%12s %d\n" "n:" "$n"
printf "%12s %s\n\n" "public key:" "$public_key"

encrypted=$($bin encrypt "$keyfile" "$message")
printf "encrypted: %s\n\n" "$encrypted"

decrypted=$($bin decrypt "$keyfile" $encrypted)
printf "decrypted: %s\n" "$decrypted"

rm $keyfile
