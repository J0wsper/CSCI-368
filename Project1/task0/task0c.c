#include <stdio.h>
#include <stdlib.h>
#include <string.h>

int your_func(char* your_string) 
{
    // Attempt C: Buffer Overflowing the scanning character array
    char* new_str = malloc(104*sizeof(char));
    memset(new_str, 'x', 100);
    new_str[100] = '3';
    new_str[101] = '6';
    new_str[102] = '8';
    strcpy(your_string, new_str);
    return 0;
}

// https://stackoverflow.com/questions/47116974/remove-a-substring-from-a-string-in-c
char *strremove(char *str, const char *sub) 
{
    size_t len = strlen(sub);
    if (len > 0) 
    {
        char *p = str;
        while ((p = strstr(p, sub)) != NULL) 
        {
            memmove(p, p + len, strlen(p + len) + 1);
        }
    }
    return str;
}

int main(int argc, char *argv[]) 
{
    if(argc != 2) 
    {
        fprintf(stderr, "Usage: %s <filename>\n", argv[0]);
        exit(EXIT_FAILURE);
    }

    char fname[100];
    char str[10];
    FILE *fp;
    
    memset(fname, '\0', sizeof(fname));
    strncpy(fname, argv[1], sizeof(fname)-1);

    // Write *your* string to the file
    fp = fopen(fname, "w+");
    your_func(str);
    fprintf(fp, "%s", str);
    fclose(fp);

    char new_str[100];
    memset(new_str, '\0', sizeof(new_str));

    // Read in the string and sanitize it
    fp = fopen(fname, "r");
    fscanf(fp, "%s", new_str);
    strremove(new_str, "368");
    fclose(fp);

    // Rewrite the file with the sanitized string
    fp = fopen(fname, "w+");
    fprintf(fp, "%s", new_str);
    fclose(fp);
    
    return 0;
}
