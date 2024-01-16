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

### Storing data

It is possible to store data in variables
```
>>> x = 3 * (3 + 1)
[x]: 12
>>> x + 2
14
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
00700 (C style octal)
0o700 (Python style octal)
```

You can use underscore for separating digits
```
1_000_000
```