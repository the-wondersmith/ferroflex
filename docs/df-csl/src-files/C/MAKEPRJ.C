#include <standard.h>
#include <flex2con.h>
#include <flex2def.h>
#include <flex2var.h>
#include <screencd.h>
#include <sfcbtype.h>

extern char okcodes[];
bool graphics,
	 forms,
	 allok,
	 show_prompts;
char prjfile[41],
	 root[10],
	 modules[81],
	 tmpstr[80];

main (argc,argv)
int argc;
char *argv[];
{
char baseprj[40],
	oneline[255];
struct sfcbtype inprj, outprj;
int  key;

	dsetup();
	ssetup();
    p2csasgn(okcodes,256,set_make(CKRETURN,CKRIGHT,CKCREATE,
        CKCLEAR,CKDOWNARROW,
		CKPRINT,CKLFIELD,CKDELETE,CKUP,CKHELP,
		CKESC,CKFIND,CKCALCULATE,CKDOWN,CKUSER2,
		CKLEFT,CKSFIND,CKUSER,CKUPARROW,CKINSERT,
		CKRUBOUT,CKBACK,e_n_d),0);

	clearscreen();

	writestr("CS&L Project file (.prj) creator");

	fillchar(prjfile,sizeof(prjfile),0);
	fillchar(modules,sizeof(modules),0);
	fillchar(root,sizeof(root),0);

	if (argc > 1) sprintf(prjfile,"%s.prj",argv[1]);
	show_prompts = TRUE;
	allok = FALSE;
	forms = FALSE;
	graphics = FALSE;

	do {
		prj_file(3,0);
		trim_spaces(prjfile,sizeof(prjfile)-1);
		if (show_prompts) setroot();
		gr_yesno(5,0);
		forms_yesno(6,0);
		get_modules(7,0);
		is_correct(10,0);
		show_prompts = FALSE;
	} while (!allok);

	if (forms)
		if (graphics) strcpy(baseprj,"gf.prj");
		else strcpy(baseprj,"nogf.prj");
	else
		if (graphics) strcpy(baseprj,"gnof.prj");
		else strcpy(baseprj,"nognof.prj");

	seqopen(&inprj,baseprj,FALSE);
	if (seqendfile) {
		writeeol();
		writestr("Error: could not open file ");
		writels(baseprj);
		press();
		chnflx();
	}

	seqopen(&outprj,prjfile,FALSE);
	if (!seqendfile) {
		writeeol();
		writestr("Overwrite existing .prj file ");
		writestr(prjfile);
		writestr("?");
		readstr(tmpstr,1,&key);
		tmpstr[0]=toupper(tmpstr[0]);
		if (tmpstr[0] != 'Y') {
			writeeol();
			writestr("Link file not created");
			press();
			chnflx();
		}
	}
	seqclose(&outprj);

	seqopen(&outprj,prjfile,TRUE);
	trim_spaces(modules,sizeof(modules)-1);

	do {
		seqrline(&inprj,oneline);
		if (!seqendfile) {
			if (oneline[0] == '*') {
				if (strcmp(oneline,"*USER*")==0)
					seqwline(&outprj,modules);
			}
			else {
				seqwline(&outprj,oneline);
			}
		}
	} while (!seqendfile);

	seqclose(&inprj);
	seqclose(&outprj);

	writeeol();
	press();
	chnflx();
}

prj_file (row,column)
int row,column;
{
char prompt[50];
int key,
	offset;

	strcpy(prompt,"Output project file: ");
	offset = strlen(prompt);

	if (show_prompts) {
		display(row,column,prompt,offset);
		display(row,column+offset+1,prjfile,40);
	}

	accept(row,column+offset+1,prjfile,40,&key);
	display(row,column+offset+1,prjfile,40);
}

gr_yesno (row,column)
int row,column;
{
char prompt[50];
int key,
	offset;
bool savecaps;

	strcpy(prompt,"Graphics: ");
	offset = strlen(prompt);

	if (show_prompts)
		display(row,column,prompt,offset);

	savecaps = capslock;
	capslock = TRUE;
	if (graphics) strcpy(tmpstr,"Y  ");
	else strcpy(tmpstr,"N  ");
	
	display(row,column+offset+1,tmpstr,3);
	accept(row,column+offset+1,tmpstr,3,&key);
	display(row,column+offset+1,tmpstr,3);
	capslock = savecaps;

	if (tmpstr[0] == 'Y') graphics = TRUE;
	else graphics = FALSE;
}

forms_yesno (row,column)
int row,column;
{
char prompt[50];
int key,
	offset;
bool savecaps;

	strcpy(prompt,"Forms support: ");
	offset = strlen(prompt);

	if (show_prompts)
		display(row,column,prompt,offset);

	savecaps = capslock;
	capslock = TRUE;
	if (forms) strcpy(tmpstr,"Y  ");
	else strcpy(tmpstr,"N  ");
	display(row,column+offset+1,tmpstr,3);
	accept(row,column+offset+1,tmpstr,3,&key);
	display(row,column+offset+1,tmpstr,3);
	capslock = savecaps;

	if (tmpstr[0] == 'N') forms = FALSE;
	else forms = TRUE;
}

get_modules (row,column)
int row,column;
{
char prompt[25];
int key,
	offset;
bool savecaps;

	strcpy(prompt,"Obj files:");
	offset = strlen(prompt);
	if (show_prompts) {
		display(row,column,prompt,offset);
		sprintf(modules,"%s.obj",root);
		display(row,column+offset+1,modules,75-offset);
	}

	savecaps = capslock;
	capslock = FALSE;
	accept(row,column+offset+1,modules,75-offset,&key);
	display(row,column+offset+1,modules,75-offset);
	capslock = savecaps;

}

is_correct (row,column)
int row,column;
{
char prompt[50];
int key,
	offset;
bool savecaps;

	fillchar(tmpstr,sizeof(tmpstr),0);
	strcpy(prompt,"Is the above correct: ");
	offset = strlen(prompt);

	strcpy(tmpstr,"Y  ");
	if (show_prompts) {
		display(row,column,prompt,offset);
		display(row,column+offset+1,tmpstr,3);
	}

	savecaps = capslock;
	capslock = TRUE;
	accept(row,column+offset+1,tmpstr,3,&key);
	display(row,column+offset+1,tmpstr,3);
	capslock = savecaps;

	if (tmpstr[0] == 'Y') allok = TRUE;
	else allok = FALSE;

}

trim_spaces (string,stend)
char string[];
int stend;
{

	while ((stend >= 0) && (string[stend] <= ' ')) string[stend--] = 0;

}

setroot ()
{
int tmp;

	strcpy(root,prjfile);
	for (tmp=0; tmp < sizeof(prjfile); ++tmp)
		if (prjfile[tmp] == '.') {
			root[tmp]=0;
			return;
		}

}
