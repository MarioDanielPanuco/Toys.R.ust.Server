use std::{
    io::{Error},
    process::Command,
};

struct Cache {

}

impl Cache {
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
            .arg("-N")
            .arg("10") // Size: 10
            .arg("-L"); // LRU
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {}
}
