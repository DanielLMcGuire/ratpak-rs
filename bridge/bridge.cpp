#include "bridge.h"
#include <string>
#include <algorithm>
#include "../thirdparty/Header Files/Rational.h"
#include "../thirdparty/Header Files/RationalMath.h"
#include "../thirdparty/Ratpack/ratpak.h"

extern "C" {

void init_ratpack(uint32_t radix, int32_t precision) {
    ChangeConstants(radix, precision);
}

void free_rat(PRAT_C rat) {
    if (rat) {
        _destroyrat(rat);
    }
}

void free_num(PNUMBER_C num) {
    if (num) {
        _destroynum(num);
    }
}

PRAT_C clone_rat(PRAT_C rat) {
    PRAT cloned = nullptr;
    DUPRAT(cloned, rat);
    return cloned;
}

void free_string(uint16_t* s) {
    delete[] s;
}

PRAT_C rat_from_i32(int32_t val) {
    return i32torat(val);
}

PRAT_C rat_from_u32(uint32_t val) {
    return Ui32torat(val);
}

uint32_t rat_to_u64(PRAT_C rat, uint32_t radix, int32_t precision, uint64_t* out) {
    try {
        *out = rattoUi64(rat, radix, precision);
        return 0;
    } catch (uint32_t err) {
        return err;
    }
}

uint32_t rat_to_string(PRAT_C rat, uint32_t radix, int32_t format, int32_t precision, uint16_t** out_str) {
    try {
        std::wstring s = RatToString(rat, static_cast<NumberFormat>(format), radix, precision);
        *out_str = new uint16_t[s.length() + 1];
        std::copy(s.begin(), s.end(), *out_str);
        (*out_str)[s.length()] = 0;
        return 0;
    } catch (uint32_t err) {
        return err;
    }
}

uint32_t wrap_addrat(PRAT_C a, PRAT_C b, int32_t precision, PRAT_C* out) {
    try {
        PRAT result = clone_rat(a);
        addrat(&result, b, precision);
        *out = result;
        return 0;
    } catch (uint32_t err) {
        return err;
    }
}

uint32_t wrap_subrat(PRAT_C a, PRAT_C b, int32_t precision, PRAT_C* out) {
    try {
        PRAT result = clone_rat(a);
        subrat(&result, b, precision);
        *out = result;
        return 0;
    } catch (uint32_t err) {
        return err;
    }
}

uint32_t wrap_mulrat(PRAT_C a, PRAT_C b, int32_t precision, PRAT_C* out) {
    try {
        PRAT result = clone_rat(a);
        mulrat(&result, b, precision);
        *out = result;
        return 0;
    } catch (uint32_t err) {
        return err;
    }
}

uint32_t wrap_divrat(PRAT_C a, PRAT_C b, int32_t precision, PRAT_C* out) {
    try {
        PRAT result = clone_rat(a);
        divrat(&result, b, precision);
        *out = result;
        return 0;
    } catch (uint32_t err) {
        return err;
    }
}

uint32_t wrap_modrat(PRAT_C a, PRAT_C b, int32_t precision, PRAT_C* out) {
    try {
        PRAT result = clone_rat(a);
        modrat(&result, b);
        *out = result;
        return 0;
    } catch (uint32_t err) {
        return err;
    }
}

uint32_t wrap_rat_equ(PRAT_C a, PRAT_C b, int32_t precision, bool* out) {
    try {
        *out = rat_equ(a, b, precision);
        return 0;
    } catch (uint32_t err) {
        return err;
    }
}

uint32_t wrap_rat_lt(PRAT_C a, PRAT_C b, int32_t precision, bool* out) {
    try {
        *out = rat_lt(a, b, precision);
        return 0;
    } catch (uint32_t err) {
        return err;
    }
}

uint32_t wrap_sinrat(PRAT_C x, int32_t angle_type, uint32_t radix, int32_t precision, PRAT_C* out) {
    try {
        PRAT result = clone_rat(x);
        sinanglerat(&result, static_cast<AngleType>(angle_type), radix, precision);
        *out = result;
        return 0;
    } catch (uint32_t err) {
        return err;
    }
}

uint32_t wrap_exprat(PRAT_C x, uint32_t radix, int32_t precision, PRAT_C* out) {
    try {
        PRAT result = clone_rat(x);
        exprat(&result, radix, precision);
        *out = result;
        return 0;
    } catch (uint32_t err) {
        return err;
    }
}

uint32_t wrap_lograt(PRAT_C x, int32_t precision, PRAT_C* out) {
    try {
        PRAT result = clone_rat(x);
        lograt(&result, precision);
        *out = result;
        return 0;
    } catch (uint32_t err) {
        return err;
    }
}

uint32_t wrap_cosrat(PRAT_C x, int32_t angle_type, uint32_t radix, int32_t precision, PRAT_C* out) {
    try {
        PRAT result = clone_rat(x);
        cosanglerat(&result, static_cast<AngleType>(angle_type), radix, precision);
        *out = result;
        return 0;
    } catch (uint32_t err) {
        return err;
    }
}

} // extern "C"