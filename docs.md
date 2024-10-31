# Documentation

## Variable Types

- ```Integer``` A 32-bit signed integer.
```1, 4, 1898, -45```
- ```Float``` A 32-bit signed floating-point number.
```0.0, 1.50924, -32958.1```
- ```String``` A string (collection of characters). Note: GScript Strings are not references to allocated memory, they instead own the memory.
```"Hello World!"```
- ```Boolean``` Either true or false.
```true, false```
- ```List_Obj``` A collection of elements. The elements can be different types. Nested lists are supported.
To assign, use this syntax: ```assign a = [1,2,[3,4]];```
To index, use this syntax: ```a[2][1];```
- ```Obj``` An instance of a blueprint. Can contain properties (which are other variables) and methods (which are functions).
To assign, use this syntax: ```assign a = new Thing(prop1, prop2);```
To access members, use this syntax: ```a.method(); a.prop1;```

## Structures

- ```Function``` A block of code with a name that accomplishes a specific task. Can sometimes be called methods or procedures.

## Keywords

- ```assign``` Creates a variable, also requiring a value to be provided.
- ```funct``` Declares a function.
- ```if``` Runs contained code if the condition inside ```()``` evaluates to ```true```
- ```param``` Declares a parameter of a function within the function definition
- ```return``` Returns a value from a function
- ```blueprint``` Defines a blueprint (class)
- ```new``` Used for creating an instance of a blueprint
- ```while``` Runs contained code as long as the condition inside ```()``` evaluates to ```true```
- ```break``` Breaks out of a loop
<br> More to come...

## Standard Functions

- ```ast_debug(args<AnyType>...)``` Prints out debug information about provided argument(s) in the form of its AST Node representation's fields
- ```write(args<AnyType>...)``` Prints out provided arguments(s) to the standard output, generally the console
- ```read()``` Prompts the user for a line from the standard input stream
- ```type(arg1<AnyType>)``` Returns the type of arg1 as a String
- ```to_int(arg1<String>)``` Converts arg1, a String, into its integer representation
- ```to_float(arg1<String>)``` Converts arg2, a String, into its floating-point representation
- ```random_int(arg1<Integer>, arg2<Integer>)``` Returns a random integer between arg1 and arg2, inclusive

## Standard Errors

- Syntax Error
- Divide By Zero Error
- File Error
- Token Error
- End Of Input Error
- Variable Definition Error
- Function Definition Error
- Function Error
- Conditional Error
- Type Error
- List Error
- Blueprint Error
- Identifier Error

## Other

For examples on writing GScript code, take a look at ```entry/examples.gsc```
