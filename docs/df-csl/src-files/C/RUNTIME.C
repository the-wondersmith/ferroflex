/************************************************************************
Confidential Trade Secret.
Copyright (c) 1986 Data Access Corporation, Miami Florida,
as an unpublished work.  All rights reserved.
DataFlex is a registered trademark of Data Access Corporation.
************************************************************************/

/* 8/12/83 11280    1	*/

#include <standard.h>
#include <flex2con.h>
#include <flex2def.h>
#include <flex2var.h> /* for type cross check */
#include <dmsdef.h>
#include <sfcbtype.h>
#include <screencd.h>
#include <command.h>

#ifdef XENIX3
struct commandtype cmd_area[MAXCOMMANDS+16];
char	arg_data_area[65530];
#endif

char    cfgfile[FCBSIZE] ;
char    *argspace;
int32     sysint[384] ;
struct commandtype *command;
struct windowinf *forminf;
char *formspace;
bool    indicators[129] ;
char    infile[SFCBSIZE] ;
char    outfile[SFCBSIZE] ;
struct pageinf *formpage;
char    flxkeys[31] ;
char    prtedit[8] ;
extern bool    err;
extern int     ern;
extern bool    errscrn;
extern int     errline;
extern char    errmsg[STRING] ;

int     lastline,
        nextline;
int16     crntline;
struct commandtype crntcommand;
int16     crntind[3] ;
int16     crntcmd;
int16     crntgroup;
struct argument crntag1;
struct argument crntag2;
bool    df_abort;
bool    debugmode;
int16     argstat;
int16     argclass;
int16     argtype;
#ifdef MSDOS
unsigned	argindex;
#else
unsigned int16     argindex;
#endif
int16     argfile;
int16     argfield;
char    valstr[LSTRING] ;
int     vallen;
char    valnum[NUMSIZE];
double  valreal;
int32     valint;
char    altstr[LSTRING] ;
int     altlen;
char    altnum[NUMSIZE];
int32     altint;
double    altreal;
int16     crntwnum;
struct windowinf crntwinf;
int16     crntwlen;
int16     crntwmode;
int16    crntwmask;
char    *crntwpos;
int     term;
int ofstx, ofsty;
int coloron;
int dfltcolor;
int crntbg;

int16     crntpage;
struct pageinf crntpinf;
int16     fkeynum;
int16     lastfkey;
char    *breakptr[21] ;
int16     entfile,
        entfield;
bool    entbottem;
int noerror;
int32     itemp,
        itemp2;
char    ctemp;
char    cmdbuf[STRING];
extern int leave_open;

main(argc,argv)
int  argc;
char *argv[];
{
    _cmd(argc,argv);
    setup();
    runtime();
    setcolor(scexit);
    freemem();
    echo_term();
#ifdef MSDOS
    if (sysint[127] == -1)
	    exit(0xdf);
    else
	    exit(0);
#endif
}


flexkey ()
{
int     itemp,
	keyon,
        it2;
/* Note: MAX fkeynum set at CKUSER2 due to size limit
in SYSINT and indicators.   3/27/87 */
  
    lastfkey = fkeynum;
    sysint[entline] = crntline;
    sysint[systerm] = term;
    if (fkeynum<=CKUSER2) indicators[fkeynum + 100] = false;
    if (term & 0x0100) fkeynum = term & 0xff;
    else fkeynum = 0;
    sysint[syskey] = fkeynum;
    if (fkeynum && (fkeynum<=CKUSER2) ) {
    	indicators[fkeynum + 100] = true;
	keyon = sysint[nokeyproc] == 0;
    	itemp = sysint[KEYPROC + fkeynum];
    	if (itemp  && keyon) {
       	  dogosub(itemp);        /* INTERUPT GOSUB TO KEY PROCEDURE */
	  sysint[nokeyproc] = sysint[sysreturn];
	  }
	it2 = sysint[KEYPROC];
	if (it2 && keyon) {
	  dogosub(it2);
	  if (! itemp) sysint[nokeyproc] = sysint[sysreturn];
	  }
	}
}


runtime ()
{

    fkeynum = 0;
    do {
        initcfg();
        if ( ! err)
            execute();
        endcfg();
    } while ((sysint[127] !=  - 1) || (crntcmd == cmdchain));
}
