/*
 * CSCI 368, Spring 2025
 * Project 1 | task2
 *
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

int secure_strcpy (char *dest, const char *src)
{
    /* TODO: Write a more secure strcpy with bound checking.
    You may use strcpy at any time, but you must check the bounds of the input
    */

    unsigned dest_len = sizeof(dest);
    strncpy(dest, src, dest_len);
    dest[dest_len-1] = '\0';
    return 0;
}

int secure_sprintf (char *dest, const char *src)
{
    /* TODO: Write a more secure sprintf with bound checking.
    You may use sprintf at any time, but you must check the bounds of the input
    */
    
    unsigned dest_len = sizeof(dest);
    snprintf(dest, dest_len, "%s", src);
    dest[dest_len-1] = '\0';
    return 0;
}

void buffer_overflow(char *input)
{
    char buffer[64];
    secure_strcpy(buffer, input); // oops!
}

int main()
{
    int local = 368;

    init_vulnerable(); // setup comms

    // Echo the greeting back to the exploit program
    recvfrom_exploit(greeting, sizeof(greeting)-1, &sender);
    secure_sprintf(response, greeting); // oops!
    sendto_exploit(response, strlen(response), &sender);

    recvfrom_exploit(input_buffer, sizeof(input_buffer), &sender); // oops!
    buffer_overflow(input_buffer);

    puts("Overflow failed; program terminated successfully.");

    return EXIT_SUCCESS;
}
