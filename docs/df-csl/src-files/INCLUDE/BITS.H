#define bit_runtime 1
#define bit_demo    2
#define bit_noquery 4
#define bit_graphics 8
#define bit_multiuser 16
#define bit_nomultiflx 32
#define bit_lockflx  64
#define bit_nomakefile 128
#define bit_ow       256
#define bit_lrt      512
#define bit_tempreg  1024
#define lrt_bits ( bit_demo | bit_lockflx | bit_lrt)

extern int32 newdac();
#define ck_code (newdac(sc.tname,88) == sc.tcode)
#define ck_bit(b)  (sexy32(sc.bits) & b)
#define bad_bit errexit()

