# neoqo
A rusty approach to the qo programming language

[Documentation](https://splittydev.github.io/neoqo/neoqo/)

## About neoqo   
Neoqo is a rewrite of the esoteric programming language qo,   
which I came up with in 2015 while experimenting with lexical analyzers in C#.

The basic instruction set is based on Brainf*ck, with a stack and lots of cool features added.   
Qo was designed to make simple tasks hard to solve, but also fun.

Writing a qo program that actual does what it's supposed to is deeply satisfying (:

## Hello, world!
```vbnet
"Hello, world!" ' push string in reverse order
(               ' -- loop while the top of the stack is not zero
  ;             '    pop the next character
  .             '    print the character
);              ' -- /loop
[-]             ' clear the cell
++**-.          ' play a tone (ASCII BELL character)
```

## Instructions
| Opcode  | Description                                                 |
| ------- | ---------------------------------------                     |
| >       | Increments the cell pointer by one                          |
| <       | Decrements the cell pointer by one                          |
| +       | Increments the cell by one                                  |
| -       | Decrements the cell by one                                  |
| *       | Doubles the cell                                            |
| /       | Halves the cell                                             |
| :       | Pushes the cell onto the stack                              |
| ;       | Pops the top value off the stack, assigning it to the cell  |
| .       | Prints the cell                                             |
| ,       | Reads a character from `stdin`, assigning it to the cell    |
| &       | Duplicates the top stac value on the stack                  |
| \       | Swaps two items on the stack with each other                |
| #       | Counts the items on the stack, assigning the result to the cell |
| =       | Compares two items on the stack, setting the value of the cell<br>to either one (equal) or zero (not equal) |
| c       | Switch to character output mode (default)                   |
| i       | Switch to integer output mode                               |
| [ and ] | Loop while the cell is not zero                             |
| ( and ) | Loop while the top value on the stack is not zero           |

There are a few more instructions, which are not yet implemented.
