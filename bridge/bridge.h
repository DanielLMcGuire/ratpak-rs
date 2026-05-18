#pragma once
#include <stdint.h>
#include <stdbool.h>

#ifdef __cplusplus
extern "C" {
#endif

typedef struct _rat* PRAT_C;
typedef struct _number* PNUMBER_C;

void init_ratpack(uint32_t radix, int32_t precision);
void free_rat(PRAT_C rat);
void free_num(PNUMBER_C num);
PRAT_C clone_rat(PRAT_C rat);
void free_string(uint16_t* s);
PRAT_C rat_from_i32(int32_t val);
PRAT_C rat_from_u32(uint32_t val);
uint32_t rat_to_u64(PRAT_C rat, uint32_t radix, int32_t precision, uint64_t* out);
uint32_t rat_to_string(PRAT_C rat, uint32_t radix, int32_t format, int32_t precision, uint16_t** out_str);
uint32_t wrap_addrat(PRAT_C a, PRAT_C b, int32_t precision, PRAT_C* out);
uint32_t wrap_subrat(PRAT_C a, PRAT_C b, int32_t precision, PRAT_C* out);
uint32_t wrap_mulrat(PRAT_C a, PRAT_C b, int32_t precision, PRAT_C* out);
uint32_t wrap_divrat(PRAT_C a, PRAT_C b, int32_t precision, PRAT_C* out);
uint32_t wrap_modrat(PRAT_C a, PRAT_C b, int32_t precision, PRAT_C* out);
uint32_t wrap_rat_equ(PRAT_C a, PRAT_C b, int32_t precision, bool* out);
uint32_t wrap_rat_lt(PRAT_C a, PRAT_C b, int32_t precision, bool* out);
uint32_t wrap_sinrat(PRAT_C x, int32_t angle_type, uint32_t radix, int32_t precision, PRAT_C* out);
uint32_t wrap_exprat(PRAT_C x, uint32_t radix, int32_t precision, PRAT_C* out);
uint32_t wrap_lograt(PRAT_C x, int32_t precision, PRAT_C* out);

#ifdef __cplusplus
}
#endif