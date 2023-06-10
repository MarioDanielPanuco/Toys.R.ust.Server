use std::{io::Error, process::Command};

struct Cache {
    n: u32,
}

struct Item<T> {
    item: T,
    id: u32,
}

impl<T> Item<T> {
    pub fn new(new_item: T) -> Option<Self> {
        todo!();
        Some(Self {
            item: new_item,
            id: 100,
        })
    }
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
        let output = Command::new("./cache")
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
