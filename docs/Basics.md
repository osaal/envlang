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

However, the expression assignment is actually a fair bit more complicated than that. As noted before, the right-hand side of an expression assignment is actually an anonymous environment. This implies the following:

1. The right-hand side is evaluated first.
2. The result of the right-hand side is returned after evaluation to the assignment function
3. The assignment function assigns the result of the right-hand side (a thus-far anonymous environment) to the left-hand side identifier
    - This takes the data from the right-hand side anonymous environment, copies it into a named environment, and destroys the original anonymous environment.
4. As the assignment is complete, the assignment operator returns the named environment to the parent environment.

Because assignments always return their values to the parent environment, you can **chain assignments**:

```
let x = y = 5;
```

Step by step, the interpreter:
1. Sees the `let` keyword and takes the following identifier `x` into memory.
    - If `x` already exists in the operating environment, the interpreter errors.
2. Sees the `=` assignment operator.
3. Sees that the right-hand side is another identifier `y`, and takes it into memory.
    - If `y` already exists in the operating environment, the interpreter errors.
4. Sees the `=` assignment operator.
5. Sees that the right-hand side is an anonymous environment with the value `5`, and evaluates it.
6. The anonymous environment is returned to the second assignment operator.
7. The assignment operator assigns the returned anonymous environment `5` to the identifier `y`, creating a named environment and destroying the original anonymous environment.
8. The named environment `y = 5;` is returned to the first assignment operator.
9. The assignment operator assigns the returned named environment `y = 5;` to the identifier `x`, creating another named environment. Because the previous environment was also named, it is not destroyed.
10. Finally, the environment `x` is returned to the global environment.

This would normally cause a situation, where the value `5` is actually accessed by `x.y` (see above). However, because of the implicit environment call `y = 5;` and subsequent assignment, the interpreter knows to temporarily 'deanonymise' the structure. Thus, the end result is two named environments `x` and `y`, who both contain the value `5`.

To avoid the flattened structure, you need to use an explicit environment assignment:

```
let x = { let y = 5 };
```

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

If you are used to more imperative languages, this nested environment behaviour might be confusing at first. A simple analogy is a map or dictionary, e.g., in Python:

``` {python}
def x = {y: 5, z: 5+2, foo:"Hello!"}
```

Note, however, that a traditional Python dictionary would not allow for recursive references within the dictionary (`z` cannot refer to `y` as it is a key in the same dictionary) without trickery. In `Envlang`, this is easily available through expression assignments within environments.

This also shows how `Envlang`'s environments can be thought of as a key-value-paired data structure, similar to `struct` in C/C++/Rust or `dict` in Python.

Environment assignments can also be chained:

```
let x = y = { let z = 5; }
```

In this case, both `x` and `y` will contain explicit environments with one element, `z = 5`. Accessing `z` would be possible through `x.z` or `y.z`. Note, that `x` and `y` are not coupled because of the chained assignment -- they do not know of each other.

Things get a bit more complex when a step within the chain is its own environment:

```
let x = { let y = z; } = { let z = 5; }
```

When an anonymous environment is assigned to another anonymous environment, the interpreter merges the identifiers from the right-hand side into the environment on the left-hand side:

1. Evaluation starts at the innermost environment `5`, which is immediately returned to the fourth assignment operator.
2. The value `5` is assigned to the identifier `z`.
3. Because the right environment `{ let z = 5; }` has completed evaluation, it returns itself to the preceding third assignment operator.
4. The interpreter notices that the left-hand side is also an environment `{ let y = z; }`, so it merges the two.
5. Merging is done before evaluation of the left-hand side, so the middle environment now contains `{ let z = 5; let y = z; }`.
6. The interpreter continues evaluation from the line where the merged environment ends, so it assigns `z` to `y` (the second assignment operator).
7. The middle environment finishes evaluation and is returned to the first assignment operator.
8. The assignment operator assigns the environment `{ let z = 5; let y = z; }` to the identifier `x`.
9. Finally, the assignment returns itself to the global environment.

As a result, the following environment members are accessible in the global environment:
- `x` (an environment with two members)
- `x.y` (an environment with one member)
- `x.z` (an environment with one member)

Note, that `y.z` is not a valid operation: `y` was assigned the value `z`, which contained the value `5`, so the value was **curried** directly into `y`.

**Maxim 2**
: When the left-hand operand of an assignment is an environment, the right-hand evaluation is merged into the environment.

If you wanted to keep the structure of `{ let z = 5; }`, you would have to assign it to an identifier (create a named environment) prior to merging:

```
let x = { let y = a.z; } = a = { let z = 5; }
```

Remember that anonymous environments (e.g., `5` and `a.z`) are evaluated immediately!

Also note the change in accessing `z` inside the environment `{ let y = a.z; }`: Because `a` is merged into the left-hand environment, `z` is accessible only through `a.z`.

This results in the following global environment members:
- `x` (evaluates to an environment)
- `x.a` (evaluates to an environment)
- `x.a.z` (evaluates to the value `5`)
- `x.y` (evaluates to the value `5`)

### THE FOLLOWING CHAPTERS NEED UPDATING

### Function assignments

The final assignment type is **function assignments**. Functions are declared with the `fun` keyword:

```
let fun my_add[a, b] = {
    return (a+b);
}
```

Remember the mantra: everything is an environment. This means that the above function is also an environment!

However, there are two main differences between functions and other environments:
1. Functions are reusable, as they instantiate a new environment every time they are called.
2. Related to the above point, function arguments are templates and not 'real' objects, so they do not exist at assignment time
3. Functions cannot automatically access environment sibling members, so identifiers will not conflict at assignment or call time

At face value, the function assignment could actually be rewritten as an environment assignment:

```
let my_add inherit(a, b) = {
    a + b;
}
```

The `inherit` keyword will be covered later, so don't worry about it.

The above environment assignment is syntactically valid, and would produce the same result as the function assignment -- but only once. Since `my_add` is defined as an environment, and all normal environment assignments are constant, the value of `my_add` (`a + b`, for whatever values `a` and `b` had in the parent environment at assignment time) can never change.

Functions, on the other hand, create new environments given their parameters. The function call to `my_add` will create a new environment with the parameters `a` and `b` (given to the function at call time). After this, it evaluates the environment. Finally, it returns whatever was specified with the `return` keyword (in this case, the anonymous environment `a+b` which itself is evaluated before returning) to the caller.

Functions can be used in regular assignments:

```
let fun my_add[a, b] = { return (a + b) };  # Whitespace does not matter, and the terminator is optional because of the braces
let result = my_add[5, 2];                  # `result` becomes `7`
```

Functions can also be used in environment assignments, where they function somewhat similarly to object methods in object-oriented languages:

```
let my_env = {
    let a = 5;                              # Despite the same identifiers, these assignments will not mess with the function
    let b = 2;
    let fun my_add[c, b] = return (a + b);  # Because of the omission of braces, we would need a terminator at the end
}

let c = 7;
let d = 3;
let result = my_env.my_add[c, d];           # Evaluates to `10`
```

A few things about the above two code blocks:
- Terminator semicolons are optional with explicit environments, including when defining functions
- A simple function can be written as a single line, either as an explicit or implicit environment
- Since functions do not have access to environment siblings, `my_add` does not know about `a` or `b` within `my_env` -- therefore, they do not conflict with one another.

## Inheritance

Environments observe the following rules:
1. Child environments cannot access parent environment members.
2. Parent environments can only access child environment members through the accessor operator `.`

These can become cumbersome or hindering at times. Because of this, `Envlang` implements an `inherit` keyword to let you specify environment inheritance:

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

The `inherit` keyword must be followed by a parenthesis-encased, comma-separated list of inheritance elements. These elements must be accessible to the calling environment:

```{tag=error}
let x = 5;
let outer = {
    let y = 2;
    let inner inherit (x, y) = { # ERROR: `x` not available in scope
        let z = x + y;
    }
}
```

Because `outer` does not actually have access to `x`, it also cannot inherit it to `inner`. This code will error once the interpreter hits the `let inner` assignment, because it finds that `x` is not available in the calling scope. A simple fix would be to let `outer` inherit `x`.

In some cases, it might be preferable to inherit everything from the parent environment. `Envlang` allows for a short-hand for this:

Note, that a function assignment does not allow for inheritance. Instead, functions should simply be called with the elements needed (inheriting previously from parent environments if necessary).

Also note, that you cannot *both* declare inherited environments and the wildcard -- it has to be either or.

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
        ...
    }
}

let fun construct_full_name[first, last] = {
    let result = first + last;
    return (*)
}

let name_env = construct_full_name[outer.inner.first_name, outer.inner.surname];
```

Let's break the above code down:
1. The first line assigns an environment to `outer`, which contains four objects (themselves containing anonymous environments) and an inner environment `inner`
2. The `inner` environment inherits all elements of the `outer` environment because of the `inherit (*)` declaration
3. The function `construct_full_name` takes two parameters: `first` and `last`.
4. The function creates a new object called `result`, itself an anonymous environment consisting of `first + last`.
5. The function returns `result`, `first`, and `last` because of the `return (*)` declaration
6. The final assignment consists of a function call to `construct_full_name`, using the elements found inside `inner`. The `first_name` could have been retrieved either from `inner` or `outer`, since the object was inherited from `outer` to `inner`.
7. The evaluation of the final assignment results in an environment containing: `result = "JohnDoe"`, `first_name = "John"`, and `surname = "Doe"`. This is because `construct_full_name` was told to return all objects with the wildcard, *including the ones passed into it as arguments*.

Note, that the performance loss on accessing inherited objects as opposed to original objects should be minimal-to-zero, as all objects are passed-by-reference.

While this code does not necessarily make sense (`construct_full_name` should probably just return the full name), it enables a very powerful functional pattern: **piping**.

## Piping

NYI.