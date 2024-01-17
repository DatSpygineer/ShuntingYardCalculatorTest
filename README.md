# Shunting yard calculator test

This software is made for testing a shunting yard algorithm based calculator using the rust programming language.

## How to use?

Simply type in a mathematical expression and press enter to calculate.
```
>>> 3 * (3 + 1)
12
```

To close the software just type in `exit` into the command line.
```
>>> exit
```

### Operators

All standard operations are available
```
+ - * / % & | ^ ~ ! = ( ) < >
```

Python like exponent operator and floor division also available
```
** //
```

### Literal types

C/C++ style prefixes for different bases are supported
```
0b001001101 (binary)
0xFFA08 (hex)
0700 (C style octal)
0o700 (Python style octal)
```

You can use underscore for separating digits
```
1_000_000
```

### Keywords

You can assign values to variables by using the "set" keyword.
```
>>> set x <expression>
```

You can get the value of the variable anytime.
```
>>> set x 3 + 3
[x]: 6
>>> x * 2
12 
```

You can display the result in different base using "bin, oct, dec and hex" keywords.
```
>>> hex 16 * 2
0x20
>>> oct 2 ** 3
010
>>> bin 2 * 2 + 2
0b110
```