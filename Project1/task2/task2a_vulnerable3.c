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
    // NOTE: 
    const unsigned dest_size = 64;
    const unsigned src_len = strnlen(src, dest_size);
    char new_src[src_len];
    memcpy(new_src, src, src_len);
    strcpy(dest, new_src);
    memset(dest+src_len, '\0', dest_size-src_len);

    return 0;
}

int secure_sprintf (char *dest, const char *src)
{
    // NOTE: This is honestly the best way I see of making this safe.
    // src might not be null-terminated, in which case strlen will fail.
    // We also cannot do sizeof(src) because if it is dynamically-allocated,
    // then it will just return the size of the pointer which is always 8 bytes.
    // on a 64-bit system. I hate C.

    // NOTE: Also, if dest is heap-allocated, then we cannot get its size.
    // However, in the original task, dest is stack-allocated and so I am going
    // to assume that it is always stack-allocated.
    const unsigned long dest_size = 512;
    const unsigned long src_len = strnlen(src, dest_size);
    char new_src[src_len];
    memcpy(new_src, src, src_len);
    sprintf(dest, "%s", new_src);
    memset(dest+src_len, '\0', dest_size-src_len);

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
