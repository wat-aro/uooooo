# uooooo
うおおおおおおおおおおおおおおおお

## Usage

A brainf**k like programming language.

Usage: uooooo [FILE]

Arguments:
  [FILE]  program file

Options:
  -h, --help     Print help information
  -V, --version  Print version information

## Installation

``` shell
git clone https://github.com/wat-aro/uooooo
cd uooooo
cargo install
```

## Instruction

| instruction  | brainf**k | description       | C               |
|--------------|-----------|-------------------|-----------------|
| う           | >         | Increment pointer | ptr++;          |
| おおおう     | <         | Decrement pointer | ptr--;          |
| おおおおおお | +         | Increment value   | (*ptr)++;       |
| おおおおおう | -         | Decrement value   | (*ptr)--;       |
| おおおおう   | .         | Print character   | putchar(*ptr);  |
| おう         | ,         | Read caracter     | *p = getchar(); |
| おおうう     | [         | loop              | while(*ptr) {   |
| おおうお     | ]         |                   | }               |
