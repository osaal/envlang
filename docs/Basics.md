# Envlang

`Envlang` is a functional-style programming language where everything is an environment.

Features of `Envlang`:
- Dynamic typing (FIX)
- Pure functions
- Constant assignments

## Environments

The basic building block of `Envlang` is the **Environment**. An Environment is defined by braces:

```
{ # Start of environment
    # Things go here
} # End of environment
```

Environments control scope: Anything inside the Environment is available to use, anything outside the Environment is not (with a few exceptions).

Most Environments (except function environments; see below) are evaluated immediately, meaning that anything inside the Environment is executed. Once the evaluation is complete, the Environment is discarded, along with any data inside it:

```
{ # `a` is not accessible, as it has not been defined yet
    let a = 5;      # `a` is assigned the value 5, making `a` accessible from hereon
} # `a` goes out of scope and is destroyed
# `a` is no longer accessible, as it was destroyed because of the scope of the anonymous Environment
```

An Environment can be anonymous or named. **Anonymous environments** (as the example above) are unassigned, meaning that they cannot be accessed after execution.

**Named Environments**, however, can be accessed later on in the code, using the accessor symbol `.`:

```
let env = { 
    let a = 5;      # `a` is assigned the value 5, making `a` accessible from hereon
} # `a` would go out of scope, but because the parent environment was named, it remains accessible until the parent environment goes out of scope

let b = env.a + 5;      # `b` evaluates to 10, because `a` exists in the `env` environment
```

In `Envlang`, everything is treated as an Environment. In fact, the above code could be written more verbosely to express this fact:

```
let env = { let a = {5} }
let b = {env.a + 5}
```

When written like this, the contents of the objects `env` and `b` are **explicit environments**. In contrast to these, assignments can also be done using **implicit environments**.

However, note the syntax change:
- Explicit environments use braces around the right-hand side of the assignment, and do not need to end in the terminator `;`
- Implicit environments do not use braces and therefore have to end in the terminator `;`

In other words, ending terminators are optional for explicit environments. For clarity, however, they are recommended.

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

Assignments always return themselves: in the above example, the values `x`, `y`, and `z` are returned immediately to the parent environment. However, since there is nothing to receive the returns, nothing outside of the assignment happens.

### Expression assignments

You've already met the first one, as expression assignments are simply evaluations of some calculation on the right-hand side, and storing of the results in the left-hand side.

However, the expression assignment is actually a fair bit more complicated than that. As noted before, the right-hand side of an expression assignment is actually an anonymous Environment. This implies the following:

1. The interpreter evaluates the right-hand side, as the expression assignment is evaluated immediately.
2. Because of the implicit environment call, the left-hand side is interpreted as an anonymous expression (e.g., `x + 2`)
3. All expressions are evaluated immediately, so the anonymous expression is evaluated.
4. The result is stored in the right-hand side because of the assignment operator (given a valid assignment identifier)

Because assignments always return their values to the parent environment, you can **chain assignments**:

```
let x = let y = 5;
```

Step by step, the interpreter:
1. Sees the `let` keyword and takes the following identifier `x` into memory.
2. Sees the `=` assignment operator.
3. Sees that the right-hand side is another `let` keyword, thus taking the following identifier `y` into memory.
4. Sees the `=` assignment operator.
5. Sees that the right-hand side is an anonymous environment with the value `5`, and evaluates it.
6. Because `5` is anonymous, it returns itself immediately.
7. Because the environment returns to the `=` assignment, the environment (containing `5`) is assigned to `y`.
8. Because `y` returns itself to the `=` assignment, the environment (containing `y`, which contains an environment containing `5`) is assigned to `x`.
9. Finally, `x` returns itself to the parent environment, where no-one is receiving the value, so nothing extra happens.

Behind the scenes, the chain is very complex with multiple steps, but the final result is as you would expect: two objects `x` and `y` available in the parent environment, who both contain the values `5`.

Note: Technically, the environment `x` contains the environment `y` which contains the environment `5`, but the interpreter is smart enough to flatten single-element environments (compare below with environment assignments).

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