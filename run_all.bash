#!/bin/bash

cd blake2_bench
cargo run
cd ../blake3_bench
cargo run
cd ../fsb_bench
cargo run
cd ../gost94_bench
cargo run
cd ../groestl_bench
cargo run
cd ../md2_bench
cargo run
cd ../md4_bench
cargo run
cd ../md5_bench
cargo run
cd ../ripemd_bench
cargo run
cd ../sha1_bench
cargo run
cd ../sha2_bench
cargo run
cd ../sha3_bench
cargo run
cd ../shabal_bench
cargo run
cd ../sm3_bench
cargo run
cd ../streebog_bench
cargo run
cd ../tiger_bench
cargo run
cd ../whirlpool_bench
cargo run

