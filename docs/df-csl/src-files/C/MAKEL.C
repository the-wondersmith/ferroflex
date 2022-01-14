#include <standard.h>
#include <flex2con.h>
#include <flex2def.h>
#include <flex2var.h>
#include <screencd.h>
#include <sfcbtype.h>

extern char okcodes[];
bool 	 graphics,
	 forms,
	 allok,
	 show_prompts;
char	 linkfile[STRING],
	 root[STRING],
	 batchfile[STRING],
	 modules[STRING],
	 exefile[STRING],
	 tmpstr[LSTRING];

main (argc,argv)
int argc;
char *argv[];
{
char 	baselink[STRING],
	oneline[LSTRING];
struct sfcbtype inlnk, outlnk, inbat, outbat;
int  	key,result;

	dsetup();
	ssetup();

	p2csasgn(okcodes,256,set_make(CKRETURN,CKRIGHT,CKCREATE,
        	CKCLEAR,CKDOWNARROW,
		CKPRINT,CKLFIELD,CKDELETE,CKUP,CKHELP,
		CKESC,CKFIND,CKCALCULATE,CKDOWN,CKUSER2,
		CKLEFT,CKSFIND,CKUSER,CKUPARROW,CKINSERT,
		CKRUBOUT,CKBACK,e_n_d),0);


	clearscreen();
	setcolor(scdim);
	box(0,0,23,78);
	setcolor(scbright);

#ifdef MSDOS
	ctrwrite(1,"Example program, CS&L Link and Batch file creator");
	ctrwrite(17,
	    "This CS&L example program does not utilize forms I/O or graphics");
#endif
#ifdef VMS
	ctrwrite(1,"Example program, CS&L Link and Command-file creator");
#endif
#ifdef UNIX
	ctrwrite(1,"Example program, CS&L Link file and Link script creator");
#endif

	ctrwrite(24,"Press the ESCape key to cancel program");
	setcolor(scdim);
	insidebox(2,0,2,78);
	insidebox(18,0,18,78);

	fillchar(linkfile,sizeof(linkfile),0);
	fillchar(modules,sizeof(modules),0);
	fillchar(exefile,sizeof(exefile),0);
	fillchar(root,sizeof(root),0);
	fillchar(batchfile,sizeof(batchfile),0);

	if (argc > 1) sprintf(linkfile,"%s.lnk",argv[1]);
	show_prompts = TRUE;
	allok = FALSE;
	forms = FALSE;
	graphics = FALSE;

	do {
		key = link_file(4,5);		if (key == KESC) chnflx();

		trim_spaces(linkfile,sizeof(linkfile)-1);
		if (show_prompts) setroot();

		key = exe_file(5,5);		if (key == KESC) chnflx();
#ifdef MSDOS
		key = gr_yesno(7,5);		if (key == KESC) chnflx();
#endif
		key = forms_yesno(8,5);		if (key == KESC) chnflx();
		key = get_modules(10,5);	if (key == KESC) chnflx();
#ifdef MSDOS
		key = get_batch(11,5);		if (key == KESC) chnflx();
#endif

		key = is_correct(13,5);		if (key == KESC) chnflx();
		show_prompts = FALSE;
	} while (!allok);

#ifndef MSDOS
	graphics = FALSE;
#endif

	setcolor(scbright);
	if (strlen(linkfile) == 0) {
		writexy("Error, No link file specified",20,5);
		press();
		chnflx();
	}

	if (forms)
		if (graphics) strcpy(baselink,"gf.lnk");
		else
#ifdef MSDOS
		     strcpy(baselink,"nogf.lnk");
#endif
#ifdef UNIX
		     strcpy(baselink,"/usr/bin/makel.fm");
#endif
#ifdef VMS
		     strcpy(baselink,"sys$system:makel.fm");
#endif

	else
		if (graphics) strcpy(baselink,"gnof.lnk");
		else
#ifdef MSDOS
		     strcpy(baselink,"nognof.lnk");
#endif
#ifdef UNIX
		     strcpy(baselink,"/usr/bin/makel.nfm");
#endif
#ifdef VMS
		     strcpy(baselink,"sys$system:makel.nfm");
#endif

	seqopen(&inlnk,baselink,FALSE);
	if (seqendfile) {
		sprintf(tmpstr,
			"Error, could not open prototype file %s",baselink);
		writexy(tmpstr,20,5);
		press();
		chnflx();
	}

	result = check_exist(linkfile,20,5);
	if (result == -1) writestr("Link file not created");
	else {
		if (result == 1) writestr("Link file overwritten");
		seqopen(&outlnk,linkfile,TRUE);
		trim_spaces(modules,sizeof(modules)-1);
		trim_spaces(exefile,sizeof(exefile)-1);

		do {
			seqrline(&inlnk,oneline);
			if (!seqendfile) {
				if (oneline[0] == '*') {
					if (strcmp(oneline,"*USER*")==0) {
#ifdef MSDOS
						sprintf(tmpstr,"%s +",modules);
#endif
#ifdef UNIX
						sprintf(tmpstr,"%s \\",modules);
#endif
#ifdef VMS
						sprintf(tmpstr,"%s -",modules);
#endif
						seqwline(&outlnk,tmpstr);
					}
					else if (strcmp(oneline,"*EXENAME*")==0)
					 {
#ifdef MSDOS
						sprintf(tmpstr,"%s +",exefile);
#endif
#ifdef UNIX
						sprintf(tmpstr,"%s \\",exefile);
#endif
#ifdef VMS
						sprintf(tmpstr,"%s",exefile);
#endif
						seqwline(&outlnk,tmpstr);
					 }
				}
				else {
					seqwline(&outlnk,oneline);
				}
			}
		} while (!seqendfile);
		seqclose(&outlnk);
#ifdef UNIX
		chmod(linkfile,00777);
#endif
	}
	seqclose(&inlnk);

#ifdef MSDOS
	trim_spaces(batchfile,sizeof(batchfile)-1);
	result = check_exist(batchfile,21,5);
	if (result != -1) {
#ifdef MSDOS
		if (result == 1) writestr(" Batch file overwritten");
		seqopen(&inbat,"batlnk.txt",FALSE);
#endif
#ifdef UNIX
		writestr(" Script overwritten");
		seqopen(&inbat,"/usr/bin/makel.cc",FALSE);
#endif
#ifdef VMS
		writestr(" Command file overwritten");
		seqopen(&inbat,"sys$system:makel.cc",FALSE);
#endif
		if (!seqendfile) {
			seqopen(&outbat,batchfile,TRUE);

			seqrline(&inbat,oneline);
			sprintf(tmpstr,"%s%s",oneline,linkfile);
			seqwline(&outbat,tmpstr);

			seqclose(&outbat);
#ifdef UNIX
			chmod(batchfile,00777);
#endif
		}
		else
#ifdef MSDOS
			writexy("Cannot find batch file prototype",21,5);
#endif
#ifdef UNIX
			writexy("Cannot find script file prototype",21,5);
#endif
#ifdef VMS
			writexy("Cannot find command file prototype",21,5);
#endif
		seqclose(&inbat);
	}
	else
#ifdef MSDOS
		writels("Batch file not created");
#endif
#ifdef UNIX
		writels("Link script not created");
#endif
#ifdef VMS
		writels("Command file not created");
#endif

#endif


	press();
	chnflx();
}

link_file (row,column)
int row,column;
{
char prompt[STRING];
int key,savecolor,
	offset;

	savecolor = crntcolor;
	strcpy(prompt,"Output link file: ");
	offset = strlen(prompt);

	if (show_prompts) {
		setcolor(scdim);
		display(row,column,prompt,offset);
		setcolor(scbright);
		display(row,column+offset+1,linkfile,40);
	}

	setcolor(scbright);
	accept(row,column+offset+1,linkfile,40,&key);
	display(row,column+offset+1,linkfile,40);

	setcolor(savecolor);
	return(key);
}

#ifdef MSDOS
gr_yesno (row,column)
int row,column;
{
char prompt[STRING];
int key,savecolor,
	offset;
bool savecaps;

	savecolor = crntcolor;
	strcpy(prompt,"Graphics:         ");
	offset = strlen(prompt);

	if (show_prompts) {
		setcolor(scdim);
		display(row,column,prompt,offset);
	}

	savecaps = capslock;
	capslock = TRUE;
	if (graphics) strcpy(tmpstr,"Y  ");
	else strcpy(tmpstr,"N  ");

	setcolor(scbright);	
	display(row,column+offset+1,tmpstr,3);
	accept(row,column+offset+1,tmpstr,3,&key);
	display(row,column+offset+1,tmpstr,3);
	capslock = savecaps;

	if (tmpstr[0] == 'Y') graphics = TRUE;
	else graphics = FALSE;

	setcolor(savecolor);
	return(key);
}
#endif

forms_yesno (row,column)
int row,column;
{
char prompt[STRING];
int key,savecolor,
	offset;
bool savecaps;

	savecolor=crntcolor;
	strcpy(prompt,"Forms support:    ");
	offset = strlen(prompt);

	if (show_prompts) {
		setcolor(scdim);
		display(row,column,prompt,offset);
	}

	savecaps = capslock;
	capslock = TRUE;
	if (forms) strcpy(tmpstr,"Y  ");
	else strcpy(tmpstr,"N  ");

	setcolor(scbright);
	display(row,column+offset+1,tmpstr,3);
	accept(row,column+offset+1,tmpstr,3,&key);
	display(row,column+offset+1,tmpstr,3);
	capslock = savecaps;

	if (tmpstr[0] == 'N') forms = FALSE;
	else forms = TRUE;

	setcolor(savecolor);
	return(key);
}

get_modules (row,column)
int row,column;
{
char prompt[STRING];
int key,savecolor,
	offset;
bool savecaps;

	savecolor = crntcolor;
	strcpy(prompt,"Object Files:     ");
	offset = strlen(prompt);
	if (show_prompts) {
		setcolor(scdim);
		display(row,column,prompt,offset);

		setcolor(scbright);
#ifndef UNIX
		sprintf(modules,"%s.obj",root);
#else
		sprintf(modules,"%s.o",root);
#endif
		display(row,column+offset+1,modules,65-offset);
	}

	savecaps = capslock;
	capslock = FALSE;

	setcolor(scbright);
	accept(row,column+offset+1,modules,65-offset,&key);
	display(row,column+offset+1,modules,65-offset);
	capslock = savecaps;

	setcolor(savecolor);
	return(key);
}

is_correct (row,column)
int row,column;
{
char prompt[STRING];
int key,savecolor,
	offset;
bool savecaps;

	savecolor = crntcolor;
	fillchar(tmpstr,sizeof(tmpstr),0);
	strcpy(prompt,"Is the above correct: ");
	offset = strlen(prompt);

	if (show_prompts) {
		setcolor(scdim);
		display(row,column,prompt,offset);
	}

	strcpy(tmpstr,"Y  ");

	setcolor(scbright);
	display(row,column+offset+1,tmpstr,3);

	savecaps = capslock;
	capslock = TRUE;
	accept(row,column+offset+1,tmpstr,3,&key);
	display(row,column+offset+1,tmpstr,3);
	capslock = savecaps;

	if (tmpstr[0] == 'Y') allok = TRUE;
	else allok = FALSE;

	setcolor(savecolor);
	return(key);
}


exe_file (row,column)
int row,column;
{
char prompt[STRING];
int key,savecolor,
	offset;

	savecolor = crntcolor;
#ifndef UNIX
	strcpy(prompt,"EXE file name:    ");
#else
	strcpy(prompt,"Program name:     ");
#endif
	offset = strlen(prompt);

	if (show_prompts) {
		setcolor(scdim);
		display(row,column,prompt,offset);

		setcolor(scbright);
		strcpy(exefile,root);
		display(row,column+offset+1,exefile,8);
	}

	setcolor(scbright);
	accept(row,column+offset+1,exefile,8,&key);
	display(row,column+offset+1,exefile,8);

	setcolor(savecolor);
	return(key);
}

trim_spaces (string,stend)
char string[];
int stend;
{

	while ((stend >= 0) && (string[stend] <= ' ')) string[stend--] = 0;

}

setroot ()
{
char	node[STRING],device[STRING],
	path[STRING],fname[STRING],extn[STRING],
	ver[STRING];

	parsefn(linkfile,node,device,path,fname,extn,ver);
	strcpy(root, fname);

}

get_batch (row,column)
int row,column;
{
char prompt[STRING];
int key,savecolor,
	offset;

	savecolor = crntcolor;
#ifdef MSDOS
	strcpy(prompt,"Link batch file:  ");
#endif
#ifdef UNIX
	strcpy(prompt,"Script file for link:");
#endif
#ifdef VMS
	strcpy(prompt,"Command file for link:");
#endif
	offset = strlen(prompt);

	if (show_prompts) {
		setcolor(scdim);
		display(row,column,prompt,offset);

		setcolor(scbright);
#ifdef MSDOS
		sprintf(batchfile,"%s.bat",root);
#endif
#ifdef UNIX
		sprintf(batchfile,"%s.sh",root);
#endif
#ifdef VMS
		sprintf(batchfile,"%s.com",root);
#endif

		display(row,column+offset+1,batchfile,40);
	}

	setcolor(scbright);
	accept(row,column+offset+1,batchfile,40,&key);
	display(row,column+offset+1,batchfile,40);

	setcolor(savecolor);
	return(key);
}

check_exist (seqfile,xx,yy)
char seqfile[];
int xx,yy;
{
struct sfcbtype tmpfcb;
char tmps[STRING];
int key,savecolor;

	savecolor = crntcolor;
	seqopen(&tmpfcb,seqfile,FALSE);
	if (!seqendfile) {
		seqclose(&tmpfcb);

		gotoxy(xx,yy);
		setcolor(scdim);
		writestr("Overwrite existing file ");
		writestr(seqfile);
		writestr(" (Y/N): ");
		sprintf(tmpstr,"N%c",8);

		setcolor(scbright);
		writestr(tmpstr);

		readstr(tmps,1,&key);
		writestr("   ");

		tmps[0]=toupper(tmps[0]);		
		if (tmps[0] != 'Y') {
			return(-1);
		}
		return(1);
	}

	setcolor(savecolor);
	return(0);
}

#ifdef MSDOS
#define row 'Ä'
#define column '³'
#define cross 'Å'
#define rightcross '´'
#define leftcross 'Ã'
#define upleft 'Ú'
#define upright '¿'
#define lowleft 'À'
#define lowright 'Ù'
#endif

#ifndef MSDOS
#define row '-'
#define column '|'
#define cross ' '
#define rightcross '|'
#define leftcross '|'
#define upleft '.'
#define upright '.'
#define lowleft '`'
#define lowright '\''
#endif

box (ulx,uly,lrx,lry)
int ulx,uly,lrx,lry; /* 0 0 24 78 */
{
int 	tmp;
char 	tmpstr[LSTRING];

	fillchar(tmpstr,256,row);
	tmpstr[lry-uly]=0;

	gotoxy(ulx,uly);
	writestr(tmpstr);

	gotoxy(lrx,uly);
	writestr(tmpstr);

	for (tmp=ulx; tmp <= lrx-1; ++tmp) {
		gotoxy(tmp,uly);
		writecon(column);
		gotoxy(tmp,lry);
		writecon(column);
	}

	gotoxy(ulx,uly);
	writecon(upleft);
	gotoxy(ulx,lry);
	writecon(upright);
	gotoxy(lrx,uly);
	writecon(lowleft);
	gotoxy(lrx,lry);
	writecon(lowright);
}

insidebox (ulx,uly,lrx,lry)
int ulx,uly,lrx,lry;
{
char 	tmpstr[LSTRING];

	fillchar(tmpstr,256,row);
	tmpstr[lry-uly]=0;

	gotoxy(ulx,uly);
	writestr(tmpstr);

	gotoxy(ulx,uly);
	writecon(leftcross);
	gotoxy(lrx,lry);
	writecon(rightcross);
}

ctrwrite(xrow,str)
int xrow;
char str[];
{
int len;

	len = strlen(str);
	gotoxy(xrow,41 - (len/2));
	writestr(str);
}

static press()
{
int	presschar;

	
	ctrwrite(24,"         Press any key to exit program         ");
	presschar = readchar();	

	return(presschar);
}

writexy(str,x,y)
char str[];
char x,y;
{

	gotoxy(x,y);
	writestr(str);
	
}
