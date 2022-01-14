/************************************************************************
Confidential Trade Secret.
Copyright (c) 1986 Data Access Corporation, Miami Florida,
as an unpublished work.  All rights reserved.
DataFlex is a registered trademark of Data Access Corporation.
************************************************************************/

#define BLOCKSIZE 512
#define PHYSBLOCKS 4
#define NUMFILES 255
#define ISNUM 10	/* MAX NUMBER OF INDEXES */
#define ISSEGS 6
#define MAXFLD 255
#define ZERO 0L
#define ONE 1L

#define LT 0
#define LE 1
#define EQ 2
#define GE 3
#define GT 4
#define CREATE 0
#define DELETEIT 1
#define FINDIT 2
#define FINDANDRETURN 3
#define SEQFIND 4
#define ASCII 0
#define BCD 1
#define DATE 2
#define OVERLAP 3

typedef char DATAHDR[2216];
typedef char FILEDESCR[43];

struct rdscr {		/* RECORD/FILE DESCR */
    longint flend;	/* FILE END  */
    longint frel;	/* FREE LIST */
    longint numrec;	/* NUMBER OF RECORDS */
    longint maxrec;	/* MAX NUMBER OF RECORDS */
    longint recnum;	/* CURRENT RECORD NUMBER */
	DATAHDR flexhdr;
}  ;

struct flist {			/* FILE LIST */
    char    pathname[41];	/* XENIX/UNIX NAME  */
    char    username[33];	/* NAME DISPLAYED TO USER */
    char    altname[11];	/* NAME USED FOR .FD FILES */
	FILEDESCR filedef;
}  ;

extern byte	cnchk, cnovr, cnfile, cnfield;
extern int	ofst;
extern byte     pts, key, mode;
extern byte 	lenth, relfil, relfld, pvlg;
extern int 	speed, ostype, osrev, osmulti;
extern struct 	rdscr *ddr[];
extern char	filelist[];
extern struct rdscr *cfile;
/*extern struct fdscr *cfld;*/
extern char	errfcb[];
extern char	done[];
