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

extern char    encbuf[LSTRING] ;
extern struct screencodes sc;

help (helppage)
int     helppage;
{
int     waspage;

    waspage = crntpage;
    if (helppage == 0) {
        if (crntpage == 0)
            return;
        helppage = crntpage + 1;
    }
    while ((tstbit(formpage[(helppage) - 1].memres,1)) && 
           (helppage <= sysint[28])) {
        formdp(helppage);
        if (readchar() != KHELP)
	    break;
        helppage = helppage + 1;
    }
L100:
    if (waspage > 0)
        formdp(waspage);
}
