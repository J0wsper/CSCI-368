CC = gcc
CFLAGS = -Wall -O0 -g -m32 -ggdb
NO_CANARY = -fno-stack-protector

EXPLOITS = exploit1.x exploit2.x exploit3.x
VULNERABLE = vulnerable1.x vulnerable2.x vulnerable3.x
EXECS = $(EXPLOITS) $(VULNERABLE)

all : $(EXECS) chown token

token :
	@echo "Setting up secret token in /var/secret/token"
	@sudo mkdir -p /var/secret
	@sudo sh -c 'echo "Token is yours!" > /var/secret/token'
	@sudo chmod go-r /var/secret/token

nochown: $(EXPLOITS) $(VULNERABLE)

comms-nocanary.o : comms.c comms.h
	$(CC) $(CFLAGS) $(NO_CANARY) -c -o $@ $<

comms.o : comms.c comms.h
	$(CC) $(CFLAGS) -c -o $@ $<

chown: vulnerable3.x
	sudo chown root $^
	sudo chmod +s $^

vulnerable1.x : vulnerable1.c comms-nocanary.o
	$(CC) $(CFLAGS) $(NO_CANARY) -o $@ $^

vulnerable2.x : vulnerable2.c comms-nocanary.o
	$(CC) $(CFLAGS) $(NO_CANARY) -o $@ $^

vulnerable3.x : vulnerable3.c comms-nocanary.o
	$(CC) $(CFLAGS) $(NO_CANARY) -z execstack -o $@ $^

# All the other executables have stack protector turned on
%.x : %.c comms.o
	$(CC) $(CFLAGS) -o $@ $^

clean : 
	rm -f $(EXECS) *.o

.PHONY: clean chown nochown
