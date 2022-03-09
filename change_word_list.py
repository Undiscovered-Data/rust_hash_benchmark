#!/usr/bin/env python3

import os

dir_list = os.listdir('.')
print("The files")
for in_file in dir_list:
    if ".txt" in in_file:
        the_size = os.path.getsize(in_file)
        print("{}  has  {}  byles.".format(in_file, the_size))

file_name = input("Enter the file you want to test:  ")
front = "    let vec_o_strings = file_to_vec(\"../"
back = "\");\n"
new_line = front + file_name + back

file_dir = ('blake2_bench', 'blake3_bench', 'fsb_bench', 'gost94_bench',
        'groestl_bench', 'md2_bench', 'md4_bench', 'md5_bench', 'ripemd_bench',
        'sha1_bench', 'sha2_bench', 'sha3_bench', 'shabal_bench', 'sm3_bench',
        'streebog_bench', 'tiger_bench', 'whirlpool_bench')

file_path = []
for a in file_dir:
    b = a + "/src/main.rs"
    file_path.append(b)

for the_file in file_path:
    f_list = []
    open_file1 = open(the_file, 'r')
    for line in open_file1:
        f_list.append(line)
    open_file1.close()

    w_list = []
    for item in f_list:
        if "vec_o_strings = file_to_vec" in item:
            w_list.append(new_line)
        else:
            w_list.append(item)

    open_file2 = open(the_file, 'w')
    for item2 in w_list:
        open_file2.write(item2)
    open_file2.close()

