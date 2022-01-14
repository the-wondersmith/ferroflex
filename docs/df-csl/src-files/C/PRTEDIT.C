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
#include <ctype.h>

#define editzero 16
#define signrt   128
#define comma    36
#define fltdlr   64
#define ucase    3

int mask;
static char fillchr;

formeinit(){
    mask    = crntwinf.auxtype;
    fillchr = prtedit[(crntwinf.maskfill & 7)];
    }

formezero()
{
      valstr[0] = 0;
      vallen = 0;
      padstr(valstr,crntwlen);
   }

formen()
{   /* edit the number in valstr */
int right,minus;
register char *first, *chp;

if ((mask & (signrt|comma|fltdlr)) != 0) {

   /* alwase need to find the first character */
   first = valstr;
   while (*first==' ') first++;
   if (*first=='-') {minus = 1;first++;} else minus = 0;

   /* Note: First is maintained by all that follow */
   right = crntwlen;
   if (!*first) return;

   /* Sign right */
   if ((mask & signrt) && (first!=valstr)) {
         *(--first) = ' ';
         chp = first;
         while (*chp = *(chp+1)) chp++; /* move left */
         if (minus) *(chp++) = '-'; else *(chp++) = ' ';
         minus = 0;
         *chp = 0;
         right--;
         }
   /* Comma  */
   if (mask & comma) {
 	 register int cdown;
	 int ncomma;
         chp = strchr( valstr, '.');
         if (!chp) chp = &valstr[right];
	 else if (sc.apoint!='.')  *chp = sc.apoint;
         cdown =  (chp-first-1); /* digits left of point */
         ncomma = cdown / 3;
         cdown = cdown - (ncomma*3) + 1; /* mod */
         if ((ncomma+minus)<=(first-valstr)) { /* ok to insert */
            register char *frm;
            frm = first;
            chp = first - ncomma;
            first = chp;
            while (ncomma--) {
               while (cdown) {
                  *(chp++) = *(frm++);
                  cdown--;
                  }
               *(chp++) = sc.acomma;
               cdown = 3;
               }
            }
         } /* end comma */

   /* float$ */
   if (mask & fltdlr) { /* Move in multi-character currency */
	right = 0;
	if ((first-valstr-minus)>=sc.curlen) {
		first = first-sc.curlen;
      		while (right<sc.curlen) 
			{ first[right] = sc.currency[right]; right++; }
		}
	}
   if (minus) {
      if (first!=valstr) first--;
      *(first) = '-';
      }
   }
/* Fill character */
if (fillchr!=' ') {
   first = valstr;
   while (*first==' ') *(first++) = fillchr;
   if (*first=='-') {
      *first = fillchr;
      *valstr = '-';
      }
   }
/* euro-point */
if ((sc.apoint!='.') && !(mask & comma)) { /* CBC - 12/7/87 */
   chp = strchr(valstr,'.');
   if (chp) *chp = sc.apoint;
   }

}
   

formes ()
{
int     blen;
char    fillc;

    fillc = prtedit[crntwinf.maskfill & 7];
 /*    padstr(valstr,crntwlen);*/
    
    if (vallen>crntwlen) vallen = crntwlen;
    if (fillc != ' ')
            while ((vallen) && (valstr[vallen-1]==' ')) vallen--;
    blen = vallen -1;
    while (vallen<crntwlen) valstr[vallen++] = fillc;
    valstr[crntwlen] = 0;

    if (tstbit(crntwinf.auxtype,ucase))
        while (blen >= 0) {
            fillc = valstr[blen];
            if (islower(fillc))
                valstr[blen] = fillc & 0xdf;
            blen--;
        }
}
