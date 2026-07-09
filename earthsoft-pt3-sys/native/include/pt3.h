#pragma once

#include "pt3_bus.h"
#include "pt3_device.h"
#include <stdint.h>

#ifdef  __cplusplus
extern "C" {
#endif

typedef struct Pt3Bus Pt3Bus;

int32_t LoadPt3Lib(void);
int32_t FreePt3Lib(void);
int32_t CreatePt3Bus(Pt3Bus **bus);

#ifdef  __cplusplus
}
#endif
