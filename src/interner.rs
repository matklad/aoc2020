use std::{collections::HashMap, mem};

pub struct Interner {
  map: HashMap<&'static str, u32>,
  vec: Vec<&'static str>,
  buf: String,
  full: Vec<String>,
}

impl Default for Interner {
  fn default() -> Self {
    Interner::with_capacity(128)
  }
}

impl Interner {
  pub fn with_capacity(cap: usize) -> Interner {
    let cap = cap.next_power_of_two();
    Interner {
      map: HashMap::default(),
      vec: Vec::new(),
      buf: String::with_capacity(cap),
      full: Vec::new(),
    }
  }

  pub fn intern(&mut self, name: &str) -> u32 {
    self.get_or_intern(name).0
  }

  pub fn get_or_intern(&mut self, name: &str) -> (u32, bool) {
    if let Some(&id) = self.map.get(name) {
      return (id, false);
    }
    let name = unsafe { self.alloc(name) };
    let id = self.map.len() as u32;
    self.map.insert(name, id);
    self.vec.push(name);

    debug_assert!(self.lookup(id) == name);
    debug_assert!(self.intern(name) == id);

    (id, true)
  }

  pub fn lookup(&self, id: u32) -> &str {
    self.vec[id as usize]
  }

  unsafe fn alloc(&mut self, name: &str) -> &'static str {
    let cap = self.buf.capacity();
    if cap < self.buf.len() + name.len() {
      let new_cap = (cap.max(name.len()) + 1).next_power_of_two();
      let new_buf = String::with_capacity(new_cap);
      let old_buf = mem::replace(&mut self.buf, new_buf);
      self.full.push(old_buf);
    }

    let interned = {
      let start = self.buf.len();
      self.buf.push_str(name);
      &self.buf[start..]
    };

    &*(interned as *const str)
  }
}
