#[cfg(target_os = "windows")]
mod hook;
mod keys;
mod mrc_test;
pub use keys::VirtualKey;
#[cfg(target_os = "windows")]
pub use hook::KeysWatcher;




#[cfg(target_os = "windows")]
#[cfg(test)]
mod tests
{
    use std::{sync::Arc, time::Duration};
    use crate::{hook::KeysWatcher, keys::VirtualKey};

    #[tokio::test]
    async fn run_test()
    {
        let _ = logger::StructLogger::new_default();
        let state = Arc::new(String::from("TEST_STATE"));
        let mut key_watcher = KeysWatcher::new();
        key_watcher
        .register(&[VirtualKey::LeftCtrl, VirtualKey::LeftAlt], callback_1)
        .register(&[VirtualKey::F5, VirtualKey::MouseLeftClick], callback_2)
        .register_with_state(&[VirtualKey::LeftCtrl, VirtualKey::RightCtrl], state, callback_3)
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