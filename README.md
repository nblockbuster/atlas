# atlas
[![dependency status](https://deps.rs/repo/github/nblockbuster/atlas/status.svg)](https://deps.rs/repo/github/nblockbuster/atlas)
[![Build Status](https://github.com/nblockbuster/atlas/workflows/CI/badge.svg)](https://github.com/nblockbuster/atlas/actions?workflow=CI)

[Download Latest Build](https://nightly.link/nblockbuster/atlas/workflows/rust/main)
---
Atlas is a tool I made in an afternoon to help me do RE work regarding hashed values.


## Hashers
Hashers use a plugin system where Atlas tries to find dynamic libraries in the `plugins` folder.

Atlas currently has the following core plugins included in this repo:
- FNV-0, FNV-1, FNV-1, from 32 to 1024 bits
- [MD2](https://crates.io/crates/md2)
- [MD4](https://crates.io/crates/md4)
- [MD5](https://crates.io/crates/md-5)
- [Murmur2](https://crates.io/crates/murmur2), 32 and 64 bit
- [Murmur3](https://crates.io/crates/murmur3), 32 and 128 bit
- [SipHash](https://crates.io/crates/siphasher) 1-3 and 2-4, 32 and 128 bit
- [Sha1](https://crates.io/crates/sha1)
- [Sha2](https://crates.io/crates/sha2), from 224 to 512 bits
- [Sha3](https://crates.io/crates/sha3), from 224 to 512 bits
- [XXHash](https://crates.io/crates/xxhash-rust), 32 and 64 bit
- [XXHash3](https://crates.io/crates/xxhash-rust), 64 and 128 bit

## Planned features

- [ ] Hash Reverser (FNV1-32)

Hashers:
- [ ] CRC32
- [ ] MD6
- [ ] Tiger
- [ ] Whirlpool


### ⚠️ Atlas does not support secrets (XXHash3), or keying (SipHash) yet!