# Hamming Coding Playground

A Hamming Coding playground built in WebAssembly using Rust and Yew.

## Help

Typing text in the textarea on the left will cause bits to show up on the right side and the (same) text to show up on the bottom left.

Clicking any of the bits will toggle the bit from high to low, likely corrupting the output text.

Hamming Code is designed to work in blocks of 16 bits. 11 data bits and 4 parity bits. The bit with index 0 could be assigned a overall parity bit, but in this demo we haven't.

Hamming Code will be able to detect and fix bit errors on a per block basis with a maximum of one error per block. Try it out for yourself; what happens if you flip multiple bits in a block?

By taping the `Corrupt` button we're randomly corrupting (flipping) at most one bit per block. Some blocks will not be affected at all. It should never flip multiple bits in a block.

The `Repair` button will invoke the repair mechanism that will repair the blocks to their original state following the parity bits of that block.

Hitting `Reset` will take the original input string and encode it again. It'll truly act as a reset in case the string got too corrupted :)

The `Sequential` button will toggle between `Blocks` mode (default rendering) and sequential. I've added a `Blocks` mode to make it easier to distinguish the various blocks, and later on decided that should be the default. The `Sequential` just renders all the bits in a long string, as they would also appear in a file or over the wire.

## License
[MIT](LICENSE)
