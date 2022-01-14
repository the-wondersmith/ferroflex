/************************************************************************
Confidential Trade Secret.
Copyright (c) 1986 Data Access Corporation, Miami Florida,
as an unpublished work.  All rights reserved.
DataFlex is a registered trademark of Data Access Corporation.
************************************************************************/

#include <stdio.h>

#define bool			unsigned char
#define TRUE			1
#define FALSE			0

#define ord(x) 			(int)(x)
#define chr(x) 			(char)(x)
#define nil  			NULL
#define eoln(p) 		(*((p)->_ptr) == '\n')
#define length(s) 		strlen(s)
#define odd(i)			((i)%2?0:1)
#define pred(x)			(x-1)
#define succ(x)			(x+1)

extern	char	*malloc();

int	ioresult;
char    p2c;
char	p2c_tstr[256];

double arctan(),ln(),log(),sqr(),pwroften();

char *copy(),*concat();

char *p2cladd(),*p2clsub(),*p2clmul(),*p2cldiv(),*p2cldiv(),*p2clplus();
char *p2clminus(),*p2clmod();

char *set_make(),*p2cset(),*p2csadd(),*p2cssub(),*p2csprod();
char *set_union(), *set_diff(), *set_inter();

