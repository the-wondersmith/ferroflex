/************************************************************************
Confidential Trade Secret.
Copyright (c) 1986 Data Access Corporation, Miami Florida,
as an unpublished work.  All rights reserved.
DataFlex is a registered trademark of Data Access Corporation.
************************************************************************/

/* 3/30/83 11250    1   FLEX2.DEF */

#define ascii 0
#define bcd 1
#define date 2
#define overlay 3

	/***************************************************************
	ARGUMENT STATUS BYTE
	BITS 0..3	= "ARGCLASS" ARGUMENT TYPE ( WHERE TO GET/PUT IT )
	BITS 4..6	= "ARGTYPE" CONVERTION CONTROL
	BIT     7	= INDEXING FLAG
	***************************************************************/
#ifdef BUG_BITFIELD
struct argument {
	union {
		struct {
/*** ARGUMENT INDEX - INDEX INTO ARGSPACE (HEAP) OR WINDOW OR SYS INDEX ***/
			unsigned	pargindex: 16;
			unsigned char	pargstat : 8;
			unsigned char	extra_pad: 8;
		} str2;
		struct {
/*** ARGUMENT FILE/FIELD USED TO GET FILE/FIELD NUMBER	***/
			unsigned char	pargfile : 8;
			unsigned char	pargfield: 8;
			unsigned char	pargstat : 8;
			unsigned char	extra_pad: 8;
		} str1;
	} variant;
};
#else
struct argument {
	union {
		struct {
/*** ARGUMENT INDEX - INDEX INTO ARGSPACE (HEAP) OR WINDOW OR SYS INDEX ***/
#ifdef MSDOS
			int16   	pargindex;
			byte	 	pargstat;
			byte	 	extra_pad;
#else
			unsigned	pargindex: 16;
			unsigned 	pargstat : 8;
			unsigned 	extra_pad: 8;
#endif
		} str2;
		struct {
/*** ARGUMENT FILE/FIELD USED TO GET FILE/FIELD NUMBER	***/
#ifdef MSDOS
			byte     	pargfile;
			byte     	pargfield;
			byte     	pargstat;
			byte     	extra_pad;
#else
			unsigned 	pargfile : 8;
			unsigned 	pargfield: 8;
			unsigned 	pargstat : 8;
			unsigned 	extra_pad: 8;
#endif
		} str1;
	} variant;
};
#endif

struct commandtype {
    int16     cmdnumber;
	/*  COMMAND FORMAT:	( BITS 0-15 )
			BIT 15	ENTRY OPTION, OTHER BITS ARE OPTIONS
			BIT 14	'IF' OPTION, MAKES 
			BITS 13-6	COMMAND GROUP NUMBER
					OR INDICATOR NUMBER FOR IF'S
			BITS 5-0	COMMAND NUMBER WITHIN GROUP
	*/
    char    pargarray[6];
    byte    indct1;
    byte    indct2;
    byte    indct3;
    byte    cextra_pad;
};

struct windowinf {
#ifdef MSDOS
    unsigned bufindex;	/* BUFFER */
    unsigned auxindex;
#else
    unsigned int16  bufindex;	/* BUFFER */
    unsigned int16  auxindex;
#endif
    long     auxtype;
		/*	BITS 0..2
			0	NONE
			1	SUBTOTAL/TOTAL
			2	SET OF VALID RESP
			3	NUMERIC RANGE
			BIT 2	ENTRY REQUIRED
			3	UPPER CASE
			4	ZERO SUPPRESS
			5	','
			6	'$'
			7	'SIGN ON RT'
		*/
    byte    spare;
    byte    color;
    byte    page;
    byte    row;
    byte    colm;
    byte    lenth;
    byte    mode;	/* 255=ASCII, 0..4=NUMERIC, 128=DATE */
    byte    maskfill;	/* CHAR TO FILL WITH */
}  ;

struct pageinf {
#ifdef MSDOS
    unsigned 	pageblk;
    unsigned 	pagechrs;
    unsigned 	numlines;
    unsigned 	srtwindow;
    unsigned 	endwindow;
#else
    unsigned int16     pageblk;
    unsigned int16 pagechrs;
    unsigned int16     numlines;
    unsigned int16     srtwindow;
    unsigned int16     endwindow;
#endif
    byte    memres;
    byte    pgx;
    byte    pgy;
    byte    fgcolor;
    byte    bgcolor;
    byte    pextra_pad;
}  ;

/* THINGS WE NEED TO KNOW ABOUT A CONFIGURATION AT RUN TIME */
/* THANKS TO CARE AND FRENDLY ATN. OF THE MACRO 1 1/2 PASS COMPILER */
/* THIS IS IN BLOCK ZERO OF THE CONFIGURATION FILE */
/* APPROPRIATLY NAMED, AT THIS TIME NAME.FLX */

struct cfgblk0 {
    char      cfgname[76]; /* Name, date:time & reg name during compile */
    int32     creser;	/* CREATION SERIAL NUMBER */

	/* MOVE FOLLOWING TO SYSINT[20] */
    int32     creprot;	/* 20 CREATION PROTECTION */
    int32     userser;	/* 21 USER SERIAL NUMBER  */
    int32     userlevel;	/* 22 USER PROTECTION LEVEL */
    int32     crereg;	/* 23 CREATION REGISTRATION */
    int32     numcommand; /* 24 NUMBER OF COMMANDS */
    int32     numsysnum;	/* 25 NUMBER OF SYSTEM MUNBERS */
    int32     numsysstr;	/* 25 NUMBER OF SYSTEM STRINGS */
    int32     argsize;	/* 27 SIZE OF ARGUMENT AREA IN BYTES */
    int32     numpage;	/* 28 NUMBER OF PAGES */
    int32     numwindow;  /* 29 NUMBER OF WINDOWS */
    int32     debugflag;
    int32     onerror;
    int32     extra1[8];

    int16     blkcommand;	/* START BLOCK OF COMMANDS */
    int16     blkarg;	/* START BLOCK OF ARGUMENT AREA */
    int16     blkwindow;  /* START BLOCK OF WINDOW TABLE */
    int16     blkpage;	/* START OF PAGE TABLE */
    int16     extra2[4];

    int16     keyproc[22]; /* FLEX KEY PROCEDURES */
    char      fillstr[8] ;
    char      blankspace[24]; /* TO MAKE 256 */
    byte      sex_byte;
    char      xextra_pad1;
    char      yextra_pad2;
    char      zextra_pad3;
};

#define MAXCOMMANDS 5000
#define MAXWINDOWS (32767 / sizeof(struct windowinf))
extern int getargi();

