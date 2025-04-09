#include <stdint.h>
#include <stdlib.h>
#include "lms_bridge.h"

// Declarations for Haskell functions
extern void* hs_sync_process_batch(const void* ops, uint64_t count);
extern uint64_t hs_sync_get_result_count(const void* result);
extern void hs_sync_get_results(const void* result, void* out, uint64_t count);
extern void hs_sync_free_result(void* result);

// C wrapper functions
void* hs_process_batch(const void* ops, uint64_t count) {
    return hs_sync_process_batch(ops, count);
}

uint64_t hs_get_result_count(const void* result) {
    return hs_sync_get_result_count(result);
}

void hs_get_results(const void* result, void* out, uint64_t count) {
    hs_sync_get_results(result, out, count);
}

void hs_free_result(void* result) {
    hs_sync_free_result(result);
}