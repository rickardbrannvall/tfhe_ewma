# tfhe_ewma

Implements Exponentially Weighted Moving Average algorithm (ewma) under homomorphic encryption in the Concrete (TFHE) library for the Rust programming language. This is to demonstrate how to use bootstrapped operations, which are necessary for the EWMA since it is a statefull process.  

create_keys 
- run this to create secret keys for encryption, key switching and bootstrap (may take several hours)

test_basic 
- test some basic (levelled) operations like addition, multiplication and negation (sign flip)

test_ewma 
- iterates the ewma process for N steps taking input from a sawtooth (triangle) wave function
