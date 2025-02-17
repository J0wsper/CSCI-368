/*
 * CSCI 368, Spring 2025
 * Project 1 | task1
 *
 * Build instructions:
 *   During grading, this will be built with the Makefile provided;
 *   your solution may not make any changes to the Makefile.
 *
 * Submission instructions:
 *   Your solution may not make changes to this file; only to exploit1.c.
 *   (Your project will be graded using an unmodified version of this file.)
 */

#include <stdio.h>
#include <stdlib.h>
#include <string.h>

// for executing and communicating with the exploit program
#include "comms.h"

static sender_info sender;

void sensitive_function()
{
    printf("Full points! (Hey wait a minute.. How did you get here?)");
    exit(EXIT_SUCCESS);
}

void buffer_overflow()
{
    char secret[32];
    char buffer[16];

    sprintf(secret, "authorized personnel only");

    recvfrom_exploit(buffer, 48, &sender); // oops!

    if(strcmp(secret, "Backdoor!") == 0)
        sensitive_function();
}

int main()
{
    int local = 368;

    init_vulnerable(); // setup comms

    buffer_overflow();

    puts("Overflow failed; program terminated successfully.");

    return EXIT_SUCCESS;
}
