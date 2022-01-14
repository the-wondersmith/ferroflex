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

/* 12/8/84  - MOVED 0 CHR CHECK INTO THIS ROUTINE */

extern char    encbuf[] ;

#ifdef SEQ_XLATE
seqpxfix (sfcb,str,len)
char    *sfcb;
char    *str;
int len;
{	/** this will strip binary zero for seqpblock **/
        /** and also turn \n into CRLF **/
	int xlen;
	char *stx;
	stx = str;
	while (len) {
		xlen = 0;
		while ((len) && (stx[xlen]) && (stx[xlen] != '\n'))
                        {xlen++;len--;};
       		if (xlen) seqpblock( sfcb, stx, xlen );
                if (stx[xlen]=='\n') {
                        seqxeol( sfcb );
                        len--;
			stx = &stx[xlen+1];
                        }
                else {
		        stx = &(stx[xlen]);
		        while ((len) && (! *stx)) {stx++;len--;};
                        }
		}
	}
#else
#define seqpxfix seqpfix
#endif	

formprt (sfile,gopage)
char    sfile[] ;
int     gopage;
{
int     i,
        stp,
        chrs,
        savepage;
char    *pageptr;
char    achar;
struct pageinf *withp1;
struct windowinf *withp0;
    savepage = crntpage;
    crntpage = gopage;
    p2crasgn(&crntpinf,sizeof(crntpinf),&formpage[(crntpage) - 1]);
    withp1 = &crntpinf;
    chrs = withp1->pagechrs;
    sysint[sysline] = sysint[sysline] + withp1->numlines;
  if (chrs)
    if (tstbit(crntpinf.memres,0)) {    /* BEGIN MEMORY PAGE PRINT   */
	seqpxfix( sfile, &formspace[withp1->pageblk], chrs );
    }
    else  {    /* DISK PAGE PRINT   */
        new(pageptr,chrs);
        bytread(cfgfile,pageptr,&i, (unsigned)chrs,
		((long)((long)withp1->pageblk*(long)BLKIOSIZE)));
        stp = withp1->srtwindow;
        i = 0;                /* FILL IN WINDOWS */
        while (i < chrs)
            if (pageptr[i] == '_') {
                withp0 = &forminf[stp];
                move(&formspace[withp0->bufindex],&pageptr[i],withp0->lenth);
                i = i + withp0->lenth;
                stp = stp + 1;
            }
            else 
                i = i + 1;
	seqpxfix( sfile, pageptr, chrs );
        dispose(pageptr);
    }
    crntpage = savepage;
}

