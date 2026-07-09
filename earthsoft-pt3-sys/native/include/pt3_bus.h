#pragma once

#include <stdint.h>

#ifdef  __cplusplus
extern "C" {
#endif

typedef struct Pt3DeviceInfo {
    uint32_t Bus;
    uint32_t Slot;
    uint32_t Function;
    uint32_t PtVersion;
} Pt3DeviceInfo;

typedef struct Pt3Bus Pt3Bus;
typedef struct Pt3Device Pt3Device;

int32_t DeletePt3Bus(Pt3Bus *bus);
int32_t GetPt3Version(Pt3Bus *bus, uint32_t *version);
int32_t ScanPt3DeviceInfo(Pt3Bus *bus, Pt3DeviceInfo *deviceInfo, uint32_t *deviceInfoCount);
int32_t CreatePt3Device(Pt3Bus *bus, const Pt3DeviceInfo *deviceInfo, Pt3Device **device/*, void **__device__ */);

#ifdef  __cplusplus
}
#endif
