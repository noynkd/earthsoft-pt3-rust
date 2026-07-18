# BonDriver

## BonDriver.h

```cpp
// IBonDriver.h: IBonDriver クラスのインターフェイス
//
//////////////////////////////////////////////////////////////////////

#if !defined(_IBONDRIVER_H_)
#define _IBONDRIVER_H_

#if _MSC_VER > 1000
#pragma once
#endif // _MSC_VER > 1000


// 凡ドライバインタフェース
class IBonDriver
{
public:
    virtual const BOOL OpenTuner(void) = 0;
    virtual void CloseTuner(void) = 0;

    virtual const BOOL SetChannel(const BYTE bCh) = 0;
    virtual const float GetSignalLevel(void) = 0;

    virtual const DWORD WaitTsStream(const DWORD dwTimeOut = 0) = 0;
    virtual const DWORD GetReadyCount(void) = 0;

    virtual const BOOL GetTsStream(BYTE *pDst, DWORD *pdwSize, DWORD *pdwRemain) = 0;
    virtual const BOOL GetTsStream(BYTE **ppDst, DWORD *pdwSize, DWORD *pdwRemain) = 0;

    virtual void PurgeTsStream(void) = 0;

    virtual void Release(void) = 0;
};


// インスタンス生成メソッド
extern "C" __declspec(dllimport) IBonDriver * CreateBonDriver();


#endif // !defined(_IBONDRIVER_H_)
```

## BonDriver2.h

```cpp
// IBonDriver2.h: IBonDriver2 クラスのインターフェイス
//
//////////////////////////////////////////////////////////////////////

#if !defined(_IBONDRIVER2_H_)
#define _IBONDRIVER2_H_

#if _MSC_VER > 1000
#pragma once
#endif // _MSC_VER > 1000


#include "IBonDriver.h"


// 凡ドライバインタフェース2
class IBonDriver2 : public IBonDriver
{
public:
    virtual LPCTSTR GetTunerName(void) = 0;

    virtual const BOOL IsTunerOpening(void) = 0;

    virtual LPCTSTR EnumTuningSpace(const DWORD dwSpace) = 0;
    virtual LPCTSTR EnumChannelName(const DWORD dwSpace, const DWORD dwChannel) = 0;

    virtual const BOOL SetChannel(const DWORD dwSpace, const DWORD dwChannel) = 0;

    virtual const DWORD GetCurSpace(void) = 0;
    virtual const DWORD GetCurChannel(void) = 0;

// IBonDriver
    virtual void Release(void) = 0;
};

#endif // !defined(_IBONDRIVER2_H_)
```

## BonDriver3.h

```cpp
// IBonDriver3.h: IBonDriver3 クラスのインターフェイス
//
/////////////////////////////////////////////////////////////////////////////

#if !defined(_IBONDRIVER3_H_)
#define _IBONDRIVER3_H_

#if _MSC_VER > 1000
#pragma once
#endif // _MSC_VER > 1000


#include "IBonDriver2.h"


/////////////////////////////////////////////////////////////////////////////
// Bonドライバインタフェース3
/////////////////////////////////////////////////////////////////////////////

class IBonDriver3 : public IBonDriver2
{
public:
// IBonDriver3
    virtual const DWORD GetTotalDeviceNum(void) = 0;
    virtual const DWORD GetActiveDeviceNum(void) = 0;
    virtual const BOOL SetLnbPower(const BOOL bEnable) = 0;

// IBonDriver
    virtual void Release(void) = 0;
};
#endif
```
