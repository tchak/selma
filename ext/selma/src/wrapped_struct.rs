use magnus::{error::Error, exception, gc, value::Value, RTypedData, TryConvert, TypedData};
use std::{marker::PhantomData, ops::Deref};

// NOTE: My Rust isn't good enough to know what any of this does,
// but it was taken from https://cs.github.com/bytecodealliance/wasmtime-rb/blob/a843e4b4582a945f2c881b8bd3e2b87688ab5509/ext/src/helpers/wrapped_struct.rs#L4

/// A small wrapper for `RTypedData` that keeps track of the concrete struct
/// type, and the underlying [`Value`] for GC purposes.
#[derive(Debug)]
#[repr(transparent)]
pub struct WrappedStruct<T: TypedData> {
    inner: RTypedData,
    phantom: PhantomData<T>,
}

impl<T: TypedData> Clone for WrappedStruct<T> {
    fn clone(&self) -> Self {
        Self {
            inner: self.inner,
            phantom: PhantomData,
        }
    }
}
impl<T: TypedData> Copy for WrappedStruct<T> {}

impl<T: TypedData> WrappedStruct<T> {
    /// Gets the underlying struct.
    pub fn get(&self) -> Result<&T, Error> {
        self.inner.try_convert()
    }

    /// Gets the underlying struct with a `'static` lifetime.
    pub fn get_static(&self) -> Result<&'static T, Error> {
        self.inner.try_convert()
    }

    /// Get the Ruby [`Value`] for this struct.
    pub fn to_value(self) -> Value {
        self.inner.into()
    }

    /// Marks the Ruby [`Value`] for GC.
    pub fn mark(&self) {
        gc::mark(&self.inner.into());
    }
}

impl<T: TypedData> From<WrappedStruct<T>> for Value {
    fn from(wrapped_struct: WrappedStruct<T>) -> Self {
        wrapped_struct.to_value()
    }
}

impl<T: TypedData> Deref for WrappedStruct<T> {
    type Target = RTypedData;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl<T: TypedData> From<T> for WrappedStruct<T> {
    fn from(t: T) -> Self {
        Self {
            inner: RTypedData::wrap(t),
            phantom: PhantomData,
        }
    }
}

impl<T> TryConvert for WrappedStruct<T>
where
    T: TypedData,
{
    fn try_convert(val: Value) -> Result<Self, Error> {
        let inner = RTypedData::from_value(val).ok_or_else(|| {
            Error::new(
                exception::type_error(),
                format!(
                    "no implicit conversion of {} into {}",
                    unsafe { val.classname() },
                    T::class()
                ),
            )
        })?;

        Ok(Self {
            inner,
            phantom: PhantomData,
        })
    }
}
