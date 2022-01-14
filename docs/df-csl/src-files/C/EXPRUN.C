#include <standard.h>
#define MINSTACK 200 /* minimum shorts on stack */

#ifdef NO_MATHERR
#include <setjmp.h>
#include <signal.h>
jmp_buf math_jmp;
static depth = 0,jmprtn = 0;
extern math_sig();
#endif

short *expptr; /* << main pointer to current place in expression */

extern void (*functionlist[])();

extern short stack[];
extern short *stackp;
extern short *stackend;

exprun( exprn )
short *exprn;
{
	/* Make exprun reentrant, save current pointer */
	short *savestack = stackp;
	short *saveptr   = expptr;

    /* check for runtime memory */
    if ((stackend-stackp) < MINSTACK) error(57);
    else {
	/* Make passed exxpression current */
	expptr = exprn;
		
#ifdef NO_MATHERR
        if (!depth++) {
	  signal(SIGFPE,math_sig);
	  jmprtn = setjmp(math_jmp);
	  }
	if (jmprtn) {
		stackp = savestack;
		depth = 1;
		}
	else { 
#endif
	/**************MAIN-LOOP*************************/
	/* For each command in the expression */
	while (*expptr) {
/******
		printf("{%d:%d}",*expptr, (stackp-stack));
*******/
		/* Execute the command in functionlist */
		(*functionlist[*(expptr++)])();			
		}
	/************************************************/
#ifdef NO_MATHERR
	}
	if (--depth) signal(SIGFPE,SIG_DFL);	
#endif
        if ((stackp!=savestack) && (!err) ) {
        	error(50);
		strcpy(errmsg,"STACK");
		}
	/* restore prior expression pointer */
	expptr = saveptr;
	stackp = savestack;
    }
}
	
