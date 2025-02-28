use std::{any::Any, collections::HashSet, future::Future, pin::Pin, sync::{atomic::AtomicBool, Arc, RwLock}};
use std::fmt::Debug;

type AsyncFn = Arc<dyn Fn() -> Pin<Box<dyn Future<Output = ()> + Send>> + Send + Sync>;
type AsyncArgFn = Arc<dyn Fn(Arc<dyn Any + Send + Sync>) -> Pin<Box<dyn Future<Output = ()> + Send>> + Send + Sync>;
type Argument = Arc<dyn Any + Send + Sync>;


#[derive(Debug, Hash, Clone, PartialEq, Eq)]
pub enum VirtualKey 
{
    Backspace,
    Tab,
    Enter,
}
#[derive(Clone)]
struct HotKeyCallback
{
    keys: HashSet<VirtualKey>,
    func:  HotKeyCallbackEnum,
}

enum HotKeyCallbackEnum
{
    WithArg(AsyncArgFn, Argument),
    WithoutArg(AsyncFn)
}
impl Clone for HotKeyCallbackEnum
{
    fn clone(&self) -> Self 
    {
        match self
        {
            HotKeyCallbackEnum::WithArg(v, a) => HotKeyCallbackEnum::WithArg(Arc::clone(v), a.clone()),
            HotKeyCallbackEnum::WithoutArg(v) => HotKeyCallbackEnum::WithoutArg(Arc::clone(v))
        }
    }
}

pub struct KeysWatcher
{
    callbacks: Arc<RwLock<Vec<HotKeyCallback>>>,
    kill: Arc<AtomicBool>
}
impl KeysWatcher
{
    pub fn new() -> Self
    {
        Self
        {
            callbacks: Arc::new(RwLock::new(Vec::new())),
            kill: Arc::new(AtomicBool::new(false))
        }
    }
    pub fn register<F, Fut>(&mut self, keys: &[VirtualKey], f: F) -> &mut Self
    where 
        F: Fn() -> Fut + Send + Sync + 'static,
        Fut: Future<Output = ()> + Send + 'static,
    {
        let hk = HotKeyCallback
        {
            keys: HashSet::from_iter(keys.to_owned().into_iter()),
            func: HotKeyCallbackEnum::WithoutArg(Arc::new( move || Box::pin(f())))
        };
        let mut guard = self.callbacks.write().unwrap();
        guard.push(hk);
        drop(guard);
        self
    }
    pub fn register_with_state<F, Fut, Arg>(&mut self, keys: &[VirtualKey], s: Arg, f: F) -> &mut Self
    where 
        F: Fn(Arg) -> Fut + Send + Sync + 'static,
        Fut: Future<Output = ()> + Send + 'static,
        Arg: Send + Sync + 'static + Clone + Debug
    {
        let callback = Arc::new(move |arg: Arc<dyn Any + Send + Sync>|
        {
            let arg = arg.downcast::<Arg>().unwrap();
            let r = Arc::try_unwrap(arg).unwrap();
            Box::pin(f(r)) as Pin<Box<(dyn Future<Output = ()> + Send + 'static)>>
        });
        
        let hk = HotKeyCallback
        {
            keys: HashSet::from_iter(keys.to_owned().into_iter()),
            func: HotKeyCallbackEnum::WithArg(callback, Arc::new(Box::new(s)))
        };
        let mut guard = self.callbacks.write().unwrap();
        guard.push(hk);
        drop(guard);
        self
    }
    pub fn watch(&self)
    {
        let callbacks = self.callbacks.clone();
        let mut cb_guard  = callbacks.write().unwrap();
        let callbacks = std::mem::replace(&mut *cb_guard, Vec::<HotKeyCallback>::new());
        drop(cb_guard);
        tokio::spawn(async move
        {
            for callback in callbacks.iter()
            {
                let funcs = callback.func.clone();
                match funcs
                {
                    HotKeyCallbackEnum::WithoutArg(f) =>
                    {
                        logger::info!("before call");
                        // tokio::spawn(async move 
                        // {
                        //     f().await;
                        // });
                        futures::executor::block_on(async {f().await});
                        logger::info!("after call");
                    },
                    HotKeyCallbackEnum::WithArg(f, a) =>
                    {
                        logger::info!("before call with args {:?}", &a);
                        // tokio::spawn(async move 
                        // {
                        //     f(a).await;
                        // });
                        futures::executor::block_on(async {f(a).await});
                        logger::info!("after call with args");
                        //f(arg).await;
                        //let arg = |a: Box<dyn Any + Send>| async {f(a).await};
                        //arg.await;
                    }
                }
                logger::debug!("keys fire");
            }
        });
    }
}
#[cfg(test)]
mod tests
{
    use std::{sync::Arc, time::Duration};

    use super::{KeysWatcher, VirtualKey};
    
    #[tokio::test]
    async fn run_test()
    {
        let _ = logger::StructLogger::new_default();
        let state = Arc::new(String::from("TEST_STATE"));
        let mut key_watcher = KeysWatcher::new();
        key_watcher
        .register(&[VirtualKey::Backspace, VirtualKey::Enter], callback_1)
        .register(&[VirtualKey::Backspace, VirtualKey::Tab], callback_2)
        .register_with_state(&[VirtualKey::Tab, VirtualKey::Enter], state, callback_3)
        .watch();
        loop 
        {
            tokio::time::sleep(Duration::from_millis(5000)).await;
        }
    }
    async fn callback_1()
    {
        logger::info!("left control + left alt!");
    }
    async fn callback_2()
    {
        logger::info!("F5 + mouse left click");
    }
    async fn callback_3(state: Arc<String>)
    {
        logger::info!("{}", ["F5 + mouse left click", &state].concat());
    }
}