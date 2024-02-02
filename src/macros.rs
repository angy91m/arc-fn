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
        ArcSyncFn::new( sync_fn!($cb))
    };
    ($cb:expr) => {
        ArcSyncFn::new( sync_fn!($cb))
    };
    ($cb:expr) => {
        ArcSyncFn::new( sync_fn!($cb))
    };
}

#[macro_export]
macro_rules! async_fn {
    ($cb:ident) => {
        Box::new($cb)
    };
    (|$a:ident| $cb:tt) => {{
        use futures::future::FutureExt;
        Box::new(|$a|async move $cb.boxed())
    }};
    (|$a:ident: $t:ty| $cb:tt) => {{
        use futures::future::FutureExt;
        Box::new(|$a: $t|async move $cb.boxed())
    }};
    (move |$a:ident| $cb:tt) => {{
        use futures::future::FutureExt;
        Box::new(move|$a|async move $cb.boxed())
    }};
    (move |$a:ident: $t:ty| $cb:tt) => {{
        use futures::future::FutureExt;
        Box::new(move|$a: $t|async move $cb.boxed())
    }};
    (|$a:tt| $cb:tt) => {{
        use futures::future::FutureExt;
        Box::new(|$a|async move $cb.boxed())
    }};
    (move |$a:tt| $cb:tt) => {{
        use futures::future::FutureExt;
        Box::new(move|$a|async move $cb.boxed())
    }};
}

#[macro_export]
macro_rules! arc_async_fn {
    ($cb:ident) => {
        ArcAsyncFn::new( Box::new($cb) )
    };
    (|$a:ident| $cb:tt) => {{
        use futures::future::FutureExt;
        ArcAsyncFn::new( Box::new(|$a|async move $cb.boxed()) )
    }};
    (|$a:ident: $t:ty| $cb:tt) => {{
        use futures::future::FutureExt;
        ArcAsyncFn::new( Box::new(|$a: $t|async move $cb.boxed()) )
    }};
    (move |$a:ident| $cb:tt) => {{
        use futures::future::FutureExt;
        ArcAsyncFn::new( Box::new(move|$a|async move $cb.boxed()) )
    }};
    (move |$a:ident: $t:ty| $cb:tt) => {{
        use futures::future::FutureExt;
        ArcAsyncFn::new( Box::new(move|$a: $t|async move $cb.boxed()) )
    }};
    (|$a:tt| $cb:tt) => {{
        use futures::future::FutureExt;
        ArcAsyncFn::new( Box::new(|$a|async move $cb.boxed()) )
    }};
    (move |$a:tt| $cb:tt) => {{
        use futures::future::FutureExt;
        ArcAsyncFn::new( Box::new(move|$a|async move $cb.boxed()) )
    }};
}
