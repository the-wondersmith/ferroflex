# DataFlex v3.0+ File Structures

## Attributions & Credits

> This description of the Dataflex 3.0+ file structure is based on
> work originally done by **David H. Lynch Jr.** which in turn was
> based on a description of the DataFlex 2.3b file structure originally
> created by *Peter M. Grillo* of **MAINSTREAM COMPUTER CONSULTING**


## License, Usage, and Redistribution

In accordance with the original usage terms in David Lynch's work, you may use and / or distribute the information
below under the terms of the GNU General Public License.


## Background

In the course of developing a DBI driver for Dataflex 3.x format .DAT files I have had to update it to reflect the
similar but different structure of a DF 3.x format database. I have focused on deciphering the information that
I needed for that purpose. So far the information I have deciphered has been more than sufficient to meet my
needs. Hopefully it can be of use to others.

## The DataFlex Table File Format

A DataFlex `.DAT` file is divided into two distinct sections:

- A header containing information describing the table contained by the file
- The table's data starting with the NULL record followed by 1 record for each "row" in the table

The .HDR file that is created if header integrity is enabled is virtually identical to the header portion of the
database up through record 0. However, it may have slightly different checksum values, and some record count fields
are sometimes slightly behind.

> **NOTE:** *All values in the header are stored in little endian order.*

### Table Header

The header itself is made up of two major sections along with a multitude of distinct values. Roughly, the structure
looks something like this:

| Description       | Starting Offset | Ending Offset |
| ----------------- | --------------- | ------------- |
| **HEADER**        | --------------- | ------------- |
| Various settings  | **0x00**        | **0xAF**      |
| Index Table       | **0xB0**        | **0x1CF**     |
| Unknown           | **0x1D0**       | **0x2CF**     |
| Table Name        | **0x2D0**       | **0x2DF**     |
| Field Definitions | **0x2E0**       | **0xAD7**     |
| Unknown           | **0xAD8**       | **0xBFF**     |
| **DATA**          | --------------- | ------------- |
| NULL Record       | **0xC00**       | **0x???**     |
| Records           | **0x???**       | **EOF**       |

#### Table Attributes & Options

The first 175 bytes of the header contain all the attributes and options that control the table's general behavior in
a DataFlex environment. Each the value associated with each attribute or option is stored at a distinct location in
the header and in a specific format. For example, the number of records currently in the table is stored as a 4-byte
little endian encoded integer whereas the value for "is file locking enabled" is obviously a boolean value but is
still occupies a full byte in the header, not one bit.

The locations and data types of the various attributes and options are as follows:

| Description                | Starting Offset | Ending Offset | Size                  | Data Type        | Notes                                                                                                                                                  |
| -------------------------- | --------------- | ------------- | --------------------- | ---------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------ |
| Highest Record Number      | **0x00**        | **0x03**      | **DWORD** *(4 bytes)* | unsigned integer | Refers to the highest record number the table has *ever* held, not *currently holds*                                                                   |
| **Unknown**                | **0x04**        | **0x07**      | **DWORD** *(4 bytes)* | unsigned integer | Has something to do with reuse of deleted space. Should be `0` if deleted space is not re-used otherwise it may be the first "available" record number |
| Current Record Count       | **0x08**        | **0x0B**      | **DWORD** *(4 bytes)* | unsigned integer |                                                                                                                                                        |
| Maximum Record Count       | **0x0C**        | **0x0F**      | **DWORD** *(4 bytes)* | unsigned integer | The absolute maximum number of records the table is "allowed" to contain                                                                               |
| Header Integrity Flag      | **0x10**        | **0x13**      | **DWORD** *(4 bytes)* | unknown          | Value should be `0` if header integrity is not enabled otherwise it may be a checksum value                                                            |
| Checksum¹                  | **0x14**        | **0x17**      | **DWORD** *(4 bytes)* | unknown          |                                                                                                                                                        |
| Checksum¹                  | **0x18**        | **0x1B**      | **DWORD** *(4 bytes)* | unknown          |                                                                                                                                                        |
| Table Version²             | **0x1C**        | **0x1C**      | **BYTE**  *(1 byte)*  | unsigned integer |                                                                                                                                                        |
| Table Version²             | **0x1D**        | **0x1D**      | **BYTE**  *(1 byte)*  | unsigned integer |                                                                                                                                                        |
| **Static Value**³          | **0x1E**        | **0x1E**      | **BYTE**  *(1 byte)*  | unsigned integer | Usually `0`                                                                                                                                            |
| Compression Type           | **0x1F**        | **0x1F**      | **BYTE**  *(1 byte)*  | unsigned integer | `0` = None, `1` = Fast, `2` = Standard, `3` = Custom                                                                                                   |
| First Available Record†    | **0x20**        | **0x23**      | **DWORD** *(4 bytes)* | unsigned integer | Should be `0` if deleted space is re-used otherwise it may be the first "available" record number                                                      |
| **Unknown**                | **0x28**        | **0x00**      | **DWORD** *(4 bytes)* | unknown          | Should be `0` if the table's compression type is `None`, otherwise its purpose is unknown                                                              |
| **Unknown**                | **0x30**        | **0x00**      | **DWORD** *(4 bytes)* | unsigned integer | Should be `0` if the table's compression type is not `Standard`, otherwise its purpose is unknown                                                      |
| Checksum¹                  | **0x38**        | **0x3B**      | **DWORD** *(4 bytes)* | unknown          |                                                                                                                                                        |
| **Static Value**³          | **0x3C**        | **0x3F**      | **DWORD** *(4 bytes)* | unsigned integer | Usually `0`                                                                                                                                            |
| **Unknown**                | **0x40**        | **0x40**      | **BYTE**  *(1 byte)*  | unsigned integer | Should be `0` if header integrity is not enabled, otherwise its purpose is unknown                                                                     |
| File Locking Flag          | **0x41**        | **0x41**      | **BYTE**  *(1 byte)*  | boolean          | `0` = disabled, `1` = enabled                                                                                                                          |
| Reuse Deleted Space⁴       | **0x4A**        | **0x4B**      | **WORD**  *(2 bytes)* | signed integer   | `0` = true, `-1` = false                                                                                                                               |
| Rapid Changing Checksum    | **0x4C**        | **0x4D**      | **WORD**  *(2 bytes)* | unknown          | Value appears to change more frequently than the larger *DWORD* checksums                                                                              |
| Checksum¹                  | **0x50**        | **0x53**      | **DWORD** *(4 bytes)* | unknown          |                                                                                                                                                        |
| Records Before Fill Bytes⁵ | **0x98**        | **0x99**      | **WORD**  *(2 bytes)* | unsigned integer | Basically records per 512-byte block or 1 the table has no remainder blocks. *See note 5 below*                                                        |
| Record Length              | **0x9A**        | **0x9B**      | **WORD**  *(2 bytes)* | unsigned integer | Denotes the number of bytes occupied by one record or one complete "row" in the table                                                                  |
| Reuse Deleted Records      | **0xA4**        | **0xA4**      | **BYTE**  *(1 byte)*  | boolean          | `0` = false, `1` = true                                                                                                                                |
| Field Count                | **0xA5**        | **0xA5**      | **BYTE**  *(1 byte)*  | unsigned integer | The number of "columns" contained by the table                                                                                                         |
| File Locking Enabled       | **0xA8**        | **0xA8**      | **BYTE**  *(1 byte)*  | boolean          | `0` = false, `1` = true                                                                                                                                |
| **Static Value**³          | **0xA9**        | **0xA9**      | **BYTE**  *(1 byte)*  | unsigned integer | Usually `1`                                                                                                                                            |
| **Static Value**³          | **0xAD**        | **0xAD**      | **BYTE**  *(1 byte)*  | unsigned integer | Usually `1`                                                                                                                                            |
| **Unknown**                | **0xAE**        | **0xAF**      | **WORD**  *(2 bytes)* | unknown          |                                                                                                                                                        |


> 1 - There appear to be **at least** four of these *4-byte checksum* values. Currently, the values
>     located at `0x14`, `0x18`, `0x38`, and `0x50` are suspected of being checksums. David Lynch's
>     original notes indicated that he initially thought these may be date/time stamps, but later
>     became suspicious that they are some type of checksum. He notes that their value(s) appear to
>     change almost any time that a change is made to a table using the `DFFILE` utility.
> 
> 2 - In DataFlex v3.0+ tables, the bytes at `0x1C` and `0x1D` appear to always have a value of `0x1E`
>     (30 in decimal). From cursory investigation / comparison this does not appear to be the case for
>     DataFlex v2.3b and older tables though. Given the apparent coincidence, it seems logical to assume
>     that the values of those two bytes represent some kind of "version number" for the table's format.
>
> 3 - These values *appear* to be static, but the testable dataset isn't large enough to say definitively
> 
> 4 - Enabling compression can / will also cause this value to be `-1`
>
> 5 - DataFlex tables group records together in 512-byte "blocks". The record length of a given table may
>     not be evenly divisible into 512 bytes though. If that's the case, the "block" is padded with fill
>     bytes (0xFF) to round out a group of records to 512 bytes. The 2-byte value stored at offset `0x98`
>     represents the total number of records that should be read (or present) in any given 512-byte "block"
>     *before* the first fill byte. A value of `1` means that either the table's record length is exactly
>     512 bytes, the table's record length is *more* than 512 bytes but still evenly divisible into blocks,
>     *or* that records should simply be read sequentially without worrying about having to skip fill bytes.
> 
> † - Suspected but not confirmed (potentially un-confirmable)

#### Index Table

The header contains information about any indexes that exist on the table. That information is stored as
"table" of sixteen sequential 18-byte "rows". The header doesn't appear to contain a value that corresponds
to *how many* indexes are associated with the table the same way it does for the number of columns or "fields".

Each "row" in the index "table" looks like this:

| Description          | Starting Offset | Ending Offset | Size                | Data Type        | Notes                                                   |
| -------------------- | --------------- | ------------- | ------------------- | ---------------- | ------------------------------------------------------- |
| Field Count¹         | **0x00**        | **0x00**      | **BYTE** *(1 byte)* | unsigned integer | Number of fields in the index                           |
| Segment Field Number | **0x01**        | **0x01**      | **BYTE** *(1 byte)* | unsigned integer | The field number to which the index segment corresponds |
| `...`                | `...`           | `...`         | `...`               | `...`            | `...`                                                   |
| Segment Field Number | **0x11**        | **0x11**      | **BYTE** *(1 byte)* | unsigned integer | The field number to which the index segment corresponds |
| Type Flag            | **0x12**        | **0x12**      | **BYTE** *(1 byte)* | unsigned integer | `0` = default, `1` = ascending, `2` = uppercase         |

> 1 - `DFFILE` adds `0x80` to this value if the index is a "batch" index, therefore it should be read something like:
>
> ```rust
> let field_count: u8 = if byte < 128 { u8::from(byte) } else { u8::from(byte - 128) };
> ```


#### File Root Name

The header stores the table's "root name" in the 16 bytes from `0x2D0` to `0x2DF` as a null-padded, ASCII-encoded string.


#### Field Definitions

The "last" and arguably most important part of the header is the field definition table. The header stores all the
information required to differentiate the "columns" of each row and read the data it contains correctly. The field
definition table technically occupies 2120 bytes, (8 bytes per field with a maximum field count of 255), but in reality
only the first `N` definitions will be populated (`N` being the number of fields in the table, corresponding to the
unsigned integer value stored at offset `0xA5` in the header).

Each "row" in the field definition "table" looks like this:

| Description           | Starting Offset | Ending Offset | Size                  | Data Type        | Notes                                                                    |
| --------------------- | --------------- | ------------- | --------------------- | ---------------- | ------------------------------------------------------------------------ |
| Offset                | **0x00**        | **0x01**      | **WORD** *(2 bytes)*  | unsigned integer | The field's offset from the start of the record                          |
| Decimal Points        | **0x02**        | **0x02**      | **NIBBLE** *(4 bits)* | unsigned integer | The number of digits to the right of the decimal in numeric-type columns |
| Main Index Number     | **0x02**        | **0x02**      | **NIBBLE** *(4 bits)* | unsigned integer | The number of the field's "main" index                                   |
| Field Length          | **0x03**        | **0x03**      | **BYTE** *(1 byte)*   | unsigned integer | The number of bytes occupied by the field                                |
| Data Type¹            | **0x04**        | **0x04**      | **BYTE** *(1 byte)*   | unsigned integer | The type of data held by the field.                                      |
| Related File Number²  | **0x05**        | **0x05**      | **BYTE** *(1 byte)*   | unsigned integer |                                                                          |
| Related Field Number³ | **0x06**        | **0x07**      | **WORD** *(2 bytes)*  | unsigned integer |                                                                          |

> 1 - `0` = ASCII, `1` = Numeric, `2` = Date, `3` = Overlap, `5` = Text, `6` = Binary
> 
> 2 - The entry number of the table in the associated `filelist.cfg` that contains the related field
> 
> 3 - The field number in the specified table to which the column is related


#### The *Null* Record

The **null** record signifies the "start" of records in the table file. It begins at offset `0x0C00` and occupies
the same number of bytes as one full record (stored in the header as a 2-byte unsigned integer at offset `0x9A`).

The **null** record is sometimes referred to as record number `0`.

#### Table Records

The **null** record is immediately followed by the "real" records contained by the table, grouped into 512-byte
"blocks" with fill bytes as required.

Each of the supported data types is stored in the table file as follows:

| Data Type | Storage Method                                | Notes                                       |
| --------- | --------------------------------------------- | ------------------------------------------- |
| ASCII     | Standard ASCII encoding, 1 character per byte | Fields are null-padded                      |
| Numeric   | BCD-encoded numbers                           | Decimal point / precision defined in header |
| Date      | 3-byte binary day numbers                     | Date day-numbers are little endian          |
| Overlap   | Standard ASCII encoding, 1 character per byte | Fields are null-padded                      |
| Text      | Standard ASCII encoding, 1 character per byte | Fields are null-padded                      |
| Binary    | Standard ASCII encoding, 1 character per byte | Fields are null-padded                      |

> **Note:**
> Wile the size values for fields are stored in bytes, the sizes of Text and Binary fields
> are actually stored 16 bytes per "size unit". For example, an ASCII column with a size
> value of 100 occupies one hundred bytes exactly. However, a Text column with a size
> value of 100 will occupy 1600 bytes.

## The DataFlex Index File Format

DataFlex index files are essentially just the values of the indexed field(s) stored sequentially in 1024 byte blocks, 
in the order indicated by the type flag value in their associated table file's header. Each block starts with 4 bytes.
The 3rd byte is the count of records in the block. The blocks are 0 filled The actual index records start at offset 4
in the block.

Ascii Values are converted to their collating sequence numbers first. In the american collating sequence A=204, and the
order of characters is AaBb... Non-ascii values are as they are stored in the DB. There is a 3 byte field pointing to
the record number in the file at the end of the index record.

I believe the index records are in order from the physical start of the file through the end and the blocking and null
filling of the index bocks is to make it easier to insert index records. Most of the indexes that I have dumped seem to
have room for several additional records at the end of each block.

There is a possible use for the index files beyond that of progressing through the file in a specific order. It should be
possible to reconstruct the data for those fields of a database that are contained in the index by extracting the data
from the indexes if they are available. In many instances many or all fields of a database participate in the index.
Aside from converting the ASCII values from their collating equivalents, this should actually be fairly simple and
there are certainly instances in the past when I wish I had that capability.

## The DataFlex `Filelist.cfg` File Format

The DataFlex `filelist.cfg` contains one 128-byte record per associated table file. The order in which the entries
occur determines each table's "file number" (as referenced by the field definitions in each table's header). Record `0`
contains the "root file" name (`filelist.cfg`) and is otherwise null filled. Subsequent entries are composed exclusively
of ASCII-encoded, fixed-length, null-padded strings and are structured thusly:

| Description        | Starting Offset | Ending Offset | Size     |
| ------------------ | --------------- | ------------- | -------- |
| File Root Name     | **0x00**        | **0x28**      | 28 bytes |
| File Description   | **0x29**        | **0x49**      | 32 bytes |
| Dataflex File Name | **0x4A**        | **0x7F**      | 48 bytes |
