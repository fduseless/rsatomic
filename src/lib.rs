use pyo3::prelude::*;
use std::sync::atomic::Ordering;

#[pyclass]
#[derive(Clone)]
pub enum Status {
    Ok,
    Err
}

macro_rules! create_interface {
    ($name: ident, $type: ident) => {
        #[pyclass]
        pub struct $name {
            inner: std::sync::atomic::$name,
        }
    }
}

macro_rules! common_operation {
    ($name: ident, $type: ident) => {
        #[pymethods]
        impl $name {
            #[new]
            pub fn new(data: $type) -> Self {
                Self {
                    inner: std::sync::atomic::$name::new(data),
                }
            }
            pub fn load(&self) -> $type {
                return self.inner.load(Ordering::SeqCst);
            }

            pub fn store(&self, val: $type) {
                return self.inner.store(val, Ordering::SeqCst);
            }

            pub fn swap(&self, val: $type) -> $type {
                return self.inner.swap(val, Ordering::SeqCst);
            }
            pub fn compare_exchange(
                &self,
                current: $type,
                new: $type
            ) -> (Status, $type) {
                return match self.inner.compare_exchange(current, new, Ordering::SeqCst, Ordering::SeqCst)  {
                    Ok(v) => (Status::Ok, v),
                    Err(v) => (Status::Err, v)
                }
            }

            pub fn compare_exchange_weak(
                &self,
                current: $type,
                new: $type,
            ) -> (Status, $type) {
                return match self.inner.compare_exchange_weak(current, new, Ordering::SeqCst, Ordering::SeqCst)  {
                    Ok(v) => (Status::Ok, v),
                    Err(v) => (Status::Err, v)
                }
            }
        }
    }
}


macro_rules! bool_operation {
    ($name: ident, $type: ident) => {
        #[pymethods]
        impl $name {
            pub fn fetch_and(&self, val: $type) -> $type {
                return self.inner.fetch_and(val, Ordering::SeqCst);
            }
            pub fn fetch_nand(&self, val: $type) -> $type {
                return self.inner.fetch_nand(val, Ordering::SeqCst);
            }
            pub fn fetch_or(&self, val: $type) -> $type {
                return self.inner.fetch_or(val, Ordering::SeqCst);
            }
            pub fn fetch_xor(&self, val: $type) -> $type {
                return self.inner.fetch_xor(val, Ordering::SeqCst);
            }
        }
    }
}

macro_rules! math_operation {
    ($name: ident, $type: ident) => {
        #[pymethods]
        impl $name {
            pub fn fetch_add(&self, val: $type) -> $type {
                return self.inner.fetch_add(val, Ordering::SeqCst);
            }
            pub fn fetch_sub(&self, val: $type) -> $type {
                return self.inner.fetch_sub(val, Ordering::SeqCst);
            }
            pub fn fetch_max(&self, val: $type) -> $type {
                return self.inner.fetch_max(val, Ordering::SeqCst);
            }
            pub fn fetch_min(&self, val: $type) -> $type {
                return self.inner.fetch_min(val, Ordering::SeqCst);
            }

            pub fn fetch_add_mod(&self,
                val: $type, m: $type) -> (Status, $type) {
                return match self.inner.fetch_update(Ordering::SeqCst, Ordering::SeqCst, |cur| Some((cur + val) % m)) {
                    Ok(v) => (Status::Ok, v),
                    Err(v) => (Status::Err, v)
                };
            }
        }
    }
}

macro_rules! create_bool_interface {
    ($name: ident, $type: ident) => {
        create_interface!($name, $type);
        common_operation!($name, $type);
        bool_operation!($name, $type);
    }
}

macro_rules! create_int_interface {
    ($name: ident, $type: ident) => {
        create_interface!($name, $type);
        common_operation!($name, $type);
        bool_operation!($name, $type);
        math_operation!($name, $type);
    };
}

create_int_interface!(AtomicI8, i8);
create_int_interface!(AtomicI16, i16);
create_int_interface!(AtomicI32, i32);
create_int_interface!(AtomicI64, i64);
create_int_interface!(AtomicIsize, isize);
create_int_interface!(AtomicU8, u8);
create_int_interface!(AtomicU16, u16);
create_int_interface!(AtomicU32, u32);
create_int_interface!(AtomicU64, u64);
create_int_interface!(AtomicUsize, usize);
create_bool_interface!(AtomicBool, bool);

/// A Python module implemented in Rust.
#[pymodule]
fn rsatomic(_py: Python, m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<AtomicI8>()?;
    m.add_class::<AtomicI16>()?;
    m.add_class::<AtomicI32>()?;
    m.add_class::<AtomicI64>()?;
    m.add_class::<AtomicIsize>()?;
    m.add_class::<AtomicU8>()?;
    m.add_class::<AtomicU16>()?;
    m.add_class::<AtomicU32>()?;
    m.add_class::<AtomicU64>()?;
    m.add_class::<AtomicUsize>()?;
    m.add_class::<AtomicBool>()?;
    m.add_class::<Status>()?;
    Ok(())
}
