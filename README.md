# Project Euler 004
Following a strange fascination, I decided to dig a bit into this problem.

## Main limitations.
I don't have a big integer catch all numbers type of struct in rust.
I've coded it in python too (look in ./python_lead), but python's inherent
ability to just have ridiculously large numbers is extremely convenient.  
However, rust takes a couple of seconds (around 4-5) to find the largest
pair of 8 digit numbers whose product is a palindrome while python takes
around 22 seconds. I'm hoping to find a suitable big int cargo in rust,
but haven't found one yet, especially because I do need the exponentiation
function, (or I could make it using a string,,,).

## Process

### The bad solution I see too often.
The most foolish solution is to loop from the largest possible number to the smallest,
checking if each one is a palindrome, and if so, factoring it. This brings you face
to face with an NP hard problem of prime factorization. Ideally, the process of finding
(factor1, factor2) would go 
```
("number with 2n digits", 1) -> ("number with 2n-1 digits", "1 digit number) ->
("number with 2n-2 digits", "2 digit number) ... ("number with n digits", "number with n digits")
```
but unless fate and the number being factorized are favorable, you may suddenly find yourself
skipping over that crucial point where both numbers are n digits.  
The only redeeming factor of this algorithm is that you can stop early because the first `2n` digit
number you find that is a palindrome, factorable into two `n` digit numbers is the solution.


