#ifndef ZKFP_USB_H
#define ZKFP_USB_H

#include "zkfp_common.h" // IWYU pragma: export

#ifdef __cplusplus
extern "C" {
#endif

int zkfp_init(void);
void zkfp_close(void);

#ifdef __cplusplus
}
#endif

#endif
