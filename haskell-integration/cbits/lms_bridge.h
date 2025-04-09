#ifndef LMS_BRIDGE_H
#define LMS_BRIDGE_H

#include <stdint.h>

#ifdef __cplusplus
extern "C" {
#endif

// Haskell runtime initialization/cleanup
void hs_init(int *argc, char ***argv);
void hs_exit(void);
void hs_perform_gc(void);

// Sync engine
void* hs_process_batch(const void* ops, uint64_t count);
uint64_t hs_get_result_count(const void* result);
void hs_get_results(const void* result, void* out, uint64_t count);
void hs_free_result(void* result);

// Blockchain verification
int hs_verify_block(const uint8_t* prev_hash, const uint8_t* current_hash);
int hs_verify_chain(const uint8_t* hashes, uint64_t hash_count);

// Query optimization
void* hs_execute_query(const char* query_json, const char* data_json);
void hs_free_query_results(void* results);

// Parser interface
char* hs_parse_completion_rule(const char* rule_text);
char* hs_parse_query(const char* query_text);
char* hs_optimize_query(const char* query_json);
void hs_free_string(char* ptr);

#ifdef __cplusplus
}
#endif

#endif // LMS_BRIDGE_H