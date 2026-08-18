#ifndef ZKFP_COMMON_H
#define ZKFP_COMMON_H

#include <stdint.h>

#ifdef __cplusplus
extern "C" {
#endif

typedef struct ZkfpEnhanceConfig {
    int apply_enhancement;
    int method;
    unsigned char bg_intensity;
    int invert;
    int flip_vertical;
    unsigned int padding;
} ZkfpEnhanceConfig;

typedef struct ZkfpTemplate {
    unsigned char* data;
    uintptr_t size;
    uint32_t quality;
} ZkfpTemplate;

typedef struct ZkfpIdentifyVerifyResult {
    uint32_t user_id;
    int identify_score;
    int verify_score;
    int identify_match;
    int verify_match;
} ZkfpIdentifyVerifyResult;

const char* zkfp_get_last_error(void);
void zkfp_free_string(char* s);
void zkfp_free_template(ZkfpTemplate* tmpl);

#ifdef __cplusplus
}
#endif

#endif
