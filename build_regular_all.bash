#!/bin/bash

cd blake2_bench
cargo build
cd ../blake3_bench
cargo build
cd ../fsb_bench
cargo build
cd ../gost94_bench
cargo build
cd ../groestl_bench
cargo build
cd ../md2_bench
cargo build
cd ../md4_bench
cargo build
cd ../md5_bench
cargo build
cd ../ripemd_bench
cargo build
cd ../sha1_bench
cargo build
cd ../sha2_bench
cargo build
cd ../sha3_bench
cargo build
cd ../shabal_bench
cargo build
cd ../sm3_bench
cargo build
cd ../streebog_bench
cargo build
cd ../tiger_bench
cargo build
cd ../whirlpool_bench
cargo build

