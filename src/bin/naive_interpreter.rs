extern crate naive_hashmap;

#[macro_use]
extern crate afl;

fn main() {
    let mut hash_map = naive_hashmap::HashMap::new();

    fuzz!(|data: &[u8]| {
        if let Ok(line) = std::str::from_utf8(data) {
            let mut cmd = line.split(" ");
            match cmd.next() {
                Some("LOOKUP") => {
                    if let Some(key) = cmd.next() {
                        let _ = hash_map.get(key);
                    } else {
                        return;
                    }
                }
                Some("INSERT") => {
                    if let Some(key) = cmd.next() {
                        if let Some(val) = cmd.next() {
                            let _ = hash_map.insert(key.to_string(), val.to_string());
                        } else {
                            return;
                        }
                    }
                }
                _ => return,
            }
        }
    });
}