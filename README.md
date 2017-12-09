# xorfinder

Given a binary needle and haystack, find if the needle occurs XORed anywhere within the haystack.

## Why?
To confirm that a far-fetched idea I had was, in fact, far-fetched.

Nothing useful has come of this tool :(. Tool itself works great though.

## Usage
`cargo build --release`

`xorfinder <needle> <haystack>`

## Example
```
$ xorfinder "DF15" "AB29F4E293F23802"
Found needle in haystack at offset 5 XOR 45
```
Offset 5: AB29F4E293**F238**02  
F238 XOR 45: DF15

## Legal
MIT license
