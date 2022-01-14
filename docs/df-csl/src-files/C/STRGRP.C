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

#define cmdstrpad 0x240
#define cmdstrleft 0x241
#define cmdstrright 0x242
#define cmdstrappend 0x243
#define cmdstrpos 0x244
#define cmdstrmid 0x245
#define cmdstrlnth 0x246
#define cmdstrasc 0x247
#define cmdstrchr 0x248
#define cmdstruc 0x249
#define cmdcmdline 0x24a
#define cmdstrip 0x24b
#define cmdstrinsert 0x24c
#define cmdstrremove 0x24d

static int     slen;

static getag1str ()
{
    getargc(&crntag1,argstr);
    slen = vallen;
}

cmdstrgroup ()
{
int     len,
        smark,
        itemp;
char    ch;
int     i,
        cnt;
char    achar;

    len = sysint[STRLEN];
    if (len < 0)
        len = 0;
    smark = sysint[strmark];
    if (smark < 0)
        smark = 0;
    switch (crntcmd) {
        case cmdstrpad:
            getag1str();
            while (vallen<smark) valstr[vallen++] = ' ';
	    valstr[vallen=smark] = 0;
            putargc(&crntag2,argstr);
            break;
        case cmdstrleft:
            getag1str();
            if (slen > smark) {
		vallen = smark;
                valstr[smark] = 0  /* string size change */;
		}
            putargc(&crntag2,argstr);
            break;
        case cmdstrright:
            getag1str();
            if (len > slen)
                len = slen;
	    vallen = len;
            moveleft(&valstr[slen - len],valstr,vallen);
	    valstr[vallen] = 0;
            putargc(&crntag2,argstr);
            break;
        case cmdstrappend:
            getargc(&crntag2,argstr);
            mv_alt();
            getag1str();
	    altlen = imin(altlen,255-vallen)+1;
	    move( altstr, &valstr[vallen], altlen );
	    vallen = vallen+altlen-1;
            putargc(&crntag1,argstr);
            break;
        case cmdstrpos:
            getag1str();
            mv_alt();
            getargc(&crntag2,argstr);
            smark = npos(altstr,altlen,valstr,vallen)+1;
            sysint[strmark] = smark;
            sysint[STRLEN] = vallen - smark + 1;
            indicators[errfile] = smark == 0;
            break;
        case cmdstrmid:
            getag1str();
            if ((smark < 1) || (smark > slen))
                len = 0;
            slen = slen - smark + 1;
            if (len > slen)
                len = slen;
            if ((vallen=imax(0,len)) > 0)
		moveleft( &valstr[smark-1], valstr, vallen );
	    valstr[vallen] = 0;
            putargc(&crntag2,argstr);
            break;
        case cmdstrlnth:
            getag1str();
            valint = slen;
            putargc(&crntag2,argint);
            break;
        case cmdstrasc:
            getag1str();
            if (slen > 0)
                valint = (byte)valstr[0];
            else 
                valint = 0;
            putargc(&crntag2,argint);
            break;
        case cmdstrchr:
            valstr[0] = getargi(&crntag1);
            valstr[1] = 0  /* string size change */;
	    vallen = 1;
            putargc(&crntag2,argstr);
            break;
        case cmdstruc:
            getag1str();
    	    while(slen--) valstr[slen] = toupper( valstr[slen] );
            putargc(&crntag2,argstr);
            break;
        case cmdcmdline:    /* GET COMMAND LINE */
	    valstr[0] = 0;
            vallen = 0;
            i = 0;
            while (achar = cmdbuf[i]) {
                if ((achar == ' ') || (achar == ',')) {
                    if (vallen) break;
                }
                else  {
                    valstr[vallen++] = achar;
                    cmdbuf[i] = ' ';
                }
                i++;
            }
L111:
            valstr[vallen] = 0  /* string size change */;
            putargc(&crntag1,argstr);
            break;
        case cmdstrip:
            getag1str();
    	    vallen--;
            while ((vallen >= 0) && ((byte)valstr[vallen] <= ' ')) vallen--;
	    valstr[++vallen] = 0;
	    len = 0;
	    while ((len<vallen) && ((byte)valstr[len] <= ' ')) len++;
	    if (len) {
		    vallen -= len; i = vallen+1;
		    slen = 0;
		    while (i--) valstr[slen++] = valstr[len++];
		    }
            putargc(&crntag2,argstr);
            break;
        case cmdstrremove:
            getargc(&crntag2,argstr);
            mv_alt();
            getag1str();
            smark = npos(altstr,altlen,valstr,vallen);
	    slen = smark+altlen;
            if ((altlen > 0) && (smark >= 0)) {
		moveleft( &valstr[slen], &valstr[smark], vallen-slen+1 );
		vallen -= altlen;
		}
            putargc(&crntag1,argstr);
            sysint[strmark] = smark+1;
            indicators[errfile] = smark == -1;
            break;
        case cmdstrinsert:
            getargc(&crntag2,argstr);
            mv_alt();
            getag1str();
	    i = vallen+altlen;
	    if (i>255)
		    { vallen -= (i-255); valstr[vallen] = 0;}
            if (smark > slen)
		move( altstr, &valstr[vallen], altlen+1 );
            else if (smark <= 0) {
		move( valstr, &altstr[altlen], vallen+1 );
		move( altstr, valstr, vallen+altlen+1 );
		}
            else {
		i = vallen-smark+1;
		smark--;
		moveright( &valstr[smark], &valstr[smark+altlen], i );
		move( altstr, &valstr[smark], altlen );
		}
	    vallen += altlen;
            putargc(&crntag1,argstr);
            break;
    }        /* CASE */
}

mv_alt()
{
	move( valstr, altstr, vallen+1 );
	altlen = vallen;
}

npos( cmp, cmplen, in, inlen )
char	cmp[],in[];
int	cmplen,inlen;
{
int	inpos = -1;
	if (cmplen) while (inlen--)  /* fixed "indicate in" bug 2/3/88 */
		if (same( &in[++inpos], cmp, cmplen ))
			return(inpos);
	return(-1);
}
