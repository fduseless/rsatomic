# RsAtomic

Atomic Operation for Python

## Feature

- Native Package base on Rust.
- Simple, Easy to use;
- High performance.

## Install

Install rsatomic with pip:

```bash
pip install rsatomic
```

## Usage

```python
import rsatomic
# AtomicI8, i8
# AtomicI16, i16
# AtomicI32, i32
# AtomicI64, i64
# AtomicIsize, isize
# AtomicU8, u8
# AtomicU16, u16
# AtomicU32, u32
# AtomicU64, u64
# AtomicUsize, usize
# AtomicBool, bool
atomic = rsatomic.AtomicI64()

v = atomic.load()
atomic.store(1)
v = atomic.swap(2)

ok_or_err, v = atomic.compare_exchange(2, 3)
ok_or_err, v = atomic.compare_exchange_weak(2, 3)

v = atomic.fetch_and(2, 3)
v = atomic.fetch_nand(2, 3)
v = atomic.fetch_or(2, 3)
v = atomic.fetch_xor(2, 3)

# only for integer
v = atomic.fetch_add(2)
v = atomic.fetch_sub(2)
v = atomic.fetch_max(2)
v = atomic.fetch_min(2)

# v = (v + 2) % 3
v = atomic.fetch_add_mod(2, 3)
```
