// SPDX-License-Identifier: GPL-2.0
//! rustqueue: a bounded FIFO message queue character device.

use kernel::{
    fs::{File, Kiocb},
    iov::{IovIterDest, IovIterSource},
    miscdevice::{MiscDevice, MiscDeviceOptions, MiscDeviceRegistration},
    new_mutex,
    prelude::*,
    sync::Mutex,
};

module! {
    type: RustQueue,
    name: "rustqueue",
    description: "rustqueue — a bounded FIFO message queue",
    license: "GPL",
}

const MAX_MESSAGES: usize = 16;
const MAX_MSG_SIZE: usize = 4096;

kernel::sync::global_lock! {
    unsafe(uninit) static QUEUE: Mutex<KVec<KVec<u8>>> = KVec::new();
}

#[pin_data]
struct RustQueue {
    #[pin]
    _miscdev: MiscDeviceRegistration<RustQueueDevice>,
}

impl kernel::InPlaceModule for RustQueue {
    fn init(_module: &'static ThisModule) -> impl PinInit<Self, Error> {
        pr_info!("module loaded (capacity {} messages)\n", MAX_MESSAGES);
        // SAFETY: Called exactly once during module init.
        unsafe { QUEUE.init() };
        let opts = MiscDeviceOptions { name: c"rustqueue" };
        try_pin_init!(Self {
            _miscdev <- MiscDeviceRegistration::register(opts),
        })
    }
}

#[pin_data]
struct RustQueueDevice {
    // Per-open: the message we dequeued for this `cat` invocation, if any.
    #[pin]
    pending: Mutex<Option<KVec<u8>>>,
}

#[vtable]
impl MiscDevice for RustQueueDevice {
    type Ptr = Pin<KBox<Self>>;

    fn open(_file: &File, _misc: &MiscDeviceRegistration<Self>) -> Result<Pin<KBox<Self>>> {
        KBox::try_pin_init(
            try_pin_init! {
                RustQueueDevice {
                    pending <- new_mutex!(None),
                }
            },
            GFP_KERNEL,
        )
    }
    // write_iter() and read_iter() below}

fn write_iter(mut kiocb: Kiocb<'_, Self::Ptr>, iov: &mut IovIterSource<'_>) -> Result<usize> {
    let mut q = QUEUE.lock();

    if q.len() >= MAX_MESSAGES {
        pr_info!("queue full, rejecting write\n");
        return Err(ENOSPC);
    }

    let mut msg: KVec<u8> = KVec::new();
    let len = iov.copy_from_iter_vec(&mut msg, GFP_KERNEL)?;

    if len > MAX_MSG_SIZE {
        return Err(EINVAL);
    }

    q.push(msg, GFP_KERNEL)?;
    *kiocb.ki_pos_mut() = 0;

    pr_info!("enqueued {} bytes ({} in queue)\n", len, q.len());
    Ok(len)
}

fn read_iter(mut kiocb: Kiocb<'_, Self::Ptr>, iov: &mut IovIterDest<'_>) -> Result<usize> {
    let me = kiocb.file();
    let mut pending = me.pending.lock();

    // First read of this open: pull one message off the queue and remember it.
    if pending.is_none() && *kiocb.ki_pos_mut() == 0 {
        let mut q = QUEUE.lock();
        if !q.is_empty() {
            *pending = Some(q.remove(0)?);
            pr_info!("dequeued ({} remaining)\n", q.len());
        }
    }

    match pending.as_ref() {
        None => Ok(0),
        Some(msg) => iov.simple_read_from_buffer(kiocb.ki_pos_mut(), msg),
    }
}
}
