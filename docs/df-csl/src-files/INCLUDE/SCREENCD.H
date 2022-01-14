/************************************************************************
Confidential Trade Secret.
Copyright (c) 1986 Data Access Corporation, Miami Florida,
as an unpublished work.  All rights reserved.
DataFlex is a registered trademark of Data Access Corporation.
************************************************************************/

#define maxli 128 /* size of state table */

#define scinit   0x8001
#define scbright 1
#define scdim    2
#define scexit   0x8002      /* not in record yet */

/* SC OPTION BITS */
extern long scoptions;
extern long worldoptions;
extern int  scstrtpos;
extern int  scexitpos;
#define CAPSLOCK   0x8
#define AUTORETURN 0x100
#define AUTOBACK   0x200
#define INSERTMODE 0x400
#define NOCALC     0x800
#define WORDWRAP   0x1000
#define INSMODE    0x10000
#define SOFTRETURN 0x20000
#define WREAL	   0x80000000

#define CKRETURN     1
#define CKPRINT      2
#define CKESC        3
#define CKLEFT       4
#define CKRIGHT      5
#define CKLFIELD     6
#define CKFIND       7
#define CKSFIND      8
#define CKCREATE     9
#define CKDELETE    10
#define CKCALCULATE 11
#define CKUSER      12
#define CKCLEAR     13
#define CKUP        14
#define CKDOWN      15
#define CKUPARROW   16
#define CKDOWNARROW 17
#define CKHELP      18
#define CKUSER2     19
#define CKINSERT    20
#define CKRUBOUT    21
#define CKBACK      22
#define CKHARDR     23
#define CKCLEOW     24

#define KRETURN    257
#define KPRINT     258
#define KESC       259
#define KLEFT      260
#define KRIGHT     261
#define KLFIELD    262
#define KFIND      263
#define KSFIND     264
#define KCREATE    265
#define KDELETE    266
#define KCALCULATE 267
#define KUSER      268
#define KCLEAR     269
#define KUP        270
#define KDOWN      271
#define KUPARROW   272
#define KDOWNARROW 273
#define KHELP      274
#define KUSER2     275
#define KINSERT    276
#define KRUBOUT    277
#define KBACK      278
#define KHARDR     279
#define KCLEOW     280

struct screencodes {

	/* protected system variables */
/*000*/      char     tname[50];  /*  */
/*050*/      char     ctrlz;
/*051*/      char     exprot1;
/*052*/      long     tnum;
/*056*/      long     bits;
/*060*/      long     morebits;
/*064*/      int16    prodclass;
/*066*/      int16    users;
/*068*/      char     cpuid[20];      
/*088*/      long     tcode;

    /* These 16 bytes are system variables */
/*092*/    char    curlen;        /* length of cuurency symbol */
/*093*/    char    currency[3];	/*  currency symbol */
/*096*/    byte    autoreturn; /*  AUTO RETURN ON FULL FIELD */
/*097*/    byte    multiuser;	/*  MULTIPLE USER OPTION */
/*098*/    byte    country;	/*  */
/*099*/    byte    altdrive;	/* LOCATION OF ALT FILELIST.CFG AND ALT DATA DR. */
			/*	BITS 0,1=DATE FMT {USA,EURO,MILITARY}
				BIT  2  =SWAP ',' AND '.' IN PRINT */
/*100*/    char    apoint;    /* decimal point character */
/*101*/    char    acomma;    /* big number separator */
/*102*/    byte    sysspecial;/* mikes special byte */
/*103*/    byte    anotherspare;
/*104*/    char    extrasystem[4];
 
	/* terminal variables */

/*108*/    char    kinit[20];	/* 0 INIT STRING */
/*128*/    char    kfback[8];	/* 20 DESTRUCTIVE BACK SPACE */
/*136*/    char    kleos[8];	/* 37 CLEAR END OF SCREEN */
/*144*/    char    kdim[12];	/* 45 LOW INT */

/*156*/    char    kbright[12]; /* 57 BRIGHT INT */
/*168*/    char    kfclear[12]; /* 69 CLEAR SCREEN */
/*180*/    char    kexit[20];   /*  screen exit code */

/*200*/    byte    kwidth;	/*  SCREEN WIDTH 0..X */
/*201*/    byte    klength;	/*  SCREEN LENGTH 0..X */
/*202*/    char    extra3;
/*203*/    char    kfill;      /*  70 fill char */

         /* Printer */
/*204*/    int16   prlen;      /* Printer Length */
/*206*/    int16   prwidth;    /* Printer Width */

         /* CURSER POS STUFF */
/*208*/    char    kcposx[8];	/*  CURSER POS FOR X */
/*216*/    char    kcposy[8];  /*  CURSER POS FOR Y */
/*224*/    char    kcpost[8];	/*  CURSOR POS TERMINATION CODE */
 
/*232*/    byte    kcaddx;	/*  COLUMN BIAS */
/*233*/    byte    kcaddy;	/*  FIELD ADDER */
/*234*/    byte    kcyx;	/*  Y BEFORE X ( ELSE X BEFORE Y) */
/*235*/    byte    outfunc;  /* for ms dos screen output mode */

   /* Graphics */
/*236*/      char grscreen[40] ; /* Graphics screen driver */
/*276*/      char groutput[40]  ; /* Printer/plotter driver */
/*316*/      int16 grmode;       /* Screen driver mode */
/*318*/      int16 grspare ;     /* Spare graphics bits */

    /* function key mapping */
#ifdef LATTICE
/*320*/    int16 mapped[16]; /* mapped input chrs set */
/*352*/*/    struct {
	int16 code;
	int16 makes;
	} litable[maxli]; /* leadin/translation state table */
#else
/*320*/    unsigned int16 mapped[16]; /* mapped input chrs set */
    struct {
/*352*/	unsigned int16 code;
	unsigned int16 makes;
	} litable[maxli]; /* leadin/translation state table */
#endif
};

#define scsize 1024
#define scblks  8 /* 1 k */

extern struct screencodes sc;




