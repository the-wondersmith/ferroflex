/************************************************************************
Confidential Trade Secret.
Copyright (c) 1986 Data Access Corporation, Miami Florida,
as an unpublished work.  All rights reserved.
DataFlex is a registered trademark of Data Access Corporation.
************************************************************************/

/* 6/09/83 11260    1	*/
/* COMMAND.DEF */
/******************* FLEX 2.00 COMMAND SPEC ******************/

/*CLASS LEGOND:   C = CONSTANT		  TYPE LEGOND:	  S = STRING
 *		  F = FILE/FIELD			  N = NUMERIC
 *		  W = WINDOW				  I = INTEGER
 *		  V = VARIABLE				  D = DATE 
 *		  A = ARG DATA SPACE			  E = EXPRESSION
 *		  Y = SYSNUM				  L = LABEL
 *		  X = FILE NUMBER ONLY
 *
 *COMMAND	 NUMBER 	# ARG	ARG1		ARG2		*/

/*** ENTRY SUPER GROUP ***/

#define cmdentry 0x8000       /*   2	FSND		WSND		 */

/*** INDICATE SUPER GROUP ***/

#define cmdinlts 0x4000       /*   2	CVFWSNDE	CVFWSNDE	 */
#define cmdinltn 0x4001       /*   2	CVFWYSNDE	CVFWYSNDE	 */
#define cmdinlti 0x4002
#define cmdinltr 0x4003

#define cmdinles 0x4004       /*   2	CVFWSNDE	CVFWSNDE	 */
#define cmdinlen 0x4005       /*   2	CVFWYSNDE	CVFWYSNDE	 */
#define cmdinlei 0x4006
#define cmdinler 0x4007

#define cmdineqs 0x4008       /*   2	CVFWSNDE	CVFWSNDE	 */
#define cmdineqn 0x4009       /*   2	CVFWYSNDE	CVFWYSNDE	 */
#define cmdineqi 0x400a
#define cmdineqr 0x400b

#define cmdinges 0x400c       /*   2	CVFWSNDE	CVFWSNDE	 */
#define cmdingen 0x400d       /*   2	CVFWYSNDE	CVFWYSNDE	 */
#define cmdingei 0x400e
#define cmdinger 0x400f

#define cmdingts 0x4010       /*   2	CVFWSNDE	CVFWSNDE	 */
#define cmdingtn 0x4011       /*   2	CVFWYSNDE	CVFWYSNDE	 */
#define cmdingti 0x4012
#define cmdingtr 0x4013

#define cmdinnes 0x4014       /*   2	CVFWSNDE	CVFWSNDE	 */
#define cmdinnen 0x4015       /*   2	CVFWYSNDE	CVFWYSNDE	 */
#define cmdinnei 0x4016
#define cmdinner 0x4017

#define cmdinbool 0x4020      /*   2	B		B		 */
#define cmdinstat 0x4029      /*   1	FSND				 */
#define cmdininstr 0x4030
#define cmdinmatch 0x4031

/*** CONTROL GROUP (#1) ***/

#define cmdabort 0x040        /*   0					 */
#define cmdgoto 0x041         /*   1	CL				 */
#define cmdgosub 0x042        /*   1	CL				 */
#define cmdreturn 0x043       /*   0					 */
#define cmddebug 0x044        /*   0					 */
#define cmdchain 0x045
#define cmdkeycheck 0x046
#define cmderror 0x047
#define cmderrclear 0x048
#define cmdchain_w 0x049
#define cmdoptset  0x04a
#define cmdoptclr  0x04b
/*** MOVE GROUP (#2) ***/

#define cmdmoveasc 0x080      /*   2	CVFWSNDE	VFWSNDE 	 */
#define cmdmovenum 0x081      /*   2	CVFWYSNDE	VFWYSNDE	 */
#define cmdmoveint 0x082      /*   2	CVFWYIS 	VFWYIS		 */
#define cmdincr 0x083
#define cmdmovedate 0x084
#define cmdmovereal 0x085
#define cmdmove     0x086     /* move anything */
/*** FILE GROUP (#3) ***/

#define cmdfind 0x0c0         /*   2	FSND		CI		 */
#define cmdclear 0x0c1        /*   1	FXSND				 */
#define cmdsave 0x0c2         /*   1	FXSND				 */
#define cmddelete 0x0c3       /*   1	FXSND				 */
#define cmdrelate 0x0c4       /*   1	FXSND				 */
#define cmdreverse 0x0c5      /*   1	FXSND				 */

/*** SEQ I/O GROUP (#4) ***/

#define cmdsqiopen 0x100      /*   1	CVFWS				 */
#define cmdsqoopen 0x101      /*   1	CVFWS				 */
#define cmdsqiclose 0x102     /*   0					 */
#define cmdsqoclose 0x103     /*   0					 */
#define cmdseqrline 0x104     /*   1	VFWYSNDE			 */
#define cmdseqwline 0x105     /*   1	CVFWYSNDE			 */
#define cmdseqwstr 0x106      /*   1	CVFWYSNDE			 */
#define cmdseqeol 0x107       /*   0					 */

/*** CONSOLE I/O GROUP (#5) ***/

#define cmdconrline 0x140     /*   1	VFWYSNDE	CVFWIE		 */
#define cmdconwline 0x141     /*   1	CVFWYSNDE			 */
#define cmdconwstr 0x142      /*   1	CVFWYSNDE			 */
#define cmdconeol 0x143       /*   0					 */
#define cmdconcls 0x144       /*   0					 */
#define cmdcongotoxy 0x145    /*   2	CVFWYIE        CVFWYIE		 */
#define cmdconcxy 0x146       /*   2	CVFWYIE        CVFWYIE		 */
#define cmdconkey 0x147

/*** FLEX FILE OPEN/CLOSE GROUP ***/

#define cmdopen 0x180         /*   2    CVI		CVI		 */
#define cmdclose 0x181        /*   1	CVI				 */

/*** FORMS GROUP (#7) */

#define cmdpage 0x1c0         /*   1	CVI				*/
#define cmdformin 0x1c1       /*   1	CVI				*/
#define cmdformd 0x1c2        /*   1	CVI				*/
#define cmdformclr 0x1c3

/*** PRINT GROUP (#8) */

#define cmdprtopen 0x200      /*   1	CVS				*/
#define cmdprtclose 0x201     /*   0					*/
#define cmdprtpage 0x202      /*   1					*/
#define cmdpagechk 0x203
#define cmdbreak 0x204

/*** STRING GROUP (#9) */

#define cmdstrpad 0x240
#define cmdstrleft 0x241
#define cmdstrright 0x242
#define cmdstrappend 0x243
#define cmdstrpos 0x244
#define cmdstrmid 0x245
#define cmdstrlnth 0x246
#define cmdcmdline 0x247

/*** ENTER GROUP (10) */

#define cmdeupdate 0x280
#define cmdefind 0x281
#define cmdesfind 0x282
#define cmdedisp 0x283
#define cmdeclear 0x284
#define cmdebtrue 0x285
#define cmdebfalse 0x286
#define cmderclr 0x287
#define cmdeback 0x288
                      
