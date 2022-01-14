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

#define LF 10

extern char    encbuf[LSTRING] ;
extern struct screencodes sc;
extern int crntcolor, crntbg, coloron;
static linecnt;

#ifndef VMS
#define writedfix writefix
#endif
static writeofst();

formxdp ()
{
unsigned i,stp,nbuf;
int     start,
	wascolor,
	acolor,
	crfgcolor,
	chrs;


int     nwindow;
char    achar;
bool    togle,
        respage;
char    *pageptr;
struct pageinf *withp1;
struct windowinf *withp0;
    wascolor = crntcolor;
    p2crasgn(&crntpinf,sizeof(struct pageinf),&formpage[(crntpage) - 1]);
    ofstx = crntpinf.pgx;
    ofsty = crntpinf.pgy;
    linecnt = ofsty;
    crntbg = crntpinf.bgcolor;
    if (crntbg == 255) crntbg = scdim;
    setcolor(crntbg);
    crfgcolor = crntpinf.fgcolor;
    if (crfgcolor == 255) crfgcolor = dfltcolor;

    if ((ofstx != 255) && (ofsty != 255)) gotoxy( ofsty, ofstx );
    else {
        ofstx = ofsty = linecnt = 0;
	crntcls = crntbg;
    	clearscreen();
    	setcolor(crntbg);
	}
    chrs = crntpinf.pagechrs;
    nwindow = crntpinf.srtwindow;
    togle = false;
    respage = tstbit(crntpinf.memres,0);
    if ((crntpage != sysint[cpage]) && (chrs>0))
      if (respage) {    /*	MEMORY PAGE DISPLAY */
        stp = crntpinf.pageblk + chrs ;
	/* check for no windows */
	if (crntpinf.endwindow<crntpinf.srtwindow) nbuf = stp;
        else nbuf = forminf[nwindow].bufindex;
	i = crntpinf.pageblk;
	togle = i == nbuf;
	while (i<stp) {
		if (togle) {
			acolor = forminf[nwindow].color; 			
			setcolor((acolor==255)?crfgcolor:acolor);
			nbuf = forminf[nwindow].lenth;
			writedfix(&formspace[i], nbuf);
			i = i + nbuf;
                     	if (++nwindow>crntpinf.endwindow) nbuf = stp;
			else nbuf = forminf[nwindow].bufindex;
			setcolor(crntbg);
			}
		else {
/*          		if ((nbuf <= 0) || (nbuf>stp)) nbuf = stp; */
			writeofst(&formspace[i], nbuf - i);
			i = nbuf;
			}
		togle = ! togle;
		}
    }
    else  {    /* DISK PAGE DISPLAY */
        new(pageptr,chrs);
        bytread(cfgfile,pageptr,&i,(unsigned)chrs,
			((long)((long)crntpinf.pageblk*(long)BLKIOSIZE)) );
        nwindow = crntpinf.srtwindow;
        i = 0;            /* FILL IN WINDOWS */
	start = 0;
        while (i < chrs) {
            if (pageptr[i] == '_') {
		writeofst( &pageptr[start], i-start );
                withp0 = &forminf[nwindow];
		acolor = withp0->color; 			
		setcolor((acolor==255)?crfgcolor:acolor);
		writedfix(&formspace[withp0->bufindex], withp0->lenth );
                setcolor(crntbg);
                i = i + withp0->lenth;
		start = i;
                nwindow++;
            }
	    else
                i++;
        }
	writeofst( &pageptr[start], i-start );
        dispose(pageptr);
    }
    setcolor(wascolor);
}

static writeofst( st, stlen )
char *st;
int stlen;
{
char *stx, tmp;
int len;
	stx = st;
	while (stlen) {
		len = 0;
		while ((--stlen) && (stx[len] != LF)) len++;
		if (stx[len] == LF) {  /* fixed 2/3/88 maz */
			writedfix( stx, len++ );
			gotoxy(++linecnt, ofstx);
			}
		else
			writedfix(stx, ++len);
		stx = &stx[len];
		}	
}

/* formxd, deleted and never used */
