use std::{env, fs};

struct Md5Result(String);

impl Md5Result {
    fn new(data: String) -> Md5Result {
        if data.len() != 32 {
            panic!("String must have 32 characters")
        }

        Md5Result { 0: data }
    }

    fn get(&self) -> String {
        self.0.clone()
    }
}

fn calculate_md5(data: &[u8]) -> Md5Result {
    let digest: md5::Digest = md5::compute(data);
    let result = Md5Result::new(format!("{:x}", digest));

    result
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let files = &args[1..];

    for filename in files {
        let contents = fs::read(filename).expect("Something went wrong reading the file");
        let md5sum = calculate_md5(&contents);
        println!("{} {}", md5sum.get(), filename);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn abcdefghijklmnopqrstuvwxyz() {
        let digest = calculate_md5(b"abcdefghijklmnopqrstuvwxyz");
        assert_eq!(digest.get(), "c3fcd3d76192e4007dfb496cca67e13b");
    }

    #[test]
    fn helloworld() {
        let digest = calculate_md5(b"helloworld");
        assert_eq!(digest.get(), "fc5e038d38a57032085441e7fe7010b0");
    }
}
