/*
 * CSCI 368, Spring 2025
 * Project 1 | task2
 *
 */

#include <stdio.h>
#include <stdlib.h>
#include <string.h>

static char greeting[128];
static char response[512];
static char input_buffer[512];

// NOTE: I added this in for convenience but it could just as easily be added into the function calls.
static unsigned long MAX_SIZE = 512;

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
    // Testing how the secure_sprintf handles format specifiers
    char test_buff[10];
    memset(test_buff, 'x', 10);
    char dest[512];
    secure_sprintf(dest, test_buff);
    printf("Test value: %s\n", dest);
    puts("Overflow failed; program terminated successfully.");
    return EXIT_SUCCESS;
}
