# Rustqueue
## A bounded FIFO message queue at /dev/rustqueue, written in safe Rust. 
Demonstrates how Rust's ownership and locking discipline make a small in-kernel IPC primitive easy to write and structurally free of buffer-handling bugs.


## Getting Started - VM Setup
Set up a new VM, or shell into an exsisting one. 

```bash
multipass launch --name NAME lts
multipass shell NAME
```

If you ever get stuck or your VM becomes unresponsive, don't be afraid to nuke it and start agan.
```bash
multipass stop NAME
multipass delete NAME && multipass purge
multipass launch --name NAME lts
```
## Toolchain Setup
```bash
sudo apt update
sudo apt install -y build-essential linux-headers-$(uname -r) kmod tree
sudo apt install -y rustc-1.93 rust-1.93-src bindgen
sudo update-alternatives --install /usr/bin/rustc rustc /usr/bin/rustc-1.93 100
```
Verify everything is in the proper place
```bash
uname -r              # which kernel you're running
rustc --version       # should report 1.93.x
ls /lib/modules/$(uname -r)/build/rust  # Rust support files for this kernel exist
```
