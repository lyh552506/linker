#! /bin/bash

test_name=$(basename "$0" .sh)
# echo "$test_name"
# echo "$0"

target_pos=out/

mkdir -p "$target_pos"

cat<< EOF| riscv64-linux-gnu-gcc -o "$target_pos"/"$test_name".o -c -xc -
#include <stdio.h>

int main() {
    printf("Hello, World\n");
    return 0;
}
EOF

./target/debug/my_linker "$target_pos"/"$test_name".o
