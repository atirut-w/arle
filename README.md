# Arle: Adaptive Run-Length Encoder
Arle is a compression and decompression tool inspired by [Adrien Soursou's implementation of Run-Length Encoding](https://github.com/ChuOkupai/rle-compression), where the compressed data is split into chunks prefixed with a single byte header indicating whether the chunk is compressed or not, and the length of the chunk.

The difference is that Arle uses a slightly different format for the header, where the most significant bit indicates if the chunk is a literal chunk (not compressed) or a compressed chunk, and the remaining 7 bits indicate the length of the chunk, and a chunk with a length of 0 is considered a termination chunk.
