# Pseudocompiler
A compiler for pseudocode, based on the [OCR specification](https://https://www.ocr.org.uk/Images/202654-pseudocode-guide.pdf) [[backup link](https://web.archive.org/web/20200118155656/https://www.ocr.org.uk/Images/202654-pseudocode-guide.pdf)].

## Scope of work
> Turn pseudocode into Javascript code.

## Language specification
### Variable assignment
```
<identifier> = <expression> // a mutable variable
global <identifier> = <expression> // a mutable global variable
```
#### Variable types
1. Numbers
    1. Integers
    2. Floating-point numbers
2. Strings
3. Arrays
### Loops
#### While loop
```
while <expression>
    ...
endwhile
```
#### For loop
```
for <identifier> in <expression>
    ...
end for
```
### If
```
if <expression>
    ...
endif
```
### Functions
#### Definition
```
function <identifier>(<identifier>, <identifier>, ..., <identifier>)
    ...
endfunction
```
#### Calling
```
<identifier>(<identifier>, <identifier>, ..., <identifier>)
```
### Arrays
### Built-in functions
#### Casting
```
str(<expression>) // Converts a value to a string
int(<expression>) // Converts a value to an integer
float(<expression>) // Converts a value to a floating-point integer
```
#### IO
1. File handling – **not planned to be implemented in the short term**
2. stdin/sdout
    1. ```input(<expression>)``` – reads an input 
    2. ```print(<expression>)``` – logs an output

#### Initialisation
```
<identifier> = [<expression>, <expression>, ..., <expression>]
```
#### Assignment
```
<identifier>[<expression>] = <expression>
```
