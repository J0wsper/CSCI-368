/*
 * comms.h and comms.c provide some extra plumbing to make
 * it easy for the exploit and vulnerable programs to send
 * and receive data from one another.
 *
 * You are not allowed to modify this file. When we grade
 * your project, we will use an unmodified version of it.
 */

#ifndef COMMS_H
#define COMMS_H

#include <sys/types.h>
#include <arpa/inet.h>
#include <sys/socket.h>

typedef struct {
	struct sockaddr_in addr;
	struct sockaddr* saddr;
	socklen_t addr_size;
} sender_info;

/* setup communication in the vulnerable program */
void init_vulnerable();

/* read up to msglen bytes; return number of bytes read */
ssize_t recvfrom_exploit(char *msg, size_t msglen, sender_info *sender);

/* send resplen bytes of resp; return number of bytes sent */
ssize_t sendto_exploit(const char *resp, size_t resplen, sender_info *sender);

/* send msglen bytes of msg, and receive up to resplen bytes into resp;
 * return number of bytes received */
ssize_t sendto_and_recvfrom_vulnerable(const char *msg, size_t msglen,
		char *resp, size_t resplen);

/* send msglen bytes of msg; return number of bytes sent */
ssize_t sendto_vulnerable(const char *msg, size_t msglen);

#endif
