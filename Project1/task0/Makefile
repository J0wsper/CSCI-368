CC = gcc
CFLAGS = -Wall -O0 -g -m32
NO_CANARY = -fno-stack-protector

EXECS = task0a.x task0b.x task0c.x

all : $(EXECS)

%.x : %.c
	$(CC) $(CFLAGS) $(NO_CANARY) -o $@ $<

clean : 
	rm -f $(EXECS)

.PHONY: clean
