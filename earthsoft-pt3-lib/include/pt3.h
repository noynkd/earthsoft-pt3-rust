#pragma once

#include "Bus.h"
#include "Device.h"
#include "Status.h"
#include <cstdint>

namespace Earthsoft::Pt3 {
    using NewBusFunction = std::int32_t (*)(Bus **bus);
}
