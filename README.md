# tfhe_ewma

Test out bootstrapped operations for stateful process by implementing Exponentially Weighted Moving Algorithm (ewma) under homomorphic encryption in the Concrete (TFHE) library for the Rust programming language.  

create_keys 
- run this to create secret keys for encryption, key switching and bootstrap (may take several hours)

test_basic 
- test some basic (levelled) operations like addition, multiplication and negation (sign flip)

test_ewma 
- iterates the ewma process for N steps taking input from a sinus wave