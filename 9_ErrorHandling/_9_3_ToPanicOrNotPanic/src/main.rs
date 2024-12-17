/*
    It's advisable to have your code panic when it's possible that your code could end up in a bad state.
    In this context, a bad state is when some assumption,
     guarantee, contract, or invariant has been broken, such as when invalid values, contradictory values, or missing values are passed to your code—plus one or more of the following:

        * The bad state is something that is unexpected, as opposed to something that will likely happen occasionally, like a user entering data in the wrong format.
        * Your code after this point needs to rely on not being in this bad state, rather than checking for the problem at every step.
        * There's not a good way to encode this information in the types you use. We'll work through an example of what we mean in the "Encoding States and Behavior as Types" section of Chapter 17.

 */



