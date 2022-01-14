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

#define cmdflist 0x300
#define cmdfnext 0x301
#define cmdfname 0x302
#define cmdfchange 0x303
#define cmdstat 0x304
#define cmdsysname 0x307
#define cmdzerofile 0x308
#define cmdfl1put 0x309
#define cmdfl2put 0x30a
#define cmdmemavail 0x30b
#define cmdgetbits  0x30c

struct flist {
    char    pathname[41] ;
    char    username[33] ;
    char    altname[11] ;
};

extern char filelist[];

cmdfinfgroup ()
{
struct flist *fptr;
int     flnum,
        result,
        i;
struct flist *withp0;

    switch (crntcmd) {
        case cmdflist:
        case cmdfnext:    /* GET INFO FROM FILE INFORMATION */
            flnum = sysint[entrfile];
            fptr = (struct flist *) keyp;
            do {
                if (crntcmd == cmdfnext)
                    flnum = flnum + 1;
                blkread(filelist,keyp,&result,1,flnum);
                if (result > 0) {
                    fillchar(keyp,sizeof(struct flist),chr(0));
                    flnum = 0;
                }
            } while ((crntcmd != cmdflist) && 
                     (!fptr->altname[0]) && (flnum));
            if (flnum > 0) {
                strcpy( valstr, fptr->username );
                putargstr(&crntag1);
                strcpy( valstr, fptr->altname );
                putargstr(&crntag2);
            }
            sysint[entrfile] = flnum;
            indicators[errfile] = (flnum == 0) || (!valstr[0]);
            break;
        case cmdstat:
            unpkarg(&crntag1);
            valint = status(argfile);
            putargc(&crntag2,argint);
            break;
        case cmdfname:
            fptr = (struct flist *) keyp;
            strcpy( valstr, fptr->pathname );
            putargstr(&crntag1);
            strcpy( valstr, fptr->altname );
            putargstr(&crntag2);
            break;
        case cmdfl1put:
            fptr = (struct flist *) keyp;
            getargc(&crntag1,argstr);
	    valstr[ 10 ] = 0;
            strcpy( fptr->altname, valstr );
            break;
        case cmdfl2put:
/* 2/25/87 changed keyp to altstr to preserve altname - maz */
	    for (altint=1;altint<sysint[entrfile];altint++) {
		blkread( filelist, altstr, &result, 1, altint );
		if (result) {
		    fillchar( altstr, 128, 0 );
		    blkwrite( filelist, altstr, &result, 1, altint );
		}
	    }

            fptr = (struct flist *) keyp;
            getargc(&crntag1,argstr);
	    valstr[ 40 ] = 0;
            strcpy( fptr->pathname, valstr );
            getargc(&crntag2,argstr);
	    valstr[ 32 ] = 0;
            strcpy( fptr->username, valstr );
            blkwrite(filelist,keyp,&result,1,(int) sysint[entrfile]);
            break;
        case cmdzerofile:
            getargc(&crntag1,argint);
            if (valint == 0)
                valint = sysint[entrfile];
            zerofile((int) valint);
            break;
        case cmdsysname:
            strcpy(valstr,sc.tname);
            putargstr(&crntag1);
	    valint = sexy32(sc.tnum);
            putargc(&crntag2,argint);
            break;
        case cmdfchange:
            unpkarg(&crntag1);
            indicators[endfor] = change(argfile);
            break;
        case cmdmemavail:
            valint = memavail();
            sysint[heapsize] = valint;
            putargc(&crntag1,argint);
            break;
	case cmdgetbits:
		valint = sexy32(sc.bits);
		putargc(&crntag1,argint);
		valint = sexy32(sc.morebits);
		putargc(&crntag2,argint);
		break;
    }
}
