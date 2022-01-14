/************************************************************************
Confidential Trade Secret.
Copyright (c) 1986 Data Access Corporation, Miami Florida,
as an unpublished work.  All rights reserved.
DataFlex is a registered trademark of Data Access Corporation.
************************************************************************/

#include <blkio.h>

#define DISK   0
#define CON    1
#define UNBUFF 2
#define LST    3
#define LSTSPL 4

struct sfcbtype {
/* 0  */   char    name[FCBSIZE] ;
/* 42 */   long    recnum;
/* 46 */   byte    seqcode;	/* CONSOLE IO FLAG */
/* 47 */   byte    wrtflag;
/* 48 */   int16     nextpos;	/* NEXT CHR POS */
/* ?? */   int16     bytesread;
/* 50 */   char    seqbuf[128] ;
};

#define SFCBSIZE sizeof( struct sfcbtype )

