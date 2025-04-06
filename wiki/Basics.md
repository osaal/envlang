# Envlang

`Envlang` is a functional-style programming language where everything is an environment.

Features of `Envlang`:
- Static typing and constant assignments
- Pure, isolated functions: always an input, always an output, and no side-effects
- Functional-style chaining and piping of inputs/outputs

## Resources for learning Envlang

This document tries to explain the basics of how Envlang functions. During alpha development, not every functionality might be implemented -- check [STRUCTURE.md](STRUCTURE.md) for a more up-to-date list of implemented functionality.

The document [grammar.ebnf](grammar.ebnf) presents the Envlang grammar in Extended Backus-Naur Form. Note, that the file is a descriptive document, and is not actually used to generate the lexer and/or parser -- please leave an Issue if the grammar deviates from actual behaviour in Envlang.

## Data Types

`Envlang` has the following data types:

| Type    | Explanation                                                                                                                                                                                                                                                                            | Syntax example                                                                                          |
|---------|----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|---------------------------------------------------------------------------------------------------------|
| String  | Text strings, conforming to Unicode letters, digits, and whitespace<br>Strings can be single- or double-quoted<br>Strings must escape all special symbols                                                                                                                              | "Hello!"<br>"hunter2"<br>'Single-quoted string'<br>"String with<br>line break"<br>"🐈 is a valid string" |
| Integer | Signed integers, or the real numbers<br>Limited by the system's maximum signed integer value<br>Leading zeros are stripped for the purposes of arithmetic                                                                                                                              | 5<br>3975<br>-364<br>092                                                                                |
| Float   | Floating-point numbers, or decimal numbers<br>Limited by the maximum and minimum 64-bit signed floating-point number<br>Leading zeros before the decimal point, and trailing zeros after the final non-zero digit after the decimal point, are stripped for the purposes of arithmetic | 5.0<br>07.3<br>3.140000<br>-67.2                                                                        |
| Bool    | Boolean truth values<br>Can be either 'true' or 'false'<br>Booleans are case-sensitive, so 'FALSE' will be treated as a string                                                                                                                                                         | true<br>false                                                                                           |                                                                                       |

## Environments

The basic building block of `Envlang` is the **environment**. An environment is defined by braces:

```
{   # Start of environment
    # Things go here
}   # End of environment
```

Environments control scope: Anything inside the environment is available to use, anything outside the environment is not (with a few exceptions).

Environments are evaluated immediately (except function environments; see below), meaning that anything inside the environment is executed. Once the evaluation is complete, the Environment is discarded, along with any data inside it:

```
{                   # `a` is not accessible, as it has not been defined yet
    let a = 5;      # `a` is assigned the value 5, making `a` accessible inside the environment
}                   # `a` goes out of scope and is destroyed
                    # `a` is no longer accessible
```

Environments can be anonymous or named. **Anonymous environments** are unassigned, meaning that they cannot be accessed after execution. In the above example, the environment declared with the braces is anonymous, because it was never assigned to an identifier.

**Named environments** are assigned to identifiers, and thus not destroyed after evaluation. Their members can be accessed later using the accessor symbol `.`:

```
let env = { 
    let a = 5;      # `a` is assigned the value 5, making `a` accessible inside the environment
}                   # `a` stays accessible through `env` because of the named environment

let b = env.a + 5;  # `b` evaluates to 10
```

**Maxim 1**
: Everything is an environment.

In fact, the above code could be written more verbosely to reveal this fact:

```
{
    let env = { let a = { 5 } }
    let b = { env.a + 5 }
}
```

This code is functionally equivalent to the first example:
- The top level of an `Envlang` code file is always its own anonymous environment, called the **global environment**
- You can make the global environment explicit by simply adding braces to the start and end, but this is not necessary (to avoid extra indentation)
- The objects `env` and `b` are named environments
- The `env` environment contains a single environment, `a`, who contains the anonymous environment `5`
- The `b` environment contains an anonymous environment `env.a + 5`

Evaluation occurs from the deepest nesting level of environments first:
1. The anonymous environment `5` is evaluated immediately and returns its result to `a`
2. The named environment `a` is evaluated and returns itself to `env`
3. The named environment `env` is evaluated and returns itself to the global environment
4. The anonymous environment `env.a + 5` is evaluated immedaitely and returns its result to `b`
5. The named environment `b` is evaluated and returns itself to the global environment

When written like this, the contents of the objects `env` and `b` are **explicit environments**. In contrast to these, assignments can also be done using **implicit environments**.

However, note the syntax change:
> **Explicit environments** use braces around the right-hand side of the assignment, and do not need to end in the terminator `;`
> 
> **Implicit environments** do not use braces and therefore have to end in the terminator `;`

## Assignment

As already implied above, assignment is done with a combination of the `let` keyword and the `=` assignment operator:
```
let x = 5;
let y = x + 2;
let z = x - y;
```

Assignments come in three types:
1. Expression assignments
2. Environment assignments
3. Function assignments

Assignments can be thought of as special functions: the function call `=` takes left-hand side and right-hand side operands, performs the evaluation in the right-hand side operand and returns the result to the left-hand side operand.

Crucially, however, assignments also return the assigned environment to the parent environment: the values `x`, `y`, and `z` are returned to the global environment once assignment is complete. However, since there is nothing to receive the returns, nothing outside of the assignment happens.

### Expression assignments

You've already met the first one, as expression assignments are simply evaluations of some operand on the right-hand side, and storing of the returned environment in the left-hand side.

However, the expression assignment is actually a fair bit more complicated than that. As noted before, the right-hand side of an expression assignment is actually an anonymous environment:

1. The interpreter steps to the element after the assignment operator, and starts parsing a new environment.
2. The environment is evaluated, and is returned to the assignment operation context.
3. The assignment operation context flattens a single-element environment (such as `let x = 5;`) and returns the result to the `let` expression context.
4. The ´let´ expression is completed and bound to the environment within which it was declared.

### Environment assignments

Assignments can also include new environments:

```
let x = {
    let y = 5;
    let z = y + 2;
    let foo = "Hello!";
}
```

In this case, `x` is an environment containing three environments `y`, `z` and `foo`. Each of the sub-environments contain the anonymous environments `5`, `y + 2` and `"Hello!"`. Because of its anonymity, the environment `y + 2` does not actually contain any further environments (as they would be inaccessible without a name), so it has been evaluated to `7`.

This shows how `Envlang`'s environments can be thought of as a key-value-paired data structure, similar to `struct` in C/C++/Rust or `dict` in Python.

### Function assignments

The final assignment type is **function assignments**. Functions are declared with the `fun` keyword:

```
let fun my_add[a, b] = {
    return a+b;
}
```

Remember the mantra: everything is an environment - including functions.

However, there are two main differences between functions and other environments:
1. Functions are reusable, as they instantiate a new environment every time they are called.
2. Related to the above point, function arguments are templates and not 'real' objects, so they do not exist at assignment time.
3. Functions cannot automatically access environment sibling members, so identifiers will not conflict at assignment or call time.

Functions create new environments given their parameters. The function call to `my_add` will create a new environment with the parameters `a` and `b` (whose values are given to the function at call time). After this, it evaluates the environment. Finally, it returns whatever was specified with the `return` keyword (in this case, the anonymous environment `a+b` which itself is evaluated before returning) to the caller.

Functions can be used in regular assignments:

```
let fun my_add[a, b] = { return a + b };    # The return statement terminator is optional because of the explicit environment.
let result = my_add[5, 2];                  # Evaluates to ´7´.
```

Functions can also be used in environment assignments, where they function somewhat similarly to object methods in object-oriented languages:

```
let my_env = {
    let a = 5;                              # The identifiers ´a´ and ´b´ are not accessible by default in the later function environment.
    let b = 2;
    let fun my_add[a, b] = return a + b;    # The return statement terminator is mandatory because of the implicit environment.
}

let c = 7;
let d = 3;
let result = my_env.my_add[c, d];           # Evaluates to `10`.
```

A few things about the above two code blocks:
- Return statement terminator semicolons are optional with explicit environments, but mandatory with implicit environments.
- A simple function can be written as a single line, either as an explicit or implicit environment.
- By default, functions do not have access to sibling elements within their declaration environments.

Regarding the last point, functions can optionally be given access to sibling elements using **inheritance**.

## Inheritance

Environments observe the following rules:
1. Child environments cannot access parent environment members.
2. Parent environments can only access child environment members through the accessor operator `.`

These can become cumbersome or hindering at times. Because of this, `Envlang` implements the `inherit` keyword.

The `inherit` keyword must be followed by a parenthesis-encased, comma-separated list of inheritance elements:

```
let outer = {
    let x = 2
    let y = 3;
    let inner inherit (x, y) = {
        let z = x + y;
    }
}
```

In the above code, the implicit environments `x` and `y` are handed as inherited environments to `inner`, making them accessible inside the inner environment, making `z` evaluate to `5`.

The elements of `inherit` must be accessible to the calling environment:

```{tag=error}
let x = 5;
let outer = {
    let y = 2;
    let inner inherit (x, y) = {            # ERROR: `x` not available in scope
        let z = x + y;
    }
}
```

Because `outer` does not actually have access to `x`, it also cannot inherit it to `inner`. This code will error once the interpreter hits the `let inner` assignment, because it finds that `x` is not available in the calling scope. A simple fix would be to let `outer` inherit `x`.

In some cases, it might be preferable to inherit everything from the parent environment. `Envlang` allows for a short-hand for this, using **the wildcard operator**.

## The wildcard operator `*`

The special operator `*` (distinguished from the multiplication operator `*` by usage context) can be used to select all available environments. There are two places where the wildcard may be used: in inheritance arguments, and in function returns:

```
let outer = {
    let x = 5;
    let y = 2;
    let first_name = "John";
    let number = "5551234567";
    let inner inherit (*) = {
        let z = 5 + 2;
        let surname = "Doe";
    }
}

let fun construct_full_name[first, last] = {
    return first + last;
}

let name = construct_full_name[outer.inner.first_name, outer.inner.surname];
```

Let's break the above code down:
1. The first line assigns an environment to `outer`, which contains four objects (themselves containing anonymous environments) and an inner environment `inner`.
2. The `inner` environment inherits all elements of the `outer` environment because of the `inherit (*)` declaration.
3. The function `construct_full_name` takes two parameters: `first` and `last`.
4. The function evaluates and returns the result of the binary operation `first + last`.
5. The final assignment consists of a function call to `construct_full_name`, using the elements found inside `inner`. The `first_name` could have been retrieved either from `inner` or `outer`, since the object was inherited from `outer` to `inner`.
6. The final assignment results in a `name` string containing the result of concatenating `outer.inner.first_name` and `outer.inner.surname`.