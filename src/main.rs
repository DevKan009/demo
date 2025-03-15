fn main(){
    let s1:String = get_string();// s1=hello
    println!("this is s1:{}",s1);
    
    let s2:String= String::from("world");//s2=world
    let s3:String= send_get_string(s2);//recieved_string is new owner of world while s2 is invalidated;
    //s3 = new owner of world
    println!("this is s2:{}",s2);
    println!("this is s3:{}",s3);//world gets printed;
}

fn get_string()-> String{
    let new_string:String = String::from("hello");
    return new_string;
}
fn send_get_string(recieved_string:String) ->String {
    return recieved_string;
}
