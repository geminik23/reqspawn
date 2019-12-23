
use reqspawn::*;

fn main(){
    let mut rs = ReqSpawn::new();
    rs.connect("tcp://127.0.0.1:1234", 3);
    rs.send("Hello", 0).unwrap();
    
    loop{
        let mut result:Vec<String> = Vec::new();
        while result.len() == 0{
            result = rs.receive(0);
        }

        for item in result.iter(){
            println!("{:?}", item);
        }
    }
    
}
