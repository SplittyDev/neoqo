```vbnet
' Let's start with a simple one:
' "Hello, World!"(;.);
'
' Explanation:
' Push the string to the stack in reverse order,
' then keep popping and printing characters one by one
' until the top value on the stack is zero (ASCII NUL).
'
'[Code]           ' [Description]
"Hello, World!"   ' push string in reverse order
(                 ' -- loop while the top of the stack is not zero
  ;               '    pop the next character
  .               '    print the character
);                ' -- /loop and pop the string terminator

'
' Print a newline character:
' ++**++.:
'
' Explanation:
' Manipulate the cell so its value is 10 (ASCII NEWLINE),
' then print the character and push its value onto the stack.
'
'[Code]           ' [Description]
++**++            ' 0 <- +1, +1, *2, *2, +1, +1 = 10
.                 ' print the character (10 = ASCII Newline)
:                 ' push the character

'
' This is the same mechanism that we used in the first example,
' but this time using a cell-conditional loop:
' "Hello, World!"+[;.]
'
' Explanation:
' Push the string to the stack in reverse order,
' then keep popping and printing characters one by one,
' until the value of the cell is zero (ASCII NUL).
' In this example we don't need to explicitly pop the string terminator,
' that is done automatically this time,
' because of the way we constructed the loop.
'
'[Code]           ' [Description]
"Hello, World!"   ' push string in reverse order
+[                ' -- increment cell by one and start looping
  ;               '    pop the next character
  .               '    print the character
]                 ' -- /loop

'
' Let's start a new line again:
' ;.
'
' Explanation:
' We previously pushed the ASCII value 10 onto the stack.
' Now we just have to pop it off the stack and print it!
'
'[Code]           ' [Description]
;                 ' pop the newline character
.                 ' print the character

'
' Doing it the brainf*ck way (kind of):
' [-]++++++++[>++++[>++>+++>+++>+<<<<-]
' >+>+>->>+[<]<-]>>.>---.+++++++..+++.>
' /.*>.<-.<.+++.------.--------.>>+.>++.
'
' Explanation:
' Obviously I'm not going to comment this line-by-line.
' All this does is calculating the ASCII values of the string
' by hand and printing the characters one by one, using different cells
' instead of the stack.
'
[-]++++++++[>++++[>++>+++>+++>+<<<<-]
>+>+>->>+[<]<-]>>.>---.+++++++..+++.>
/.*>.<-.<.+++.------.--------.>>+.>++.

' Run the program by invoking the following command in the base directory:
' $ cargo run "examples/hello_world.qo"
'
' If all went well, you should see the following output:
' Hello, World!
' Hello, World!
' Hello, World!
'
' I hope that this helped you understanding neoqo a little better.
' Have a nice day and happy coding!
```