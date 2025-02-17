/*
 * comms.h and comms.c provide some extra plumbing to make
 * it easy for the exploit and vulnerable programs to send
 * and receive data from one another.
 *
 * You are not allowed to modify this file. When we grade
 * your project, we will use an unmodified version of it.
 */

#include "comms.h"

#include <stdarg.h>
#include <unistd.h>
#include <stdlib.h>
#include <string.h>
#include <stdio.h>
#include <sys/socket.h>
#include <fcntl.h>
#include <sys/errno.h>

static int g_vuln_sockfd;
static const unsigned short VULN_PORT = 41480;

void init_vulnerable()
{
	// set up our socket for communication
	g_vuln_sockfd = socket(AF_INET,SOCK_DGRAM,IPPROTO_UDP);
	struct sockaddr_in addr;
	bzero(&addr, sizeof(addr));
	addr.sin_family = AF_INET;
	addr.sin_addr.s_addr = htonl(INADDR_ANY);
	addr.sin_port = htons(VULN_PORT);
	bind(g_vuln_sockfd, (struct sockaddr*)&addr, sizeof(addr));
}

int recvfrom_exploit(char *msg, size_t msglen, sender_info *sender) 
{
	// reinitialize the 'sender' info
	bzero(&sender->addr, sizeof(sender->addr));
	sender->saddr = (struct sockaddr*) &sender->addr;
	sender->addr_size = sizeof(sender->addr);

	return recvfrom(g_vuln_sockfd, msg, msglen, 0, sender->saddr, &sender->addr_size);
}

int sendto_exploit(const char *resp, size_t resplen, sender_info *sender)
{
	return sendto(g_vuln_sockfd, resp, resplen, 0, sender->saddr, sender->addr_size);
}

ssize_t sendto_and_recvfrom_vulnerable(const char *msg, size_t msglen, char *resp, size_t resplen)
{
	ssize_t 	    rv;
	int                 sockfd;
	struct sockaddr_in  srv_addr;
	struct sockaddr*    srv_saddr;
	socklen_t           srv_addr_size;

	// 'connect' to the vulnerable program
	sockfd = socket(AF_INET,SOCK_DGRAM,IPPROTO_UDP);
	bzero(&srv_addr, sizeof(srv_addr));
	srv_addr.sin_family = AF_INET;
	srv_addr.sin_addr.s_addr = htonl(INADDR_LOOPBACK);
	srv_addr.sin_port = htons(VULN_PORT);
	srv_addr_size = sizeof(srv_addr);
	srv_saddr = (struct sockaddr*) &(srv_addr);

	// Send the msg and receive the resp
	rv = sendto(sockfd, msg, msglen, 0, srv_saddr, srv_addr_size);

	if(rv < 0)
		return rv;

	if(resp != NULL) {
		bzero(resp, resplen);
		rv = recvfrom(sockfd, resp, resplen, MSG_WAITALL, NULL, NULL);
	}

	return rv;
}

ssize_t sendto_vulnerable(const char *msg, size_t msglen)
{
	return sendto_and_recvfrom_vulnerable(msg, msglen, NULL, 0);
}
