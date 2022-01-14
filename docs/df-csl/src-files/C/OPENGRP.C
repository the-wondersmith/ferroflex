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

#define cmdopen 0x180
#define cmdclose 0x181
#define cmdvopen 0x182

extern char    *ddr[] ;

cmdopengroup ()
{
    unpkarg(&crntag1);
    if ((argfile == 0) || (argfile == 255))
        argfile = sysint[entrfile];
    switch (crntcmd) {
        case cmdopen:
            itemp = argfile;
            xopen((int)itemp,(int)getargi(&crntag2),8,0);
            sysint[heapsize] = memavail();
            break;
        case cmdclose:
            xclose(argfile);
            break;
	case cmdvopen:
		itemp = argfile;
		getargc(&crntag2, argstr);
		/* entrfield is index to buffer */
		vxopen( (int)itemp, valstr, (int)sysint[entrfield], 12, 0);
		sysint[heapsize] = memavail();
		break;
    }        /* CASE */
    indicators[errfile] = err;
}
