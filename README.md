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

![seats put](https://github.com/Kuaaro/Joker6/assets/120309479/229e7cdd-c214-47c5-b76c-d562184a950d)

Close method doesn't take any input and returns current state of cart as one array, which contains exit, entrance and seats in this order.
### JEU (Joker Execution Unit)
This structure combines structures mentioned above and is responsible for single round of encryption. There are two JEU types, encrypting and decrypting.
## Algorithm Descryption (Joker6)
Disclamer: Text inside `  ` is a new addition for 6th version of the algorithm.
1. Take the key from user and use SHAKE256 hash function on it and `save the output of size 256 bytes`.
2. `Copy and modify the array, so that all elements, except the first, are smaller or equal to their index`.
3. Create a Swapper with swap and reswap table, by creating an array with bytes from 0 to 255, then swap elements on nth index with element on index `specified by nth element of the key array, except element at index 0`. Create reswap table, so thet *reswap_table[swap_table[n]] == n for all possible byte values.
4. Create a Cart with random initial values.
5. For every byte from input file:

    A. Swap the byte with another from swap table `offseted to the right by the index of read byte in the file * the first value from the key created from hash`.

    B. Put it through the cart.

    C. Reswap the value, `once again offseted`, so that if the same value was swapped and reswapped, `with the same offset`, pre-swapped and after-reswapped values would be equal. (Done by using swap_array[(n+offset)%256] to swap and (reswap_array[n]-offset)%256, may be different depending if modulo converts negative values to positive in given language/architecture)

6. Close the cart and reswap values with the reswap table `and offset for every value`.
7. Repeat the points from one to six 15 more times (16 times total), but instead of using the hash provided by user, `use SHAKE256 to create a new hash from the previous one`, not the key used to create swap table.
## Differences between Joker5 and Joker6 and their reasons.
+ Using output of size 256 from SHAKE256 was done for several reasons:
  + Better for creating swap table: there is $256!$ possible permutations of 256 unique bytes, which 
