fn main() {

    let adult = Adult::new("Jojo",25);
    let child = Adult::new("Nana",9);

    match child{
        Ok(child) => println!("{} is {} years old",child.name,child.age),
        Err(e) => println!("{e}")
    }
    match adult{
        Ok(adult) => println!("{} is {} years old",adult.name,adult.age),
        Err(e) => println!("{e}")
    }
}
#[derive(Debug)]
 struct Adult{
    name: String,
    age:u8,
 }

 impl Adult {
    fn new(name:&str,age:u8) ->Result<Self,&str>{
          if age >= 21{
            Ok(Self{
                name: name.to_string(),
                 age,
            })
         }else{
            Err("No")  
        }}
      
 }