# DataFlex v2.3b File Structures

## Attributions & Credits

> This description of the Dataflex v2.3b file structure was originally
> published by *Peter M. Grillo* of **MAINSTREAM COMPUTER CONSULTING**.
> The contents of this file bear minor changes, mostly in the form of
> clarifications and reformatting. Any credit and / or praise for the
> information contained herein should be directed entirely to him.


## License, Usage, and Redistribution

As notated in Mr. Grillo's original publication, Data Access Corporation has deemed the structure of the .DAT file
proprietary information. The following definition of a 2.3 .DAT file was derived by Mr. Grillo independently and as
notated in his original work -

> "... any problem arising from the use of this information will be your problem. Please do not call DAC and snivel.
> Use {it} at your own risk. Please do not upload this to DAC's BBS."

Data Access Corporation allegedly indicated to Mr. Grillo that he could release this information provided the above
disclaimer was included, so it is reproduced and included here as well. Mr. Grillo's work was later expanded upon by
**David H. Lynch Jr.**, the results of which were released under the terms of the GNU General Public License. It is
not clear whether any terms other than those above apply to *this* version of Mr. Grillo's work. As such, it is likely
best to proceed as though the only applicable grants of license or usage are the ones stated above. Proceed as you
see fit.

## The DataFlex Table File Format

A DataFlex `.DAT` file is divided into two distinct sections:

    - A header containing information describing the table contained by the file
    - The table's data starting with the NULL record followed by 1 record for each "row" in the table

The header contains information about the file definition. Just about everything you define in `DFFILE` can be found in
the header except for tag names. It is possible to read the header of a 2.3 `.DAT` file, and the corresponding `.TAG`
file to produce a perfect `.DEF` file.

> **NOTE:** *In the expanded description of the DataFlex v3.0+ file format, David Lynch notes that all values in the*
> *header are stored in little endian order. This contradicts Mr. Grillo's notation(s) of the endianness of various*
> *values. Through testing, it appears that reading all the header values as little endian still produces correct*
> *data. It is advisable, however, not to do so when reading table headers that are known to be in the v2.3 format.*

### Table Header

The header itself is made up of two major sections along with a multitude of distinct values. Roughly, the structure
looks something like this:

| Description        | Starting Offset | Ending Offset |
| ------------------ | --------------- | ------------- |
| **HEADER**         | --------------- | ------------- |
| Various settings   | **0x00**        | **0x63**      |
| Index Table        | **0x64**        | **0xB3**      |
| Table Name         | **0xB4**        | **0xBC**      |
| Unknown            | **0xBD**        | **0xC3**      |
| Field Definitions¹ | **0xC4**        | **0x1FC**     |
| Unknown            | **0x1FD**       | **0x1FF**     |
| **DATA**           | --------------- | ------------- |
| NULL Record        | **0x200**       | **0x???**     |
| Records            | **0x???**       | **EOF**       |


> 1 - While there is a note in **David Lynch Jr.**'s expanded definition of the DataFlex v3.0+ file format
>     specifying that v3.0+ tables have a cap of 255 fields, there is no such notation in **Mr. Grillo**'s
>     original work. The ending offset of `0x1FC` for the field definition table is based on field definitions
>     requiring at least 8 bytes each and there being exactly 316 bytes "available" between the field definition
>     table and **NULL** record's known-good starting offsets of `0xC4` and `0x200` respectively.
> 
>     To clarify - this means that while a value of 40 would be more numerically satisfying, DataFlex v2.3b
>     files have an effecitve maximum cap of 39 fields.


#### Table Attributes & Options

The first 175 bytes of the header contain all the attributes and options that control the table's general behavior in
a DataFlex environment. Each the values associated with each attribute or option is stored at a distinct location in
the header and in a specific format. For example, the number of records currently in the table is stored as a 4-byte
little endian encoded integer whereas the value for "is file locking enabled" is obviously a boolean value but still
occupies a full byte in the header, not one bit.

The locations and data types of the various attributes and options are as follows:

| Description              | Starting Offset | Ending Offset | Size                  | Data Type        | Notes                                                                                 |
| ------------------------ | --------------- | ------------- | --------------------- | ---------------- | ------------------------------------------------------------------------------------- |
| Highest Record Number    | **0x00**        | **0x03**      | **DWORD** *(4 bytes)* | unsigned integer | Refers to the highest record number the table has *ever* held, not *currently holds*  |
| Current Record Count     | **0x08**        | **0x0B**      | **DWORD** *(4 bytes)* | unsigned integer |                                                                                       |
| Maximum Record Count     | **0x0C**        | **0x0F**      | **DWORD** *(4 bytes)* | unsigned integer | The absolute maximum number of records the table is "allowed" to contain              |
| Record Length            | **0x4E**        | **0x4F**      | **WORD**  *(2 bytes)* | unsigned integer | Denotes the number of bytes occupied by one record or one complete "row" in the table |
| Reuse Deleted Space      | **0x58**        | **0x58**      | **BYTE**  *(1 byte)*  | boolean          |                                                                                       |
| Field Count              | **0x59**        | **0x59**      | **BYTE**  *(1 byte)*  | unsigned integer | The number of "columns" contained by the table                                        |
| Multi-User Reread Active | **0x5C**        | **0x5C**      | **BYTE** *(1 byte)*   | boolean          |                                                                                       |


#### Index Table

The header contains information about any indexes that exist on the table. That information is stored as a
"table" of sixteen sequential 18-byte "rows". The header doesn't appear to contain a value that corresponds
to *how many* indexes are associated with the table the same way it does for the number of columns or "fields".

Each "row" in the index "table" looks like this:

| Description          | Starting Offset | Ending Offset | Size                | Data Type        | Notes                                                   |
| -------------------- | --------------- | ------------- | ------------------- | ---------------- | ------------------------------------------------------- |
| Field Count¹         | **0x00**        | **0x00**      | **BYTE** *(1 byte)* | unsigned integer | Number of fields in the index                           |
| Segment Field Number | **0x01**        | **0x01**      | **BYTE** *(1 byte)* | unsigned integer | The field number to which the index segment corresponds |
| `...`                | `...`           | `...`         | `...`               | `...`            | `...`                                                   |
| Segment Field Number | **0x11**        | **0x07**      | **BYTE** *(1 byte)* | unsigned integer | The field number to which the index segment corresponds |

> 1 - `DFFILE` adds `0x80` to this value if the index is a "batch" index, therefore it should be read something like:
>
> ```rust
> let field_count: u8 = if byte < 128 { u8::from(byte) } else { u8::from(byte - 128) };
> ```

#### File Root Name

The header stores the table's "root name" in the 8 bytes from `0xB4` to `0xBC` as a null-padded, ASCII-encoded string.

#### Field Definitions

The "last" and arguably most important part of the header is the field definition table. The header stores all the
information required to differentiate the "columns" of each row and read the data it contains correctly. The field
definition table technically occupies 312 bytes, (8 bytes per field with a maximum field count of 39), but in reality
only the first `N` definitions will be populated (`N` being the number of fields in the table, corresponding to the
unsigned integer value stored at offset `0x59` in the header).

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

> 1 - `0` = ASCII, `1` = Numeric, `2` = Date, `3` = Overlap
>
> 2 - The entry number of the table in the associated `filelist.cfg` that contains the related field
>
> 3 - The field number in the specified table to which the column is related

#### The *Null* Record

The **null** record signifies the "start" of records in the table file. It begins at offset `0x200` and occupies
the same number of bytes as one full record (stored in the header as a 2-byte unsigned integer at offset `0x4E`).

The **null** record is sometimes referred to as record number `0`.

#### Table Records

The **null** record is immediately followed by the "real" records contained by the table, grouped into 512-byte
"blocks". Not all record lengths, however, divide evenly into 512, which causes the occurrence of fill bytes (usually
`0xFF`) to round out a group of records to 512 bytes.

Consider the following:

| Record Length | End Result                                                                                                           |
| ------------- | -------------------------------------------------------------------------------------------------------------------- |
| 128           | Divides into 512 evenly so no fill bytes are used                                                                    |
| 170           | Divided by 512 is 3 with a remainder of 2 so after every 3 records (starting at record 0) there will be 2 fill bytes |

For convenience, here is a table of common record lengths along with the associated group and fill byte counts:

| Record Length | Records Per "Block" | Number of Fill Bytes |
| ------------- | ------------------- | -------------------- |
| 256           | 2                   | 0                    |
| 170           | 3                   | 2                    |
| 128           | 4                   | 0                    |
| 102           | 5                   | 2                    |
| 85            | 6                   | 2                    |
| 73            | 7                   | 1                    |
| 64            | 8                   | 0                    |
| 56            | 9                   | 8                    |
| 51            | 10                  | 2                    |
| 46            | 11                  | 6                    |
| 42            | 12                  | 8                    |
| 39            | 13                  | 5                    |
| 36            | 14                  | 8                    |
| 34            | 15                  | 2                    |
| 32            | 16                  | 0                    |
| 30            | 17                  | 2                    |
| 28            | 18                  | 8                    |
| 26            | 19                  | 18                   |
| 25            | 20                  | 12                   |
| 24            | 21                  | 8                    |
| 23            | 22                  | 6                    |
| 22            | 23                  | 6                    |
| 21            | 24                  | 8                    |
| 20            | 25                  | 12                   |
| 19            | 26                  | 18                   |
| 18            | 28                  | 8                    |
| 17            | 30                  | 2                    |
| 16            | 32                  | 0                    |
| 15            | 34                  | 2                    |
| 14            | 36                  | 8                    |
| 13            | 39                  | 5                    |
| 12            | 42                  | 8                    |
| 11            | 46                  | 6                    |
| 10            | 51                  | 2                    |
| 9             | 56                  | 8                    |
| 8             | 64                  | 0                    |

Deleted records are filled with 00h's until reused.

Each of the supported data types is stored in the table file as follows:

| Data Type | Storage Method                                | Notes                                       |
| --------- | --------------------------------------------- | ------------------------------------------- |
| ASCII     | Standard ASCII encoding, 1 character per byte | Fields are null-padded                      |
| Numeric   | BCD-encoded numbers                           | Decimal point / precision defined in header |
| Date      | 3-byte binary day numbers                     | Date day-numbers are little endian          |
| Overlap   | Standard ASCII encoding, 1 character per byte | Fields are null-padded                      |


## The DataFlex `Filelist.cfg` File Format

The DataFlex `filelist.cfg` is contains one 128-byte record per associated table file. The order in which the entries
occur determines each table's "file number" (as referenced by the field definitions in each table's header). Record `0`
contains the "root file" name (`filelist.cfg`) and is otherwise null filled. Subsequent entries are composed exclusively
of ASCII-encoded, fixed-length, null-padded strings and are structured thusly:

| Description        | Starting Offset | Ending Offset | Size     |
| ------------------ | --------------- | ------------- | -------- |
| File Root Name     | **0x00**        | **0x28**      | 41 bytes |
| File Description   | **0x29**        | **0x49**      | 33 bytes |
| Dataflex File Name | **0x4A**        | **0x7F**      | 54 bytes |