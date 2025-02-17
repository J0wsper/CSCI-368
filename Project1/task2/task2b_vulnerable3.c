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


char* your_input_filter (char *input)
{
    FILE *config_file;
    config_file = fopen("allowlist", "r");
    char f_buffer[128];

    if (config_file == NULL) {
        perror("Error opening the configuration file.");
        return EXIT_SUCCESS;
    }

    while (fgets(f_buffer, sizeof(f_buffer), config_file) != NULL) { 
        // read the allowed input from the config file, line by line
        if (f_buffer[strlen(f_buffer) - 1] == '\n') {
            f_buffer[strlen(f_buffer) - 1] = '\0';
        }

        /* TODO: Check the input with allow list*/
    }

    /* TODO: Write the remaining code to filter the input. */

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
