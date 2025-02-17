# Project 1: Buffer overflows (test)

Note: your code will be graded in the provided virtual machine (VM), so I highly recommend that you write and test your code in the provided VM. (If you are reading this shortly after it was posted, the VM may not be available just yet -- stay tuned! Once it is available, the file itself and setup instructions will be on Moodle.)

## Task 0: Off to a malicious start!

To start things off, you've been provided a program that you get to write part of.

The program we provide:
- Takes as a command-line argument the name of a file, which it opens.
- The program then asks your function (`your_fcn()`) for a string, writes
  that string to the file, and closes the file.
- It then re-opens the file, and attempts to "sanitize" the string you provided
  by removing all instances of the substring "`368`" from it.
- Finally, it overwrites the file with this "sanitized" version of the string.

For example, if your function generated the string "I love CSCI 368!" then
we would get the following output:

```
csci368@csci368:~$ ./task0a.x file.txt
csci368@csci368:~$ cat file.txt
I love CSCI !
```

**Your goal is to write `your_fcn()` in such a way that the file whose name is
provided on the command-line still has the substring "`368`" in it.**

A simple way to test whether your solution correct is with the command `grep 368`.
You can use it to return lines
containing the string "`368`" as follows:

```
csci368@csci368:~$ grep 368 file.txt
```

If this returns nothing (as the above case would), then that means the file
does not have "`368`" in it.

We provide three copies of this program: `task0a.c`, `task0b.c`, and
`task0c.c`. All of them have the same goal, but **you must provide a
*functionally unique* solution in each of the three**; they cannot simply be
slight variations of one another (e.g., if one of your solutions was
`printf(x)` then another cannot be `puts(x)` as those are functionally the same
thing).

There are many different ways to solve this! Some might involve buffer
overflows, and some might not. We are not requiring any one particular
solution; just like with any attack, what matters is whether it achieves
the goal, even if it's not a method we anticipated. *Be creative* and have fun
with it!

Please note that your solution is *only* allowed to modify `your_fcn()` in these files.
(You may change other parts of the files for testing, but keep in mind that
when I grade your project, I will paste your `your_fcn()` into a fresh copy of the starter code.)

## Task 1: Attacking a vulnerable program with malicious inputs

In Task 1, you will be exploiting programs that have a buffer overflow
vulnerability. Unlike Task 0, you are not allowed to modify the program itself;
instead, you will be attacking it by cleverly constructing malicious inputs to
the program.

The vulnerable programs are `vulnerable1.c` through `vulnerable3.c`; your solutions
cannot involve modifying those programs (when I grade your project, I will use
unmodified versions of the vulnerable programs). Rather, you will be modifying
the corresponding exploit programs, `exploit1.c` through `exploit3.c`.

Each of the vulnerable programs is a little different. They've been designed so that
they increase in difficulty; you'll apply what you learn in solving `vulnerable1` to
inform how to do `vulnerable2`, which will in turn inform you how to do
`vulnerable3`, where you will be injecting code that will reveal a secret!

**In `vulnerable1` and `vulnerable2`, your goal is to get `sensitive_function` to run. In `vulnerable3`, your goal is to run the provided shellcode as root.**

The way these programs interact with one another is through some networking
code that has been provide for you in `comms.h` and `comms.c`; you do not have to
concern yourself with the networking code. Just focus on constructing the right
`greeting` (when appropriate), analyzing the vulnerable program's `response`
(when appropriate), and constructing the correct `buffer` to solve the given
challenge.

### Building and running the programs

There is a provided Makefile for compiling the files in Task 1. Your solutions cannot
involve modifying the Makefile (when I grade your projects, I will use
the unmodified version of the Makefile).

To run, first build using `make`. Then, run the `vulnerable` program first,
followed by the corresponding `exploit` program. For example:

```
csci368@csci368:~$ ./vulnerable1.x &
csci368@csci368:~$ ./exploit1.x
```

The first command's use of `&` means to run the vulnerable program in the background;
this means that it will immediately return us to the terminal but it will keep
running. Alternatively, you could run `vulnerable` in one terminal and `exploit`
in another (this is especially helpful if you want to run the `vulnerable` programs
in `gdb`, and trust us, you do!).

In `exploit3.c`, you will see that you have been provided with the shell code. In fact,
there are two examples of shell code: one (commented out) is traditional shell
code that actually launches a shell (`/bin/sh`). The other "shell code" 
provided executes a different command: `/bin/cat /var/secret/token`; this is a
file created by the Makefile that only root has access to read. Because
`vulnerable3.x` is compiled as root, when injected with this shell code, it will
be able to read the token---even if the user running `vulnerable3.x` is not root!
This shows the power of privilege escalation.

## Task 2: Writing secure code

In the last task, your job is to patch `vulnerable3.c` to defend against exactly the sort of exploits you wrote in part 1.  `task2a_vulnerable3.c`
and `task2b_vulnerable3.c` contain the same `main` function as `vulnerable3.c`. Each of those files contains the beginnings of a defense. In 2a, you'll write more secure versions of `strcpy` and `sprintf`; in 2b, you'll write a filtering function that checks user input against a file (`allowlist`). Replace the `TODO`s with your own code (leaving other parts alone, as before); for part 2b, you may also 
edit the `allowlist` file.

**Your goal for both `task2a_vulnerable3.c`
and `task2b_vulnerable3.c` is to ensure that `main` exits normally, despite an attacker's best efforts to craft clever responses.** (To receive full credit, your defense should be robust, in the sense that it should defend against an attacker able to apply the general techniques you leveraged in Part 1, rather than any single specific implementation of an attack you may have written for in Part 1.)

## Submission Checklist

A finished project will consist of the following (completed) files:
- task0a.c
- task0b.c
- task0c.c
- exploit1.c
- exploit2.c
- exploit3.c
- allowlist
- task2a_vulnerable3.c
- task2b_vulnerable3.c

You don't need to upload any other files (and your solutions shouldn't rely on any additional files beyond the (unmodified) starter code).

Submissions that are missing some of these files will still be graded, but you won't earn any points for the missing parts.

## Grading Criteria

A solution is correct if running the program results in the goal behavior (while abiding by any restrictions or limitations mentioned above). Correct solutions will receive full credit. Incorrect code that shows evidence of progress towards a correct solution may receive a limited amount of partial credit.

* Task 0 [30 points]
  - 10 points awarded per distinct correct implementation of `your_fnc` across `task0a.c`, `task0a.c`, and `task0a.c`.
* Task 1 [40 points]
  - 15 points for each of `vulnerable1.c` and `vulnerable2.c`; 10 points for `vulnerable3.c`
* Task 2 [30 points]
  - 15 points for each of `task2a_vulnerable3.c` and `task2a_vulnerable3.c`

Total: 100 points

## Miscellaneous helpful tips

### Not all character arrays are "C strings"

Recall: `strlen()` only works for null-terminated ASCII strings; if you are
sending binary data then you either want to compute the length of the data
you're sending or at least use `sizeof(buffer)`.
     
### gdb is your friend!

Here are some useful commands for running and stepping through a running program:
- `b <function name>`: Set a breakpoint at `<function name>` (pause execution whenever
  that function is reached).
- `r`: Run the program from the beginning.
- `s`: (Used after you've hit a breakpoint.) Step to the next line of code.
- `c`: (Used after you've hit a breakpoint.) Continue: let the program execute
  until it hits another breakpoint.

Here are some useful commands for inspecting the memory of the running process:
- `i f` (short for `info frame`): Shows info about the current stack frame,
  including the addresses where the saved %eip and %ebp are.
- `x/32xw <address>`: Print 32 words (`w`) in hex format (`x`) starting at
  address `<address>`. The `<address>` can either be a raw address (e.g.,
  `0x12345678`) or a function name (e.g., `main`).
- `p <C expression>`: Print the result of the expression, which could be a
  variable (`p var`), the address of a variable (`p &var`), or a more 
complicated C expression (`p (int) strlen(var)`).
