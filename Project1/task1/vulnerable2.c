/*
 * CSCI 368, Spring 2025 
 * Project 1 | task1
 *
 * Build instructions:
 *   We will be building this with the Makefile provided; you may not make
 *   any changes to the Makefile.
 *
 * Submission instructions:
 *   You may not make changes to this file; only to exploit2.c
 *   When we grade your project, we will use an unmodified version of this file.
 */

#include <stdio.h>
#include <stdlib.h>
#include <string.h>

// for communicating with the exploit program
#include "comms.h"

static char greeting[128];
static char response[512];
static sender_info sender;

void buffer_overflow()
{
    char buffer[32];
    recvfrom_exploit(buffer, 48, &sender); // oops!
}

void sensitive_function()
{
    puts("Full points! (Hey wait a minute.. How did you get here?)");
    exit(EXIT_SUCCESS);
}

int main()
{
    int local = 368;

    init_vulnerable(); // setup comms

    // Echo the greeting back to the exploit program
    recvfrom_exploit(greeting, sizeof(greeting)-1, &sender);
    sprintf(response, greeting); // oops!
    sendto_exploit(response, strlen(response), &sender);

    buffer_overflow();

    puts("Overflow failed; program terminated successfully.");

    return EXIT_SUCCESS;
}
