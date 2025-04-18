# Project 3: Cryptography (Fun with Block Ciphers)

There is no VM for this project; you can complete it on your regular machine.

## Task 1 (Warmup: Cracking hashes)

Say you know the result of a hash from a password using the md5 hash function,
for example:

`56207fe91845a32a95b83409fd063715`

`c78a25d0720594169769fd7d2baebc1c`

`7e9e3c392a3143b6d0dea01adc0c389e`

`427ff3c70c42e5a0be09b6307054d8c7`

You also know each hash has been produced like this:
`const salt = "likes";` `const password = [unknown];` `const hash = md5(salt + password);` "+" here means concatenation

You need to find a password from *any* of the above four hashes.
Note that each password is a string of 4 characters, and the characters are all
lowercase letters or numbers.
You can consider to use brute force to find the password using tools like
[Hashcat](https://hashcat.net/hashcat/) or
[John the Ripper](https://www.openwall.com/john/).

Note that to reduce the search space, you can use the fact that the salt is a
fixed string.

**Grading:** We will need your cracked password stored in `task1_password.txt`.

## General Description (Task 2-5)

In the rest of the project, you will be attacking a weak ciphersuite in an
online bank.
The bank is using a secure **128-bit block cipher**, with a *unique key* per
session.
To make a streaming cipher from this, the bank is using the cipher in electronic
codebook (ECB) mode.
(Hmm...)

As an attacker, you were able to inspect the ciphertext stream sent by you to
the bank's server.
Due to some technical limitations, you were unable to capture the server's
responses, and you were only able to do this once.
However, you know exactly what operations you performed, and in what order, so
you should be able to figure out the encrypted message formats, despite *not
knowing the format of the plaintext*.
That is, the *only* things you have are what you entered into the bank's
website, and the corresponding ciphertext stream.

From this, you will need to do the following:

- Learn the format of the messages
- Write a program to parse these messages and generate new messages
- Perform a number of passive- and active-wiretapping attacks on the session of
  *another* customer (the target), which uses a *different* key

**Please note that you will _not_ be decrypting the streams!** Assume the
cryptosystem (in this case, the block cipher) itself is unbreakable.
You will be exploiting the shoddy way the bank is using the block cipher, rather
than trying to break the block cipher itself.

## The Reference Streams

Ciphertext streams can be found on under `task2` folder.
The file `reference.in` contains the following sequence of requests, in the
order given:

1. A balance request for your checking account
1. A $100 transfer from your checking account to your savings account, to be
   executed immediately
1. A balance request for your checking account
1. A balance request for your savings account
1. A $1000 transfer from your savings account to your checking account, to be
   executed immediately
1. A $1000 transfer from your savings account to your checking account, to be
   executed tomorrow
1. A balance request for your savings account
1. An invoice request issued by your checking account for payment of $1000 from
   your savings account
1. A balance request for your checking account

You are also given additional ciphertext streams, which are from *different*
sessions, so they are not encrypted with the same key.
In addition, you do *not* know what requests were used to generate this.
This will allow you to test your code against an unknown request sequence.

Take a look at `reference.in` by running `xxd reference.in`; this will format
the output so that each line corresponds to 16 bytes of data (128-bits:
i.e., one line per block in the block cipher).
Each of the above request types (balance, transfer, and invoice) might require
different numbers of blocks (for instance, a transfer request clearly has to
take more arguments than a balance request).
See if, using `xxd`, you can visually match up the blocks in `reference.in` to
the above 9 separate requests.
A big part of this project involves creating a program that will *automatically*
figure out which blocks correspond to which commands.

A good use of encryption would reveal nothing about which requests were being
made.
However, because the system that generated these ciphertexts used ECB, you can
detect (and exploit) patterns.

## Tasks 2-5

Now there are four tasks (tasks 2-5) under this online bank system, and each
will require a separate executable.
For instance, the executable for task 2 will be named `task2`.
Note, this should ***not*** be `task2.x`, `task2.exe`, `task2.sh`, `task2.py`,
or any other file extension.
If this executable must be compiled, you must provide a `Makefile` to do the
compilation, which will be called *without arguments* in the directory for that
task.
E.g., during grading, I will simply run `make` in the `task2` directory; don't
expect I will run `make task2`.

Each task will operate on a *separate session*.
That is, they will all be encrypted with different keys.
You will have limited information about each session.
We will guarantee the following about sessions:

- **No transaction will have the same source and destination account.**
- **Except where noted in the task description, all account numbers will appear
  at least twice.**
- **All three transaction types will appear.**
- **At a minimum, there will be one repeated BALANCE request transaction.**
- **There are no partial transactions, so every transaction that begins in the
  session will be complete.**

See the Implementation Notes below for formatting of output and other
requirements.

### Task 2

For this task, you will provide an executable named `task2` that reads the
ciphertext for a session and outputs

- The types of messages, in order, with each on a separate line

The ciphertext will be provided as the only command-line argument to your
executable, and will be in the same format as the reference stream.
That is, we will call it as (for an input file `task2.in`)

```
./task2 task2.in
```

#### My Notes

- For `reference.in`, we have that the requests to the checking account both
  begin with the same 16-bit signature:
  `07ef`.
- The checking account requests are composed of 2 blocks each; we can use this
  to orient ourself in the task and find the rest of the requests.
- This means that the $100 transfer to the checking account is 5 blocks starting
  with `be54`.
- This leads me to believe that the first block is just the "request" syntax
  because it is shared with the savings account request.
- Similarly, the "transfer" request starts with `be54` regardless of if it is to
  checking or savings.
- I think that the blocks starting with `c501` denote the checking account.
- Meanwhile, the block starting with `c461` denotes the savings account.
- I think that the syntax of a transfer request looks like this:
  1. Transfer request signature.
  1. Source account.
  1. Destination account.
  1. Time the request is to be executed.
  1. Amount of money in the request.
- An invoice account looks like this:
  1. Invoice request signature
  1. Account requesting money.
  1. Account sending money.
  1. Time the request is to be executed.
- The transfer request signature is:
  `be54 1528 f397 89ef 8749 6921 7b0a 7caa .T.(.....Ii!{.|.`
- For a request to be executed immediately, its signature is:
  `be51 fa0d e2fb 7cef e6bd 16bc b6c1 74f3 .Q....|.......t.`
- For a request to transfer $1000, its signature is:
  `ca26 370d 5788 6584 a78e 36f8 203d dd83 .&7.W.e...6. =..`
- I tried the DP solution but I don't actually think it works because it while
  it does pick out repeated substrings, it can erroneously pick out substrings
  that consist of requests that look similar chained together.
- I don't really know how to fix this?
  - We can perhaps do a thing where we orient based on know that the first line
    is always guaranteed to be a request signature and not anything else?
  - Perhaps search for similar substrings from there?
    The problem with this is that we aren't guaranteed that the first request is
    ever repeated.
  - Perhaps what we can do instead is exhaustively check all of the ways that we
    can divide our string's length into blocks of 2, 4 and 5 and make
    conjectures about how many of a given request type there are.
  - This would work (kinda we'd have to do prime factorization which is not
    fast) if 4 wasn't a multiple of 2.
  - This can still allow us to know how many transfer requests those are if we
    perform repeated division by 2.
- One piece of information we have is that the first block will always be the
  header of some request.
  - From there maybe we can find out what is different?
  - For instance, with `task2.in`, we have the first request header begins with
    `4846`.
  - We can then ripgrep to find that there are 4 instances of a line beginning
    with the digits `4846`.
  - From there, we know that the second line of the text file is always going to
    be an address of some sort.
  - In this instance of `task2.in`, we can also infer that the `4846` command is
    a transfer because it appears 5 lines from the end.
  - Maybe we can check for the distance between two instances of the first line?
    Then that'll give us a smaller number that can tell us what kinds of
    requests are between it.
  - We can also try to figure out what the last request is.
  - Because `4846` is a transfer request, it certainly contains at least two
    addresses; the source and the target addresses.
  - If there are two instances of a single line and they're separated by only
    one line, then we know that that's a repeated balance request.
- Based on the fact that we now (probably) have at least two account numbers
  because of `find_accs`, what does this let us do next?
  - Analyze the end of the trace because we know that there will be a last
    request that may or may not contain one of the request headers we have seen
    so far
- From here, we have one of the three request headers and a bunch of account
  numbers.
  We can then try brute force to find the other two request headers?
  Not the most elegant solution in the world but it might yield something nice.
- We can also perform modular arithmetic on the differences between the
  requests.
  - What I mean by this is that if we have two instances of our initial request
    that are separated by 5 blocks, then we know that the initial request is a
    transfer request.
  - Alternatively, if any two instances of our initial request are separated by
    2 blocks, then we know that the initial request is a balance request.
  - For instance, we know that the initial request in `task2.in` is a transfer
    request because it has one instance at line 26 and one instance at line 31.
  - Alternatively, if we have that the minimum distance between any two of the
    initial requests is 7, then we know that either the initial request is a
    transfer or it is a balance.
    Whichever request type the initial request is not, there must be another one
    immediately following it.

### Task 3

For this task, you will provide an executable named `task3` that reads the
ciphertext for a session and outputs

- The types of messages, in order, with each on a separate line
- A replay (unmodified copy) of a message that transfers money into your
  account, written to a file named `task3.out`

For this session, you know that *exactly one* message includes your account, and
it is transferring money to you.
The file `task3.out` should include the **entire** stream, with your replay
added to it.
That is, there should be exactly one more message in `task3.out` than the input
stream.

As before, the input ciphertext stream will be provided as the only command-line
argument to your executable, and will be in the same format as other streams.

```
./task3 task3.in
```

### Task 4

For this task, you will provide an executable named `task4` that reads the
ciphertext for a session and outputs

- The types of messages, in order, with each on a separate line
- A modified money transfer to your account, where the amount in the transfer is
  changed to a valid new value, written to a file `task4.out`

For this session, you know that the target sent a money transfer to your account
for $10.
No other requests involving your account are in this session.
In addition, the target requested payments (invoices) from at least one other
account.

Your executable should produce a file `task4.out` containing the input
ciphertext with the modified message.
That is, `task4.out` should contain the same messages as the input, but with the
amount changed in the payment to your account, so the target is sending you a
different amount of money.

As before, the input ciphertext stream will be provided as the only command-line
argument to your executable, and will be in the same format as other streams.

```
./task4 task4.in
```

### Task 5

For this task, you will provide an executable named `task5` that reads the
ciphertext for a session and outputs

- The types of messages, in order, with each on a separate line
- A money transfer to your account *instead of* a payment request from your
  account, written to a file `task5.out`

For this session, you know that the target requested payment from your account,
and this is the only request involving your account.
You must convert this payment request *from* your account into a money transfer
*to* your account.

Your executable should produce a file `task5.out` containing the input
ciphertext with the modified message.
That is, `task5.out` should contain the same messages as the input, but with the
request for payment from you changed to a transfer to you.

As before, the input ciphertext stream will be provided as the only command-line
argument to your executable, and will be in the same format as other streams.

```
./task5 task5.in
```

## Implementation Notes

The required output must match what we have asked for **exactly**.
In particular, anything not part of the required output should be printed to
standard error, not standard output.

The following table shows the expected way to print message types:

| **Message Type** | **Value to Print** |
| ------------------- | ------------------ |
| balance request | BALANCE |
| money transfer | TRANSFER |
| request for payment | INVOICE |

## Submission

Submit your project to Gradescope.
Please leave the structure of the directories as-is, so files for task 1 go in
the `task1` directory, and so on.
Do not include compiled files, only scripts, source code, and (if needed) a
`Makefile`.

## Grading

- Correctly cracking an md5 hash in task 1 is worth 5 points.
  (Note:
  you don't need to crack all 4, just 1!)
- Identifying all messages correctly will cumulatively be worth 20 points (tasks
  2-5).
- Producing valid messages will cumulatively be worth 15 points (tasks 3-5).
- Correctly modifying messages will cumulatively be worth 10 points (tasks 4 and
  5).
- Correctly changing the type of a message will be worth 5 points (task 5).

## Tips

### Parsing the Session Structure

There is no limit on the number of accounts showing up in a session.
It might be useful to think of each account as a principal in the system (which
it is).
You can consider all of the accounts originating transactions as being
principals that share a single identity.
The attacker, in contrast, is a separate identity, and will not be initiating a
transaction in tasks 2-4.

You should be able to start by assuming the first transaction is of a particular
type, and see what that implies for the next transaction, which will either be
the same type or one of the other two.
You can do this iteratively, and ultimately you’ll see one of the following:

1. You have consumed the entire session, with only three transaction types
   appearing.
1. You have found some number of transaction types other than three.
1. You either have bytes left in the session that cannot form a new transaction,
   or your final transaction is incomplete.

The first of these should indicate successful parsing.
Either of the others means you have made an incorrect assumption/guess during
your parsing, and you need to unwind.
Make sure you keep track of what ciphertext corresponds to what type of field as
you parse --- no two fields of different types will encrypt to the same
ciphertext.

You should be able to enumerate all of the possibilities in your code, since
there are only six ways to order three message types by the order in which they
*first* occur.

### Examining Binary Files

You are strongly encouraged to view the ciphertext streams through a
hex-formatting program like `xxd` or `hexdump`.

### Common Code

While you must provide four separate executables, you will probably want to have
some code in common between them.
This might be an additional `.c` and `.h` file, a python file to import, or
something else.
You might even have a single binary, and bash scripts to call it with
appropriate arguments for the individual tasks.

### Using C

C is a good language for working with binary data, though you are free to use
any language already installed in the `baseline` image.
We *will not* install additional packages for you, and you *should not* assume
Internet access when building or running your code.

If you would like to use C, here are some things you might find useful.

#### Writing to STDERR

Since we're expecting very specific output in STDOUT, all of your debugging
output should go to STDERR.
You can do this with:

```C
fprintf(stderr,"This is an error message\n");
```

You might also find the following useful:

```C
fprintf(stderr,"%s: %d\n",__FILE__,__LINE__);
```

The preprocessor will replace `__FILE__` with the current file name, and
`__LINE__` with the current line of the file.
That means you can add this exact line anywhere you like in your code to trace
the program flow.
This can be extremely helpful when you're trying to track down where a segfault
is occurring.

#### Working with Binary Data

Be careful when working with ciphertext that you *do not* use string functions.
Instead, you should use `memcpy` to copy data from one binary array to another,
and `memcmp` to compare arrays.
See the documentation for both of these.

#### Linked Lists

C does not have a library of basic data structures.
Rather than creating overly-large arrays or reallocating memory and copying as
you need more space, it is easy to write a simple linked list.
We generally do this structure-by-structure.

Consider a structure:

```C
struct foo {
   int a;
   double b;
}
```

There are two approaches to creating a linked list of `struct foo`.
One is to define a new enclosing structure:

```C
struct foo_list {
   struct foo item;
   struct foo_list* next;
}

struct foo_list  foo_HEAD;
struct foo_list* foo_TAIL = &foo_HEAD;
```

The global (or file-static) `foo_HEAD` is a dummy entry, and we add to the list
by allocating a new `struct foo_list` (with `malloc`), assigning it to
`foo_TAIL->next`, and updating `foo_TAIL` to this new pointer.

The other way is to combine everything into a single `struct`:

```C
struct foo {
   int a;
   double b;
   struct foo* next;
}

struct foo  foo_HEAD;
struct foo* foo_TAIL = &foo_HEAD;
```

Which you use is largely a matter of style and preference.
(It is also possible to define a generic linked list with a `void*` item, but
you lose any type information when you do this.)

### Using Python

#### Writing to STDERR

Since we're expecting very specific output in STDOUT, all of your debugging
output should go to STDERR.
You can do this with:

```python
print("This is an error message", file=sys.stderr)
```

You might also find the following useful:

```python
import sys

_lno = lambda: print(__file__,sys._getframe().f_back.f_lineno,file=sys.stderr)
```

Put this after your other imports.
This grabs the current stack frame, and extracts the current line number.
The `__file__` variable is replaced with the current file name.
To use this, call `_lno()` anywhere in your code, so that you can trace the
program flow.
This can be extremely helpful when you're trying to track down where an error is
occurring.

#### Working with Binary Data

Binary data in python is a little more cumbersome than in C, but it is otherwise
generally easier to write code in, and has useful structures like `list` and
`dict`.

Section 7.3 of *A General Systems Handbook* briefly discusses files, but this is
from the perspective of text files.
For binary files, you need to add the specifier `b` when opening the file:

```python
with open('input_file.bin', 'rb') as in_file:
    data = in_file.read(nbytes)       # data is of type "bytes"
    hexdata = data.hex()              # hexdata is a string of hex digits
```

If you want to convert a string of hexadecimal digits back to bytes, you can do
this with:

```python
    bindata = bytes.fromhex(hexdata)  # bindata should be the same as data
```

Writing is similar:

```python
with open('output_file.bin', 'wb') as out_file:
    out_file.write(bindata)   # the bytes object above
```

Python also makes it easy to tell if an item is in a list:

```python
l = ['a' , 'b', 'c']
'a' in l # evaluates to True
'd' in l # evaluates to False
```

This also works for dictionary keys:

```python
d = dict()
d['a'] = 1
d['b'] = 2
d['c'] = 3
'a' in d # evaluates to True
'd' in d # evaluates to False
```
