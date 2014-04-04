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


/// Basic options parser (Only parses very simple options)
///
/// Attributes
///   * defs - A HashMap of option names and the 
///           defaults for those options 
///   * vals - A HashMap of option names and the
///            value that the option holds
///
pub struct OptionParser {
  defs: HashMap<~str, OptionType>,
  vals: HashMap<~str, OptionType>
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

      if self.vals.contains_key(&key) {
        match self.vals.get(&key) {
          &StrOption(_) => match val {
            Some(s) => { self.vals.insert(key, StrOption(s)); },
            _ => fail!("expected an option value")
          },
          &BoolOption(_) => match val {
            Some(s) => match from_str(s) {
              Some(b) => { self.vals.insert(key, BoolOption(b)); },
              _ => fail!("expected a valid boolean value")
            },
            _ => fail!("expected an option value")
          },
          &UintOption(_) => match val {
            Some(s) => match from_str(s) {
              Some(i) => { self.vals.insert(key, UintOption(i)); },
              _ => fail!("expected a valid uint value")
            },
            _ => fail!("expected an option value")
          },
          &ToggleOption(b) => match val {
            None => { self.vals.insert(key, ToggleOption(!b)); },
            _ => fail!("did not expect a value")
          }
        }
      } else {
        warn!("{:s} not a valid option", key)
      }
    }
  }
  pub fn addOption(&mut self, k: ~str, t: OptionType) {
    self.defs.insert(k, t);
  }
}


// Getters
impl OptionParser {
  pub fn getBoolOption(&self, k: &str) -> bool {
    match self.vals.get(&k.to_owned()) {
      &BoolOption(b) | 
      &ToggleOption(b) => b,
      _ => fail!("`{:?}` not a boolean option", k)
    }
  }
  pub fn getUintOption(&self, k: &str) -> uint {
    match self.vals.get(&k.to_owned()) {
      &UintOption(i) => i,
      _ => fail!("`{:?}` not a uint option", k)
    }
  }
  pub fn getStrOption<'a>(&'a self, k: &str) -> &'a ~str {
    match self.vals.get(&k.to_owned()) {
      &StrOption(ref s) => s,
      _ => fail!("`{:?}` not a boolean option", k)
    }
  }
}
