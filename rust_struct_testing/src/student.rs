use crate::major::Major;
mod major;

#[derive(Debug)]
pub struct Student {
    name:String,
    major::Major,
}

impl Student {
    pub fn new(name:&str,major:&str) -> Student {
        Student {
            name:name.to_string(),
            major::Major::calssify(major),
            
        }
    }
}