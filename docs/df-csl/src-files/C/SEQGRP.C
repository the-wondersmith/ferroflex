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

/*** SEQ I/O GROUP (#4) ***/

#define cmdsqiopen 0x100    /*	1	CVFWS				 */
#define cmdsqoopen 0x101    /*	1	CVFWS				 */
#define cmdsqiclose 0x102   /*	0					 */
#define cmdsqoclose 0x103   /*	0					 */
#define cmdseqrline 0x104   /*	1	VFWYSNDE			 */
#define cmdseqwline 0x105   /*	1	CVFWYSNDE			 */
#define cmdseqwstr 0x106    /*	1	CVFWYSNDE			 */
#define cmdseqeol 0x107     /*	0					 */
#define cmdseqread 0x108
#define cmdseqrfixed 0x109  

static bool    match;
static char    matchchar;
static char    achar;
static bool    seqeline;
extern bool    seqendfile;


seqg ()
{
    if (seqeline)
        achar = matchchar;
    else  {
        seqgchar(infile,&achar);
        seqeline = (achar == '\n') || seqendfile;
    }
    match = seqeline || (achar == matchchar);
}


cmdseqiogroup ()
{
int     cnt;
bool    inquote;
char ach;
    switch (crntcmd) {
        case cmdsqiopen:
            if (crntag1.variant.str1.pargstat == 0)
                getcline(valstr);
            else 
                getargc(&crntag1,argstr);
                /* FILE NAME */
            seqclose(infile);
            if (strcmp(valstr ,  " ") <= 0)
                strcpy(valstr,"CON:");
            seqopen(infile,valstr,false);
#ifdef SEQ_XLATE
	    seq_set_xlate_mode(infile,FALSE);
#endif
	    indicators[seqeol] = FALSE;
            break;
        case cmdsqoopen:
            if (crntag1.variant.str1.pargstat == 0)
                getcline(valstr);
            else 
                getargc(&crntag1,argstr);
            seqclose(outfile);
            sysint[sysline] = 0;
            if (strcmp(valstr ,  " ") <= 0)
                strcpy(valstr,"LST:");
            seqopen(outfile,valstr,true);
#ifdef SEQ_XLATE
	    seq_set_xlate_mode(outfile,FALSE);
#endif
            indicators[seqeol] = seqendfile;
            break;
        case cmdsqiclose:
            seqclose(infile);
            seqopen(infile,"CON:",false);
            break;
        case cmdsqoclose:
            seqclose(outfile);
            seqopen(outfile,"LST:",false);
            break;
        case cmdseqrline:
            if (seqendfile || indicators[seqeol])
		vallen = valstr[0] = 0;
            else 
                vallen = seqrline(infile,valstr);
            indicators[seqeol] = seqendfile;
            putargc(&crntag1,argstr);
            break;
        case cmdseqwline:
            getargc(&crntag1,argstr);
            if (vallen) seqpblock( outfile, valstr, vallen);
	    seqxeol(outfile);
            sysint[sysline]++;
            break;
        case cmdseqwstr:
            getargc(&crntag1,argstr);
            if (vallen) seqpblock( outfile, valstr, vallen);
            break;
        case cmdseqeol:
            sysint[sysline]++;
	    seqxeol(outfile);
            break;
        case cmdseqread:    /** BASIC LIKE READ **/
            seqeline = indicators[seqeol] || seqendfile;
            matchchar = ' ';
            if ( ! seqeline)
                do {
                    seqg();
                } while (!(seqeline || (achar > ' ')));
            valstr[0] = 0;
            inquote = (achar == '"') || (achar == chr(39));
            if (inquote && ! seqeline) {
                matchchar = achar;
                seqg();
            }
            else 
                matchchar = ',';
            match = achar == matchchar;
	    cnt = 0;	    
            while (( ! match)) {
		valstr[cnt++] = achar;
                seqg();
            }
	    valstr[cnt] = 0;
            vallen = cnt;
            if (inquote && ( ! seqeline))
                while ((achar != ',') && ( ! seqeline))
                    seqg();
            indicators[seqeol] = seqeline;
            putargc(&crntag1,argstr);
            break;
        case cmdseqrfixed:
            valint = getargi(&crntag2);
            for (cnt = 0; cnt < valint; cnt++) {
                seqgchar(infile,&achar);
                if (cnt < 255)
                    valstr[cnt] = achar;
            }
            if (valint > 255)
                valint = 255;
            valstr[valint] = 0  /* string size change */;
            vallen = valint;
            putargc(&crntag1,argstr);
            break;
    }    /* CASE */
    indicators[errseq] = seqendfile;
}
