# Solcrypt
A "hobby project" ransomware that isn't really meant to be used for illegal purposes.
I made this for fun, ok? Don't get me in legal trouble for it

## Setup
Arch
```
sudo pacman -S rustup
rustup default stable
rustup target add x86_64-pc-windows-gnu # if compiling for windows targets
git clone https://git.xorycode.dev/xory/solcrypt
cd solcrypt 
bash build.sh # this builds both encryptor and decryptor
```
Then run it on your target PC/server

## Roadmap
- [x] Basic home directory encryption
- [x] Randomly generated encryption key
- [x] Ransom prompt?
- [x] Server
- [ ] Meow?

