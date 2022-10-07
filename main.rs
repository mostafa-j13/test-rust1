use std::collections::VecDeque;
use std::error::Error;
use std::io::Read;

fn main() {
    let mut x = HolderImpl::new();
    loop {
        let mut buffer = String::new();
        let char_cnt = std::io::stdin().read_line(&mut buffer);
        if let Ok(char_cnt) = char_cnt {
            match x.add(&buffer.trim()){
                Ok(_)=>x.current_status(),
                Err(e)=>println!("error in input {}",e.msg)
            }

            println!("data size is {}",x.size().unwrap_or(0))

        }
    }
}

trait Holder {
    fn add(&mut self, new_data: &str)->Result<(),MyError>;
    fn current_status(&self);
}

struct HolderImpl {
    data: VecDeque<String>,
}

impl HolderImpl {
    pub fn new() -> Self {
        HolderImpl { data: VecDeque::new() }
    }

    pub fn size(&self)->Option<usize>{
        if self.data.is_empty() {
            None
        }else{
            Some(self.data.len())
        }
    }
}
struct MyError{
    msg:String
}

impl Holder for HolderImpl {
    fn add(&mut self, new_data: &str) ->Result<(),MyError>{
        if ! new_data.is_empty() {
            self.data.push_front(new_data.into());
            Ok(())
        }else{
            Err(MyError{msg:"string in empty".into()})
        }
    }

    fn current_status(&self)  {
        println!("{:?}", self.data);
    }


}