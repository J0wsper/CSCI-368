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

int allowlist_finder(char* input, char* allowlist_line) {
    for (int i = 0; input[i] != '\0' && i < 128; i++) {
        int pass = 0;
        for (int j = 0; allowlist_line[j] != '\0'; j++) {
            if (input[i] == allowlist_line[j]) {
                pass = 1;
                break;
            }
        }
        if (!pass) {
            return 0;
        } 
    }
    return 1;
}

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
        int pass = allowlist_finder(input, f_buffer);
        if (!pass) {
            perror("Input did not pass the allowlist");
            return EXIT_SUCCESS;
        }
    }
    const size_t input_size = 64;
    const size_t input_len = strnlen(input, input_size);
    memcpy(input, input, input_len);

    // NOTE: This will always cause a segmentation faults in some cases. However, in the use case present
    // in vulnerable3.c, it should be fine.
    memset(input+input_len, '\0', 1);

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
