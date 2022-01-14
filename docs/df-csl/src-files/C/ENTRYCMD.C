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

#define fkreturn 1
#define fkclear 13
#define cmdreturn 0x43
#define cmdformin 0x1c1
#define autofind 0
#define findreq 1
#define noput 2
#define noenter 3
#define skipfound 4
#define retain 5
#define xrepeat 6
#define nomulti 7
#define autoge 8

#define cmdeupdate 0x280
#define cmdefind 0x281
#define cmdesfind 0x282
#define cmdedisp 0x283
#define cmdeclear 0x284
#define cmdebtrue 0x285
#define cmdebfalse 0x286
#define cmderclr 0x287
#define cmdeback 0x288
#define cmdestart 0x289
#define cmdedr 0x28a

static int     ltemp;
extern struct screencodes sc;
extern int16   fkeynum;
extern int16   entfile,
               entfield;
extern bool    entbottem;
extern int     nextline;
extern char    done[34] ;	/* set */
extern bool    ok,
               more;
extern int16   lastfkey;

static int     estart;

int	savestack,saveline;

bool entstat();
bool enthelp();

entrycommand ()
{
/* ENTRY*/
int    skipit,
        redo,
        autoerr;
int	savecmd, savekey;

/* ENTRY COMMAND */

    estart = sysint[entstart];
    unpkarg(&crntag1);
    entfile = argfile;
    entfield = argfield;
    redo = (entfile == sysint[entrfile]) && (entfield == sysint[entrfield]);
    sysint[entrfile] = entfile;
    sysint[entrfield] = entfield;
    savestack = sysint[sysreturn]; /* 2/21/86 */
    saveline = crntline;           /* 4/17/86 */
    savekey  = sysint[nokeyproc];
    savecmd  = crntcmd;
    unpkarg(&crntag2);    /* GET WINDOW */
    formgw();
    skipit = tstbit(crntcmd,noenter) || entbottem;
    if ( ! skipit)
        skipit = ( ! redo) && (tstbit(crntcmd,skipfound) && entstat());
    if (skipit && tstbit(crntwinf.auxtype,2)) {
        formgs();
        skipit = valstr[0];
        entbottem = entbottem && skipit;
    }
    if (skipit) {        /* SKIP ENTRY */
            term = KRETURN;            /* PRETEND TO DO RETURN */
            flexkey();
    }
    else  {
        formi();        /* <<<<<<<<<< INPUT <<<<<<<<<<< */
        if (waserr)
            clearwarning();
    }    /******* AUTO-FIND ********/
    autoerr = false;
    if (tstbit(savecmd,autofind) && (fkeynum == fkreturn) && 
       ((entfile == sysint[entmfile]) || indicators[entcompat] || 
        ! indicators[entquery])) {
        if (tstbit(savecmd,autoge))
            entfind(ge,false,0);
        else 
            entfind(eq,false,0);
/*        getcommand(); */
        if ( ! err)
            entdisp();
        else  {
            err = false;
            autoerr = true;
        }
    }    /******** FIND REQUIRED *******/

    if (tstbit(savecmd,findreq) && 
       (!(set_in(fkeynum,0,set_make(3,6,7,8,13,14,15,18,e_n_d),0)
/* CKESC,CKLFIELD,CKFIND,CKSFIND,CKCLEAR,CKUP,CKDOWN,CKHELP */
	&& (sysint[sysreturn] != savestack)))
	)
        if (autoerr || ! entstat()) {
            error(90);
            entbottem = false;
	    sysint[sysreturn] = savestack;
	    sysint[nokeyproc] = savekey;
            nextline = saveline;
        }
}


/* ENTRYCMD*/

bool entstat ()
{
    return( (bool) (status(entfile) >= 2) );
}


bool enthelp ()
{
    getline();
    getcommand();
    return( (bool) tstbit(crntcmd,15));
}


entdisp ()
{
int     saveline;
int     temtype;

    saveline = nextline;
    nextline = estart;
    do {
        if (enthelp()) {            /* ENTRY CMD */
            if ((crntag2.variant.str1.pargstat & 0xf) == argwndnum)
                temtype = argnum;
            else
                temtype = argstr;
            if (((set_in(crntag1.variant.str1.pargfile,256,done,256)) ||
                ((crntag1.variant.str1.pargstat & 15) == argexp))) {

                getput(temtype);

                /** RESET CHANGE BIT FOR MULTIUSER **/

                forminf[crntwnum].maskfill = crntwmask & 0x3f;
            }
        }        /* IF */
    } while (crntcmd != cmdreturn) ;

    nextline = saveline;
}


entupdate (filenum,doall)
int     filenum;
bool    doall;
{
int     saveline;
int     temtype;

    saveline = nextline;
    nextline = estart;
    do {
        if (enthelp()) {            /* ENTRY CMD */
            if ((crntag2.variant.str1.pargstat & 0xf) == argwndnum)
                temtype = argnum;
            else 
                temtype = argstr;
            if ((doall || ( ! tstbit(crntcmd,noput))) &&
                (!(tstbit(crntcmd,noput) &&
                tstbit(crntcmd,noenter))) &&
                ((crntag1.variant.str1.pargstat &15) != argexp) &&
                ((filenum == 0) ||
                (filenum == crntag1.variant.str1.pargfile))) {
                getargc(&crntag2,temtype);
                if (doall || tstbit(crntwmask,7) || tstbit(crntcmd,nomulti)) {
                    if (doall)
                        hold(crntag1.variant.str1.pargfile);
                    putargc(&crntag1,temtype);
		}
            }
        }        /* IF */
    } while (crntcmd != cmdreturn) ;

    nextline = saveline;
}


entclear ()
{
int     saveline;
bool    xxchange,
        xclr;

/*  NOCHANGE   := ((LASTFKEY) = (FKCLEAR));	*/

    xxchange = (lastfkey != 13);
    saveline = nextline;
    nextline = estart;
    fillchar(valstr,sizeof(valstr),sc.kfill);
    valstr[255] = 0;
    vallen = 255;
    do {
        if (enthelp()) {            /* ENTRY CMD */
            xclr = (xxchange && tstbit(crntcmd,6)) || 
                      tstbit(crntcmd,retain);
            unpkarg(&crntag2);
            formgw();
            if (xclr)            /* SET CHANGED */
                forminf[crntwnum].maskfill = crntwmask | 0x80;
            else  {
                formpsl();
		valstr[crntwlen] = sc.kfill;
                vallen = 255;
            }
        }        /* IF */
    } while (crntcmd != cmdreturn) ;
    nextline = saveline;
}


entfind (amode,uptree,upfile)
int     amode;
bool    uptree;
int     upfile;
{
bool    ractive;
/***
char    tempset[32];
    p2csasgn(tempset,256,set_make(e_n_d),0);
    p2csasgn(done,256,tempset,128);
***/
    fillchar(done, 34, 0);
    if ((sysint[entline] < estart) || (estart == 0))
        error(79);
    if (err)
        return;
    uptree = uptree && (upfile != entfile);
    ractive = entstat();    /** ADDED FOR FIND ERRORS **/

    if (uptree) {
        ractive = status(upfile) >= 2;
        entupdate(0,true);
        uptree = superfind(upfile,entfile,entfield,amode) >= 0;
        if (uptree)
            entfile = upfile;
        else
            error(87);
    }
    if ( ! uptree) {
        hold(entfile);
        entupdate(entfile,true);
        reverse(entfile);
        find(entfile, - entfield,amode);
    }
    indicators[errfile] = err;
/***
    p2csasgn(tempset,256,set_make(e_n_d),0);
    p2csasgn(done,256,tempset,128);
****/
    fillchar(done, 34, 0 );
    if (err) {
        if ( ! ractive)
            return;
        err = false;
        find(entfile,0,eq);        /* RESTORE ORIGINAL RECORD */
        err = true;
    }
    else  {
        relate(entfile);
        if (entfile == sysint[entmfile])
            indicators[entquery] = true;
    }
}


backone ()
{
    itemp = sysint[sysreturn] + sysreturn;
    ltemp = sysint[itemp] - 1;
    sysint[itemp] = ltemp;
}


static xxent ()
{

    getargc(&crntag1,argint);
    entfile = sysint[entrfile];
    entfield = sysint[entrfield];
    if ((entfield < 0) || (entfile <= 0)  || !saveline)
        error(79);
}


cmdentergroup ()
{
int     saveline;
struct commandtype *withp0;

    estart = sysint[entstart];
    switch (crntcmd) {
        case cmdeupdate:
            unpkarg(&crntag1);
            entupdate(argfile,false);
            break;
        case cmdefind:
            xxent();
            entfind((int) valint,false,0);
            break;
        case cmdesfind:
            xxent();
            entfind((int) valint,true,(int) sysint[entmfile]);
            break;
        case cmdedisp:
            entdisp();
            break;
        case cmdeclear:
            entclear();
            break;
        case cmdebtrue:
            entbottem = true;
            break;
        case cmdebfalse:
            entbottem = false;
            break;
        case cmderclr:
            backone();
            break;
        case cmdeback:            /* GO BACK ONE FIELD */
            saveline = nextline-1;
            backone();
            do {
                ltemp = ltemp - 1;
                if ((ltemp < estart))
                    goto L101;
                nextline = ltemp;
                getline();
                getcommand();
                if (crntcmd == cmdreturn)
                    goto L101;
                ok = (tstbit(crntcmd,15)) && ( ! tstbit(crntcmd,noenter));
                /* ENTRY */           /* ~NOENTER */
                if (ok && (tstbit(crntcmd,skipfound)))
                    ok = (status(crntag1.variant.str1.pargfile) < 2);
                /* STATUS */
                ok = ok || (crntcmd == cmdformin);          /* FORMIN */
                if (ok) {
                    withp0 = &crntcommand;
                    indcthelp(withp0->indct1);           /* INDICATORS */
                    if (ok && more) {
                       indcthelp(withp0->indct2);
                       if (ok && more)
                           indcthelp(withp0->indct3);
                    }
                }
            } while (!(ok));
            sysint[itemp] = ltemp;
L101:
            nextline = saveline;
            getline();
            break;
    }    /* CASE */
}
