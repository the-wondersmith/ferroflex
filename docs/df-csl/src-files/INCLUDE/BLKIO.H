/************************************************************************
Confidential Trade Secret.
Copyright (c) 1986 Data Access Corporation, Miami Florida,
as an unpublished work.  All rights reserved.
DataFlex is a registered trademark of Data Access Corporation.
************************************************************************/

struct fcbrec {
        char            pname[41];
        int             pathnumber;
        long            lastused;
	bool		noclose;
        struct fcbrec   *nextfcb;
	unsigned char	fileindex;
	int		open_mode;
};

#define FCBSIZE sizeof(struct fcbrec)

#ifdef MSDOS
#define BLK_RDWR  O_RDWR | O_BINARY
#define BLK_RDONLY O_RDONLY | O_BINARY
#define BLK_TRUNC O_RDWR | O_TRUNC | O_BINARY
#define BLK_CREAT O_RDWR | O_CREAT | O_BINARY
#define SQF_TRUNC O_RDWR | O_TRUNC | O_TEXT
#define SQF_CREAT O_RDWR | O_CREAT | O_TEXT
#define BLK_SHARED O_DENYNONE | O_NOINHERIT
#define BLK_EXCLUSIVE 0 /* O_DENYALL */
#else
#ifdef CPM
#define BLK_RDWR  O_RDWR | O_BINARY
#define BLK_RDONLY O_RDONLY | O_BINARY
#define BLK_TRUNC O_RDWR | O_TRUNC | O_BINARY
#define BLK_CREAT O_RDWR | O_CREAT | O_BINARY
#define SQF_TRUNC O_RDWR | O_TRUNC | O_TEXT
#define SQF_CREAT O_RDWR | O_CREAT | O_TEXT
#define BLK_SHARED O_DENYNONE
#define BLK_EXCLUSIVE 0
#else
#define BLK_RDWR  O_RDWR
#define BLK_RDONLY O_RDONLY
#define BLK_TRUNC O_RDWR | O_TRUNC
#define BLK_CREAT O_RDWR | O_CREAT
#define SQF_TRUNC O_RDWR | O_TRUNC
#define SQF_CREAT O_RDWR | O_CREAT
#define BLK_SHARED 0
#define BLK_EXCLUSIVE 0
#endif
#endif

