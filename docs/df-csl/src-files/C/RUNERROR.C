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

#define timetopress 3000

extern bool    errscrn,
               entbottem;
extern int     nextline;

extern  char  readcon();

runerror ()
{
int     errto;
    sysint[syserrnum] = ern;
    sysint[syserrline] = errline;
    errscrn = crntpage > 0;
    entbottem = false;
    errto = sysint[ONERROR];        /* ABORT ERRORS */
    if ((errto > 0) && (!indicators[errflag]) && (!noerror)) {
        dogosub(errto);
        noerror = sysint[sysreturn];
        df_abort = df_abort || (ern == 97);
        err = df_abort;
    }
    else  {
        df_abort = df_abort || ((set_in(ern,0,set_make(3,10,18,19,20,21,22,
		        43,70,72,74,75,78,80,97,e_n_d),0)) && (ern<100));
    }
    if (err) {
        errto = timetopress;
        if (df_abort) {
	    writeeol();
            errto = 32766;
        }
        errprint();
        do {
            errto = errto - 1;
        } while (!((errto == 0)));
    }
    indicators[errflag] = true;
    if (df_abort)
        sysint[127] =  - 1;
}


/* unreached code removed -cbc-12/15/87*/
