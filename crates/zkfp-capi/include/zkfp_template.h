#ifndef ZKFP_TEMPLATE_H
#define ZKFP_TEMPLATE_H

#include "zkfp_common.h"

#ifdef __cplusplus
extern "C" {
#endif

int zkfp_extract_template(const unsigned char* bmp_data, uintptr_t bmp_size, ZkfpTemplate* out_template);
int zkfp_capture_full(ZkfpTemplate* out_template, char** out_base64_png);
int zkfp_capture_and_extract_template(ZkfpTemplate* out_template);
int zkfp_extract_from_image_file(const char* path, ZkfpTemplate* out_template, char** out_base64_png);
int zkfp_extract_from_bmp_file(const char* path, ZkfpTemplate* out_template, char** out_base64_png);

#ifdef __cplusplus
}
#endif

#endif
