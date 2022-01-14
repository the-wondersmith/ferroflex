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

/*** FORMS GROUP (#7) */

#define cmdpage 0x1c0      /*	1	CVI				*/
#define cmdformin 0x1c1    /*	1	CVI				*/
#define cmdformd 0x1c2     /*	1	CVI				*/
#define cmdformclr 0x1c3
#define cmdformblank 0x1c4
#define cmdhelp 0x1c5
#define cmdifchange 0x1c6
#define cmdmkwindow 0x1c7
#define cmdpgxy     0x1c8
#define cmdpgcolor  0x1c9

extern struct screencodes sc;
extern int crntcolor, crntbg;
extern int saveline; /* from entrycmd */

cmdformgroup ()
{
/* FORM GROUP */
int     temtype;
int     startw,
	wascolor,
        endw,
        tnum;
char    achr;
struct pageinf *withp1,*withpg;
struct windowinf *withp0;

    switch (crntcmd) {
        case cmdpage:
            formdp(getargi(&crntag1));
            break;
        case cmdformin:
            /*	    SYSINT[ENTRFIELD] := -1;  FLAG NOT AN ENTRY CMD */
	    saveline = 0; /* disable ent-find */
            unpkarg(&crntag1);
            formgw();
            formi();
            temtype = argtype;
            unpkarg(&crntag2);
            if ((argstat & 0xf) != 7) {
		getargc(&crntag1,temtype);
                putargc(&crntag2,temtype);
		}
            clearwarning();
            break;
        case cmdformd:
		getargnc(&crntag1);
		putargc( &crntag2,argtype);
            break;
        case cmdformblank:
        case cmdformclr:
	    wascolor = crntcolor;
            unpkarg(&crntag1);
            achr = ' ';
            if (crntcmd == cmdformclr)
                achr = sc.kfill;
            else
                if (!coloron) setcolor(crntbg);
            if ((argclass == argwndrl) ||
		    (argclass == argwndnum) || (argclass == argwndstr)) {
                startw = argindex;
                unpkarg(&crntag2);
                endw = argindex;
                if (endw == 0)
                    endw = startw;
            }
            else if (argindex == 0) {
                startw = 1;
                endw = sysint[29];
            }
            else  {
                withp1 = &formpage[(argindex) - 1];
                startw = withp1->srtwindow;
                endw = withp1->endwindow;
            }
            fillchar( valstr, sizeof( valstr ), achr );
            valstr[255] = 0;
            for (argindex = startw; argindex <= endw; argindex++) {
		vallen = 255;
                formgw();
                formpsl();
       		valstr[crntwlen] = achr;
            }
            if (crntcmd == cmdformblank)
                setcolor(wascolor);
            break;                /* FORMCLR */
        case cmdhelp:
            help(getargi(&crntag1));
            break;                /** MULTIUSER TEST AND MARK WINDOW **/
        case cmdifchange:
            unpkarg(&crntag1);
            indicators[endfor] = tstbit(forminf[argindex].maskfill,6);
            break;
        case cmdmkwindow:
            unpkarg(&crntag1);
            withp0 = &forminf[argindex];
            withp0->maskfill = withp0->maskfill | 0xc0;
            break;
        case cmdpgxy:
	    withpg = &formpage[ sysint[strmark] - 1 ];
	    withpg->pgy = getargi(&crntag1);
	    withpg->pgx = getargi(&crntag2);
	    break;
	case cmdpgcolor:
	    withpg = &formpage[ sysint[strmark] - 1 ];
	    getargc(&crntag1, argint);
	    if (valint) withpg->fgcolor = valint;
	    getargc(&crntag2, argint);
	    if (valint) withpg->bgcolor = valint;
	    break;
    }        /* CASE */
}
