#[allow(dead_code)];
#[feature(phase)];
#[phase(syntax, link)] extern crate log;
extern crate collections;

use std::os;
use std::vec;
use parse::parseFile;
use pentomino::Pentomino;
use solve::{generatePlacements, solve};
use cmd::{OptionParser, BoolOption, StrOption,
          ToggleOption, UintOption}; 


mod cmd;
mod solve;
mod parse;
mod pentomino;


/// Finds the Board in a vector of pentominoes, and removes it
/// and returns it.
#[inline]
pub fn discoverBoard(pentominoes: &mut Vec<Pentomino>) -> Pentomino {
  let mut index = 0;
  let mut max = 0;
  
  for (i, pentomino) in pentominoes.iter().enumerate() {
    if pentomino.area() > max { 
      max = pentomino.area();
      index = i; 
    }
  }

  match pentominoes.remove(index) {
    Some(piece) => piece,
    None => fail!("no board to remove")
  }
}


/// Help Text
static USAGE_TEXT: &'static str = "
  usage: ./pentominoes <filename> [--reflections=true|false]         
         [--rotations=true|false] [--help] [--solutions=0|..|n]       
         [--output=filename.txt]
                                                              
  options:                                                    
    reflections     include reflections in the solution space 
    rotations       include rotations in the solution space   
    output          write the solutions to an output file, otherwise print to stdout
    solutions       number of solutions to look for (set to 0 to look for all possible)          
    help            print help and exit 
";


fn main() {
  let args = os::args();
  let mut parser = OptionParser::new();

  parser.addOption(~"rotations", BoolOption(true));
  parser.addOption(~"reflections", BoolOption(true));
  parser.addOption(~"output", StrOption(~""));
  parser.addOption(~"help", ToggleOption(false));
  parser.addOption(~"solutions", UintOption(0));

  // Not enough arguments supplied
  if !(args.len() > 1) { println!("{:s}", USAGE_TEXT); return }

  parser.parse(args.slice_from(2));

  // Help option supplied
  if parser.getBoolOption("help") { println!("{:s}", USAGE_TEXT); return }

  let path = Path::new(args[1]);
  let mut pentominoes = parseFile(&path);
  let board = discoverBoard(&mut pentominoes);

  // Begin Solving
  let mut solutions: uint = 0;
  let (mut cols, mut placements) = generatePlacements(&board, &pentominoes, 
                                                      parser.getBoolOption("rotations"), 
                                                      parser.getBoolOption("reflections"));

  println!("{:u}x{:u} Board", board.dimX, board.dimY);
  println!("Columns: {:u}", cols.len());
  println!("Rows: {:u}", placements.len()); 
  println!("Target: {:u}", board.size());

  //solve(&mut placements, &mut vec::with_capacity(pentominoes.len()), 
  //      board.size(), 0, &mut solutions);

  println!("Solutions Found: {:u}", solutions);
}
