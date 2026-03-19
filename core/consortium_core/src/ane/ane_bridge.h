#ifndef ANE_BRIDGE_H
#define ANE_BRIDGE_H

#include <stdint.h>
#include <stdbool.h>
#include <stddef.h>

typedef void* AneModelHandle;
typedef void* AneRequestHandle;
typedef void* AneIOSurfaceHandle;

typedef struct {
    uint8_t* data;
    size_t size;
} AneBuffer;

// Initialization & Framework Loading
bool ane_bridge_init();

// Model Lifecycle
AneModelHandle ane_model_create_from_mil(const char* mil_text, const uint8_t* weights, size_t weights_size);
void ane_model_destroy(AneModelHandle model);

bool ane_model_compile(AneModelHandle model);
bool ane_model_load(AneModelHandle model);
void ane_model_unload(AneModelHandle model);

// IO Surface Management (Zero-Copy)
AneIOSurfaceHandle ane_iosurface_create(size_t size);
void ane_iosurface_destroy(AneIOSurfaceHandle surface);
uint8_t* ane_iosurface_get_ptr(AneIOSurfaceHandle surface);

// Inference
bool ane_model_evaluate(AneModelHandle model, AneIOSurfaceHandle input, AneIOSurfaceHandle output);

#endif
