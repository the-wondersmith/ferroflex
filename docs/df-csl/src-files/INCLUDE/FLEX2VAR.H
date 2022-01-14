/************************************************************************
Confidential Trade Secret.
Copyright (c) 1986 Data Access Corporation, Miami Florida,
as an unpublished work.  All rights reserved.
DataFlex is a registered trademark of Data Access Corporation.
************************************************************************/

extern char    cfgfile[] ;
extern char    *argspace;	/* POINTER INTO OUR STATIC DATA AREA */

	/* CONFIGURATION DATA */
extern bool    ok,more;

extern int32   sysint[384] ;	/* SYSTEM INTEGERS */
extern struct commandtype *command;
extern struct windowinf *forminf;
extern char *formspace;
extern bool    indicators[] ;
extern char    infile[] ;
extern char    outfile[] ;
extern struct pageinf *formpage;    /* ARRAY[1..25] OF PAGEINF;  PAGE DATA */
extern char    flxkeys[] ;
extern char    prtedit[] ;
extern int lastline, nextline;
extern int16   crntline;		/* CURRENT COMMAND LINE */
extern struct commandtype crntcommand;
extern int16   crntind[] ;
extern int16   crntcmd;
extern int16   crntgroup;
extern struct argument crntag1;
extern struct argument crntag2;
extern bool    df_abort, debugmode;
extern int noerror;
extern int16 entfile, entfield;
extern bool entbottem;

	/* UNPACKED ARGUMENTS */

extern int16   argstat;
extern int16   argclass;
extern int16   argtype;
#ifdef MSDOS
extern unsigned argindex;
#else
extern unsigned int16   argindex;
#endif
extern int16   argfile;
extern int16   argfield;
extern char    valstr[256] ;
extern int     vallen;
extern char    valnum[NUMSIZE];
extern int32   valint;
extern double valreal;
extern char    altstr[] ;
extern int     altlen;
extern char    altnum[];
extern int32   altint;
extern double altreal;

/* FORMS */

extern int16   crntwnum;
extern struct windowinf crntwinf;
extern int16   crntwlen;
extern int16   crntwmode;
extern int16   crntwmask;
extern char    *crntwpos;
extern int     term;
extern int16   crntpage;
extern struct pageinf crntpinf;
extern int16 fkeynum, lastfkey;
extern int ofstx, ofsty;
extern int coloron;
extern int dfltcolor;

/* TEMP VAR */
extern int32   itemp,
               itemp2;
extern char    ctemp;
extern char    cmdbuf[] ;

