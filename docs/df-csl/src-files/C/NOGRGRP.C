/************************************************************************
Confidential Trade Secret.
Copyright (c) 1986 Data Access Corporation, Miami Florida,
as an unpublished work.  All rights reserved.
DataFlex is a registered trademark of Data Access Corporation.
************************************************************************/

#include <standard.h>
#include <flex2con.h>
#include <flex2def.h>
#include <flex2var.h>

#define NOGRAPH_PERMISS	48
/**********************************************************************
		dummy graphics module for non graphics runtime
************************************************************************/

cmdgraphgroup ()
{
		error( NOGRAPH_PERMISS );
}

