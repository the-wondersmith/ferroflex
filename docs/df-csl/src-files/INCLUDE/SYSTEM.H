/************************************************************************
Confidential Trade Secret.
Copyright (c) 1986 Data Access Corporation, Miami Florida,
as an unpublished work.  All rights reserved.
DataFlex is a registered trademark of Data Access Corporation.
************************************************************************/


/* msdos definition module */


#define int8 unsigned char
#define byte unsigned char
#define int16 short
#define best int
#define int32 long
#define word unsigned short
#define byte unsigned char
#define longint long

#define PRINTERNAME "=PRN:"
#define AUXNAME "%AUX"
#define AUTOSPOOL 0
#define AUTOSEARCH 1
#define BYTESWAP 1
#define NIBLESWAP 1
#define WORDALIGN 3
/* DRIVESPEC = cpm or msdos */

extern int errno;

#define sqfclose(sfcb,result)		blkclose(sfcb,result)

#define spool_open(sfcb, fname )	sqfcreate( sfcb, fname, &blkresult )

#define lockf( hdl, func, where )	locking( hdl, func, where )

#define wait( sec )			sleep( sec )

#define ANSII			1	/* ANSII compatible compiler */
#define MSDOS			1	/* for dos */
#define NO_USER_COUNT		1	/* dont count users */
#define PRODCLASS 		0	/* DataFlex product class */
#define SEQ_XLATE		1	/* add CR to all LF's */

/* ms-dos fast character routines */
#define fillchar setmem
#define moveleft movmem
#define moveright movmem
#define mov32 movmem
extern byte		funnymode,samepath;



#ifdef COMMENTS_ABOUT_DEFINES
		--- Operating systems ---
#define CONVERGENT		1	/* for CT machine ls command */
#define CPM			1	/* for CPM specific code */
#define LIFENET			1	/* for LIFENET DBE */
#define MSDOS			1	/* for dos */
#define NOVELL461		1	/* for NOVELL version 4.61 */
#define UNIX 			1	/* for all UNIX */
#define VAX			1	/* for VAX (use VMS) */
#define VMS			1	/* for VAX/VMS */
#define XENIX			1	/* for XENIX (use XENIX3 or 5) */
#define XENIX3			1	/* for XENIX 1.0 (III) */
#define XENIX5			1	/* for XENIX 2.0 (V) */

		--- Compiler bugs ---
#define BIG_STRUCT_BUG		1	/* arrays of pointers to structs */
#define BUG_CASTING 		1	/* complex lhs casting expressions */
#define BUG_BITFIELD		1	/* pointers to arrays of structs */
#define NEG_POW_BUG		1	/* negative power expressions */
#define REAL_BUG		1	/* lhs double pointer expressions */
#define TO_BIG_BUGxx		1	/* ???? */

		--- Memory ---
#define DYNAMIC_MAX		1	/* CPM max memory check method */
#define MEMORY_DEBUG		1	/* check all disposes for validity */
#define MPM_DYNAMIC		1	/* MPM max memory check method */


		--- I/O configuration ---
#define ASYNC			1	/* VMS async block i/o */
#define AUTOSEARCH		1	/* does OS automatically search? */
#define BLOCKSEQ		1	/* use blk_io for seq_io? */
#define SEQ_XLATE		1	/* add CR to all LF's */
#define TRANSLATE		1	/* translate PC line set to chrs */

		--- Compiler specific ---
#define ANSII			1	/* ANSII compatible compiler */
#define LATTICE			1	/* LATTICE 'C' compiler */
#define __TURBOC__		1	/* TURBO 'C' compiler */


		--- DataFlex configuration ---
#define BETA			1	/* show beta sign on message */
#define DEBUG			1	/* generic debug flag */
#define DEBUG1			1	/* yet another debug flag */
#define EXP_INIT		1	/* initialize expression stackp */
#define INTEGER_ROUND		1.xxx	/* round double to int conversions */
#define LRTONLY			1	/* LRT flag */
#define NOISY			1	/* ????? */
#define NO_AUTO_LEAP		1	/* fix localtime leap bug */
#define NO_CVT			1	/* CVFS to use sprintf */
#define NO_DATE_WRAP		1	/* days in next month dont wrap */
#define NO_MATHERR		1	/* no matherr() provided */
#define NO_USER_COUNT		1	/* dont count users */
#define OPEN_ON_EXEC		1	/* can files stay open on exec? */
#define PRODCLASS 		1-N	/* DataFlex product class */
#define SHOWIT			1	/* ?????? */
#define SLOW_CHARACTER		1	/* use unoptimized string funcs */
#define TRACE_ENQ		1	/* VMS debug */
#define USE_GCVT		7	/* use gcvt instead of sprintf */
#define VIEW_BUFFERS		1	/* VMS i/o debug */
#endif

