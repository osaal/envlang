{
    # This is an anonymous environment, as signalled by the curly braces without an assignment
    # If an environment is anonymous, it is evaluated immediately
    # The evaluation proceeds line-by-line, assigning and evaluating any commands within
    # Once the environment is signalled as ending with a closing curly brace, its elements are destroyed

    # Keywords:
    ## let: Declare a new environment
    ## inherit (): Inherit either a list of environments (a, b, c) or all environments (*) from the parent environment as read-only

    let env = {
        # This is a named environment, as signalled by the curly braces with an assignment
        # A named environment evaluates immediately, but stores the results of the evaluation in the environment call
        # The objects in the environment call are accessible to the parent environment, but not to sibling environments

        let x = 1
        # This is a named object, where 'name = x' and 'value = 1'
        # Assignment equals evaluation, so the right-hand side value is evaluated immediately and stored in the object
        # The object is currently only accessible within the enclosing environment
    }
    # Because the above environment was named, its evaluations were stored in the object 'env'
    # This object is now accessible to the parent environment

    let second = {
        # This environment, however, does not automatically have access to the objects in the sibling environment object 'env'
        # Trying to access 'x' would give an error
    }

    let third inherit (*) = {
        # This named environment has access to the parent environments objects evaluated up to this point, because of the 'inherit (*)' phrase
        # Currently, the objects 'env', 'env.x', and 'second' would be available for reading
        # 'env.x' accesses the 'x' object from the 'env' environment

        # The 'inherit' keyword applies only to environments
        # The 'inherit' keyword takes an argument defining which objects to inherit.
        # These objects are inserted as references into the new environment, making them accessible as read-only
        # An inherited object cannot be modified in the inheriting scope!
        # The phrase 'inherit (*)' gives all parent objects to the environment
        # Objects can also be given as a comma-separated list: 'let a inherit (b, c, d) = {...}'
        # This would make the objects 'b', 'c', and 'd' read-only accessible inside 'a'
        
        let x = env.x
        # The left-hand side is an object enclosed in the 'third' environment
        # The right-hand side refers to the inherited 'x' object from the parent environment
        # Due to scoping rules, from this point onwards, calls to 'x' refer to the inner-most environment 'third'
        # We say that objects in the 'third' environment mask the inherited objects
    }

    1
    # This is an anonymous object, whose scope is automatically within the enclosing environment
    # Because the object is anonymous, it is destroyed immediately and inaccessible after the line
    # In practice, this call does nothing
    # Under the hood, the call sets up an anonymous environment and evaluates its calls
    # Because '1' is anonymous and evaluates to nothing, the environment exists
    # Because the environment was anonymous, it is destroyed
    # Thus, the end result is that nothing happened -- but CPU cycles were consumed!
    
    1 + 2 
    # This is an anonymous function call
    # It is equivalent to add(1, 2)
    # It implicitly creates its own environment, with a pointer to the parent enclosing environment
    # The anonymous environment it creates has three objects: the anonymous object 1, the anonymous object 2, and the anonymous function call add()
    # Since it is anonymous, it is evaluated immediately
    # Post-evaluation, the anonymous environment contains one anonymous object with 'value: 3'
    # However, since the environment is anonymous, the object is immediately destroyed, and subsequently also the environment

    let result = 1 + 2
    # This is a named function call, also being equivalent to add(1, 2)
    # It also creates its own environment
    # However, the environment is created upon evaluation of the right-hand side:
    # let result = { 1 + 2 }
    # Since the right-hand side is named, but contains no 
}