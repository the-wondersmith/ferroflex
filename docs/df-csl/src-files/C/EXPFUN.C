#include <math.h>
#include <stdlib.h>
#include <standard.h>
#include <flex2con.h>
#include <flex2def.h>
#include <flex2var.h>
#ifdef XENIX3
#include <signal.h>
#endif
#ifdef REAL_BUG
int rloffset;
#else
#define rloffset 1
#endif


#define STACKSIZE 2048 /* size in shorts */

extern short *expptr; /* Next location in expression */
extern int math_jmp[]; /* for math_sig */

typedef char anumber[NUMSIZE];

/* This is where the stack is allocated! */
short stack[STACKSIZE];
short *stackend = &stack[STACKSIZE-1];

union {
	short *s;
	long *l;
	double *r;
	struct argument *flxarg;
	anumber *n;
	unsigned char *c;
#ifndef EXP_INIT
	}stackp = stack;
#else
	}stackp;

exp_init()
{
	stackp.s = stack;
#ifdef XENIX3
	signal( SIGFPE, SIG_IGN );
	signal( SIGEMT, SIG_IGN );
#endif
#ifdef REAL_BUG
	rloffset = 1;
#endif
}
#endif

/*********debug stuf!
nchk()
{
	register unsigned char *cp =stackp.c-1;
	if ((*cp!=0) && (*cp!=0xff))
		printf("\nSign error on stack %d\n",*cp);
	}
**********/
void no()
{
	error(50); /** call to invalid function, should not happen ! */
	ltoa(*(expptr-1),errmsg,10);
/*** take out after debug ******
	printf("\n Invalid function call In Expression #%d\n",*(expptr-1));
**/
}
void nothing(){}

void invalid(){
	error(56); /** expression did not compile */
	}

#ifndef NO_MATHERR
matherr(excep)
register struct exception *excep;
{
	excep->retval = 0.0;  	/* I like to return 0's */
	error(59);		/* give the error to DF */
	strcpy(errmsg,excep->name);
	return(59);		/* Return "I took care of it" */
	}	
#else
math_sig()
{
	error(59);
	valint = 0;
	valreal = 0.0;
	fillchar( valnum, 0, NUMSIZE );
	*valstr = 0;
	argtype = argnum;
	longjmp(math_jmp,59);
	}
#endif

/******************** CONSTANTS *******************/
void connum(){
	*(stackp.s++) = *(expptr++);
	*(stackp.s++) = *(expptr++);
	*(stackp.s++) = *(expptr++);
	*(stackp.s++) = *(expptr++);
	*(stackp.s++) = *(expptr++);
#if NUMSIZE==12
	*(stackp.s++) = *(expptr++);
#endif

	}
void conint(){
	*(stackp.s++) = *(expptr++);
	*(stackp.s++) = *(expptr++);
	}
void conreal(){					
	*(stackp.s++) = *(expptr++);
	*(stackp.s++) = *(expptr++);
	*(stackp.s++) = *(expptr++);
	*(stackp.s++) = *(expptr++);
	}
/***************************FLEX ARGUMENTS**********************/

void flxint(){
	getargc( (struct argument *)expptr, argint );
	expptr += 2;
	*(stackp.l)++ = valint;
	}
void flxnum(){
	getargc( (struct argument *)expptr, argnum );
	expptr += 2;
	move( valnum, stackp.n++, NUMSIZE);

	}
void flxrl(){
	getargc( (struct argument *)expptr, argreal );
	expptr += 2;
	*(stackp.r)++ = valreal;
	}
void flxstr(){
	getargc( (struct argument *)expptr, argstr );
	expptr += 2;
	/* strings are stored in valstr, not stacked */
	}

			/**** variables ***/
void vrstr(){
        vallen = (argspace[*expptr-1] & 0xff);
	move(&argspace[*(expptr++)], valstr, vallen+1);
 /*	strcpy( valstr, &argspace[*(expptr++)] ); */
	}
void vrint(){
	*(stackp.l++) = sysint[*(expptr++)];
	}
void vrnum(){
	move( &argspace[*(expptr++)], stackp.n++, NUMSIZE);

	}
void vrrl(){
/*	*(stackp.r++) = (double)*argspace[*(expptr++)]; */
	move( &argspace[*(expptr++)], stackp.n, sizeof(double));
	stackp.r++;
	}

			/***** FIELDS *****/
void dbstr(){
	register int fil,fld;
	fil = *(expptr++);
	fld = *(expptr++);
	vallen = sget( fil,fld,valstr);
	}
void dbnum(){
	register int fil,fld;
	fil = *(expptr++);
	fld = *(expptr++);
	nget( fil,fld,stackp.n++);
	}

			/******COMPILED EXPRESSIONS ****/

/* note: these assume result from exp is still on the stack! */

void eprstr(){
	exprun( &argspace[*(expptr++)] );
	}
void eprint(){
	exprun( &argspace[*(expptr++)] );
	stackp.l++;
	}
void eprnum(){
	exprun( &argspace[*(expptr++)] );
	stackp.n++;
	}
void eprrl(){
	exprun( &argspace[*(expptr++)] );
	stackp.r++;
	}
			
/***************************CONVERSION FUNCTIONS****************/

			/* from ints */
void xintrl(){
	stackp.l--;
	*stackp.r = *stackp.l;
	stackp.r++;
	}
void xintnum(){
	stackp.l--;
	cvln( *stackp.l, stackp.n);
	stackp.n++;

	}
void xintstr(){
	cvls( *(--stackp.l), valstr);
   getlen();
	}

			/* from real */

void xrlint(){
	register long tmp;
	stackp.r--;
#ifdef __TURBOC__
	if (*stackp.r<0.0) {
		tmp = -*stackp.r;
		tmp = -tmp;
		}
	else tmp = *stackp.r;
#else
#ifdef INTEGER_ROUND
	tmp = (long)(*stackp.r * INTEGER_ROUND);
#else
	tmp = *stackp.r;
#endif
#endif
	*(stackp.l++) = tmp;
	}
void xrlnum(){
	stackp.r--;
	cvfn( *stackp.r, stackp.n);
	stackp.n++;
	}
void xrlstr(){
	stackp.r--;
#ifdef USE_GCVT
	gcvt( *stackp.r, USE_GCVT, valstr);
#else
	sprintf( valstr,"%lg", *stackp.r);
#endif
   getlen();
	}

			/* from string */

void xstrint(){
	expeval( valstr, argint);
	*(stackp.l++) = valint;
        }
void xstrrl(){
	expeval( valstr, argreal);
	*(stackp.r++) = valreal;
	}
void xstrnum(){
	expeval( valstr, argnum );
	move( valnum, *(stackp.n++), NUMSIZE);
	}
void xstrdat(){
	cvdn(valstr, stackp.n++);
	}

			/* from number */
void xnumint(){

	stackp.n--;
	*stackp.l = cvnl( stackp.n);
	stackp.l++;
	}
void xnumrl(){
	stackp.n--;
	*stackp.r = (double)cvnf( stackp.n );
	stackp.r++;
	}
void xnumstr(){
	stackp.n--;
	move( stackp.n, valnum, NUMSIZE);
	cvtnum();
	}

void xdatstr(){
	stackp.n--;
	cvnd( stackp.n, valstr);
   getlen();
	}
		/******** MISC FUNCTIONS ********/

void rndrl(){
	stackp.r--;
#ifdef __TURBOC__
	if (*stackp.r>0)	*stackp.l = *stackp.r + 0.5;
	else {
		register long tmp = -(*stackp.r - 0.5);
		*stackp.l = -tmp;
		}
#else
	*stackp.l = (long)((double)(*stackp.r + (double)((*stackp.r>0.0)?0.5:-0.5)));
#endif
	stackp.l++;
	}
void lnstr(){
	*(stackp.l++) = vallen;
	}

/***********************BINARY INTEGER FUNCTIONS****************/

void addint(){
	stackp.l--;
	*(stackp.l-1) += *stackp.l;
	}
void subint(){
	stackp.l--;
	*(stackp.l-1) -= *stackp.l;
	}
void mulint(){
	stackp.l--;
	*(stackp.l-1) *= *stackp.l;
	}
void divint(){
	stackp.l--;
	if (*stackp.l != 0) *(stackp.l-1) /= *stackp.l;
	else		   *(stackp.l-1) = 0;
	}
void andint(){
	stackp.l--;
	*(stackp.l-1) &= *stackp.l;
	}
void orint(){
	stackp.l--;
	*(stackp.l-1) |= *stackp.l;
	}


/*******
void rdvint(){
	register long *des;
	stackp.l--;
	des = stackp.l-1;
	if (*des != 0) *des = *stackp.l / *des;
	}
**********/
void ltint(){
	register long *des;
	stackp.l--;
	des = stackp.l-1;
	if (*des>*stackp.l) *des = *stackp.l;
	}
void gtint(){
	register long *des;
	stackp.l--;
	des = stackp.l-1;
	if (*des<*stackp.l) *des = *stackp.l;
	}
	

/************************BINARY NUMBER FUNCTIONS*****************/


void addnum(){
	register anumber *des;
	stackp.n--;
	des = stackp.n-1;
	bcd_add(des,stackp.n,des);

	}
void subnum(){
	register anumber *des;
	stackp.n--;
	des = stackp.n-1;
	bcd_sub(des,stackp.n,des);
	}
void mulnum(){
	register anumber *des;
	stackp.n--;
	des = stackp.n-1;
	bcd_mult(des,stackp.n,des);
	}
void divnum(){
	register anumber *des;
	stackp.n--;
	des = stackp.n-1;
	bcd_div(des,stackp.n,des);
	}
/***********
void rdvnum(){
	register char *des;
	stackp.n--;
	des = stackp.n-1;
	div(stackp.n,des,des);
	}
***********/
void ltnum(){
	register anumber *des;
	stackp.n--;
	des = stackp.n-1;
	if (ncmp(stackp.n,des)==numlt)
		move(stackp.n,des,NUMSIZE);
	}
void gtnum(){
	register anumber *des;
	stackp.n--;
	des = stackp.n-1;
	if (ncmp(stackp.n,des)==numgt)
		move(stackp.n,des,NUMSIZE);
	}


/************************BINARY REAL FUNCTIONS*******************/

void addrl(){
	stackp.r--;
	*(stackp.r-rloffset) += *stackp.r;
	}
void subrl(){
	stackp.r--;
	*(stackp.r-rloffset) -= *stackp.r;
	}
void mulrl(){
	stackp.r--;
	*(stackp.r-rloffset) *= *stackp.r;
	}
void divrl(){
	stackp.r--;
	if (*stackp.r != 0.0) *(stackp.r-rloffset) /= *stackp.r;
	else 		      *(stackp.r-rloffset) = 0.0;
	}
/*****************
void rdvrl(){
	register double *des;
	stackp.r--;
	des = stackp.r-rloffset;
	if (*des != 0.0) *des = *stackp.r / *des;
	}
*****************/
/************
void perrl(){
	register double *des;
	stackp.r--;
	des = stackp.r-rloffset;
	if (*stackp.r != 0.0) *des = ((*des) * 100.00)/(*stackp.r);
	else *des = 0.0;
	}
*************/
void ltrl(){
	register double *des;
	stackp.r--;
	des = stackp.r-rloffset;
	if (*des>*stackp.r) *des = *stackp.r;
	}
void gtrl(){
	register double *des;
	stackp.r--;
	des = stackp.r-rloffset;
	if (*des<*stackp.r) *des = *stackp.r;
	}
void powrl(){
	register double *des;
#ifdef  NEG_POW_BUG
	double intpart;
#endif
	stackp.r--;
	des = stackp.r-rloffset;
#ifdef NEG_POW_BUG
	if (*des<0.0) {
		*des = pow( -(*des), *stackp.r );
		if (modf(*stackp.r,&intpart)!=0.0) error(59);
		if (((long) intpart) & 1) *des = -(*des);
		}
	else
#endif	
	*des = pow( *des, *stackp.r );	
	}

/**************************UNARY MINUS FUNCTIONS **********************/
void umnint(){
	*(stackp.l-1) = -*(stackp.l-1);
	}
void umnnum(){
	neg( stackp.n-1 );
	}
void umnrl(){
	*(stackp.r-rloffset) = -((double) *(stackp.r-rloffset));
	}
/**************************ABSOLUTE VALUE FUNCTIONS **********************/
void absint(){
#ifdef __TURBOC__
        register long *tp = stackp.l-1;
        if (*tp<0l) *tp = -*tp;
#else
	*(stackp.l-1) = labs(*(stackp.l-1));
#endif
	}
void absnum(){
	if (*(stackp.c-1)) neg( stackp.n-1 );
	}
void absrl(){
	*(stackp.r-rloffset) = fabs(*(stackp.r-rloffset));
	}
/**************************UNARY REAL FUNCTIONS ***********************/
void logrl(){
	register double *des = stackp.r-rloffset;
	if (*des<=0.0) *des = 0.0;
	else *des = log(*des);
	}
void exprl(){
	*(stackp.r-rloffset) = exp(*(stackp.r-rloffset));
	}
void sinrl(){
	*(stackp.r-rloffset) = sin(*(stackp.r-rloffset));
	}
void asinrl(){
	*(stackp.r-rloffset) = asin(*(stackp.r-rloffset));
	}
void cosrl(){
	*(stackp.r-rloffset) = cos(*(stackp.r-rloffset));
	}
void acosrl(){
	*(stackp.r-rloffset) = acos(*(stackp.r-rloffset));
	}
void tanrl(){
	*(stackp.r-rloffset) = tan(*(stackp.r-rloffset));
	}
void atanrl(){
	*(stackp.r-rloffset) = atan(*(stackp.r-rloffset));
	}
void sqrtrl(){
	register double *tmp = stackp.r-rloffset;
	*tmp = sqrt(fabs(*tmp));
	}

/************************* OUTPUT FUNCTIONS **********************/

void outint(){
	valint = *(--stackp.l);
	argtype = argint;
	}
void outnum(){
	argtype = argnum;
	move(--stackp.n,valnum,NUMSIZE);
	}

void outrl(){
	valreal = *(--stackp.r);
	argtype = argreal;
	}
void outstr(){
	argtype = argstr;
/* valstr is already there! */
	}
/****************************THE FUNCTION LIST*************************/

void (*functionlist[])() = {

/*			string	number	date	int	xxx	real	0 0  */
/*00 null            */	no,	no,	no,	no,	no,	no,	no,no,
/*01 FNA_CONSTANT    */	no,	connum,connum, conint, no,	conreal,no,no,
/*02 FNA_FIELD       */	dbstr,	dbnum,	dbnum,	no,	no,	no,	no,no,
/*03 null            */	no,	no,	no,	no,	no,	no,	no,no,
/*04 null            */	no,	no,	no,	no,	no,	no,	no,no,
/*05 FNA_FLEXARG     */	flxstr,	flxnum,	flxnum,	flxint,	no,	flxrl,	no,no,
/*06 FNA_VARIABLE    */	vrstr,	vrnum,	vrnum,	vrint,	no,	vrrl,	no,no,
/*07 FNA_EXPRESSION  */	eprstr,	eprnum,	eprnum,	eprint,	no,	eprrl,	no,no,
/*08 FN_ADD          */	no,     addnum, addnum, addint,	no,	addrl,  no,no,
/*09 FN_SUBTRACT     */	no,	subnum, subnum, subint,	no,	subrl,	no,no,
/*10 FN_TIMES        */	invalid,mulnum,	mulnum,	mulint,	no,	mulrl,	no,no,
/*11 FN_DIVIDE       */ no,	divnum,	divnum,	divint,	no,	divrl,  no,no,
/*12 FN_AND          */	no,	no,	no,	andint, no,     no,	no,no,
/*13 FN_OR           */	no,	no,	no,	orint,	no,	no,	no,no,
/*14 FN_LT           */	no,	ltnum,	ltnum,	ltint,	no,	ltrl,	no,no,
/*15 FN_GT           */	no,	gtnum,	gtnum,	gtint,	no,	gtrl,	no,no,
/*16 FN_UMINUS       */	no,	umnnum,	umnnum,	umnint,	no,	umnrl,	no,no,
/*17 null            */	no,	no,	no,	no,     no,     no,	no,no,
/*18 null            */	no,	no,	no,	no,     no,     no,	no,no,
/*19 null            */	no,	no,	no,	no,	no,	no,	no,no,
/*20 null            */	no,	no,	no,	no,	no,	no,	no,no,
/*21 null            */	no,	no,	no,	no,	no,	no,	no,no,
/*22 null            */	no,	no,	no,	no,	no,	no,	no,no,
/*23 FN_ABS          */	no,	absnum, absnum,	absint, no,	absrl,  no,no,
/*24 FN_STRING       */	no,	xnumstr,xdatstr,xintstr,no,     xrlstr, no,no,
/*25 FN_DATE         */	xstrdat,no,	no,	xintnum,no,	xrlnum,	no,no,
/*26 FN_INTEGER      */	xstrint,xnumint,xnumint,no,	no,	xrlint,	no,no,
/*27 FN_NUMBER       */	xstrnum,no,	no,	xintnum,no,	xrlnum,	no,no,
/*28 FN_REAL         */	xstrrl,	xnumrl,	xnumrl,	xintrl,	no,	no,	no,no,
/*29 FN_ROUND        */	no,	no,	no,	no,	no,	rndrl,	no,no,
/*30 FN_LENGTH       */	lnstr,	no,	no,	no,	no,	no,	no,no,
/*31 FN_OUTPUT       */ outstr, outnum, outnum, outint, no,     outrl,  no,no,
/*			string	number	date	int	xxx	real	0 0  */

/* Single type functions */

/*32 - 256 FN_LOG          */	logrl,
/*33 - 257 FN_EXP          */	exprl,
/*34 - 258 FN_SIN          */	sinrl,
/*35 - 259 FN_ASIN         */	asinrl,
/*36 - 260 FN_COS          */	cosrl,
/*37 - 261 FN_ACOS         */	acosrl,
/*38 - 262 FN_TAN          */	tanrl,
/*39 - 263 FN_ATAN         */	atanrl,
/*40 - 264 FN_SQRT         */	sqrtrl,
/*41 - 265 FN_POWER	   */   powrl,
no,no,no,no,no,no,no,no,no,no,no};
	
