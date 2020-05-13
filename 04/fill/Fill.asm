// This file is part of www.nand2tetris.org
// and the book "The Elements of Computing Systems"
// by Nisan and Schocken, MIT Press.
// File name: projects/04/Fill.asm

// Runs an infinite loop that listens to the keyboard input.
// When a key is pressed (any key), the program blackens the screen,
// i.e. writes "black" in every pixel;
// the screen should remain fully black as long as the key is pressed.
// When no key is pressed, the program clears the screen, i.e. writes
// "white" in every pixel;
// the screen should remain fully clear as long as no key is pressed.

// Put your code here.

// keypress = 0
@keypress
M=0

// fill
@fill
M=0

(LOOP)
  // read key pressed
  @KBD
  D=M

  // set keypress state
  @SET_KEYPRESS_FILL
  D;JGT

  // unset keypress
  @keypress
  M=0

  @SET_KEYPRESS_END
  0;JMP

(SET_KEYPRESS_FILL)
  @0
  D=!A
  @keypress
  M=D

(SET_KEYPRESS_END)
  // jump to LOOP if fill - keypress == 0
  @fill
  D=M
  @keypress
  D=D-M
  @LOOP
  D;JEQ

  // fill = keypress
  @keypress
  D=M
  @fill
  M=D

  // fill screen
  // i = 0
  @i
  M=0

  // i = i + 16384(0x4000)
  @SCREEN
  D=A
  @i
  M=D+M

(SCREEN_LOOP)
  // fill screen
  @fill
  D=M
  @i
  A=M
  M=D

  // i = i + 1
  @i
  M=M+1
  D=M

  // jump to SCREEN_LOOP if 24576 (0x6000) - i >= 0
  @KBD
  D=D-A

  @SCREEN_LOOP
  D;JLT

  // infinite loop
  @LOOP
  0;JMP

