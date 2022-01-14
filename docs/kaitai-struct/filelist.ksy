meta:
  id: filelist
  title: DataFlex v2.3 / v3 Filelist
  application: DataFlex v2.3 / v3
  file-extension: cfg
  ks-version: 0.9
  encoding: ASCII
doc: |
    A DataFlex table file list.
doc-ref: https://web.archive.org/web/20210115231834/hwiegman.home.xs4all.nl/fileformats/dat/DATAFLEX.txt
seq:
  - id: header
    doc: |
      This is usually just 'filelist.cfg' + 116 bytes of padding.
    type: strz
    size: 128
  - id: tables
    type: entry
    repeat: eos
types:
  entry:
    seq:
      - id: root_name
        doc: |
          The root name of the table's .DAT file
        type: strz
        size: 41
      - id: table_name
        doc: |
          The table's human-readable name
        type: strz
        size: 33
      - id: description
        doc: |
          A description of the table's use or purpose
        type: strz
        size: 54



