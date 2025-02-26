mod hook;
mod keys;
mod error;


fn main() 
{
   let _ = logger::StructLogger::new_default();
   hook::start(); 
}