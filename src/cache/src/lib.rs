use std::{
    io::{Error},
    process::Command,
};

struct Cache {

}

pub fn update(self, bytez: u8) -> Result<bool, anyhow::Error> {
    let output = Command::new("./assets/services/cache")
        .arg(bytez.to_string())
        .output()
        .expect("Failed to exec");

    todo!()
}

pub fn init_cache(size: u32) {
    let output = Command::
        new("./cache")
       

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {}
}
