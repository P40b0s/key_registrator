use std::{any::Any, collections::HashSet, fmt::Debug, future::Future, pin::Pin, ptr::null_mut, sync::{atomic::AtomicBool, Arc, RwLock}};
use windows_sys::Win32::{
    Foundation::{LPARAM, LRESULT, WPARAM},
    UI::WindowsAndMessaging::{
        CallNextHookEx,
        DispatchMessageA,
        GetMessageA,
        SetWindowsHookExA,
        TranslateMessage,
        UnhookWindowsHookEx,
        HHOOK,
        KBDLLHOOKSTRUCT,
        MSLLHOOKSTRUCT,
        WH_KEYBOARD_LL,
        WH_MOUSE_LL,
        WM_KEYDOWN,
        WM_KEYUP,
        WM_LBUTTONDOWN,
        WM_LBUTTONUP,
        WM_RBUTTONDOWN,
        WM_RBUTTONUP,
        WM_SYSKEYDOWN,
        WM_SYSKEYUP
    }
};
use crate::keys::VirtualKey;

static SENDER: std::sync::OnceLock<std::sync::RwLock<std::sync::mpsc::Sender<Arc<VirtualKey>>>> = std::sync::OnceLock::new();
static ACTIVE_KEYS: std::sync::LazyLock<std::sync::RwLock<HashSet<Arc<VirtualKey>>>> = std::sync::LazyLock::new(|| std::sync::RwLock::new(HashSet::new()));

type AsyncFn = Arc<dyn Fn() -> Pin<Box<dyn Future<Output = ()> + Send>> + Send + Sync>;
type AsyncArgFn = Arc<dyn Fn(Argument) -> Pin<Box<dyn Future<Output = ()> + Send>> + Send + Sync>;
type Argument = Arc<Box<dyn Any + Send + Sync>>;
//#[derive(Clone)]
struct HotKeyCallback
{
    keys: HashSet<VirtualKey>,
    func: HotKeyCallbackEnum,
}

enum HotKeyCallbackEnum
{
    WithArg(AsyncArgFn, Argument),
    WithoutArg(AsyncFn)
}

///hook handle
static mut HOOK: HHOOK = null_mut();
static mut MOUSE_HOOK: HHOOK = null_mut();
///handle callback
unsafe extern "system" fn hook_callback(n_code: i32, w_param: WPARAM, l_param: LPARAM) -> LRESULT
{
    if n_code >= 0
    {
        let kb_struct = *(l_param as *const KBDLLHOOKSTRUCT);
        //can obitain click coordinates, but in this case its not necessary
        let _mouse_struct = *(l_param as *const MSLLHOOKSTRUCT);
        match w_param as u32
        {
            WM_KEYDOWN | WM_SYSKEYDOWN => 
            {
                let key: Arc<VirtualKey> = Arc::new(kb_struct.vkCode.into());
                let mut active_keys_guard = ACTIVE_KEYS.write().unwrap();
                active_keys_guard.insert(key.clone());
                let guard = SENDER.get().unwrap().read().unwrap();
                let _  = guard.send(key);
                
            },
            WM_KEYUP | WM_SYSKEYUP => 
            {
                let key: Arc<VirtualKey> = Arc::new(kb_struct.vkCode.into());
                let mut active_keys_guard = ACTIVE_KEYS.write().unwrap();
                active_keys_guard.remove(&key);
            },
            
            WM_LBUTTONDOWN => 
            {
                let key = Arc::new(VirtualKey::MouseLeftClick);
                let mut active_keys_guard = ACTIVE_KEYS.write().unwrap();
                active_keys_guard.insert(key.clone());
                let guard = SENDER.get().unwrap().read().unwrap();
                let _  = guard.send(key);
            },
            WM_LBUTTONUP =>
            {
                let key = Arc::new(VirtualKey::MouseLeftClick);
                let mut active_keys_guard = ACTIVE_KEYS.write().unwrap();
                active_keys_guard.remove(&key);
            }
            WM_RBUTTONDOWN => 
            {
                let key = Arc::new(VirtualKey::MouseRightClick);
                let mut active_keys_guard = ACTIVE_KEYS.write().unwrap();
                active_keys_guard.insert(key.clone());
                let guard = SENDER.get().unwrap().read().unwrap();
                let _  = guard.send(key);
            },
            WM_RBUTTONUP => 
            {
                let key = Arc::new(VirtualKey::MouseRightClick);
                let mut active_keys_guard = ACTIVE_KEYS.write().unwrap();
                active_keys_guard.remove(&key);
            }
            _ => ()
        }
    }
    CallNextHookEx(HOOK, n_code, w_param, l_param)
}

pub trait IsArc {}
impl<T> IsArc for Arc<T>{}


/// Create key watcher with given keys and async callback
/// 
/// # Examples
/// ```
/// use key_registrator::VirtualKey;
/// use key_registrator:: KeysWatcher;
/// use std::time::Duration;
/// use std::sync::Arc;
/// #[tokio::main]
/// async fn main() 
/// {
///     //state must be wrapped into Arc
///     let state = Arc::new(String::from("TEST_STATE"));
///     let mut key_watcher = KeysWatcher::new();
///     key_watcher
///         .register(&[VirtualKey::LeftCtrl, VirtualKey::LeftAlt], callback_1)
///         .register(&[VirtualKey::F5, VirtualKey::MouseLeftClick], callback_2)
///         .register_with_state(&[VirtualKey::LeftCtrl, VirtualKey::RightCtrl], state, callback_3)
///         .watch();
///     //this code run in another thread, add loop for watcher still alive
///     loop 
///      {
///          tokio::time::sleep(Duration::from_millis(5000)).await;
///      };
/// }
/// 
///  async fn callback_1()
/// {
///     println!("left control + left alt!");
/// }
/// async fn callback_2()
/// {
///     println!("F5 + mouse left click");
/// }
/// async fn callback_3(state: Arc<String>)
/// {
///     println!("{}", ["left control + right control and state: ", &state].concat());
/// }
/// ```
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
        Fut: Future<Output = ()> + Send + Sync + 'static,
        Arg: IsArc + Send + Sync + 'static + Clone + Debug
    {
        let callback = Arc::new(move |arg: Argument|
        {
            //downcast_ref take &self + clone as Arg: IsArc
            let arg = arg.downcast_ref::<Arg>().unwrap().clone();
            Box::pin(f(arg)) as Pin<Box<(dyn Future<Output = ()> + Send + 'static)>>
        });
        //boxing argument
        let arg  = Arc::new(Box::new(s) as Box<dyn Any + Send + Sync>);
        let hk = HotKeyCallback
        {
            keys: HashSet::from_iter(keys.to_owned().into_iter()),
            func: HotKeyCallbackEnum::WithArg(callback, arg)
        };
        let mut guard = self.callbacks.write().unwrap();
        guard.push(hk);
        drop(guard);
        self
    }
    pub fn watch(&self)
    {
        let (sender, receiver) = std::sync::mpsc::channel();
        //if dropping previous receiver, set new sender
        if let Some(s) = SENDER.get()
        {
            let mut guard = s.write().unwrap();
            *guard = sender
        }
        else 
        {
            let _ = SENDER.set(std::sync::RwLock::new(sender));
        }
        let killer = self.kill.clone();
        Self::run_winhook();
        let callbacks = self.callbacks.clone();
        let mut cb_guard  = callbacks.write().unwrap();
        let callbacks = std::mem::replace(&mut *cb_guard, Vec::<HotKeyCallback>::new());
        drop(cb_guard);
        tokio::spawn(async move
        {
            let callbacks = Arc::new(callbacks);
            while let Ok(r) = receiver.recv()
            {
                let callbacks = Arc::clone(&callbacks);
                if killer.load(std::sync::atomic::Ordering::SeqCst)
                {
                    drop(receiver);
                    break;
                }
               'c: for callback in callbacks.iter()
                {
                    {
                        let active_keys = ACTIVE_KEYS.read().unwrap();
                        for k in &callback.keys
                        {
                            if !active_keys.contains(k)
                            {
                                continue 'c;
                            }
                        }
                    }
                    match &callback.func
                    {
                        HotKeyCallbackEnum::WithoutArg(f) =>
                        {
                            let f = Arc::clone(f);
                           // let s1 = tokio::spawn(async move 
                            //{
                                f().await;
                            //});
                            //let _ = s1.await;
                        },
                        HotKeyCallbackEnum::WithArg(f, a) =>
                        {
                            let f = Arc::clone(f);
                            let a = Arc::clone(a);
                            //let s1 = tokio::spawn(async move 
                            //{
                                f(a).await;
                            //});
                            //let _ = s1.await;
                        }
                    }
                }
                logger::debug!("pressed: {}", r);
            }
        });
    }
    fn run_winhook()
    {
        std::thread::spawn(move ||
        {
            unsafe
            {
                HOOK = null_mut();
                MOUSE_HOOK = null_mut();
                HOOK = SetWindowsHookExA(WH_KEYBOARD_LL, Some(hook_callback), null_mut(), 0);
                if HOOK.is_null()
                {
                    logger::error!("error register keyboard hook");
                    return;
                }
                MOUSE_HOOK = SetWindowsHookExA(WH_MOUSE_LL, Some(hook_callback), null_mut(), 0);
                if MOUSE_HOOK.is_null()
                {
                    logger::error!("error register mouse hook");
                    return;
                }
                let mut msg = std::mem::zeroed();
                //for process alive
                while GetMessageA(&mut msg, null_mut(), 0, 0) > 0
                {
                    TranslateMessage(&msg);
                    DispatchMessageA(&msg);
                }
            }
        });
    }
}

impl Drop for KeysWatcher
{
    fn drop(&mut self) 
    {
        self.kill.swap(true, std::sync::atomic::Ordering::SeqCst);
        unsafe 
        {
            UnhookWindowsHookEx(HOOK);
            UnhookWindowsHookEx(MOUSE_HOOK);
        }
    }
}