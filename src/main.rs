fn main() {
let result = get_users("yes");

match result {
    Ok(users)=> print_users(&users),
    Err(err)=> println!("{:?}", err)
}
    
}
#[derive(Debug)]

enum Role{
    Admin,_User
}
#[derive(Debug)]
struct User{
    name: String,
    email: String,
    role:Role
}
fn print_users(users: &[User]){
println!("{:?}", users);
}

fn get_users(input: &str)->Result<Vec<User>,String>{
    match input{
        "yes" =>  Ok ( vec![User{name: String::from("Jojo"),email: String::from("Jojo@gmail.com"),role:Role::Admin}]),
        other=> return Err(String::from(other)),

    }
}

