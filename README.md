# Introduction
The following algorithm is a hybrid between substitution and transposition ciphers, with multiple options to encrypt/decrypt files and possibility to extend functionality.
# Algorithm Description
## Structures (Simplified)
### Swapper
Simple structure, containing two 256 long byte arrays, which swaps and re-swaps bytes into another based on a provided key. The key is an array of 256 bytes, where every byte (except first) is lower or equal to its index. During the creation of swapper, an array of 256 bytes from 0 to 255 is created. After that, the created array swaps byte at n-th index with the byte at index which is specified by n-th index of the key, with the exception of the first one. An array for re-swapping is also created, so that *reswap_array[swap_array[n]] == n*, as it's faster to use reswap array compared to searching for correct byte in swap array.
### Cart
Cart is a structure, which contains an entrance and exit, which are bytes and seats which is an array of bytes. It has two methods, put and close. Put method takes a byte and returns a byte, between those two events it:
+ Takes the exit value and stores it for later return.
+ Moves value from entrance to exit.
+ Moves value from seats at index of exit to entrance.
+ Replaces moved value in seats with inputed byte.
+ Returns previous exit.
Close method doesn't take any input and returns current state of cart as one array, which contains exit, entrance and seats in this order.
### JEU (Joker Execution Unit)
