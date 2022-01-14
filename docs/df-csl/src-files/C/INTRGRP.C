/************************************************************************
Confidential Trade Secret.
Copyright (c) 1986 Data Access Corporation, Miami Florida,
as an unpublished work.  All rights reserved.
DataFlex is a registered trademark of Data Access Corporation.
************************************************************************/

#include <standard.h>
#include <flex2con.h>
#include <flex2def.h>
#include <flex2var.h>
#include <screencd.h>
#include <fcntl.h>
#include <blkio.h>
#include <bits.h>
#include <errno.h>

#ifdef CPM
#include <dos.h>
#endif

#ifdef MSDOS
#define FP_OFF(fp)	((unsigned)(fp))
#define FP_SEG(fp)	((unsigned)((unsigned long)(fp) >> 16))
struct REGS
	{
	unsigned int	ax, bx, cx, dx, si, di, cflag, flags;
	};
struct	SREGS	{
	unsigned int	es;
	unsigned int	cs;
	unsigned int	ss;
	unsigned int	ds;
	};
#include <process.h>
#define COPY      "copy %s %s"
#define RENAME    "rename %s %s"
#define DIRECTORY "dir %s %cw"
#define ERASE     "del %s >\\dev\\nul"
#endif

#ifdef UNIX
#define COPY      "cp %s %s"
#define RENAME    "mv %s %s"
#if XENIX3 | XENIX5
#define DIRECTORY "lc %s"
#else
#ifdef CONVERGENT
#define DIRECTORY "ls -F %s"
#else
#define DIRECTORY "ls -CF %s"
#endif
#endif
#define ERASE     "rm %s >/dev/null 2>&1"
#endif

#ifdef VMS
#define COPY      "copy %s %s"
#define RENAME    "rename %s %s"
#define DIRECTORY "dir %s"
#endif

/***************************************************************/
/********** THIS IS AN OPERATING SYSTEM DEPENDANT MODULE *******/
/***************************************************************/

#define cmdrunprog 0x2c0
#define cmderase 0x2c1
#define cmdrename 0x2c2
#define cmdreset 0x2c3
#define cmddirectory 0x2c4
#define cmdcopyfile 0x2c5
#define cmddespool 0x2c6
#define cmdsystem 0x2c7
#define cmdos 0x2c8
#define cmdgdate 0x2c9
#define cmdmemavail 0x2ca
#define cmdxrun 0x2cb
#define cmdrunasync 0x2cc
#define cmdrunsync 0x2cd
#define cmdsleep 0x2ce
#define cmdgetenv 0x2cf

static int 	suppress_error = 1;
extern int	nest_level;
extern int	leave_open;
extern char    *cpmbufp;
extern char    filelist[] ;
extern bool    sysabort;
extern byte    lockcount;

#ifdef UNIX
extern char **environ; /* VMS has another type spec for this */
#endif

struct envlist {
        char            *name;
        struct envlist  *next;
};

extern struct	envlist *blkname();
extern char 	*getenv();
extern long	tell();

#ifdef MSDOS
char	*switchr()
{
struct REGS inregs;
	inregs.ax = 0x3700;
	intdos(&inregs,&inregs);
	sprintf(errmsg,"%cC",inregs.dx & 0xff);
	return(errmsg);
}
#endif

runasync( cmd )
char	*cmd;
{
char	*shell,*cmdptr;
int 	pid,r,result;
#ifdef NOVELL461
#define OPEN_ON_EXEC 1
long	inptr,outptr;
	inptr = is_open(infile)?tell(setpath(infile)):-1;
	outptr = is_open(outfile)?tell(setpath(outfile)):-1;
	close_all(TRUE);	/* close them ALL */
#endif
#ifndef OPEN_ON_EXEC
	close_all(FALSE);	/* close all but noclose files */
#endif
	if (suppress_error)
		redir_err( TRUE );
#ifdef MSDOS
	shell = getenv("COMSPEC");
	if (strcmp(cmd," !") >0) {
		result = spawnlp(P_WAIT,shell,shell,switchr(),cmd,NULL);
	}
	else
		result = spawnlp(P_WAIT,shell,shell,NULL);
#ifdef __TURBOC__
	_fpreset();
#endif
#else
#ifdef UNIX 
	if (strcmp(cmd," !") < 0)
		strcpy(cmd,"sh");	/* no arguments */
/*	strcpy(cmd,nstrcat(NULL,"exec ",cmd,NULL)); */
	if ((pid=fork())==0)
	    execle("/bin/sh","sh","-c",cmd,(char *)0,environ);
        else
	    while( (r = wait(&result) != pid) && (r != -1));
#else
#ifdef CPM
	    if (sc.sysspecial == PROD_TURBO) {
		cmdptr = strpbrk(" ",cmd);
		*(cmdptr++) = 0;
		sprintf( errmsg, "%s.cmd", cmd );
		result = spawnl( errmsg, cmdptr, cmdptr, (char *) 0 );
		}
	    else system(cmd);
#else
	    system(cmd);
#endif
#endif
#endif
	sysint[strmark] = result;
	redir_err( FALSE );
	suppress_error = 1;
#ifdef NOVELL461
	if (inptr != -1) {
		sqfopen(infile,infile,&result);
		lseek( setpath(infile), inptr, 0 );
		}
	if (outptr != -1) {
		blkfile(outfile,outfile,&result, O_RDWR | O_TEXT);
		lseek( setpath(outfile), outptr, 0 );
		}
#endif
}

firstpath( name )
char	name[];
{
struct	envlist	*env;

	env = blkname( name, name );
	strcpy(name,nstrcat(NULL,env->name,name,NULL));
	lowercase(name);
}

redir_err( mode )
int	mode;
{
#ifdef MSDOS
	close(2);
	if (mode)
		open( "NUL", O_RDWR, 0666 );
	else
		dup2( 1,2 );
#endif
}

get2str()
{
	getargc( &crntag1, argstr );
	strcpy(altstr,valstr);
	getargc( &crntag2, argstr );
}

cmdintrgroup ()
{
char    anfcb[FCBSIZE] ;
int     result,wascolor;

    if ((crntcmd==cmdrunprog) && (nest_level>0)) {
	    crntcmd=cmdrunasync;
	    df_abort = TRUE;
	}
    switch (crntcmd) {
        case cmdrunprog:
	case cmdxrun:
	    get2str();
	    lowercase(altstr);
	    sprintf(p2c_tstr,"%s %s",altstr,valstr);
	    setcolor( scexit );
	    endcfg();
#ifdef CPM
		dfbye();
		if (strcmp(altstr," !") >0)
			execl(altstr,valstr,NULL);
		exit(0);
#endif
#ifdef MSDOS
	    { char *shell;
		dfbye();
		shell = getenv("COMSPEC");
		if (strcmp(p2c_tstr," !") >0)
			execl(shell,shell,switchr(),p2c_tstr,NULL);
		exit(0);
	    }
#else
	    runasync( p2c_tstr );
	    setcolor( scinit );
	    df_abort = true;
#endif
            break;
        case cmdrunsync:
	case cmdrunasync:
	    suppress_error = 0;
	    wascolor = crntcolor;
	    setcolor(scexit);
	    get2str();
	    lowercase(altstr);
	    sprintf(p2c_tstr,"%s %s",altstr,valstr);
#ifdef MSDOS
	    getcwd( valstr, 250 );
    	    result = getdisk();
#endif
	    if (crntcmd == cmdrunasync)
		runasync( p2c_tstr );
	    else {     /* let shell do redirection of stdin/out */
#ifdef UNIX
		sprintf( valstr, "%s &",p2c_tstr );
#else
#ifdef VAX
		sprintf( valstr, "spawn/nowait/nonotify %s", p2c_tstr );
#else
		strcpy( valstr, p2c_tstr );
#endif
#endif
		runasync( valstr );
		}
#ifdef MSDOS
	    chdir( valstr );
	    setdisk( result );
#endif
	    setcolor(scinit); 
	    setcolor( wascolor );
	    dmrefresh();
	    multiuser = ck_bit(bit_multiuser);
            break;
        case cmderase:
            getargc(&crntag1,argstr);
#ifdef VMS
	/* note, someone has to deside! 
	    if (!strchr(valstr,';')) strcat(valstr,";0"); */
		firstpath( valstr );
		blkdelete( anfcb,valstr,&result );
#else
#ifdef CPM
	    if (valstr[0]) {
		firstpath( valstr );
		erasefile( valstr );
		}
#else
	    if (valstr[0]) {
		firstpath( valstr );
		sprintf(p2c_tstr,ERASE,valstr);
		runasync(p2c_tstr);
	    }
#endif
#endif
            break;
        case cmdrename:
	    get2str();
	    firstpath( altstr );
#ifdef CPM
	    rename( altstr, valstr );
#else
#ifndef MSDOS
	    firstpath( valstr );
#endif
	    sprintf(p2c_tstr,RENAME,altstr,valstr);
            if ((altstr[0]) && (valstr[0]))
		runasync(p2c_tstr);
#endif
	    break;
        case cmdsystem:
            sysint[127] =  - 1;
 	    df_abort = true;
	    if (!nest_level)
		    leave_open = FALSE;
#ifdef CPM
	    /* ABORT PARENT PROCESS FLEX.CMD */
	    kill_parent();
#endif
            break;
        case cmdcopyfile:
	    get2str();
	    firstpath( altstr );
	    firstpath( valstr );
#ifdef CPM
	    copyfile( altstr, valstr, crntpage == 0 );
#else
#ifdef MSDOS
	    if (crntpage != 0)
		strcat(valstr," >\\dev\\nul");
#endif
            sprintf(p2c_tstr,COPY,altstr,valstr);
            if ((altstr[0]) && (valstr[0]))
		runasync(p2c_tstr);
#ifdef MSDOS
	    if (crntpage == 0)
		gotoxy(sc.klength,0);
#endif
#endif
            break;
        case cmddirectory:
            sysint[cpage] = crntpage = 0;
            getargc(&crntag1,argstr);
	    firstpath( valstr );
#ifdef CPM
	    directory( valstr );
#else
#ifdef MSDOS
	    switchr();
	    sprintf(p2c_tstr,DIRECTORY,valstr,errmsg[0]);
	    runasync(p2c_tstr);
#else
	    sprintf(p2c_tstr,DIRECTORY,valstr);
	    runasync(p2c_tstr);
#endif
#endif
#ifdef MSDOS
	    gotoxy(sc.klength,0);
#endif
	    break;
        case cmdreset:
            break;
        case cmddespool:
            seqclose( outfile );
	    seqopen( outfile, "LST:", true );
            break;
        case cmdos:            /* O/S CALL */
#ifdef CPM
	    {	    
		union REGS inregs,outregs;
		struct SREGS segregs;
		getargc( &crntag1, argint );
		result = (valint>>16);
		if (!result) result = 0xe0;
		inregs.cx = valint;
		inregs.ax = (valint << 8);
		getargc( &crntag2, argint );
		segregs.ds = (valint>>16);
		inregs.dx = (valint & 0xffff);
		int86x(result,&inregs,&outregs,&segregs);
		sysint[strmark] = outregs.ax;
		}
#endif
#ifdef MSDOS
	    {	    
		struct REGS inregs,outregs;
		struct SREGS segregs;
		getargc( &crntag1, argint );
		inregs.ax = (valint << 8);
		getargc( &crntag2, argint );
		segregs.ds = (valint>>16);
		inregs.dx = (valint & 0xffff);
		intdosx(&inregs,&outregs,&segregs);
		sysint[strmark] = outregs.ax;
		}
#endif
            break;
        case cmdgdate:
	    get_date_time( valnum, &sysint[STRLEN], &sysint[strmark],
			   &sysint[37], &sysint[38] );
	    putargc(&crntag1,argdate);
            break;
        case cmdmemavail:
            sysint[heapsize] = memavail();
            valint = bigmem();
            putargc(&crntag1,argint);
            break;
	case cmdsleep:
	    sleep( (unsigned) getargi( &crntag1 ) );
	    break;
	case cmdgetenv:
	    { char *envp;
    		getargc( &crntag1, argstr );
		allcaps(valstr);
		if (envp = getenv(valstr))
			strcpy( valstr, envp );
		else
			valstr[0] = 0;
		putargstr(&crntag2);
		}
	    break;
    }    /* CASE */

}

