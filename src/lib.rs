#[macro_use] extern crate log;
extern crate zmq;


struct SpawnElement(bool, zmq::Socket);

pub use zmq::Result;

#[derive(Default)]
pub struct ReqSpawn{
    context:zmq::Context,
    spawns:Vec<SpawnElement>,
}


impl ReqSpawn{
    pub fn new()->Self{
        let ctx = zmq::Context::new();
        ReqSpawn{
            context:ctx,
            ..Default::default()
        }
    }

    pub fn connect(&mut self, endpoint:&str, count:u32){
        let c = count as i32 - self.spawns.len() as i32;
        if c<0{
            for i in 0..c.abs(){
                self.spawns.pop();
            }
        }else if c>0{
            for i in 0..c{
                let socket = self.context.socket(zmq::REQ).unwrap();
                socket.connect(endpoint).expect(format!("ReqSpawn::connect new req at {} failed", i).as_str());
                self.spawns.push(SpawnElement(false, socket));
            }
        }
    }

    pub fn send(&mut self, msg:&str, flag:i32) -> Result<()>{
        let mut res:Result<()> = Ok(());
        for item in self.spawns.iter_mut(){
            if !item.0{
                res = item.1.send(msg, flag);
                if res.is_ok(){
                    item.0 = true;
                    return res;
                }
            }
        }
        res = Err(zmq::Error::EBUSY);
        res
    }

    pub fn receive(&mut self, msecs:u32) -> Vec<String>{
        let mut res:Vec<String> = Vec::new();
        let mut residx:Vec<usize> = Vec::new();

        {
            let mut msg = zmq::Message::new();
            let mut items:Vec<zmq::PollItem> = Vec::new();

            for item in self.spawns.iter(){
                items.push(item.1.as_poll_item(zmq::POLLIN));
            };

            zmq::poll(&mut items, msecs as i64).unwrap();

            for i in 0..self.spawns.len(){
                if items[i].is_readable() && self.spawns[i].1.recv(&mut msg, 0).is_ok(){
                    if let Some(s) = msg.as_str(){
                        res.push(String::from(s));
                        residx.push(i);
                    }
                }
            }
        }

        for i in residx{
            self.spawns[i].0 = false;
        }

        return res;
    }
}
