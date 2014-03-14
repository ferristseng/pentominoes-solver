SRC 	= src
TEST 	= test
BUILD = build

OPT_LEVEL = 3
FLAGS = --opt-level=$(OPT_LEVEL)

FILES = main.rs parse.rs pentomino.rs \
				solve.rs
SOURCES = $(addprefix $(SRC)/, $(FILES))

all: $(BUILD)/main

$(BUILD)/main: $(SOURCES)
	rustc $(FLAGS) -o $(BUILD)/main $(SRC)/main.rs

clean:
		rm -r build/

