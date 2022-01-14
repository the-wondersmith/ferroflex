meta:
  id: table
  title: DataFlex v3 Database File
  application: DataFlex v3
  file-extension: dat
  ks-version: 0.9
  encoding: ASCII
  endian: le
  bit-endian: le
doc: |
    A DataFlex flat-file table.
doc-ref: https://web.archive.org/web/20210115231834/hwiegman.home.xs4all.nl/fileformats/dat/DATAFLEX.txt
seq:
  - id: header
    size: 3072
    type: header
  - id: null_record
    size: header.record_length
  - id: record_blocks
    type: record_block
    repeat: expr
    repeat-expr: header.record_count - 1
    

enums:
  enabled_disabled:
    0: disabled
    1: enabled
  true_false:
    0: false
    1: true
  active_inactive:
    0: active
    1: inactive
  compression_type:
    0: none
    1: fast
    2: standard
    3: custom
  index_type:
    0: default
    1: descending_ascending
    2: uppercase
  field_type:
    0: ascii
    1: numeric
    2: date
    3: overlap
    5: text
    6: binary

types:
  index_definition:
    seq:
      - id: field_count
        doc: |
          DOCUMENTATION NEEDED
        type: u1
      - id: field_numbers
        doc: |
          DOCUMENTATION NEEDED
        size: 16
      - id: flag_byte
        doc: |
          DOCUMENTATION NEEDED
        type: u1
        enum: index_type
  field_definition:
    seq:
      - id: offset
        doc: |
          DOCUMENTATION NEEDED
        type: u2le
      - id: main_index
        doc: |
          DOCUMENTATION NEEDED
        type: b4
      - id: decimal_points
        doc: |
          DOCUMENTATION NEEDED
        type: b4
      - id: length
        doc: |
          DOCUMENTATION NEEDED
        type: u1
      - id: type
        doc: |
          DOCUMENTATION NEEDED
        type: u1
        enum: field_type
      - id: file_number
        doc: |
          DOCUMENTATION NEEDED
        type: u1
      - id: field_number
        doc: |
          DOCUMENTATION NEEDED
        type: u2le
  header:
    instances:
      higest_record_count_ever:
        pos: 0x00
        doc: |
          The total number of records that have ever been stored in a given table
        type: u4le
      first_reused_record:
        pos: 0x04
        doc: |
          Un-deciphered value, has something to do with reuse of deleted space.
          Should be 0 if deleted space is not re-used, but could possibly be the
          first available record # otherwise.
        type: u4le
      record_count:
        pos: 0x08
        doc: |
          The total number of records currently in the table
        type: u4le
      max_record_count:
        pos: 0x0C
        doc: |
          The total number of records the table can store at any given time
        type: u4le
      header_integrity1:
        pos: 0x10
        doc: |
          A flag value for header integrity. Should be 0 if header integrity
          is disabled, or a checksum value otherwise.
        type: u4le
      checksum1:
        pos: 0x14
        doc: |
          A table / header integrity checksum value.
        type: u4le
      checksum2:
        pos: 0x18
        doc: |
          A table / header integrity checksum value.
        type: u4le
      table_version1:
        pos: 0x1C
        doc: |
          The version of the table
        type: u1
      table_version2:
        pos: 0x1D
        doc: |
          The version of the table
        type: u1
      static_zero1:
        pos: 0x1E
        doc: |
          Should always be 0
        type: u1
      compression_type:
        pos: 0x1E
        doc: |
          The type of compression used by the table
        type: u1
        enum: compression_type
      first_available_record:
        pos: 0x20
        doc: |
          The location of the first available record
        type: u4le
      static_zero2:
        pos: 0x28
        doc: |
          Should be 0, use uknown if compresion is enabled
        type: u4le
      static_zero3:
        pos: 0x30
        doc: |
          Should be 0, use uknown if compresion is enabled
        type: u4le
      checksum3:
        pos: 0x38
        doc: |
          A table / header integrity checksum value.
        type: u4le
      static_zero4:
        pos: 0x3C
        doc: |
          Should always be 0
        type: u4le
      header_integrity2:
        pos: 0x40
        doc: |
          Should be 0 if header integrity is disabled, otherwise
          its use is unknown.
        type: u1
      file_locking1:
        pos: 0x41
        doc: |
          Flag value for file locking. 1=True 0=false
        type: u1
        enum: enabled_disabled
      reuse_deleted_space:
        pos: 0x4A
        doc: |
          0=Reuse deleted space -1= do not.  -1= Compression
        type: s2le
      unknown_checksum1:
        pos: 0x4C
        doc: |
          0=Reuse deleted space -1= do not.  -1= Compression
        type: s2le
      checksum4:
        pos: 0x50
        doc: |
          A table / header integrity checksum value.
        type: u4le
      record_remainder:
        pos: 0x98
        doc: |
          Records until remainder block. Basically number of records per
          512 byte block or, 1 for anything that has no remainder blocks.
        type: u2le
      record_length:
        pos: 0x9a
        doc: |
          The total number of bytes occuped by one record in the table
        type: u2le
      reuse_deleted_records:
        pos: 0xA4
        doc: |
          0=Reuse deleted space -1= do not.  -1= Compression
        type: u1
        enum: true_false
      fields_per_record:
        pos: 0xA5
        doc: |
          Number of fields per record.
        type: u1
      file_locking:
        pos: 0xA8
        doc: |
          Flag value for if file locking is enabled.
        type: u1
        enum: true_false
      static_one1:
        pos: 0xA9
        doc: |
          Unknown value, should always be 1.
        type: u1
      static_one2:
        pos: 0xAD
        doc: |
          Unknown value, should always be 1.
        type: u1
      unknown_byte:
        pos: 0xAA
        doc: |
          Unknown value.
        type: u1
      index_table:
        pos: 0xB0
        doc: |
          All of the indexes associated with the table
        type: index_definition
        repeat: expr
        repeat-expr: 16
      table_name:
        pos: 0x2D0
        doc: |
          File root name, null padded to 16 bytes.
        type: str
        size: 16
        encoding: ASCII
      fields:
        pos: 0x2E0
        doc: |
          Deffinitions of all the fields in the table
        type: field_definition
        repeat: expr
        repeat-expr: 256
  record:
    seq:
      - id: record
        size: _root.header.record_length
  record_block:
    seq:
      - id: records
        type: record
        repeat: expr
        repeat-expr: _root.header.record_remainder
#      - id: padding
#        size: _root.header.record_remainder == 1 ? 0 : 12
