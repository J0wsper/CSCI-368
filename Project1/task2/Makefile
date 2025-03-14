CC = gcc
CFLAGS = -Wall -O0 -g -m32 -ggdb
NO_CANARY = -fno-stack-protector

EXPLOITS = exploit1.x exploit2.x exploit3.x
VULNERABLE = vulnerable1.x vulnerable2.x vulnerable3.x
EXECS = $(EXPLOITS) $(VULNERABLE)

all : $(EXECS)

comms-nocanary.o : comms.c comms.h
	$(CC) $(CFLAGS) $(NO_CANARY) -c -o $@ $<

comms.o : comms.c comms.h
	$(CC) $(CFLAGS) -c -o $@ $<

vulnerable1.x : vulnerable1.c comms-nocanary.o
	$(CC) $(CFLAGS) $(NO_CANARY) -o $@ $^

vulnerable2.x : vulnerable2.c comms-nocanary.o
	$(CC) $(CFLAGS) $(NO_CANARY) -o $@ $^

# All the other executables have stack protector turned on
%.x : %.c comms.o
	$(CC) $(CFLAGS) -o $@ $^

clean : 
	rm -f $(EXECS) *.o *.x
