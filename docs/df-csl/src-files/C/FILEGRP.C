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

/*** FILE GROUP (#3) ***/

#define cmdfind 0x0c0      /*	2	FSND		CI		 */
#define cmdclear 0x0c1     /*	1	FXSND				 */
#define cmdsave 0x0c2      /*	1	FXSND				 */
#define cmddelete 0x0c3    /*	1	FXSND				 */
#define cmdrelate 0x0c4    /*	1	FXSND				 */
#define cmdreverse 0x0c5   /*	1	FXSND				 */
#define cmdxfind 0x0c6
#define cmdlock 0x0c7
#define cmdunlock 0x0c8
#define cmdreread 0x0c9
#define cmdsetmode 0x0cb

cmdfilegroup ()
{
/* FILE GROUP */
int     altfile,
        altfield;

    unpkarg(&crntag1);
    switch (crntcmd) {
        case cmdxfind:    /* INDEXED FIND */
        case cmdfind:
            altfile = argfile;
            altfield = argfield;
            if (crntcmd == cmdfind)
                altfield =  - altfield;
            find(altfile,altfield,getargi(&crntag2));
            indicators[errfile] = err;
            if ((ern == 41) || (ern == 42) || (ern == 25))
                err = false;
            break;
        case cmdclear:
            xclear(argfile);
            break;
        case cmdsave:
            save(argfile);
            break;
        case cmddelete:
            rdelete(argfile);
            break;
        case cmdrelate:
            relate(argfile);
            break;
        case cmdreverse:
            reverse(argfile);
            break;
        case cmdlock:
            lock();
            break;
        case cmdunlock:
            unlock();
            break;
        case cmdreread:
            reread();
            break;
	case cmdsetmode:
	    altint = argfile;
	    set_iomode( (int) altint, (int)getargi(&crntag2));
	    break;
    }
}
