use collections::hashmap::HashMap;


#[deriving(Clone, Show)]
pub enum OptionType {
  StrOption(~str),
  BoolOption(bool),
  UintOption(uint),
  ToggleOption(bool)
}


/// Limited way of parsing options,
/// can only parse options formatted like:
///
///   * --long-option=<X>
///   * -s=<X>
///   * --long-option
///   * -s
///
fn parseOption(opt: &~str) -> (~str, Option<~str>){
  if opt.len() < 2 { fail!("invalid option specified") }

  let mut pos = 0;
  let mut key = ~"";
  let mut val = ~"";

  for c in opt.chars() {
    pos += 1;

    if c == '-' && pos < 3 { continue }
    if c == '=' { break } 

    key.push_char(c);
  }

  for c in opt.slice_from(pos).chars() {
    val.push_char(c);
  }

  if val.len() > 0 {
    (key, Some(val))
  } else {
    (key, None)
  }
}


// A static string to initialize pointers to...
static DEAD: &'static str = ""; 


/// Basic options parser (Only parses very simple options)
///
/// Attributes
///   * defs - A HashMap of option names and the 
///           defaults for those options 
///   * vals - A HashMap of option names and the
///            value that the option holds
///
pub struct OptionParser {
  defs: HashMap<&'static str, OptionType>,
  vals: HashMap<&'static str, OptionType>
}


// Creation
impl OptionParser {
  pub fn new() -> OptionParser {
    OptionParser { defs: HashMap::new(), vals: HashMap::new() }
  }
  pub fn parse(&mut self, args: &[~str]) {
    self.vals.clone_from(&self.defs);

    for arg in args.iter() {
      let (key, val) = parseOption(arg);

      if self.vals.contains_key_equiv::<~str>(&key) {
        // Find the equivalent key in the HashMap...
        //
        // Unfortunately, there is no other way of 
        // getting a mutable value with an equivalent 
        // key
        let equivKey = {
          let mut equivKey = &DEAD;

          for k in self.defs.keys() {
            if k.equiv(&key) { equivKey = k; break; }
          }

          assert!(*equivKey != DEAD)

          equivKey
        };

        // Set the old value to the new one
        match self.vals.get_mut(equivKey) {
          &StrOption(ref mut s) => match val {
            Some(s0) => *s = s0, 
            _ => fail!("expected an option value")
          },
          &BoolOption(ref mut b) => match val {
            Some(s) => match from_str(s) {
              Some(b0) => *b = b0, 
              _ => fail!("expected a valid boolean value")
            },
            _ => fail!("expected an option value")
          },
          &UintOption(ref mut i) => match val {
            Some(s) => match from_str(s) {
              Some(i0) => *i = i0, 
              _ => fail!("expected a valid uint value")
            },
            _ => fail!("expected an option value")
          },
          &ToggleOption(ref mut b) => match val {
            None => *b = !*b, 
            _ => fail!("did not expect a value")
          }
        }
      } else {
        warn!("{:s} not a valid option", key)
      }
    }
  }
  pub fn addOption(&mut self, k: &'static str, t: OptionType) {
    self.defs.insert(k, t);
  }
}


// Getters
impl OptionParser {
  pub fn getBoolOption(&self, k: &'static str) -> bool {
    match self.vals.get(&k) {
      &BoolOption(b) | 
      &ToggleOption(b) => b,
      _ => fail!("`{:?}` not a boolean option", k)
    }
  }
  pub fn getUintOption(&self, k: &'static str) -> uint {
    match self.vals.get(&k) {
      &UintOption(i) => i,
      _ => fail!("`{:?}` not a uint option", k)
    }
  }
  pub fn getStrOption<'a>(&'a self, k: &'static str) -> &'a ~str {
    match self.vals.get(&k) {
      &StrOption(ref s) => s,
      _ => fail!("`{:?}` not a boolean option", k)
    }
  }
}
