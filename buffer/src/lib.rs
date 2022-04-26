use std::collections::HashMap;

static mut S_BUFFER: Option<Box<HashMap<String, String>>> = None;

pub fn get(k: String) -> Option<String> {
    new();
    unsafe {
        let map = S_BUFFER.as_ref().unwrap();
        match map.get(&k) {
            None => None,
            Some(v) => Some(v.clone())
        }
    }
}

pub fn set(k: String, v: String) {
    new();
    unsafe {
        let map = S_BUFFER.as_mut().unwrap();
        map.insert(k, v);
    }
}


pub fn remove(k: String) -> Option<String> {
    new();
    unsafe {
        let map = S_BUFFER.as_mut().unwrap();
        map.remove(&k)
    }
}


fn new() {
    unsafe {
        if S_BUFFER.is_none() {
            S_BUFFER = Some(Box::new(HashMap::new()))
        }
    }
}