/************************************************************************
Confidential Trade Secret.
Copyright (c) 1986 Data Access Corporation, Miami Florida,
as an unpublished work.  All rights reserved.
DataFlex is a registered trademark of Data Access Corporation.
************************************************************************/

#include <standard.h>
#include <flex2con.h>
#include <flex2def.h>
#include <screencd.h>
#include <flex2var.h>


extern byte    numbersonly;
extern struct screencodes sc;
extern int ofstx, ofsty;

formi ()
{
/* GENERAL FORM INPUT TO BUFFER */
int     i;
char    achr;
bool    cmode,
        sflag,
        xflag,
        ucase;
long     aux;
int        auxx,
        waschange;
byte    *wchange;
union rtype {
    struct  {
       char    low[NUMSIZE];
       char    high[NUMSIZE];
       } bcn;
    struct {
    	double low;
    	double high;
        } rl;
    char str;
    } *rptr;


union rtype *withp0;

    formdp(crntwinf.page);
    do {
        aux = crntwinf.auxtype | (worldoptions & 0xfffffffc);
	capslock = (aux & CAPSLOCK) != 0;
        auxx = aux & 3;
        if (auxx != 0)
            rptr = (union rtype *) addr(argspace[(crntwinf.auxindex) - 1]);
        if (err) {
	    errscrn = TRUE;
            errprint();
	}
        scstrtpos = sysint[CURSET] ;
        formgs();
        wchange = addr(forminf[crntwnum].maskfill);
        waschange = *wchange & 0xc0;
        strcpy(altstr,valstr);
	padstr(altstr,crntwlen);
	if ((crntwmode != 255) && (argtype == argstr)) /* points = 0 */
		argtype = argnum;
        numbersonly = argtype;
	scoptions = aux;
        accept(crntwinf.row+ofsty,crntwinf.colm+ofstx,valstr,crntwlen,&term);
	scoptions = 0;
	sysint[CURRETURN] = scexitpos;
        cmode = (term == KCALCULATE) && !(aux & NOCALC);
        if (cmode) {
	    int atype = argtype;
            clearwarning();
            valstr[0] = 0  /* string size change */;
                /* NO DEFALT */
            numbersonly = 0;
            accept(sc.klength,6,valstr,70,&term);
	    numbersonly = atype;
            expeval(valstr,-1); /* returns type in argtype */
            formpt();
	    waserr = TRUE;
	    argtype = atype;
            waserr = true;
        }

	formupdate();

        if (strcmp(altstr ,  valstr) != 0)
            waschange = 0xc0;

	/* Check for no-check keys, usualy escape or help */
	if ( (1l<<(term & 63)) & sysint[nokeycheck]) goto L100;
/*
        if ((term == KESC) || (term == KHELP))
            goto L100;
*/
        if (auxx == 2) {    /* VALID RESPONCE CHK */
            if (pos(valstr,&rptr->str) == -1)
                error(15);
        }

        if (tstbit(aux,2)) {
		for (i=0; i<crntwlen; i++) /* check for somthing in wind */
			if (valstr[i] != ' ') break;
		if (i==crntwlen) error(13);
		}

        if ((numbersonly) && ( ! err)) {    /*** NUMBER ***/
            if (crntwmode == 128)
                cvdn(valstr,valnum);
            else  {
                i = poschr('.',valstr,1);
                if (i > 0)
                    i = poschr('.',valstr,i + 1);
                if (i > 0)
                    valstr[chr(i - 1)] = 0  /* string size change */;
                i = pos(" ",valstr)+1;
                    /* TAKE CARE OF TWO NUMBERS IN A WINDOW */
                if ((i > 1))
                    valstr[chr(i - 1)] = 0  /* string size change */;
		expeval(valstr,argtype);
/*                cvsn(valstr,valnum);*/
            }
            if (auxx == 3) {    /* NUMERIC RANGE */
		if ((argtype==argnum) || (argtype == argdate))
	                if ((ncmp(valnum, rptr->bcn.low)==numlt) ||
        	            (ncmp(valnum, rptr->bcn.high)==numgt))
                	    error(17);
		if ((argtype==argreal)) {
			if ((valreal<rptr->rl.low) || (valreal>rptr->rl.high))
				error(17);
			}			
	        }
            formpt();
        }
	else formps();
	

        /** SET CHANGED BIT FOR MULTIUSER **/
L100:
        *wchange = (crntwmask & 0x3f) | waschange;
    } while (err);
/*L100:*/
    sysint[CURRETURN] = scexitpos;
    sysint[CURSET]    = scstrtpos;
    flexkey();
    capslock = false;
    numbersonly = false;
}
