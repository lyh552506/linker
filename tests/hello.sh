#! /bin/bash

test_name=$(basename "$0" .sh)
# echo "$test_name"
# echo "$0"

target_pos=out
rm -rf "$target_pos"
mkdir -p "$target_pos"

cat<< EOF| riscv64-linux-gnu-gcc -o "$target_pos"/"$test_name".o -c -xc -
#include <stdio.h>

int main() {
    printf("Hello, World\n");
    return 0;
}
EOF

# echo $CC
$CC -B. -static "$target_pos"/"$test_name".o -o "$target_pos"/"$test_name".out
