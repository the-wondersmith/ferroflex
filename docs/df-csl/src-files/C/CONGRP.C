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

/*** CONSOLE I/O GROUP (#5) ***/

#define cmdconrline 0x140   /*	1	VFWYSNDE	CVFWIE		 */
#define cmdconwline 0x141   /*	1	CVFWYSNDE			 */
#define cmdconwstr 0x142    /*	1	CVFWYSNDE			 */
#define cmdconeol 0x143     /*	0					 */
#define cmdconcls 0x144     /*	0					 */
#define cmdcongotoxy 0x145  /*	2	CVFWYIE        CVFWYIE		 */
#define cmdconcxy 0x146     /*	2	CVFWYIE        CVFWYIE		 */
#define cmdconkey 0x147
#define cmdconscreen 0x148
#define cmdconlkey 0x149

extern byte     seqendfile;
extern struct screencodes sc;


cmdcongroup ()
{
bool    live;

    switch (crntcmd) {
        case cmdconrline:
            do {
                errprint();
                readstr(valstr,getargi(&crntag2),&term);
                writecon(chr(10));
                putargstr(&crntag1);
            } while (!( ! err));
            flexkey();
            break;
        case cmdconwline:
            getargc(&crntag1,argstr);
            writefix(valstr,vallen);
	    writeeol();
            break;
        case cmdconwstr:
            getargc(&crntag1,argstr);
            writefix(valstr,vallen);
            break;
        case cmdconeol:
            writecon(chr(10));
            break;
        case cmdconcls:
            if (crntag1.variant.str1.pargstat != 0x00)
		    crntcls = getargi( &crntag1 );
            clearscreen();
	    sysint[cpage] = crntpage = 0;
            break;
        case cmdcongotoxy:
            getargc(&crntag1,argint);
            altint = valint;
            gotoxy((int) altint,getargi(&crntag2));
            break;
        case cmdconcxy:
            getargc(&crntag1,argint);
            altint = valint;
            cleareos((int) altint,getargi(&crntag2));
            break;
        case cmdconlkey:
        case cmdconkey:
/**********
            do {
                term = readcnd();
                live = term > chr(0);
            } while (!(live || (crntcmd == cmdconlkey)));
***********/
	    if (crntcmd == cmdconlkey)
		term = readcnd();
	    else
		term = readchar();
	    live = term;
            if (term)
                flexkey();
            else indicators[100+fkeynum] = FALSE;
            indicators[livekey] = live;
            if (crntag1.variant.str1.pargstat != 0x00) {
/*		strccpy(valstr,term); */
                valstr[0] = term;
                valstr[1] = 0;
                vallen = 1;
                putargc(&crntag1,argstr);
            }
            break;
        case cmdconscreen:
            getargc(&crntag1,argint);
	    if (valint & 0x20000) coloron = (valint & 0x40000)>0;
            if (!(valint & 0x10000)) {
		dfltcolor = valint & 0xffff;
		setcolor(dfltcolor);
		}
	    valint = dfltcolor | 0x20000;
	    if (coloron) valint |= 0x40000;
	    sysint[LASTCOLOR] = valint;

            break;
    }        /* CASE */
}
