### Pentominoes Solver

A generic polyomino solver (mostly tested with pentominoes puzzles).

# Installation

First make the executable.

```
make
```

There is a web based GUI written in python using Flask.

To launch:

```
python gui.py
```

### Puzzle Format

Pieces should be separated by spaces. The largest area piece is 
considered as the board.

```

                            #
  ###        #    #     #   #
   #   # #   #    ##   ###  ## 
   #   ###   ###   ##   #   #

       #  #        #
   ##  #  #   ##   #  ##
  ##   #  #   ##  ##   #
   #   #  ##  #   #    ##
       #


  ########
  ########
  ########
  ###  ###
  ###  ###
  ########
  ########
  ########


```

### CLI Usage

```

  usage: ./pentominoes <filename> [--reflections=true|false]         
         [--rotations=true|false] [--help] [--solutions=0|..|n]       
         [--output=filename.txt] [--all-solutions]
                                                              
  options:                                                    
    all-solutions   toggle showing all solutions (including isometric ones)
    reflections     include reflections in the solution space 
    rotations       include rotations in the solution space   
    output          write the solutions to an output file, otherwise print to stdout
    solutions       number of solutions to look for (set to 0 to look for all possible)          
    help            print help and exit 

```
