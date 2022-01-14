/* Error Recovery Functions */

void clearwarning();
void error(int num);
void errprint();
void runerror();

/* Memory Management Functions */

long bigmem();
/*void dispose(char *ptr);*/
/*void fillchar(char *memory, int length, char value);*/
int memavail();
/*void move(char *source, char *destination, int length);*/
/*void new(char *ptr, int size);*/
bool same(char *memptr1, char *memptr2, int length);
/*void stralloc(char *ptr, char *astr);*/

/* Multiuser Functions */

void lock();
void reread();
void unlock();

/* Database Management Functions */

void dsetup();
void allclear();
void allclose();
void find(int filenum, int findby, int mode);
byte ftype(int filenum, int fieldnum);
void hold(int filenum);
void nput(int filenum, int fieldnum, char *number);
int numflds(int filenum);
void point(int filenum, int fieldnum);
void rdelete(int filenum);
void relate(int filenum);
void reread();
void reverse(int filenum);
void save(int filenum);
void set_iomode(int filenum, int filemode);
void sput(int filenum, int fieldnum, char *string);
byte status(int filenum);
void xclear(int filenum);
void xclose(int filenum);
void xopen(int filenum, int mainindex, int levelsmain, int other_levels);

/* Console I/O Functions */

void ssetup();
void accept(byte row, byte column, char *buffer, byte length, int *term);
void cleareos(int row, int column);
void clearscreen();
void display(int row, int column, char *buffer, int length);
void echo_term();
void gotoxy(int row, int column);
int press();
void raw_term();
int readchar();
int readcnd();
char readcon();
void readstr(char *into, int maxlength, int *term);
void setcolor(int colorcode);
void writecon(char achar);
void writeeol();
void writels(char *string);
void writestr(char *string);

/* Forms I/O Functions */

int img_accept(int window, int page);
void img_autopage(int window, int page);
void img_clear(int startwindow, int startpage, int endwindow, int endpage);
void img_close();
void img_color(int pagenum, int windowcolor, int backround);
void img_display(int fromtype, int window, char *var, int page);
void img_get(int totype, int window, int page);
void img_init(char *imagename);
void img_page(int page);
void img_pos(int pagenum, int row, int column);
void img_put(int fromtype, int window, int page);

/* Block I/O Functions */

void bkread(struct fcbrec *fcb, char *buf, int *result, int number, long start_block);
void bkwrite(struct fcbrec *fcb, char *buf, int *result, int number, long start_block);
void blkclose(struct fcbrec *fcb, int *result);
void blkcreate(struct fcbrec *fcb, char *name, int *result);
void blkname(struct fcbrec *fcb, char *name, int *result);
void blkopen(struct fcbrec *fcb, char *name, int *result);
void bytread(struct fcbrec *fcb, char *buf, int *result, int number, long start_byte);
void bytwrite(struct fcbrec *fcb, char *buf, int *result, int number, long start_byte);

/* Sequential I/O Functions */

void seqclose(struct sfcbtype *sfcb);
void seqgchar(struct sfcbtype *sfcb, char *achar);
void seqopen(struct sfcbtype *sfcb, char *name, bool seqout);
void seqpchar(struct sfcbtype *sfcb, char achar);
void seqrline(struct sfcbtype *sfcb, char *intext);
void seqwline(struct sfcbtype *sfcb, char *outtext);

/* Conversion Functions */

void allcaps(char *string);
void cvdn(char *datestr, char *num);
char *cvfn(double num, char *bcd_num);
void cvfs(double num, char *string, int length, int points);
char *cvis(int from, char *astr);
void cvln(long longarg, char *num);
char *cvls(long num, char *astr);
void cvnd(char *num, char *datestr);
double cvnf(char *num);
long cvnl(char *num);
void cvns(char *num, char *astr, int length, int points);
double cvsf(char *string);
int cvsi(char *astr);
long cvsl(char *astr);
void cvsn(char *astr, char *num);
int imax(int a, int b);
int imin(int a, int b);
void lowercase(char *string);
void padstr(char *string, int length);
void pkdec(char *out, char *in, int totallen, int points);
void updec(char *in, int totallen, int points, char *out);

/* Runtime Functions */

void cvtarg(int cvtfrom, int cvtto);
void expeval(char *string, int argtype);
void flexkey();
void formdp(int page);
void getargc(struct argument *arg, int cvtto);
int getargi(struct argument *arg);
void getargn(struct argument *arg, char *num);
void getcline(char *into);
void getlen();
void getput(int argtemp);
void putargc(struct argument *arg, int cvtfrom);
void putargstr(struct argument *arg);
void unpkarg(struct argument *arg);

/* BCD Math Functions */

void bcd_add(char *bcdarg1, char *bcdarg2, char *bcdresult);
void bcd_div(char *bcdarg1, char *bcdarg2, char *bcdresult);
void bcd_mult(char *bcdarg1, char *bcdarg2, char *bcdresult);
void bcd_sub(char *bcdarg1, char *bcdarg2, char *bcdresult);
