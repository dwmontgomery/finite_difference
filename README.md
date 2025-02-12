# CS417 Machine Assignment 3
@author Dylan Montgomery

## Purpose

The purpose of this program is to compute an approximate value for the derivative of *f(x) = sin(x)* using 
the finite difference formula. 

    *f′(x)≈(f(x+h)−f(x))/h*

## Requirements

This program is written in Rust using Cargo for build management.

    Rust 1.84.0

## Input

All input values are hard coded into the program using:

- *f(x) = sin(x)*
- *x = 1*

## Sample Execution and Output

### Execution

The program can be fun using the command

    ./finite_difference 

## Output

The output should be similar to the following:

|  h   |       x       | Approx. f'(x) |  Known f'(x)  |  Abs. Error   |
|:----:|--------------:|--------------:|--------------:|--------------:|
|2^-01 |    1.00000000 |    0.31204800 |    0.54030231 |    0.22825430 |
|2^-02 |    1.00000000 |    0.43005454 |    0.54030231 |    0.11024777 |
|2^-03 |    1.00000000 |    0.48637287 |    0.54030231 |    0.05392943 |
|2^-04 |    1.00000000 |    0.51366321 |    0.54030231 |    0.02663910 |

**Note that the actual output values go up to (2^{-30}).**