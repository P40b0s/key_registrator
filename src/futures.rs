// use std::{any::Any, collections::HashSet, future::Future, pin::Pin, sync::{atomic::AtomicBool, Arc}};

// type AsyncFn = Arc<dyn Fn() -> Pin<Box<dyn Future<Output = ()> + Send>> + Send + Sync>;
// type AsyncArgFn = Arc<dyn Fn(Box<dyn Any + Send + Sync>) -> Pin<Box<dyn Future<Output = ()> + Send>> + Send + Sync>;
// type Argument = Arc<dyn Any + Send + Sync>;

// #[derive(Debug, Hash, Clone, PartialEq, Eq)]
// pub enum VirtualKey 
// {
//     Backspace,
// }

// #[derive(Clone)]
// struct HotKeyCallback
// {
//     keys: HashSet<VirtualKey>,
//     func:  HotKeyCallbackEnum,
// }

// enum HotKeyCallbackEnum
// {
//     WithArg(AsyncArgFn, Argument),
//     WithoutArg(AsyncFn)
// }
// impl Clone for HotKeyCallbackEnum
// {
//     fn clone(&self) -> Self 
//     {
//         match self
//         {
//             HotKeyCallbackEnum::WithArg(v, a) => HotKeyCallbackEnum::WithArg(Arc::clone(v), a.clone()),
//             HotKeyCallbackEnum::WithoutArg(v) => HotKeyCallbackEnum::WithoutArg(Arc::clone(v))
//         }
//     }
// }

// pub struct KeysWatcher
// {
//     callbacks: Arc<RwLock<Vec<HotKeyCallback>>>,
//     kill: Arc<AtomicBool>
// }
// impl KeysWatcher
// {
//     pub fn new() -> Self
//     {
//         Self
//         {
//             callbacks: Arc::new(RwLock::new(Vec::new())),
//             kill: Arc::new(AtomicBool::new(false))
//         }
//     }
//     pub async fn register<F, Fut>(&mut self, keys: &[VirtualKey], f: F) -> &mut Self
//     where 
//         F: Fn() -> Fut + Send + Sync + 'static,
//         Fut: Future<Output = ()> + Send + 'static,
//     {
//         let hk = HotKeyCallback
//         {
//             keys: HashSet::from_iter(keys.to_owned().into_iter()),
//             func: HotKeyCallbackEnum::WithoutArg(Arc::new( move || Box::pin(f())))
//         };
//         let mut guard = self.callbacks.write().await;
//         guard.push(hk);
//         drop(guard);
//         self
//     }
//     pub async fn register_with_state<F, Fut, Arg>(&mut self, keys: &[VirtualKey], s: Arg, f: F) -> &mut Self
//     where 
//         F: Fn(Arg) -> Fut + Send + Sync + 'static,
//         Fut: Future<Output = ()> + Send + 'static,
//         Arg: Send + Sync + 'static + Clone
//     {
//         let callback = Arc::new(move |arg: Box<dyn Any + Send + Sync>|
//         {
//             let arg = arg.downcast::<Arg>().unwrap();
//             Box::pin(f(*arg)) as Pin<Box<(dyn Future<Output = ()> + Send + 'static)>>
//         });
        
//         let hk = HotKeyCallback
//         {
//             keys: HashSet::from_iter(keys.to_owned().into_iter()),
//             func: HotKeyCallbackEnum::WithArg(callback, Arc::new(Box::new(s)))
//         };
//         let mut guard = self.callbacks.write().await;
//         guard.push(hk);
//         drop(guard);
//         self
//     }
//     pub fn watch(&self)
//     {
//         let (sender, receiver) = std::sync::mpsc::channel();
//         //if dropping previous receiver, set new sender
//         if let Some(s) = SENDER.get()
//         {
//             let mut guard = s.write().unwrap();
//             *guard = sender
//         }
//         else 
//         {
//             let _ = SENDER.set(std::sync::RwLock::new(sender));
//         }
//         let killer = self.kill.clone();
//         let callbacks = self.callbacks.clone();
//         tokio::spawn(async move
//         {
//             while let Ok(r) = receiver.recv()
//             {
//                 if killer.load(std::sync::atomic::Ordering::SeqCst)
//                 {
//                     drop(receiver);
//                     break;
//                 }
//                 let guard = callbacks.read().await;
//                'k: for g in guard.iter()
//                 {
//                     {
//                         let active_keys = ACTIVE_KEYS.read().unwrap();
//                         for k in &g.keys
//                         {
//                             if !active_keys.contains(k)
//                             {
//                                 continue 'k;
//                             }
//                         }
//                     }
//                     let funcs = g.func.clone();
//                     match funcs
//                     {
//                         HotKeyCallbackEnum::WithoutArg(f) =>
//                         {
//                             f().await
//                         },
//                         HotKeyCallbackEnum::WithArg(f, a) =>
//                         {
                           
//                             let f = f.to_owned();
//                             f(Box::new(a)).await;
//                             logger::info!("after call with args");
//                             //f(arg).await;
//                             //let arg = |a: Box<dyn Any + Send>| async {f(a).await};
//                             //arg.await;
//                         }
//                     }
//                     logger::debug!("keys fire");
//                 }
//                 logger::debug!("pressed: {}", r);
//             }
//         });
//     }
// }
// std::thread::spawn(move ||
//     {
//         ...
//         HotKeyCallbackEnum::WithArg(f: Arc<dyn Fn(Box<dyn Any + Send + Sync>) -> Pin<Box<dyn Future<Output = ()> + Send>> + Send + Sync>, a: Arc<dyn Any + Send + Sync>) =>
//         {
        
//             let f = f.to_owned();
//             futures::executor::block_on(async {f(Box::new(a)).await});
//             logger::info!("На предыдущем шаге дедлок... или что-то еще но до сюда не доходит...");
//         }
//     });


//     HotKeyCallbackEnum::WithoutArg(f: Arc<dyn Fn() -> Pin<Box<dyn Future<Output = ()> + Send>> + Send + Sync>) =>
//     {
//         futures::executor::block_on(async  {f().await});
//     },