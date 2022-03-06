:: This is the windows version of build_release_all.bash

cd blake2_bench
cargo build --release
cd ../blake3_bench
cargo build --release
cd ../fsb_bench
cargo build --release
cd ../gost94_bench
cargo build --release
cd ../groestl_bench
cargo build --release
cd ../md2_bench
cargo build --release
cd ../md4_bench
cargo build --release
cd ../md5_bench
cargo build --release
cd ../ripemd_bench
cargo build --release
cd ../sha1_bench
cargo build --release
cd ../sha2_bench
cargo build --release
cd ../sha3_bench
cargo build --release
cd ../shabal_bench
cargo build --release
cd ../sm3_bench
cargo build --release
cd ../streebog_bench
cargo build --release
cd ../tiger_bench
cargo build --release
cd ../whirlpool_bench
cargo build --release
cd ..
