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

int remove_percent(char* input, const size_t length) {
    char* src = input;
    char* dest = input;
    
    // Kind of hacky but it works
    for (int i = 0; *src != '\0' && i < length; src++) {
        *dest = *src;
        if (*dest != '%') {
            dest++;
        }
    }

    // Add the null terminator
    *dest = '\0';
    input = dest;
    return 0;
}

char* your_input_filter (char *input)
{
    //NOTE: This solution doesn't use allowlist. I opted instead to just remove percent signs
    //indiscriminately because they could be format specifiers and this is the main type of 
    //behavior we are trying to avoid here.
    const size_t input_size = 64;
    const size_t input_len = strnlen(input, input_size);

    // Copying over our input into its bounds.
    memcpy(input, input, input_len);
    int res = remove_percent(input, input_len);
    
    // If somehow res is not 0, return that something bad happened
    if (!res) {
       perror("Sanitization unsuccessful, stopping now.");
       return NULL;
    }
    return input;
}

void buffer_overflow(char *input)
{
    input = your_input_filter(input);
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
