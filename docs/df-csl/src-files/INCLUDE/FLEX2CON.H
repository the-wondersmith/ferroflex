/************************************************************************
Confidential Trade Secret.
Copyright (c) 1986 Data Access Corporation, Miami Florida,
as an unpublished work.  All rights reserved.
DataFlex is a registered trademark of Data Access Corporation.
************************************************************************/

/***** numeric constants *******/
#define numlt 1
#define numeq 2
#define numgt 3
/******  ARGUMENT CLASSES  **********/

#define argnothing 0
#define argfldstr 1
#define argfldnum 2
#define argwndstr 3
#define argwndnum 4
#define argdatstr 5
#define argdatnum 6
#define argconint 7
#define argsysint 8
#define argsysdate 9
#define argsystime 10
#define argdatreal 11
#define argexp 12
#define argwndrl 13
/*********************************/

/******* ARGUMENT TYPES **********/

#define argstr 0
#define argnum 1
#define argdate 2
#define argint 3
#define argexpr 4	/* INTERNAL ONLY */
#define argreal 5	/* cbc-9/2/87 */
#define arguntype 8

/************************************/

/*** ASSIGNED SYSTEM ARRAY (SYSINT) VALUES ***/

#define syscmd 0
#define sysfldindex 1	/* FIELD INDEX */
#define syswndindex 2	/* WINDOW INDEX */
#define systerm 3	/* LAST TERMINATION CHAR */
#define syskey 4	/* LAST TERMINATION CHAR POSITION IN FLEX CODES */
#define syserrnum 5	/* LAST ERROR NUMBER */
#define syserrline 6	/* LAST ERROR LINE */
#define cwindow 7	/* CURRENT WINDOW  */
#define entmfile 8	/* ENTRY COMMAND MODE */
#define cpage 9		/* CURRENT PAGE */
#define sysline 10	/* LINE COUNTER */
#define syspage 11	/* PAGE COUNTER */
#define strmark 12	/* STRING POSITION MARK */
#define STRLEN 13	/* STRING MID LENGTH */
#define entstart 14	/* START OF ENTRY PROC */
#define entrfile 15	/* LAST ENTERED FILE */
#define entrfield 16	/* LAST ENTERED FIELD */
#define screenend 17	/* LAST LINE ON SCREEN */
#define heapsize 18	/* HEAP MEMORY AVAILABLE */
#define entline 19	/* ENTRY LINE NUMBER */
#define sysdata 20	/* 20..39   DATA FROM BLOCK 0 */
#define ONERROR 31	/* ON ERROR ADDRESS */
#define errortemp 32    /* on error temp */
#define nokeyproc 33    /* keyproc off when != 0 */
#define LASTCOLOR 34    /* last screenmode */
/*      savecolor 35    place to save lastcolor at runtime */
#define KEYPROC 40	/* 40..59   FLEX KEY PROCEDURES */
#define sysreturn 60	/* GOSUB RETURN ADDRESS STACK POINTER */

	/*  SYSSTACK	= 61..79   SYSTEM RETURN STACK */

	/* 80..127  AVIALABLE FOR internal CONFIGURATION USE */
#define nokeycheck 80
#define chain_level 81
#define CURSET 82
#define CURRETURN 83
#define WRAPCNT 84
#define NEXTMENU 127
#define userspace 128
/*********************************************/
#define lt 0
#define le 1
#define eq 2
#define ge 3
#define gt 4
#define ne 5
/**** PRE ASSIGNED INDICATORS ***/

#define errflag 127
#define errfile 126
#define errseq 125
#define pagebreak 124
#define livekey 123
#define endfor 122
#define seqeol 121
#define multi 120
#define entquery 9
#define entcompat 10
/********************************/
