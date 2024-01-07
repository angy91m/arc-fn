#[macro_export]
macro_rules! sync_fn {
    ($cb:ident) => {
        Box::new($cb)
    };
    ($cb:expr) => {
        Box::new({$cb})
    };
    ($cb:expr) => {
        Box::new({$cb})
    };
}

#[macro_export]
macro_rules! arc_sync_fn {
    ($cb:ident) => {
        $crate::ArcSyncFn::new($crate::sync_fn!($cb))
    };
    ($cb:expr) => {
        $crate::ArcSyncFn::new($crate::sync_fn!($cb))
    };
    ($cb:expr) => {
        $crate::ArcSyncFn::new($crate::sync_fn!($cb))
    };
}

#[macro_export]
macro_rules! async_fn {
    ($cb:ident) => {
        Box::new($cb)
    };
    (|$a:ident| $cb:tt) => {
        Box::new(|$a|async move $cb.boxed())
    };
    (move |$a:ident| $cb:tt) => {
        Box::new(|$a|async move $cb.boxed())
    };
    (|$a:tt| $cb:tt) => {
        Box::new(|$a|async move $cb.boxed())
    };
    (move |$a:tt| $cb:tt) => {
        Box::new(|$a|async move $cb.boxed())
    };
}

#[macro_export]
macro_rules! arc_async_fn {
    ($cb:ident) => {
        $crate::ArcAsyncFn::new( $crate::async_fn!($cb) )
    };
    (|$a:ident| $cb:tt) => {
        $crate::ArcAsyncFn::new( $crate::async_fn!(|$a| $cb) )
    };
    (move |$a:ident| $cb:tt) => {
        $crate::ArcAsyncFn::new( $crate::async_fn!(move |$a| $cb) )
    };
    (|$a:tt| $cb:tt) => {
        $crate::ArcAsyncFn::new( $crate::async_fn!(|$a| $cb) )
    };
    (move |$a:tt| $cb:tt) => {
        $crate::ArcAsyncFn::new( $crate::async_fn!(move |$a| $cb) )
    };
}