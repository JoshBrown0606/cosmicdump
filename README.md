# cosmicdump
This language is named as such because if you have to program in it it's as if the universe took a large dump on your fate. Program at your own risk.

Cosmicdump focuses on manipulating a tape of unsigned integers. The first four elements of this tape have special values (hereby called "reserved value") used by specific commands respectively: addition/subtraction, traversing the tape, jumping, and conditional statements. 

All reserved values are set to 1, and the current element of the tape begins at its 0th index (the addition/subtraction reserve value).
  
The `+` operator increments the current element of the tape by its reserved value, and the `-` operator decrements the current element of the tape by its reserved value.

The `>` operator makes the current element of the tape increase by its reserved value to a greater index. The `<` operator makes the current element of the tape decrease by its reserved value to a lesser index.

The `}` operator jumps ahead a number of operators in your code equal to its reserved value, while the `{` operator jumps backward the same number of operators in your code.

The final operator, `!`, makes it so that the next operator will only be executed if the current element of the tape is equal to the `!` operator's reserved value.

Any characters that are not operators are considered comments.

###  *I have no example code for this language as I am too lazy to write it.*
