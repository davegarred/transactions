# Simple Transaction Processor

This is a simple transaction processor as part of the [Rust coding test](RustCodingTest.pdf).

## Notes
The need to dispute any transaction means that we need to track all the 'deposit' and 'withdrawal'
transactions (handled on the `Client`). This could be a hefty burden for very large files. Depending
on the actual requirements some possible solutions:
- Add a persistence mechanism to move these details out of memory (I left that out as the "toy payments engine"
description implied that I shouldn't go overboard).
- Set a time limit on transactions that may be disputed, and a limit on how long the dispute may last.

I've used the 'flexible' feature of the csv package to handle the event if a fourth field is entirely
absent from some transaction (e.g., no trailing comma on a dispute, resolve or chargeback).

It felt way out of bounds but this would have been a great place to show off 
[a library that I built](https://github.com/serverlesstechnology/cqrs)
that does just this sort of thing. That being said it uses CQRS which is very much overkill for domain logic this simple. 

## Response to "Scoring" questions 
- Basics: I sure hope so on all these fronts.
- Completeness: Yes, everything should be covered, possibly at the expense of memory as I'm keeping
all deposit and withdrawal transactions in memory.
- Correctness: Certainly all the cases that I noted are covered. It was rather a small set given the
instructions to drop all transaction errors. Sample test files are in the 'data' directory (note that 
these _do not_ check final state after tests, that is left to the unit tests to verify).
- Safety: Well there's no persistence here so that feels dangerous as hell, but other than that I'm not using any concurrency
(and it's certainly not going to improve anything with such simple logic) so I think we're pretty safe.
File errors stop processing and report the error to the user, any transactional errors are ignored per 
the instructions.
- Efficiency: Transactions are processed as they are read from disk so little memory should be wasted.
The deposit and withdrawal transactions remain in memory but each transaction is only 15 bytes so we've 
got a lot of room to work with.
- Maintainablity: On readability, I mean I think so, but you tell me. There are a number of additional 
traits I'd implement for the `Amount` struct (e.g., `AddAssign`) if we were going to use that rather than
something in a library for a prod application, but felt like overkill here. 