/************************************************************************
Confidential Trade Secret.
Copyright (c) 1986 Data Access Corporation, Miami Florida,
as an unpublished work.  All rights reserved.
DataFlex is a registered trademark of Data Access Corporation.
************************************************************************/

#include <standard.h>
#include <flex2con.h>
#include <command.h>
#include <flex2def.h>
#include <flex2var.h>

/* 2/7/84  - GOSUB OUT OF RANGE */

#define grpctrl 1
#define grpmove 2
#define grpfile 3
#define grpsio 4
#define grpcon 5
#define grpopen 6
#define grpform 7
#define grpprint 8
#define grpstr 9
#define grpenter 10
#define grpintr 11
#define grpfinf 12
#define grpfact 13
#define grpgraf 14
#define grpcom 15
#define grpuser 16

bool     ok,
        more;

extern bool    debugmode;
extern int     nextline,
               lastline;
extern int     dmisam,
               seqisam;
extern bool    entbottem;
extern bool    errscrn;
extern int	leave_open;
extern int	nest_level;
extern long	worldoptions;

getline ()
{
    crntline = nextline;
    errline = crntline;
    df_abort = df_abort || (crntline > lastline) || (crntline < 0);
    nextline = crntline + 1;
    p2crasgn(&crntcommand,sizeof(crntcommand),&command[crntline]);
}


getcommand ()
{
struct commandtype *withp0;

    withp0 = &crntcommand;
    crntcmd = withp0->cmdnumber;

    move(&(withp0->pargarray[0]), &crntag1, 3);
    move(&(withp0->pargarray[3]), &crntag2, 3);
}


indcthelp (ind)
int     ind;
{
    more = ind;
    if (more) {
        ok = indicators[ind & 127];
        if (ind & 0x80)
            ok =  ! ok;
    }
}


dogosub (adr)
int     adr;
{
int     itemp;

    itemp = sysint[sysreturn] + 1;
    sysint[sysreturn + itemp] = nextline;
    sysint[sysreturn] = itemp;
    sysint[syscmd] = crntline;
    nextline = adr;
    if (itemp >= 19)
        error(97);

    /* TO MANY GOSUBS */
}


/***************** MOVE GROUP *********************/

getput (argtemp)
int     argtemp;
{
    getargc(&crntag1,argtemp);
    putargc(&crntag2,argtemp);
}


cmdmovegroup ()
{

/* MOVE GROUP */
int     argtemp;

    switch (crntcmd) {
        case cmdmoveasc:
            getput(argstr);
            break;
        case cmdmovenum:
            getput(argnum);
            break;
        case cmdmoveint:
            getput(argint);
            break;
        case cmdmovedate:
            getput(argdate);
            break;
	case cmdmovereal:
	    getput(argreal);
	    break;
	case cmdmove:
	    getargnc(&crntag1);
	    putargc(&crntag2,argtype);
	    break;
        case cmdincr:
            valint = crntag1.variant.str2.pargindex;
            altint = sysint[valint] + 1;
            sysint[valint] = altint;
            getargc(&crntag2,argint);
            indicators[endfor] = altint > valint;
            break;
    }        /*CASE */
}


/******************* CONTROL GROUP ***********************/

cmdctrlgroup ()
{
/* CONTROL GROUP */

    switch (crntcmd) {
        case cmdabort:
            df_abort = true;
    	    if (!nest_level)
		    leave_open = FALSE;
            break;
        case cmdgoto:
            nextline = getargi(&crntag1);
            break;
        case cmdgosub:
            dogosub(getargi(&crntag1));
            break;
        case cmdreturn:
            itemp = sysint[sysreturn];
	    if (itemp == sysint[nokeyproc]) sysint[nokeyproc] = 0;
	    if (itemp == noerror) noerror = 0;
            nextline = sysint[sysreturn + itemp--];
            sysint[sysreturn] = itemp;

            /* RETURN TO LABEL, MUST BE CI */

            itemp = crntag1.variant.str2.pargindex;
            if (itemp)
                nextline = itemp;
            break;
        case cmddebug:
            debugmode =  ! debugmode;
            break;
        case cmdchain:
            getargc(&crntag1,argstr);
            strcpy(cmdbuf,valstr);
/**
			   if (sysint[NEXTMENU] < 0)
                sysint[NEXTMENU] = 0;
**/
	    leave_open = ((getargi(&crntag2)) | (leave_open));
            df_abort = true;
            break;
        case cmdkeycheck:
            indicators[livekey] = readcnd();   /* KEYPRESS */
            break;
        case cmderror:
            error(getargi(&crntag1));
            getargc(&crntag2,argstr);
            strcpy(errmsg,valstr);
            break;
        case cmderrclear:
            clearwarning();
            indicators[errflag] = false;
            break;
	case cmdchain_w:
	    getargc(&crntag1,argstr);
	    strcpy( altstr, valstr );
	    chain_wait( altstr, getargi(&crntag2) );
	    break;
	case cmdoptset:
	    worldoptions |= 1 << (getargi(&crntag1) & 31) ;
	    break;
	case cmdoptclr:
	    worldoptions &= ~( 1 << (getargi(&crntag1) & 31) );
	    break;
    }        /* CASE */
}


/* REGCMD */

/***************** EXECUTE *******************/

execute ()
{

    do {
        getline();            /* PROCESS INDICATORS */
        ok = ! df_abort;
        indcthelp(crntcommand.indct1);
        if (ok && more) {
            indcthelp(crntcommand.indct2);
            if (ok && more)
                indcthelp(crntcommand.indct3);
        }
        if (ok) {
            getcommand();
            if (debugmode)
		{writecon('[');writeint(crntline);writecon(']');}
            crntgroup = shr(crntcmd,6) & 0x3ff;
            switch (crntgroup) {    /* GOTO PROPER CMD GROUP */
                case 0:
                    break;                        /* NUL COMMAND */
                case grpmove:
                    cmdmovegroup();
                    break;
                case grpfile:
                    cmdfilegroup();
                    break;
                case grpctrl:
                    cmdctrlgroup();
                    break;
                case grpsio:
                    cmdseqiogroup();
                    break;
                case grpcon:
                    cmdcongroup();
                    break;
                case grpopen:
                    cmdopengroup();
                    break;
                case grpform:
                    cmdformgroup();
                    break;
                case grpprint:
                    cmdprtgroup();
                    break;
                case grpstr:
                    cmdstrgroup();
                    break;
                case grpenter:
                    cmdentergroup();
                    break;
                case grpintr:
                    cmdintrgroup();
                    break;
                case grpfinf:
                    cmdfinfgroup();
                    break;
                case grpfact:
                    cmdfactgroup();
                    break;
		case grpgraf:
		    cmdgraphgroup();
		    break;
/****************
		case grpcom:
		    cmdcomgroup();
		    break;
*****************/
#ifdef TESTGRP
		case TESTGRP:
			cmdtstgroup();
			break;
#endif
                default:
                    if (crntcmd & 0x8000)
                        entrycommand();
                    else if (crntcmd & 0x4000)
                        indctcommand();
                    break;
            }                /* CASE */
        }            /* OK */
        if (err)
            runerror();
    } while (!(df_abort));
}  /* EXECUTE */

