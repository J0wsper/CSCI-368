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

// for communicating with the exploit program
#include "comms.h"

static char greeting[128];
static char response[512];
static char input_buffer[512];
static sender_info sender;

void buffer_overflow(char *input)
{
    char buffer[64];
    strcpy(buffer, input); // oops!
}

int main()
{
    int local = 368;

    init_vulnerable(); // setup comms

    // Echo the greeting back to the exploit program
    recvfrom_exploit(greeting, sizeof(greeting)-1, &sender);
    sprintf(response, greeting); // oops!
    sendto_exploit(response, strlen(response), &sender);

    recvfrom_exploit(input_buffer, sizeof(input_buffer), &sender); // oops!
    buffer_overflow(input_buffer);

    puts("Overflow failed; program terminated successfully.");

    return EXIT_SUCCESS;
}
