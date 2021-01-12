# Hamming Coding Playground

A Hamming Coding playground built in WebAssembly using Rust and [Yew](https://yew.rs/docs/en/).

## Demo
Play with the demo over at [https://tmw.github.io/hamming-wasm/](https://tmw.github.io/hamming-wasm/)

## Help

Text typed in the top left textarea will be converted and displayed in bits using Hamming Coding. These bits are then decoded and shown as text again in the bottom left area. Introducing noise (corrupting) bits in the right view, will impact the text being outputted on the bottom left. Clicking any of the bits will toggle the bit's value and _likely_ corrupting the output text.

Hamming Code is designed to work in blocks of 16 bits. 11 data bits and 4 parity bits. The bit with index 0 could be assigned a overall parity bit, but in this demo we haven't.

Hamming Code will be able to detect and fix bit errors on a per block basis with a maximum of one error per block. Try it out for yourself; what happens if you flip multiple bits in a block?

**Corrupt**

By clicking the `Corrupt` button we're randomly corrupting (flipping) at most one bit per block. Some blocks will not be affected at all. It should never flip multiple bits in a block.

**Repair**

The `Repair` button will invoke the repair mechanism that will repair the blocks to their original state following the parity bits of that block.

**Repair**

Hitting `Reset` will take the original input string and encode it again. It'll truly act as a reset in case the string got too corrupted :)

**Sequential / Blocks**

The `Sequential` button will toggle between `Blocks` mode (default rendering) and sequential. I've added a `Blocks` mode to make it easier to distinguish the various blocks, and later on decided that should be the default. The `Sequential` just renders all the bits in a long string, as they would also appear in a file or over the wire.

## License
[MIT](LICENSE)
