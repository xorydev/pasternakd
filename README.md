
![](https://git.xorycode.dev/xory/pasternakd/-/raw/main/demo.mp4)
# pasternakd
A daemon for the pasternak network-activated killswitch, intended to detach GPT & LUKS headers, making all LUKS encrypted data unrecoverable.

## Installation Instructions

### Arch Linux
```
sudo pacman -S rust cargo
git clone https://git.xorycode.dev/xory/pasternakd
cd pasternakd
cargo build --release
sudo cp ./target/release/pasternakd /bin/
```

### Debian Linux & Permutations
```
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
git clone https://git.xorycode.dev/xory/pasternakd
cd pasternakd
cargo build --release
sudo cp ./target/release/pasternakd /bin/
```