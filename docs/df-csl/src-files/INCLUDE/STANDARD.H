/************************************************************************
Confidential Trade Secret.
Copyright (c) 1986 Data Access Corporation, Miami Florida,
as an unpublished work.  All rights reserved.
DataFlex is a registered trademark of Data Access Corporation.
************************************************************************/

#include <stdio.h>
extern int newcount;

#define ord(x) 			(int)(x)
#define chr(x) 			(char)(x)
#define nil  			NULL
#define eoln(p) 		(*((p)->_ptr) == '\n')
#define length(s) 		strlen(s)
#define odd(i)			((i)%2?0:1)
#define pred(x)			(x-1)
#define succ(x)			(x+1)

#define clrbit(i,n)	(i &= ~(1 << (n)))
#define setbit(i,n)	(i |= (1 << (n)))
#define tstbit(i,n)	((i & (1 << (n))) == (1 << (n)))
#define shl(i,n)	(i << (n))
#define shr(i,n)	(i >> (n))
#define addr(x)		&(x)
#define wrd(x)		(int)(x)
#define gnb(f)		fgetc(f)
#define wnb(f,c)	fputc(c,f)
#define uppercase 	toupper
#define writeint(ii)	writelong( (long) ii )
#define p2crasgn(d,n,s) mov32(s,d,n)

/*****
#define new( ptr, size )	{(char *)ptr = malloc( size );newcount++;}
#define stralloc( ptr, astr )   {ptr = strcpy(malloc(strlen(astr)+1),astr);newcount++;}

#define dispose( ptr )		{if (ptr) {free( ptr );ptr = NULL;newcount--;}}
*****/
#define new( ptr, size )	df_new( &ptr, size );
#define stralloc( ptr, astr )	{df_new( &ptr,strlen(astr)+1); strcpy( ptr, astr);}
#define dispose( ptr )		df_dispose( &ptr );

#define STRING 81
#define LSTRING 256
#define NAMESIZE 40
#define NAMESIZE2 41
#define TRUE 1
#define FALSE 0
#define BLKIOSIZE 128
#define true 1
#define false 0
#define t_o -2
#define e_n_d -1
#define T_O -2
#define E_N_D -1
/*#define TESTGRP 20*/

#define NUMSIZE 	12
#define SIGN 		11
#define POINTPLACE	6
#define MAXDEC 		8
#define MAXLEFT		14
#define PLUS 		0
#define MINUS 		0xff
#define ROOTDIR 	"/" /* default*/

extern int     ioresult;
extern char    p2c;
extern char    p2c_tstr[LSTRING];

double arctan(),ln(),log(),sqr(),pwroften();

extern char *copy(),*concat();
extern char *fgetdt();
#ifdef __TURBOC__
void *_Cdecl malloc (unsigned size); /* for TURBOC conflict! */
#else
extern char *malloc();
#endif

extern double cvnf();
#ifndef __TURBOC__
extern long labs();
#endif
extern long   atol();
extern double atof();
extern char *nstrcat(), *cvis(), *cvls(), *strccpy(), *strccat();
extern char readcon(), *passtr();
extern int  readchar(), readcnd(), press();

extern char *strcat(), *strchr(), *strrchr(), *strcpy(), *strpbrk();
extern char *p2cladd(),*p2clsub(),*p2clmul(),*p2cldiv(),*p2cldiv(),*p2clplus();
extern char *p2clminus(),*p2clmod();

extern char *set_make(),*p2cset(),*p2csadd(),*p2cssub(),*p2csprod();
extern char *set_union(), *set_diff(), *set_inter();

#include <system.h>

#if (WORDALIGN == 0)
#define moveleft mov32
#define move mov32
#else
#define move(d,s,n)	moveleft(d,s,n)
#endif

#if (BYTESWAP==1)
#define sexy16(x) (x)
#define sexy32(x) (x)
#else
#define sexy16(x) (((x & 255)<<8) | ((x>>8) & 255))
#define sexy32(x) ( ((x<<24) & 0xff000000) | ((x<<8)  & 0xff0000) | ((x>>8)  & 0xff00) | ((x>>24) & 0xff))		
#endif
#define swap16(x) (((x & 255)<<8) | ((x>>8) & 255))
#define swap32(x) ( ((x<<24) & 0xff000000) | ((x<<8)  & 0xff0000) | ((x>>8)  & 0xff00) | ((x>>24) & 0xff))		

#define cswap16(c,x) (c?(((x & 255)<<8) | ((x>>8) & 255)):x)
#define cswap32(c,x) (c?( ((x<<24) & 0xff000000) | ((x<<8)  & 0xff0000) | ((x>>8)  & 0xff00) | ((x>>24) & 0xff)):x)

#define bool			byte

extern byte ftype(),status();
extern int32 bigmem(), cvsl(), cvnl();
extern bool valid_name();

/**** global flex variables ****/

extern bool 	err, errscrn, waserr, errdsk;
extern int  	ern, errline, errxline;
extern char	errmsg[];
extern bool	thrdchar;
extern byte	multiuser;
extern int	iomask;
extern bool	seqendfile, capslock, numbersonly;
extern byte	enclen;
extern char	encbuf[LSTRING];
extern int	crntcolor;
extern int	crntcls;
extern char	*keyp;
extern byte 	lockcount;
extern int32	 *cnsame;
extern int	blkresult;

