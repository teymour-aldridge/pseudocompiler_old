# Pseudocompiler
A compiler for pseudocode, based on the [OCR specification](https://https://www.ocr.org.uk/Images/202654-pseudocode-guide.pdf) [[backup link](https://web.archive.org/web/20200118155656/https://www.ocr.org.uk/Images/202654-pseudocode-guide.pdf)].

## Scope of work
> Turn pseudocode into Javascript code.

## Language specification
### Variable assignment
```
<identifier> = <expression>
```
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
#### Initialisation
```
<identifier> = [<expression>, <expression>, ..., <expression>]
```
#### Assignment
```
<identifier>[<expression>] = <expression>
```