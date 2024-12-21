# Front Ends: Turning Computer Programs Into Circuits


## Introduction
Goal: We need an efficient way to turn high-level computer programs into arithmetic circuits.

### How to implementations
Most general purpose argument system implementations work in this two-step manner. 
1. First, a computer program is compiled into a model amenable to probabilistic checking, such as an arithmetic circuit or 
arithmetic circuit satisfiability instance(eg. R1CS).
2. Second, an interactive proof or argument system is applied to check that the prover correctly evaluated the circuit.

### Frontend and Backend
In these implementations, the program-to-circuit compiler is referred to as the frontend,
and the argument system used to check correct evaluation of the circuit is called the back end.


## RAM

### Hareware Model
Our hardware model will be a simple Random Access Machine (RAM). A RAM consists of the following
components.

* (Main) Memory. That contains cells of storage, where each cell can store 64 bits of data.
* A constant number of registers. Registers are special memory cells storing instructions for RAM to manipulate data. 
* A set of ℓ = O(1) allowed machine instructions. Typically, these instructions are of the form:
  * Write: data from register to Main Memory. Write the value currently stored in a given register to a specific location in Main Memory.
  * Read: data from Main Memory to register. Read the value from a specific location in Main Memory into a register.
  * Perform basic manipulations of data in registers. For example, adding, subtracting, multiplying,
  dividing, or comparing the values stored in two registers, and storing the result in a third register.
  Or doing bitwise operations on the values stored in two registers (e.g., computing the bit-wise
  AND of two values).
* A program counter(aka. pc). This is a special register that tells the machine what is the next instruction to execute.

### Machine Code
Machine code is a set of basic instructions that can each be executed in unit time on the machine’s hardware.

### Trace(transcript) of RAM
the trace describes the changes to M’s configuration at each step of its execution. For each step i that M takes, the 
trace lists just the value of each register and the program counter at the end of step i. Since M has only O(1) registers, 
the trace can be specified using O(T) words, where a word refers to a value that can be stored in a single register or memory cell.



## transformation 

### arithmetic circuit evaluation and circuit satisfiability problem
* circuit evaluation problem: 
  
  the input specifies an arithmetic circuit C, input x, and output y, and the goal is to determine whether `C(x) = y`.

  eg: P claims to have applied a specific circuit C or run a specific RAM M on a public input x that is known to both verifier and prover.

* arithmetic circuit satisfiability problem (aka. circuit-SAT):

  The circuit C takes two inputs, x and w. The first input x is public and fixed. The second input w is often called the witness, 
  or the non-deterministic/auxiliary input. 
  Given the first input x and output y, the goal is to determine whether there exists a w such that `C(x, w) = y`.

  eg: P claims it knows some witness w(not known to the verifier) which applying C to (x, w), or running M on (x, w), yields output y.


### Basic idea
The transformation from RAM execution to circuit satisfiability produces a circuit satisfiability instance (C, x, y), 
where x is the (public-)input to M, y is the claimed output of M, and the witness w is supposed to be the trace of M’s execution on input x. 

The circuit C will simply check that w is indeed the trace of M’s execution on input x.
If this check passes, then C outputs the same value as M does according to the ending configuration in the trace. 
If the check fails, C outputs a special rejection symbol.


### Details
TODO 6.5.4


### Alternative transformation
TODO 6.6