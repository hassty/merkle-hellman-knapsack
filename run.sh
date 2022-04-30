#!/bin/sh

bin=./target/release/knapsack
message='tihomirov timofey mihaylovich'
keys_file=generated.keys

printf "message: %s\n\n" "$message"

$bin keys >$keys_file
private_key=$(awk 'NR == 1' $keys_file)
a=$(awk 'NR == 2' $keys_file | cut -d' ' -f1)
n=$(awk 'NR == 2' $keys_file | cut -d' ' -f2)
public_key=$(awk 'NR == 3' $keys_file)

printf "%12s %s\n" "private key:" "$private_key"
printf "%12s %d\n" "a:" "$a"
printf "%12s %d\n" "n:" "$n"
printf "%12s %s\n\n" "public key:" "$public_key"

encrypted=$($bin encrypt $public_key "$message")
printf "encrypted: %s\n\n" "$encrypted"

decrypted=$($bin decrypt -a "$a" -n "$n" $private_key -- $encrypted)
printf "decrypted: %s\n" "$decrypted"

rm $keys_file
