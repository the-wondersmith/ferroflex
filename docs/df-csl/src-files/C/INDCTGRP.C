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

#include <command.h>

/*** CONTROL GROUP (#1) ***/

/* #define cmdabort 0x040    	0					 */

extern bool    ok,
               more;

bool boolexp (arg,allmode)
struct argument *arg;
bool    allmode;
{
    ok = allmode;
    indcthelp(arg->variant.str1.pargstat);
    if ((ok == allmode) && more) {
        indcthelp(arg->variant.str1.pargfile);
        if ((ok == allmode) && more)
            indcthelp(arg->variant.str1.pargfield);
    }
    return(ok);
}


indctcommand ()
{
int     ind;
bool    truth,
        left,
        right;
int     cmdnum,
        subnum;
int     strmode;

    truth = false;
    cmdnum = crntcmd & 0x403f;
        /* STRIP OUT INDICATOR NUMBER */
    if (cmdnum < 0x4020) {    /* TWO ARGUMENT COMMAND */
        subnum = cmdnum & 3;
	switch (subnum) {
        case 0:    /* ASCII ARGUMENTS */
            getargc(&crntag1,argstr);
	    move(valstr,altstr,vallen+1);
	    altlen = vallen;
            getargc(&crntag2,argstr);
            strmode = strncomp(altstr,valstr,altlen,vallen);
	    break;
	case 1: /* NUMERIC ARGUMENTS */
            getargc(&crntag1,argnum);
            move( valnum, altnum, NUMSIZE );
            getargc(&crntag2,argnum);
	    break;
	case 2: /* INTEGER ARGUMENTS */
	    getargc(&crntag1,argint);
	    altint = valint;
	    getargc(&crntag2,argint);
	    break;
        case 3: /* REAL ARGUMENTS */
            getargc(&crntag1,argreal);
            altreal = valreal;
	    getargc(&crntag2,argreal);
            break;
	    }
        switch (cmdnum) {
	/* less than */
            case cmdinlts:
                truth = strmode == lt;
                break;
            case cmdinlti:
		truth = altint<valint;
		break;
            case cmdinltn:
                truth = ncmp(altnum,valnum) == numlt;
                break;
	    case cmdinltr:
		truth = altreal<valreal;
		break;
	/* less than or equal to */
            case cmdinles:
                truth = strmode <= eq;
                break;
            case cmdinlei:
		truth = altint<=valint;
		break;
            case cmdinlen:
                truth = ncmp(altnum,valnum) <= numeq;
                break;
	    case cmdinler:
		truth = altreal<=valreal;
		break;
	/* equal to */
            case cmdineqs:
                truth = strmode == eq;
                break;
            case cmdineqi:
		truth = altint==valint;
		break;
            case cmdineqn:
                truth = ncmp(altnum,valnum) == numeq;
                break;
	    case cmdineqr:
		truth = altreal==valreal;
		break;
	/* greater than or equal to */
            case cmdinges:
                truth = strmode >= eq;
                break;
            case cmdingei:
		truth = altint>=valint;
		break;
            case cmdingen:
                truth = ncmp(altnum,valnum) >= numeq;
                break;
	    case cmdinger:
		truth = altreal>=valreal;
		break;
	/* greater than */
            case cmdingts:
                truth = strmode == gt;
                break;
            case cmdingti:
		truth = altint > valint;
		break;
            case cmdingtn:
                truth = ncmp(altnum,valnum) == numgt;
                break;
	    case cmdingtr:
		truth = altreal>valreal;
		break;		
	/* not equal to */
            case cmdinnes:
                truth = strmode != eq;
                break;
            case cmdinnei:
		truth = valint != altint;
		break;
            case cmdinnen:
                truth = ncmp(altnum, valnum) != numeq;
                break;
	    case cmdinner:
		truth = altreal != valreal;
		break;
        }    /* CASE */    /* TWO ARGUMENT IF */
    }
    else if (cmdnum < 0x4028) {    /*	BOOL EXPRESSION */
        left = boolexp(&crntag1,(bool)((cmdnum & 4)?1:0));
        right = boolexp(&crntag2,(bool)((cmdnum & 2)?1:0));
	if ((cmdnum & 1))
            truth = left && right;
        else 
            truth = left || right;
    }
    else  {    /* OTHER IF'S */
        if (cmdnum == cmdinstat) {
            unpkarg(&crntag1);
            truth = status(argfile) >= 2;
        }
        else if (cmdnum >= cmdininstr) {    /* OTHER STRING INDICATES */
            getargc(&crntag1,argstr);
	    move(valstr,altstr,vallen+1);
	    altlen = vallen;
            getargc(&crntag2,argstr);
	    if (cmdnum == cmdininstr)
                truth = npos(altstr,altlen,valstr,vallen) >= 0;
            else  {
		int slen;
                slen = pos("*",altstr);
                if (slen < 0)
			slen = imax(vallen,altlen);
		else {  /* trunc to position of '*' */
			vallen = imin(vallen,slen);
			altlen = imin(altlen,slen);
			}
                for (subnum = 0; subnum < slen; subnum++)
                    if (altstr[subnum] == '?')
                        valstr[subnum] = '?';
                truth = strncomp(altstr,valstr,altlen,vallen) == eq;
            }
        }
    }        /* OTHER IFS */
    if ((crntgroup & 0x80))
        truth =  ! truth;
    indicators[crntgroup & 0x7f] = truth;
}
