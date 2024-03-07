## ChaCha20Poly1305

The extension of the ChaCha20 stream cipher to encrypt and decrypt messages with additional data.

#### Features

- [x] Stack allocated 
- [x] In-place decryption
- [x] Optional additional data

#### License

MIT

#### Usage

```rust
let key = hex::decode("85d6be7857556d337f4452fe42d506a80103808afb0db2fd4abff6af4149f51b").unwrap();
let key = key.as_slice();
let nonce = [0u8; 12];
let mut message = *b"Cryptographic Forum Research Group";
let cipher = ChaCha20Poly1305::new(key.try_into().unwrap(), nonce);
let mut buffer = [0u8; 50];
cipher.encrypt(message.as_mut_slice(), None, buffer.as_mut_slice());
```