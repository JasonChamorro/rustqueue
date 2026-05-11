# Rustqueue
## A bounded FIFO message queue at /dev/rustqueue, written in safe Rust. 
### Demonstrates how Rust's ownership and locking discipline make a small in-kernel IPC primitive easy to write and structurally free of buffer-handling bugs.


## Getting Started - VM Setup
Set up a new VM, or shell into an exsisting one. 

```bash
multipass launch --name NAME lts
multipass shell NAME
```

