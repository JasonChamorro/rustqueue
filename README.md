# Rustqueue
## A bounded FIFO message queue at /dev/rustqueue, written in safe Rust. 
Demonstrates how Rust's ownership and locking discipline make a small in-kernel IPC primitive easy to write and structurally free of buffer-handling bugs.


# Getting Started 
## VM Setup
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
Verify everything is in the proper place.
```bash
uname -r              # which kernel you're running
rustc --version       # should report 1.93.x
ls /lib/modules/$(uname -r)/build/rust  # Rust support files for this kernel exist
```


## Project Directory
Clone the repository into the vm. Running ```ls``` should show Makefile, README, and rustqueue.rs.

# Running
To build the makefile, run:
```bash
make clean && make
```
```/dev/rustqueue``` is created with mode ```0600 root:root```. This gives read and write access, but we need ```bash sudo``` for either. Running ```ls``` again will show you everything the makefile built.

## Example 1: Queue and dequeue three messages.
To observe how the queue works, lets run a basic example to see the queue in action.

```bash
sudo insmod rustqueue.ko
ls -la /dev/rustqueue

echo "first message"  | sudo tee /dev/rustqueue > /dev/null
echo "second message" | sudo tee /dev/rustqueue > /dev/null
echo "third message"  | sudo tee /dev/rustqueue > /dev/null

sudo cat /dev/rustqueue   # → "first message"
sudo cat /dev/rustqueue   # → "second message"
sudo cat /dev/rustqueue   # → "third message"
sudo cat /dev/rustqueue   # → (empty, EOF — no output)

sudo dmesg | tail -10
sudo rmmod rustqueue
```

You should see that our queue has a maximum of 16 items, and three messages getting queued and dequeued.
```bash
rustqueue: module loaded (capacity 16 messages)
rustqueue: enqueued 14 bytes (1 in queue)
rustqueue: enqueued 15 bytes (2 in queue)
rustqueue: enqueued 14 bytes (3 in queue)
rustqueue: dequeued (2 remaining)
rustqueue: dequeued (1 remaining)
rustqueue: dequeued (0 remaining)
```
## Example 2: Overloading the queue.
Try running the following and observers what happens:
```bash
sudo insmod rustqueue.ko
for i in {1..20}; do echo "message $i" | sudo tee /dev/rustqueue > /dev/null || echo "write $i FAILED"; done
sudo dmesg | tail -20
sudo rmmod rustqueue
```
There should be 16 successful enqueues, followed by four rejected writes.
