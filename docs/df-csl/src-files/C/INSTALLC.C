/* CS&L Installation program */
/* Last updated: 5/16/88 */


#include <stdio.h>

main ()
{
char	tmpstr[256],
		turboc[81],
		dflib[81],
		dataflex[81],
		destination[81],
		dfinclude[81],
		control[256];
int		result;
char	quit;
char	direct[81];

	getcwd(direct,80);

	/* Installation default directories */
	strcpy(turboc,"\\turboc");
	strcpy(destination,"\\dfcsl\\run");
	strcpy(dflib,"\\dfcsl\\dflib");
	strcpy(dfinclude,"\\dfcsl\\include");
	strcpy(dataflex,"\\dataflex");

	view_file("a:read.me");

	quit = 0;
	printf("\n");
	printf("\n");
	printf("CS&L Installation Information\n");
	printf("=============================\n");
	printf("\n");
	printf("Please enter the following information, or press <return>\n");
	printf("for the default:\n");

	do {
		printf("\n");
		printf("\n");
		printf("Enter Turbo-C directory (full path excluding drive):\n");
		printf("[default is %s] ",turboc);
		gets(tmpstr); tmpstr[80]=0;
		if (strlen(tmpstr) != 0) strcpy(turboc,tmpstr);

		printf("\n");
		printf("Enter desired CS&L Work directory (full path excluding drive):\n");
		printf("[default is %s] ",destination);
		gets(tmpstr); tmpstr[80]=0;
		if (strlen(tmpstr) != 0) strcpy(destination,tmpstr);

		printf("\n");
		printf("Enter desired CS&L Library directory (full path excluding drive):\n");
		printf("[default is %s] ",dflib);
		gets(tmpstr); tmpstr[80]=0;
		if (strlen(tmpstr) != 0) strcpy(dflib,tmpstr);

		printf("\n");
		printf("Enter desired CS&L Include-File directory (full path excluding drive):\n");
		printf("[default is %s] ",dfinclude);
		gets(tmpstr); tmpstr[80]=0;
		if (strlen(tmpstr) != 0) strcpy(dfinclude,tmpstr);

		printf("\n");
		printf("Enter DATAFLEX FULL DEVELOPMENT directory (full path excluding drive):\n");
		printf("[default is %s] ",dataflex);
		gets(tmpstr); tmpstr[80]=0;
		if (strlen(tmpstr) != 0) strcpy(dataflex,tmpstr);

		printf("\n\n");
		printf("Is the above correct (Y/N): N%c",8);
		gets(tmpstr); 
		tmpstr[0] = toupper(tmpstr[0]);
		if (tmpstr[0] == 'Y') quit = 1;

	} while (!quit);

	printf("\n");
	printf("\n");
	printf("\n");
	printf("  D a t a F l e x    C    S o u r c e    a n d    L i b r a r y\n");
	printf("  = = = = = = = =    =    = = = = = =    = = =    = = = = = = =\n");
	printf("\n");
	printf("\n");
	printf("  APRIL 1988.\n");
	printf("\n");
	printf("\n");

	printf("\n");
	printf("Installation beginning.  Please wait.\n");
	printf("\n");

	printf("\n");
	printf("\nChecking for directories.  Please wait.");
	printf("\n");

	make_check(dflib);
	make_check(dfinclude);
	make_check(destination);

	sprintf(control,"$TURBOC$ %s $DFLIB$ %s $DFINCLUDE$ %s $DATAFLEX$ %s",
		turboc,dflib,dfinclude,dataflex);

	/* change diskettes */
	changedisk("A:/SOURCE","CS&L SOURCE FILES");

	chdir(destination);

	printf("\n");
	printf("Copying C source files.  Please wait.\n");
	printf("\n");

	chdir("A:/C");
	sprintf(tmpstr,"COPY A:%c%c%c\n",42,46,42);
	issuecmd(tmpstr);

	printf("\n");
	printf("Making Turbo-C link and project files.  Please wait.\n");
	printf("\n");

	chdir("A:/LNK");
	replace("A:GF.LNK","GF.LNK",control);
	replace("A:GNOF.LNK","GNOF.LNK",control);
	replace("A:MAKEL.LNK","MAKEL.LNK",control);
	replace("A:MAKEPRJ.LNK","MAKEPRJ.LNK",control);
	replace("A:NOGF.LNK","NOGF.LNK",control);
	replace("A:NOGNOF.LNK","NOGNOF.LNK",control);
	replace("A:RUN.LNK","RUN.LNK",control);
	replace("A:RUNG.LNK","RUNG.LNK",control);

	replace("A:GF.PRJ","GF.PRJ",control);
	replace("A:GNOF.PRJ","GNOF.PRJ",control);
	replace("A:NOGF.PRJ","NOGF.PRJ",control);
	replace("A:NOGNOF.PRJ","NOGNOF.PRJ",control);

	printf("\n");
	printf("Making batch files.  Please wait.\n");
	printf("\n");

	chdir("A:/BAT");
	replace("A:OBJ.BAT","OBJ.BAT",control);
	replace("A:X.BAT","X.BAT",control);
	replace("A:RUNLNK.BAT","RUNLNK.BAT",control);
	replace("A:RUNGLNK.BAT","RUNGLNK.BAT",control);
	replace("A:RCMP.BAT","RCMP.BAT",control);

	printf("\n");
	printf("Copying miscellaneous files.  Please wait.\n");
	printf("\n");

	replace("A:BATLNK.TXT","BATLNK.TXT",control);

	/* include files */

	printf("\n");
	printf("Copying include files.  Please wait.\n");
	printf("\n");

	chdir(dfinclude);
	chdir("A:/INCLUDE");
	sprintf(tmpstr,"COPY A:%c%c%c\n",42,46,42);
	issuecmd(tmpstr);

	/* change diskettes */
	printf("\n");
	changedisk("A:/OBJECT","CS&L OBJECT FILES");
	printf("\n");

	/* copy object files */
	printf("\n");
	printf("Copying object files.  Please wait.\n");
	printf("\n");

	chdir(destination);
	chdir("A:/");
	sprintf(tmpstr,"COPY A:%c%c%c\n",42,46,42);
	issuecmd(tmpstr);

	/* change diskettes */
	printf("\n");
	changedisk("A:/LIBRARY","CS&L LIBRARY FILES");
	printf("\n");

	/* copy libraries */

	printf("\n");
	printf("Copying library files.  Please wait.\n");
	printf("\n");

	chdir(dflib);
	chdir("A:/");
	sprintf(tmpstr,"COPY A:%c%c%c\n",42,46,42);
	issuecmd(tmpstr);

	/* finished */
	chdir(&direct[2]);

	printf("\n\n");
	printf("The CS&L installation process has finished.\n");
	printf("The CS&L work area is in the directory %s\n",destination);
	printf("\n");

	printf("Press <return> to exit.");
	gets(tmpstr);
}

#include <stat.h>
#include <fcntl.h>
#include <io.h>
#include <alloc.h>

replace(infile, outfile, control)
char infile[], outfile[];
char control[];
{
int 	fin, fout,
		fileptr,
		lineptr,
		tmpptr,
		tmpint,
		strings,
		size;
char	tmpbool,quit;
char 	*wholefile,
		*strfind,
		nextchar,
		cpy[255],
		line[255];
struct REPS {
char 	*from,
		*to;
} rep[10];


	fin = open(infile,O_RDONLY);

	if (fin == -1) return(-1);
	else {
		size = (int) filelength(fin);
		wholefile = malloc( size );
		size = read(fin, wholefile, size);
		close(fin);
	}

	fout = creat(outfile,S_IWRITE);
	if (fout == -1) return(-2);

	printf("    From %s to ...",infile);

	setmem(&rep, sizeof(struct REPS),0);
	setmem(cpy, sizeof(cpy),0);
	strcpy(cpy,control);

	tmpptr = 0;
	strings = 0;

	rep[0].from = &cpy[0];
	tmpbool = 0;

	do {
		nextchar = cpy[tmpptr];
		if (nextchar <= 32) {
			if (!tmpbool) {
				rep[strings].to = &cpy[tmpptr+1];
				cpy[tmpptr] = 0;
				tmpbool = 1;
				++strings;
			}
			else {
				rep[strings].from = &cpy[tmpptr+1];
				cpy[tmpptr] = 0;
				tmpbool = 0;
			}
		}
		++tmpptr;
	} while (nextchar != 0);


	fileptr = 0;
	lineptr = 0;
	quit = 0;
	setmem(line,sizeof(line),0);

	do {
		nextchar = wholefile[fileptr];
		if (nextchar < 32 || quit) {
			char outline[255];
			line[lineptr] = 0;

			/* do replace */

			for (tmpint = 0; tmpint < strings; ++tmpint) {
				static int strplace;
				strplace = 0;
				while ( (strfind = (char *) strstr(&line[strplace],
											rep[tmpint].from)) != NULL) {
					static char start[80],end[80];
					static int endlength,fromlength;

					/* left piece */
					tmpptr = (int) strfind - (int) line;
					strplace = tmpptr+strlen(rep[tmpint].to);
					memmove(start,line,tmpptr);
					start[tmpptr] = 0;

					/* right piece */
					fromlength = strlen(rep[tmpint].from);
					endlength = strlen(line) - fromlength - tmpptr;
					memmove(end,&line[tmpptr+fromlength],endlength);
					end[endlength] = 0;

					/* left piece + new middle + right piece */
					sprintf(line,"%s%s%s",start,rep[tmpint].to,end);
				}
			}

			/* write line */
			sprintf(outline,"%s\n",line);
			write(fout,outline,strlen(outline));

			if (quit) {
				close(fout);
				printf("... to %s\n",outfile);
				free( wholefile );
				return(0);
			}

			setmem(line,sizeof(line),0);
			lineptr = 0;
			++fileptr;
			nextchar = wholefile[fileptr];
		}

		line[lineptr] = nextchar;
		++fileptr;
		++lineptr;
		if (fileptr >= size) {
			line[lineptr-1] = 0;
			wholefile[fileptr] = 0;
			quit = 1;
		}
	} while (1);
}


xmdir (path)
char path[];
{
char tmpstr[80];
int len,curpos;
int result;

	strcpy(tmpstr,path);
	len = strlen(tmpstr);
	result = mkdir(tmpstr);
	if (result == -1) {
		if (len == 0) return(-1);
		while ((tmpstr[len] != '/' && tmpstr[len] != '\\') && len >= 0) {
			--len;
		}
		if (len <= 0) return(-1);
		tmpstr[len] = 0;
		result = xmdir(tmpstr);

		if (result == -1) return(-1);
		tmpstr[len] = 92; /* \ */
		result = xmdir(tmpstr);
		if (result == -1) return(-1);
	}
	else printf("    Directory %s created\n",tmpstr);
	return(0);
}

make_check(dir)
char dir[];
{
int result;

	result = xmdir(dir);
	result = chdir(dir);
	if (result == -1) {
		printf("Cannot create or access directory %s.  Installation stopped.\n",dir);
		exit(1);
	}
}

changedisk(id,label)
char id[],label[];
{
char tmp[255];
int fd;

	fd = open(id,O_RDONLY);

	if (fd == -1)
	do {
		printf("\nPlease insert diskette labelled: %s",label);
		printf("\nPress <return> when ready, or Q to Quit: ");
		gets(tmp);
		tmp[0] = toupper(tmp[0]);
		if (tmp[0] == 'Q') {
			printf("\nInstallation stopped.\n");
			exit(1);
		}
		fd = open(id,O_RDONLY);
		if (fd == -1) printf("\nYou have inserted the wrong diskette\n");
	} while (fd == -1);

	close(fd);
}

issuecmd (cmd)
char	cmd[];
{

	system(cmd);

}

view_file(notes)
char notes[];
{
int fd, size, cline, cchar;
char *wholefile, c;
char tmpstr[255];

	/* open file */
	fd = open(notes, O_RDONLY);
	if (fd == -1) return(-1);

	/* read in file */
	size = (int) filelength(fd);
	wholefile = malloc( size );
	size = read(fd, wholefile, size);
	close(fd);

	/* cat file */
	cline = 0;
	cchar = 0;

	do {
		c = wholefile[cchar] & 0x7f;
		if (c == 0x0d || c == '^') { /* ^ == force end of page */
			++cline;
			if (cline > 22 || c == '^') {

				/* press a key */

				printf("\n");
				printf("\nPlease press <return> to continue");
				gets(tmpstr);

				printf("\n");
				printf("\n");
				cline = 0;
			}
			printf("\n");
			if (wholefile[cchar+1] == 0x0a) c = wholefile[++cchar];
		}
		else
			printf("%c",c);

	} while (++cchar < size);

	free(wholefile);
	return(0);

}
