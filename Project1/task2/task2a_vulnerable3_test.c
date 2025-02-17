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
    unsigned src_len = strlen(src);
    unsigned* min;
    if (src_len < dest_len) {
	min = &src_len;
    }
    else {
	min = &dest_len;
    }
    snprintf(dest, *min, "%s", src);
    strcat(dest, "\0");
    return 0;
}

void buffer_overflow(char *input)
{
    char buffer[64];
    secure_strcpy(buffer, input); // oops!
}

int main()
{
    int local = 0x80;

    // Testing how the secure_sprintf handles format specifiers
    char* test = "\x080";
    char dest[4] = "hi";
    secure_sprintf(dest, test);
    printf("Test value: %s", dest);

    puts("Overflow failed; program terminated successfully.");

    return EXIT_SUCCESS;
}
