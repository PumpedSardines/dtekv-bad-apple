cargo build --release
objdump -D ./target/riscv32im-unknown-none-elf/release/bare_metal_riscv  > ./main.elf.txt
objcopy -O binary ./target/riscv32im-unknown-none-elf/release/bare_metal_riscv ./main.bin
