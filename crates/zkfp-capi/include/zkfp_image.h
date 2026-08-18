#ifndef ZKFP_IMAGE_H
#define ZKFP_IMAGE_H

#include "zkfp_common.h"

#ifdef __cplusplus
extern "C" {
#endif

char* zkfp_capture_image_base64(const char* format);
int zkfp_image_file_to_base64(const char* path, const char* format, char** out_base64);
int zkfp_set_enhance_config(const ZkfpEnhanceConfig* config);
int zkfp_get_enhance_config(ZkfpEnhanceConfig* config);
int zkfp_set_contrast_method(int method);
int zkfp_set_invert(int invert);
int zkfp_set_flip_vertical(int flip);
int zkfp_set_bg_intensity(unsigned char intensity);
int zkfp_set_padding(unsigned int padding);
int zkfp_set_enhancement_enabled(int enabled);

#ifdef __cplusplus
}
#endif

#endif
