use std::sync::{Arc,Mutex};
use futures::future::BoxFuture;

pub trait SyncFnMut<T,O>: FnMut(T) -> O + Send + Sync  + 'static {}

impl<T,O,F> SyncFnMut<T,O> for F
where F: FnMut(T) -> O + Send + Sync + 'static {}

pub trait SyncFn<T,O>: SyncFnMut<T,O> + Clone {}
impl<T,O,F> SyncFn<T,O> for F
where F: SyncFnMut<T,O> + Clone {}

pub trait AsyncFnMut<T,O>: SyncFnMut<T,BoxFuture<'static,O>> {}
impl<T,O,F> AsyncFnMut<T,O> for F
where F: SyncFnMut<T,BoxFuture<'static,O>> {}
pub trait AsyncFn<T,O>: AsyncFnMut<T,O> + Clone {}
impl<T,O,F> AsyncFn<T,O> for F
where F: AsyncFnMut<T,O> + Clone {}

pub struct ArcSyncFn<T,O> {
    v: Arc<Mutex<Box<dyn SyncFnMut<T,O>>>>
}
impl<T,O> Clone for ArcSyncFn<T,O> {
    fn clone(&self) -> Self {
        Self {v: self.v.clone() }
    }
}

impl<T,O> ArcSyncFn<T,O> {
    pub fn new( f: Box<dyn SyncFnMut<T,O>> ) -> Self {
        Self {v: Arc::new(Mutex::new(f))}
    }
    pub fn set( &self, f: Box<dyn SyncFnMut<T,O>> ) {
        let mut mg = self.v.lock().unwrap();
        *mg = f;
    }
    pub fn run( &self, a: T ) -> O {
        println!("{}",self.v.is_poisoned());
        let mut f = self.v.lock().unwrap();
        (f)(a)
    }
}

pub struct ArcAsyncFn<T,O> {
    v: Arc<Mutex<Box<dyn AsyncFnMut<T,O>>>>
}
impl<T,O> Clone for ArcAsyncFn<T,O> {
    fn clone(&self) -> Self {
        Self {v: self.v.clone() }
    }
}

impl<T,O> ArcAsyncFn<T,O> {
    pub fn new( f: Box<dyn AsyncFnMut<T,O>> ) -> Self {
        Self {v: Arc::new(Mutex::new(f))}
    }
    pub fn set( &self, f: Box<dyn AsyncFnMut<T,O>> ) {
        let mut mg = self.v.lock().unwrap();
        *mg = f;
    }
    pub fn run( &self, a: T ) -> BoxFuture<'static,O> {
        println!("{}",self.v.is_poisoned());
        let mut f = self.v.lock().unwrap();
        (f)(a)
    }
}

#[cfg(test)]
mod tests {
    use std::{thread::{spawn,sleep,JoinHandle},time::Duration, panic::catch_unwind};
    use futures::{future::FutureExt,executor::block_on};

    use super::ArcAsyncFn;

    fn data_producer(f: ArcAsyncFn<String,Result<(),String>>) -> JoinHandle<()> {
        let handle = spawn(||
            block_on( async move {
                for i in 0..10 {
                    println!("Cycle n. {i}");
                    let f = f.clone();
                    let _ = f.run("Hello world!".to_string()).await;
                    sleep(Duration::from_secs(1));
                }
            })
        );
        handle
    }

    #[test]
    fn it_works() {
        let cb = ArcAsyncFn::new(Box::new(|s| async move {
            println!("{s} This is first cb!");
            Ok(())
        }.boxed()));
        let handle = data_producer(cb.clone());
        spawn(||block_on(async move {
            sleep(Duration::from_secs(5));
            cb.set(Box::new(|s| async move {
                println!("{s} This is the second cb!");
                match catch_unwind(|| panic!("Error")) {
                    Ok(_) => Ok(()),
                    Err(_) => Err("Error".to_string())
                }
            }.boxed()));
        }));
        let _ = handle.join();
    }
}
