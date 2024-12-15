#!/bin/zsh

if [ -z "$1" ]; then
  echo "Usage: $0 XX"
  echo "XX should be the day number"
  exit 1
fi

if ! [[ $1 =~ ^[0-9]{2}$ ]]; then
  echo "XX should be a two digit number"
  exit 1
fi

DAY=$1

INPUT_FOLDER="inputs"
SRC_BIN_FOLDER="src/bin"
INPUT_FILE="$INPUT_FOLDER/input$DAY.txt"
RS_FILE="$SRC_BIN_FOLDER/Day$DAY.rs"

if [ ! -f "$INPUT_FILE" ]; then
  touch "$INPUT_FILE"
  echo "Input file created"
else
    echo "File already exists"
fi
if [ ! -f "$RS_FILE" ]; then
    cat << EOF > "$RS_FILE"
use std::fs::File;
use anyhow::Result;

fn main() -> Result<()> {
    let file = File::open("inputs/input$DAY.txt");

    Ok(())
}
EOF
    echo "Created file: $RS_FILE"
else
    echo "File already exists: $RS_FILE"
fi
