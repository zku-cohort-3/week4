# Freivalds Algorithm

This is an algorithm that solves the problem $AB = C$ for 3 compatible matrices A, B and C in time $O(N^2)$.

Note that the best known algorithm for matrix multiplication is $O(n^2.37)$, so we're going to assume the possibility of
being fooled by the verifier by some arbitrary chance $\epsilon$. This $/epsilon$ can be arbitrarily low since we can
repeat Freivald's algorithm and the probability of being fooled decreases rapidly with the number of iterations because 
of the multiplicative effect of multiplying probabilities.

It is based on the fact that matrices can be interpreted as a low power polynomial and two different low power
polynomials disagree at most inputs. So we can generate a random vector and check instead if 
$A * B * rand_vec == C * rand_vec$

One known implementation to this algorithm decides to not use a fully random vector but instead generate 
a vector v such that $v = [rand_value^1, rand_value^2, ..., rand_value^n]$


Check a more thorough explanation of the algorithm here: https://people.cs.georgetown.edu/jthaler/ProofsArgsAndZK.pdf 
at page 16 of the book.
