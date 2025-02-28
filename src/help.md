Здравствуйте, подскажите, явно что-то делаю не так, почему то код дедлочится...
```
std::thread::spawn(move ||
    {
        ...
        HotKeyCallbackEnum::WithArg(f: Arc<dyn Fn(Box<dyn Any + Send + Sync>) -> Pin<Box<dyn Future<Output = ()> + Send>> + Send + Sync>, a: Arc<dyn Any + Send + Sync>) =>
        {
        
            let f = f.to_owned();
            futures::executor::block_on(async {f(Box::new(a)).await});
            logger::info!("На предыдущем шаге дедлок... или что-то еще но до сюда не доходит...");
        }
    });
```
на том конце просто пустая функция вызывается
причем чуть выше, немного отличается сигнатура но нормально отрабатывает:
``` 
HotKeyCallbackEnum::WithoutArg(f: Arc<dyn Fn() -> Pin<Box<dyn Future<Output = ()> + Send>> + Send + Sync>) =>
{
    futures::executor::block_on(async  {f().await});
},
```