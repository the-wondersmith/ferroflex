/*** Expression compile-and-go for runtime **/

#include <standard.h>
#include <flex2con.h>
#include <flex2def.h>
#include <flex2var.h>

extern short argtype;
extern long valint;
extern char valnum[NUMSIZE];
extern double valreal;


extern double atof();
extern long atol();
extern char *numparse();

extern union {
	short *s;
	long  *l;
	double *r;
	}stackp;
extern short *stackend;

static int putcnt ;

putshort( aval ) /* required for expgen */
int aval;
{
    if (stackp.s>=stackend) error(57);
    else {
	*(stackp.s++) = aval;
	putcnt++;
	}		
    }

/* runtime-get window subtotal */
getwsubt( wnum )
int wnum;
{
	/* window must already be a subtotal at runtime */
	if ((forminf[wnum].auxtype & 3) != 1) error(56);
	return( forminf[wnum].auxindex );
	}	


expeval(astring,type)
char *astring;
int type;
{
	char *bstr,*nstr;
	int dtype;
	bstr = astring;
	nstr = (char *)numparse( &bstr, &dtype);
	
	if (! *bstr)  {
		/* if number ends on a null it's not an expr */
	    if (type<0) argtype = dtype; else argtype = type;
/*
printf("\ncvt <%s> frm %d to %d\n",nstr,dtype,argtype);
*/
	    if (dtype==argreal) {
		valreal = atof(nstr);
		if (argtype!=argreal) cvtarg(argreal,argtype);
		}
	    else if (dtype==argdate) {
		cvdn(nstr,valnum);
		if (argtype!=argdate) cvtarg(argnum,argtype);		
		}
	    else {
		switch (argtype) {
	case argint:
			valint = atol(nstr);
			break;
	case argnum:
			cvsn(nstr,valnum);
			break;
	case argreal:
			valreal = atof(nstr);
			break;
			}
		}
	    }
	else {
		 /* compile and run expression */

		short *savestack = stackp.s;
		putcnt = 0;
/*
		printf("Compile and run <%s>",astring);
*/	
		/* compile the expression to the stack */
		expcompile(astring,type);

		if (putcnt & 1) putshort(0); /* allign stack! */

		/* run the compiled expression */
		if (!err) exprun(savestack);

		/* restore the stack */
		stackp.s = savestack;
		}
	}	
