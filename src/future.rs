use crate::error::{KubeError, KubeResult};
use futures::{future, Async, Future as OldFuture, Stream};
use log::Level;
use serde::Deserialize;
use tokio_async_await::compat::{backward, forward};

use std::fmt::Debug;
use std::future::Future as NewFuture;
use std::marker::Unpin;

/// Converts new future to the old future, so that we can use the combinators.
pub fn downgrade<I, E>(
    f: impl NewFuture<Output = Result<I, E>>,
) -> impl OldFuture<Item = I, Error = E> {
    backward::Compat::new(f)
}

/// Converts the old future to the new future, so that we can use await.
pub fn upgrade<I, E>(
    f: impl OldFuture<Item = I, Error = E> + Unpin,
) -> impl NewFuture<Output = Result<I, E>> {
    forward::IntoAwaitable::into_awaitable(f)
}

/// Old `try!` like syntax for futures.
#[macro_export]
macro_rules! future_try {
    ($expr:expr) => {
        match $expr {
            Ok(v) => v,
            Err(e) => return Err(KubeError::from(e)).into(),
        }
    };
}

/// Convenience macro for converting future types into KubeFuture.
#[macro_export]
macro_rules! future {
    ($expr:expr) => {
        Box::new($expr).into()
    };
}

/// Future type used throughout the library. It's an unit type
/// solely for us to be able to add our own extensions.
pub struct KubeFuture<T>(Box<OldFuture<Item = T, Error = KubeError> + Send>);

/// Stream type used throughout the library.
pub type KubeStream<T> = Box<Stream<Item = T, Error = KubeError> + Send>;

impl<T> KubeFuture<T>
where
    for<'de> T: Deserialize<'de>,
    T: Debug + Send + 'static,
{
    /// Parse the given bytes as JSON and return a future that resolves to that object.
    pub fn parse_json<B>(bytes: B) -> KubeFuture<T>
    where
        B: AsRef<[u8]>,
    {
        if log_enabled!(Level::Debug) {
            let string = String::from_utf8_lossy(bytes.as_ref());
            debug!("Actual response: {}", string);
        }

        let result: Result<T, _> = serde_json::from_reader(bytes.as_ref());
        result.map_err(|e| KubeError::from(e)).into()
    }
}

impl<T> KubeFuture<T>
where
    for<'de> T: Deserialize<'de>,
    T: Debug + Send + 'static,
    KubeError: From<T>,
{
    /// Parse the given bytes as JSON and return a future that resolves to an error.
    pub fn parse_json_as_err<B, U>(bytes: B) -> KubeFuture<U>
    where
        B: AsRef<[u8]>,
        U: Send + 'static,
    {
        future!(Self::parse_json(bytes).and_then(|e| Err(KubeError::from(e))))
    }
}

/// `Future` impl for our wrapper.
impl<T> OldFuture for KubeFuture<T> {
    type Item = T;
    type Error = KubeError;

    fn poll(&mut self) -> Result<Async<Self::Item>, Self::Error> {
        self.0.poll()
    }
}

/// Allow boxed future types to be converted into our future.
impl<T, F> From<Box<F>> for KubeFuture<T>
where
    F: OldFuture<Item = T, Error = KubeError> + Send + 'static,
{
    fn from(f: Box<F>) -> KubeFuture<T> {
        KubeFuture(f as Box<_>)
    }
}

/// Allow result types to be converted directly into our future.
impl<T> From<KubeResult<T>> for KubeFuture<T>
where
    T: Send + 'static,
{
    fn from(r: KubeResult<T>) -> KubeFuture<T> {
        future!(future::result(r))
    }
}
