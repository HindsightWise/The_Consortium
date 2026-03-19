#import "ane_bridge.h"
#import <Foundation/Foundation.h>
#import <objc/runtime.h>
#import <objc/message.h>
#import <dlfcn.h>
#import <IOSurface/IOSurface.h>

// Private Framework Path
#define ANE_FRAMEWORK_PATH "/System/Library/PrivateFrameworks/AppleNeuralEngine.framework/AppleNeuralEngine"

// Global Framework Handles
static void* g_ane_handle = NULL;

bool ane_bridge_init() {
    if (g_ane_handle) return true;
    
    g_ane_handle = dlopen(ANE_FRAMEWORK_PATH, RTLD_NOW);
    if (!g_ane_handle) {
        printf("   [ANE-Silicon] ❌ Failed to load AppleNeuralEngine private framework: %s\n", dlerror());
        return false;
    }
    
    printf("   [ANE-Silicon] ✅ AppleNeuralEngine (aarch64) Substrate Linked.\n");
    return true;
}

AneIOSurfaceHandle ane_iosurface_create(size_t size) {
    NSDictionary *dict = @{
        (id)kIOSurfaceWidth: @(size),
        (id)kIOSurfaceHeight: @1,
        (id)kIOSurfaceBytesPerElement: @1,
        (id)kIOSurfaceBytesPerRow: @(size),
        (id)kIOSurfaceAllocSize: @(size),
        (id)kIOSurfacePixelFormat: @0
    };
    
    IOSurfaceRef surf = IOSurfaceCreate((__bridge CFDictionaryRef)dict);
    return (AneIOSurfaceHandle)surf;
}

void ane_iosurface_destroy(AneIOSurfaceHandle surface) {
    if (surface) {
        CFRelease((IOSurfaceRef)surface);
    }
}

uint8_t* ane_iosurface_get_ptr(AneIOSurfaceHandle surface) {
    if (!surface) return NULL;
    IOSurfaceRef surf = (IOSurfaceRef)surface;
    IOSurfaceLock(surf, 0, NULL);
    void* ptr = IOSurfaceGetBaseAddress(surf);
    IOSurfaceUnlock(surf, 0, NULL);
    return (uint8_t*)ptr;
}

AneModelHandle ane_model_create_from_mil(const char* mil_text, const uint8_t* weights, size_t weights_size) {
    @autoreleasepool {
        @try {
            NSData *milData = [NSData dataWithBytes:mil_text length:strlen(mil_text)];
            NSData *weightBlob = [NSData dataWithBytes:weights length:weights_size];
            
            Class Desc = NSClassFromString(@"_ANEInMemoryModelDescriptor");
            Class IMM = NSClassFromString(@"_ANEInMemoryModel");
            
            if (!Desc || !IMM) {
                printf("   [ANE-Silicon] ❌ Private classes _ANEInMemoryModelDescriptor/Model not found.\n");
                return NULL;
            }

            // Options: Empty dictionary for now
            NSDictionary *wdict = @{
                @"@model_path/weights/weight.bin": @{@"offset": @0, @"data": weightBlob}
            };

            id desc = ((id(*)(Class,SEL,id,id,id))objc_msgSend)(
                Desc, @selector(modelWithMILText:weights:optionsPlist:),
                milData, wdict, @{});
                
            id model = ((id(*)(Class,SEL,id))objc_msgSend)(
                IMM, @selector(inMemoryModelWithDescriptor:), desc);
                
            // Keep the model alive by bridging it to void* (requires ARC management outside)
            return (__bridge_retained void*)model;
        } @catch (NSException *exception) {
            printf("   [ANE-Silicon] ❌ EXCEPTION in create_from_mil: %s\n", exception.reason.UTF8String);
            return NULL;
        }
    }
}

void ane_model_destroy(AneModelHandle model) {
    if (model) {
        id m = (__bridge_transfer id)model;
        (void)m; // Trigger dealloc
    }
}

bool ane_model_compile(AneModelHandle model) {
    if (!model) return false;
    @autoreleasepool {
        @try {
            id m = (__bridge id)model;
            NSError *e = nil;
            BOOL ok = ((BOOL(*)(id,SEL,unsigned int,id,NSError**))objc_msgSend)(
                m, @selector(compileWithQoS:options:error:), 21, @{}, &e);
            if (e) {
                printf("   [ANE-Silicon] ❌ NSError in compile: %s\n", e.localizedDescription.UTF8String);
            }
            return (bool)ok;
        } @catch (NSException *exception) {
            printf("   [ANE-Silicon] ❌ EXCEPTION in compile: %s\n", exception.reason.UTF8String);
            return false;
        }
    }
}

bool ane_model_load(AneModelHandle model) {
    if (!model) return false;
    @autoreleasepool {
        @try {
            id m = (__bridge id)model;
            NSError *e = nil;
            BOOL ok = ((BOOL(*)(id,SEL,unsigned int,id,NSError**))objc_msgSend)(
                m, @selector(loadWithQoS:options:error:), 21, @{}, &e);
            if (e) {
                printf("   [ANE-Silicon] ❌ NSError in load: %s\n", e.localizedDescription.UTF8String);
            }
            return (bool)ok;
        } @catch (NSException *exception) {
            printf("   [ANE-Silicon] ❌ EXCEPTION in load: %s\n", exception.reason.UTF8String);
            return false;
        }
    }
}

void ane_model_unload(AneModelHandle model) {
    if (!model) return;
    @autoreleasepool {
        @try {
            id m = (__bridge id)model;
            NSError *e = nil;
            ((BOOL(*)(id,SEL,unsigned int,NSError**))objc_msgSend)(
                m, @selector(unloadWithQoS:error:), 21, &e);
            if (e) {
                printf("   [ANE-Silicon] ❌ NSError in unload: %s\n", e.localizedDescription.UTF8String);
            }
        } @catch (NSException *exception) {
            printf("   [ANE-Silicon] ❌ EXCEPTION in unload: %s\n", exception.reason.UTF8String);
        }
    }
}

bool ane_model_evaluate(AneModelHandle model, AneIOSurfaceHandle input, AneIOSurfaceHandle output) {
    if (!model || !input || !output) return false;
    @autoreleasepool {
        @try {
            id m = (__bridge id)model;
            Class AR = NSClassFromString(@"_ANERequest");
            Class AIO = NSClassFromString(@"_ANEIOSurfaceObject");
            
            id wIn = ((id(*)(Class,SEL,IOSurfaceRef))objc_msgSend)(AIO, @selector(objectWithIOSurface:), (IOSurfaceRef)input);
            id wOut = ((id(*)(Class,SEL,IOSurfaceRef))objc_msgSend)(AIO, @selector(objectWithIOSurface:), (IOSurfaceRef)output);
            
            id req = ((id(*)(Class,SEL,id,id,id,id,id,id,id))objc_msgSend)(AR,
                @selector(requestWithInputs:inputIndices:outputs:outputIndices:weightsBuffer:perfStats:procedureIndex:),
                @[wIn], @[@0], @[wOut], @[@0], nil, nil, @0);

            NSError *e = nil;
            BOOL ok = ((BOOL(*)(id,SEL,unsigned int,id,id,NSError**))objc_msgSend)(
                m, @selector(evaluateWithQoS:options:request:error:),
                21, @{}, req, &e);
                
            if (e) {
                printf("   [ANE-Silicon] ❌ NSError in evaluate: %s\n", e.localizedDescription.UTF8String);
            }
            return (bool)ok;
        } @catch (NSException *exception) {
            printf("   [ANE-Silicon] ❌ EXCEPTION in evaluate: %s\n", exception.reason.UTF8String);
            return false;
        }
    }
}
