# Project Euler 004
Following a strange fascination, I decided to dig a bit into this problem.
Both the language choice, implementation, and writing are inspired by
https://adamdrake.com/an-unreasonably-deep-dive-into-project-euler-problem-4.html
but I'll go a bit further and even bring up/implement a non-constant memory solution.

## Main limitations (for rust).
I don't have a big integer catch all numbers type of struct in rust.
I've coded it in python too (look in ./python_lead), but python's inherent
ability to just have ridiculously large numbers is extremely convenient.  
However, rust takes a couple of seconds (around 4-5) to find the largest
pair of 8 digit numbers whose product is a palindrome while python takes
around 22 seconds. I'm hoping to find a suitable big int cargo in rust,
but haven't found one yet, especially because I do need the exponentiation
function, (or I could make it using a string,,,).

## Where to look
The python file details my thought process and the optimizations along the way.