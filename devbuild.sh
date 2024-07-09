#! /bin/bash
cargo b --target x86_64-pc-windows-gnu
rm -f ~/solcrypt-ds.iso
mkisofs -o ~/solcrypt-ds.iso target/x86_64-pc-windows-gnu/debug/solcrypt.exe

