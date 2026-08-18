#ifndef ZKFP_MATCH_H
#define ZKFP_MATCH_H

#include "zkfp_common.h"

#ifdef __cplusplus
extern "C" {
#endif

int zkfp_verify_templates(const unsigned char* tmpl1_data, uintptr_t tmpl1_size, const unsigned char* tmpl2_data, uintptr_t tmpl2_size);
void zkfp_gallery_clear(void);
int zkfp_gallery_add(uint32_t user_id, const unsigned char* tmpl_data, uintptr_t tmpl_size);
int zkfp_gallery_load_from_db(const char* table_name, const char* user_id_column, const char* template_column);
void zkfp_gallery_remove(uint32_t user_id);
int zkfp_gallery_identify(const unsigned char* probe_data, uintptr_t probe_size, uint32_t* out_id, int* out_score);
int zkfp_gallery_identify_with_verification(const unsigned char* probe_data, uintptr_t probe_size, ZkfpIdentifyVerifyResult* out_result);

#ifdef __cplusplus
}
#endif

#endif
