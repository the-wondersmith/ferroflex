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

/*** PRINT GROUP (#8) */

#define cmdprtopen 0x200
#define cmdprtclose 0x201
#define cmdprtpage 0x202
#define cmdpagechk 0x203
#define cmdbreak 0x204
#define cmdedits 0x205
#define cmdeditn 0x206
#define cmdbrkinit 0x207
#define cmdeditr 0x208

#define editzero 16

extern bool    seqendfile;
extern char    *breakptr[21] ;
extern int	mask;

numzero(str)
char *str;
{
	register int i;
	for (i=0;i<(NUMSIZE-1);i++) if (str[i]) return(0);
	return(1);
	}

cmdprtgroup ()
{
int     maxlines,
        uselines,
        prtlines,
        point,
	bklen,
        i;
bool    change;

    switch (crntcmd) {
        case cmdprtopen:
            if (crntag1.variant.str1.pargstat == 0)
                getcline(valstr);
            else 
                getargc(&crntag1,argstr);
            if (strcmp(valstr ,  " ") <= 0)
                strcpy(valstr,"LST:");
            sysint[sysline] = 0;
            seqclose(outfile);
            seqopen(outfile,valstr,true);
#ifdef SEQ_XLATE
	    seq_set_xlate_mode(outfile,FALSE);
#endif
            break;
        case cmdprtclose:
            seqclose(outfile);
            seqopen(outfile,"CON:",false);
            break;
        case cmdprtpage:
            getargc(&crntag1,argint);
            formprt(outfile,(int)valint);
            break;
        case cmdpagechk:
            maxlines = sysint[26];
            uselines = sysint[sysline];
            prtlines = sysint[25];
            if (prtlines > 0 && ((uselines + getargi(&crntag1)) > prtlines)) {
                if (uselines < 10000)
                    if (maxlines == 0)
                        seqpchar(outfile,chr(12));
                    else if (maxlines < 0) {
                        writestr("PLEASE PRESS ANY KEY TO CONTINUE ");
                        do {
                            term = readcon();
                        } while (!term);
                        sysint[systerm] = ord(term);
			writeeol();
                                            }
                    else 
                        while (uselines < maxlines) {
                            uselines = uselines + 1;
                            seqxeol(outfile);
                        }
                indicators[pagebreak] = true;
                sysint[sysline] = 0;
                sysint[syspage] = sysint[syspage] + 1;
            }
            else 
                indicators[pagebreak] = false;
            break;
        case cmdbreak:
            point = getargi(&crntag1);
            unpkarg(&crntag2);
            fillchar( valstr, sizeof(valstr), chr(0xff) );
	    bklen = sizeof(valstr)-1;
            if ((argclass & 15) <= argfldnum)
                bklen = vallen = sget(argfile,argfield,valstr);
            else  {
                getargc(&crntag2,argnum);
                move(valnum,valstr,sizeof(valnum));
                valstr[chr(sizeof(valnum))] = 0  /* string size change */;
                bklen = vallen = sizeof(valnum);
            }
/** taken out when vallen put in **
	    while ( (bklen>0) && (valstr[bklen]=='\377' ) )
		bklen--;
**/
            if (! breakptr[point] ) {
                new(breakptr[point],256);
		move( valstr, breakptr[point], bklen+1 );
                change = true;
            }
            else  {
                change = !(same(breakptr[point],valstr,bklen+1));
                if (change)
		    move( valstr, breakptr[point], bklen+1 );
            }
            indicators[point] = indicators[point - 1] || change;
            break;
        case cmdbrkinit:
            for (i = 0; i <= 20; i++)
                if (breakptr[i] != nil)
                    breakptr[i][0] = 0;
            break;
        case cmdedits:
            getargc(&crntag1,argstr);
            unpkarg(&crntag2);
            formgw();
            formes();
            move(valstr,crntwpos,crntwlen);
            formx();
            break;
        case cmdeditn:
            getargc(&crntag1,argnum);
            unpkarg(&crntag2);
            formgw();

            formeinit();
            if ((mask & editzero) &&
                numzero(valnum) ) formezero();
	    else if (crntwmode==128) {
		cvnd(valnum,valstr);
		padstr(valstr,crntwlen);
		}
            else {
               cvns(valnum,valstr,crntwlen, (crntwmode<128)?crntwmode:0);
               if ((mask & 3)==1) { /* total */
                  char *tmpchp = &(argspace[crntwinf.auxindex-1]);
                  cvsn(valstr,valnum);
                  bcd_add(valnum,tmpchp,tmpchp);
                  }
               formen(); /* edit valstr */
               }
            move(valstr,crntwpos,crntwlen);
            formx();
            break;
        case cmdeditr:
            getargc(&crntag1,argreal);
            unpkarg(&crntag2);
            formgw();

            formeinit();
            if ((mask & editzero) &&
                (valreal==0.0) ) formezero();
            else {
               cvfs(valreal,valstr,crntwlen, crntwmode);
               if ((mask & 3)==1) { /* total */
                  double *tmpr = (double *)&argspace[crntwinf.auxindex-1];
                  valreal = atof(valstr);
		  *tmpr += valreal;
                  }
               if (crntwmode!=129) 
                        formen();
                else valstr[2] = sc.apoint;                        
               }
            move(valstr,crntwpos,crntwlen);
            formx();
            break;
    }    /* CASE */
    indicators[errseq] = seqendfile;
}
