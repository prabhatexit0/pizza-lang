# Pizza Lang

Pizza Lang is a fun, experimental programming language designed to explore LLVM IR and JIT compilation. The syntax is inspired by pizza-related terminology.


## Variables

Variables in Pizza Lang are declared using the keyword topping.

**Syntax**

```
topping <variable_name> = <value>
```

**Example**

```
topping cheese = "mozzarella"
topping slices = 8
```

## Functions

Functions are declared using the keyword recipe.

**Syntax**

```
recipe <function_name>(<ingredients>) {
    // function body
}
```

**Example**

```
recipe bake(pizza) {
    print("Baking " + pizza)
}
```

## Control Strucutres

Pizza Lang supports basic control structures with pizza-themed terminology.

- If statements are declared using slice.
- Else statements are declared using extra.
- While loops are declared using oven.

**Example**

```
topping slices = 8

slice (slices > 4) {
    serve("More than half a pizza left")
} extra {
    serve("Less than half a pizza left")
}

topping slice_count = 0
oven (slice_count < slices) {
    serve(slice_count)
    slice_count = slice_count + 1
}
```

## Types

Types in Pizza Lang are declared using the keyword `cheese`.

**Syntax**

```
cheese <type_name> {
    // type definition
}
```

**Example**

```
cheese pizza {
    topping cheese
    topping slices
}
```

## Printing

Printing to the console is done using the serve function.

**Example**

```
serve("Pizza is ready!")
```

## Keywords

Pizza Lang uses the following pizza-themed keywords:

- `topping`: Declares a variable
- `recipe`: Defines a function
- `slice`: Begins an if statement
- `extra`: Introduces an else clause
- `oven`: Initiates a while loop
- `serve`: Outputs to the console
- `cheese`: Defines a custom type or struct

These keywords form the core of Pizza Lang's syntax, allowing you to create delicious code!
