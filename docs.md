# Key Words

Key words are used to denote what a line will do.

## Fn
`fn` is the beginning of a function, followed by a name and the registers and their types that will be passed in. The return type is defined at the end of the function creation using the `:` operator after the arguments followed by the return type.

```
; Function with no arguments
fn my_func

; Function with arguments
fn my_arg_func i32 reg1 i64 reg2

; Function with ret value
fn my_arg_func : i32
```

## End Fn
`endfn` marks the end of a function, the register to return is placed after the end fn keyword like `endfn reg1`

## Let
`let` is the naming of virtual register, wrapping parenthesies around any basic operation will create a virtual register.

```
; Immediate vreg creation
let my_reg (i32 1)

; From another vreg
let reg2 (i32 my_reg)

; From operation
let reg3 (i32 my_reg + 2)
```

## Const
`const` is placed outside of the running code and is a constant value that can be used anywhere in code, created as `const my_const 1`

## Call
`call` is used to call a defined function, with the name coming after followed by any arguments for the call, then followed by `: reg_name`, which acts as a way for the return value of the call to be used in the future. Note that any returning function must be given a return reg, _ can be used for a return value you don't want to be accessible

```
fn my_fn
; do stuff
endfn

fn my_arg_fn i32 reg1
; do stuff with reg1
endfn

fn my_returning_fn : i32
; do stuff
endfn (1)

; fn call with no arguments
call my_fn

; fn call with arguments
let my_reg (i32 2)
call my_arg_fn my_reg

; Using a function return value
call my_returning_fn : new_vreg_name
let reg3 (i32 new_vreg_name + 2)

; Calling a returning function and not using return value
call my_returning_fn : _
```

## Label
`label` is used to define a spot that can be jumped to.

## Jump
`jmp` is used to jump to a label. Jumping requires a boolean value. If you want an unconditional jump, you can just make that value always true. Its usage is as follows

```
let my_boolean (bool 0 == 0)

jmp my_boolean option_one
label option_one
; do stuff
label option_two
; do stuff
```

# Types

Types allow more compression of the stack, while also providing safety and easier code comprehension. All type casting is explicit.

## Casting
Casting is done using the `as` operator. e.g. `(my_u32 as i32)`. For information on what can be cast into what, look at the type's section.

## Signed Integer Types
Signed integer types include types `i8`, `i16`, `i32`, `i64`, and `isize`. All signed integer types can be cast into other sizes of integer types, unsigned integer types, and float types.

## Unsigned Integer Types
Unsigned integer types include types `u8`, `u16`, `u32`, `u64`, and `usize`. All unsigned integer types can be cast to integer types, other sized unsigned integer types, and float types. `usize` can also be cast to a pointer.

# Float types
Floats include types `f32` and `f64`. All float types can be can be cast to signed and unsigned integer types, aswell as other sizes of float types. Note that casting a float to an integer is a truncating operation that will loose data after the decimal place.

## Boolean Type
`bool` is the boolean type. It can be assigned via `true` and `false` values.

## Pointer
`ptr` is the pointer type. It is used as `ptr type`. e.g. `ptr i32`. `ptr` can be cast into `usize`

## Label
`_labelt` is a type used exclusivly for jumping. It cannot be assigned to a register.

# Operators
Operators are used to run expressions. All expressions are within paretheses. Any expression can only contain two operands.

## Add
The `+` operand is used to add two numbers. It can be used between any two unsigned or signed integer types or float types. `+` is used like `(my_reg + my_2nd_reg)`

## Sub
The `-` operand is used to subtract two numbers. It can be used between any two unsigned or signed integer types or float types. `-` is used like `(my_reg - my_2nd_reg)`

## Mul
The `*` operand is used to multiply two numbers. It can be used between any two unsigned or signed integer types or float types. `*` is used like `(my_reg * my_2nd_reg)`

## Div
The `/` operand is used to divide two numbers. It can be used between any two unsigned or signed integer types or float types. `/` is used like `(my_reg / my_2nd_reg)`

## Mod
The `%` operand is used to get the remainder of division of two numbers. It can be used between any two unsigned or signed integer types. `%` is used like `(my_reg % my_2nd_reg)`

## And
The `&` operator is the and operator. It can be used on signed and unsigned integers to act as a bitwise and, or it can be used on a boolean value to act as a logical and.

## Or
The `|` operator is the or operator. It can be used on signed and unsigned integers to act as a bitwise or, or it can be used on a boolean value to act as a logical or. 

## Xor
The `^` operator is the xor operator. It can be used on signed and unsigned integers to act as a bitwise xor.

## Not
The `!` operator is the not operator. It can be used on signed and unsigned integers to act as a bitwise not, or it can be used on a boolean value to act as a logical not. 

## As
The `as` operand is used to cast a type to another type. What types can be cast to what can be seen in the **Types** section.