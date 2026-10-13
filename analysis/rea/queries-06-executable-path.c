
undefined8 * FUN_14019d900(undefined8 *param_1,longlong *param_2)

{
  longlong lVar1;
  DWORD DVar2;
  DWORD DVar3;
  ulonglong uVar4;
  longlong lVar5;
  longlong lVar6;
  longlong lVar7;
  LPWSTR lpFilename;
  ulonglong uVar8;
  LPVOID lpAddress;
  longlong lVar9;
  
  lpAddress = (LPVOID)param_2[5];
  lVar7 = 0;
  uVar8 = param_2[3];
  *param_1 = 0;
  param_1[1] = 0;
  lVar5 = param_2[4];
  uVar4 = (ulonglong)((int)lVar5 + (int)lpAddress & 1);
  lVar6 = 2 - uVar4;
  if (uVar4 == 0) {
    lVar6 = lVar7;
  }
  lVar9 = lVar6 + 0x1000;
  if (uVar8 < (ulonglong)(lVar5 + lVar9)) {
    if ((ulonglong)param_2[2] < (ulonglong)(lVar5 + lVar9)) {
      FUN_1401b5c50(param_2,lVar9,2);
      lpAddress = (LPVOID)param_2[5];
      lVar5 = param_2[4];
      uVar8 = param_2[3];
      uVar4 = (ulonglong)((int)lVar5 + (int)lpAddress & 1);
      lVar6 = 2 - uVar4;
      if (uVar4 == 0) {
        lVar6 = lVar7;
      }
      lVar9 = lVar6 + 0x1000;
    }
    lVar1 = *param_2;
    if ((lVar1 != 0) && (uVar4 = lVar1 - 1U & lVar5 + lVar9, uVar4 != 0)) {
      lVar7 = lVar1 - uVar4;
    }
    uVar4 = lVar7 + lVar5 + lVar9;
    if (uVar8 < uVar4) {
      param_2[3] = uVar4;
      VirtualAlloc(lpAddress,uVar4,0x1000,4);
    }
  }
  lpFilename = (LPWSTR)(param_2[5] + lVar6 + param_2[4]);
  FUN_1401bace0(lpFilename,0,0x1000);
  param_2[4] = param_2[4] + lVar9;
  param_2[6] = (longlong)lpFilename;
  DVar2 = GetModuleFileNameW((HMODULE)0x0,lpFilename,0x800);
  if ((DVar2 == 0x800) && (DVar3 = GetLastError(), DVar3 == 0x7a)) {
    return param_1;
  }
  *param_1 = lpFilename;
  param_1[1] = (ulonglong)DVar2;
  return param_1;
}

