# DataFlex Specifications

> **See Also:**
>
> - [General Reference](https://docs.dataaccess.com/dataflexhelp/mergedProjects/DevelopmentGuide/General_Reference.htm)
> - [DataFlex Data Types](https://docs.dataaccess.com/dataflexhelp/mergedProjects/LanguageGuide/Types.htm)
> - [Database Essentials](https://docs.dataaccess.com/dataflexhelp/mergedProjects/DevelopmentGuide/Database_Essentials.htm)
> - [File Names & Extension Types](https://docs.dataaccess.com/dataflexhelp/mergedProjects/DevelopmentGuide/Development_File_Names___Extensions.htm)
> - [Logical Structure of a Database File](https://docs.dataaccess.com/dataflexhelp/mergedProjects/DevelopmentGuide/logical_structure_of_a_database_file.htm)

## Structure

Extended relational DBMS with data-independent utilities and command language.

### Database Specifications (All Databases used with DataFlex)

|                                  |      |
| :------------------------------- | :--: |
| Maximum DBMS tables per filelist | 4095 |

### Embedded (DataFlex) Database Record & Table Specifications

|                                                              |                                                                              |
| :----------------------------------------------------------- | :--------------------------------------------------------------------------: |
| Maximum data elements (columns) per table                    |                                     255                                      |
| Maximum characters in physical table name (root name)        |                                      40                                      |
| Maximum characters in addressable table name (DataFlex name) | 31 (only use up to 8 if compatibility with revisions prior to DataFlex 17.0) |
| Maximum table file size                                      |                      2 Gigabytes (2,147,483,647 Bytes)                       |
| Maximum records per table                                    |                          16.7 Million (16,700,000)                           |
| Maximum record size                                          |                         16 Kilobytes (16,384 Bytes)                          |
| Data table type                                              |                      Packed, fixed-length random access                      |
| Numeric storage formats                                      |                    Packed BCD fixed point. Floating point                    |
| Numeric precision                                            |                  Fixed point: 8 places after decimal point.                  |
|                                                              |                    Floating point: 16 significant digits.                    |
|                                                              |                       DECIMAL type adds 16.16 support.                       |
| Numeric range                                                |      Fixed: ±9,999,999,999,999,999. 9999999999999999 with DECIMAL type.      |
|                                                              |                             Floating: ±1.0e±306                              |

### Embedded (DataFlex) Database Index Specifications

|                            |           |
| :------------------------- | :-------: |
| Maximum indexes per table  |    15     |
| Maximum segments per index |    16     |
| Maximum index key length   | 256 Bytes |

### Embedded (DataFlex) Database Column Specifications

|                                    |                                                                               |
| :--------------------------------- | :---------------------------------------------------------------------------: |
| Maximum characters in column name  | 32 (only use up to 15 if compatibility with revisions prior to DataFlex 17.0) |
| Maximum data element (column) size |                          16 Kilobytes (16,384 Bytes)                          |
| Maximum size of ASCII column       |                                255 characters                                 |
| Maximum size of text column        |                          16 Kilobytes (16,384 Bytes)                          |
| Size of date column                |                                3 Bytes (Fixed)                                |
| Maximum size of numeric column     |   99,999,999,999,999.99999999 to -9,999,999,999,999.99999999 (14.8 places)    |
| Maximum size of binary column      |                          16 Kilobytes (16,384 Bytes)                          |

## Application / Coding

Specifications for DataFlex data types can be found in the
[Language Guide](https://docs.dataaccess.com/dataflexhelp/mergedProjects/LanguageGuide/Types.htm).

All of the below specifications are per application. Each DataFlex application can call other applications
(including other DataFlex applications), and even return to the same place in the code after doing so using
the [runprogram](https://docs.dataaccess.com/dataflexhelp/mergedProjects/LanguageReference/runprogram_command.htm)
command.

|                                                                                     |                                                                                                                                                                         |
| :---------------------------------------------------------------------------------- | :---------------------------------------------------------------------------------------------------------------------------------------------------------------------: |
| Maximum lines per program                                                           |                                                                        128 million (128,000,000)                                                                        |
| Maximum line length in Code Editor                                                  |                                                                             4096 characters                                                                             |
| Maximum compilable statement length                                                 | 4095 characters (a statement exceeding 255 characters must be broken up into multiple lines of code, separated by a semicolon, which indicates a continuous statement). |
| Maximum custom error message length                                                 |                                                                             2048 characters                                                                             |
| Maximum number of devices that can be simultaneously opened for sequential file I/O |                                                                                   10                                                                                    |
| Program file characteristics                                                        |                                                                     Protected source, semi-compiled                                                                     |
| Maximum number of objects                                                           |                                                                              2,147,483,647                                                                              |
| Maximum object messages                                                             |                                                                                  3,072                                                                                  |
| Maximum number of classes                                                           |                                                                                 64,869                                                                                  |
| Maximum size of compiled program without symbols                                    |                                                                    2 Gigabytes (2,147,483,647 Bytes)                                                                    |
| Maximum data tables open concurrently                                               |                                                                           Memory limited only                                                                           |

### DataFlex Identifiers

DataFlex identifiers must begin with a letter `a-z` or `A-Z`.
This includes all identifiers in the language: variable names,
class names, object names, table names, etc. (i.e. any name
used in your program code).

### DataFlex Program File

When a DataFlex application is compiled, a file with a `.exe` extension is created (e.g. `MyApplication.exe`).
To execute a DataFlex application, simply double-click on the `.exe` file in Windows Explorer.
