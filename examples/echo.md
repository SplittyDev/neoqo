# Example: echo
In this example you'll learn how to work with input!   

The full source for this program is:
```vbnet
"In> "(;.);,"Out> "#[;[.[-]]#]
```

Here's a commented version:
```vbnet
'[Code]           ' [Description]
"In> "            ' push the string "In> " onto the stack
(                 ' -- loop while the top of the stack is not zero
  ;               '    pop the next character
  .               '    print the character
);                ' -- stop looping and pop the string terminator
,                 ' read until the user presses the [ENTER] key
"Out> "           ' push the string "Out> " onto the stack
#                 ' set the cell to the number of elements on the stack
[                 ' -- loop while the value of the cell is not zero
  ;               '    pop the next character
  [               '    -- loop while the character is not zero
    .             '       print the character
    [             '       -- loop while the cell is not zero
      -           '          decrement the cell
    ]             '       -- stop looping
  ]               '    -- stop looping
  #               '    set the cell to the number of elements on the stack
]                 ' stop looping
```

That's probably a little too much for a beginner,   
so let's break it up into small pieces!   

First, we see `"In> "(;.);`   
The `(;.);` is a common pattern in qo.
It takes values from the stack and prints them until a zero-value   
is encountered. Since strings always push a zero-value first,   
this is very convenient for printing a whole string.   

Next, we have the `,` (read) instruction.   
The read instruction can either read a single character assign the value of   
it to the current cell, or read a string with a specific max amount of   
characters and push them onto the stack, just like the `"string"` instruction.   
In this case, we are operating in `string-mode`, because the stack is either   
empty or the top value on the stack is zero. If the top value on the stack   
would be anything else, that value would be used as the number of characters   
to read.   

After that, the string `"Out> "` is pushed onto the stack, and the number   
of elements on the stack is assigned to the current cell using the   
count `#` operator.   

The pattern `#[;[.[-]]#]` is very useful for printing multiple strings.   
We already talked about the `#` instruction, so let's skip that and jump   
right into the first loop (`;[.[-]]#`). First, a character is popped from   
the stack. Then we enter another loop (`.[-]`). What that loop does should   
be pretty obvious: It prints (`.`) the current character and sets the cell to   
zero (`[-]`). The pattern `[.[-]]` is commonly used to print a character only   
if its value is not zero. We need that here, because the strings are separated   
by zero (qo string terminator) values, and of course we don't want them printed!   
After that, there's just one thing that remains: Another count (`#`)   
instruction which sets the cell to the (now updated) amount of items on the stack.   

So, to wrap it all up:   
The `#[;[.[-]]#]` pattern prints all strings on the stack and ignores zero values.   
It's not that hard, is it? :)
