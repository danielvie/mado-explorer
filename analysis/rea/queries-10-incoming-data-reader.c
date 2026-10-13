
void FUN_1401949d0(longlong *param_1,longlong *param_2)

{
  undefined4 *puVar1;
  undefined1 auVar2 [16];
  char cVar3;
  longlong lVar4;
  int iVar5;
  UINT UVar6;
  UINT UVar7;
  HDROP hDrop;
  ulonglong uVar8;
  undefined1 (*pauVar9) [16];
  longlong lVar10;
  ulonglong uVar11;
  ulonglong uVar12;
  uint uVar13;
  ulonglong uVar14;
  ulonglong uVar15;
  LPVOID lpAddress;
  ulonglong uVar16;
  longlong lVar17;
  ulonglong uVar18;
  longlong local_2d8;
  undefined4 uStack_2d0;
  undefined4 uStack_2cc;
  STGMEDIUM local_2c8;
  WCHAR *local_2a8;
  ulonglong local_2a0;
  undefined2 local_298 [4];
  undefined8 local_290;
  undefined4 local_288;
  undefined4 local_284;
  undefined4 local_280;
  undefined8 local_278;
  undefined8 uStack_270;
  undefined8 local_268;
  undefined1 local_260 [24];
  WCHAR local_248 [264];
  
  local_288 = 1;
  local_284 = 0xffffffff;
  local_298[0] = 0xf;
  local_280 = 1;
  local_2c8.pUnkForRelease = (IUnknown *)0x0;
  uVar12 = 0;
  local_290 = 0;
  local_2c8.tymed = 0;
  local_2c8._4_4_ = 0;
  local_2c8.u.hBitmap = (HBITMAP)0x0;
  iVar5 = (**(code **)(*param_2 + 0x18))(param_2,local_298,&local_2c8);
  if (iVar5 < 0) {
    return;
  }
  hDrop = GlobalLock(local_2c8.u.hGlobal);
  if (hDrop == (HDROP)0x0) goto LAB_140194cf6;
  UVar6 = DragQueryFileW(hDrop,0xffffffff,(LPWSTR)0x0,0);
  lVar10 = param_1[0xb];
  uVar14 = (ulonglong)UVar6;
  if (lVar10 == 0) {
    uVar11 = 1;
    if (1 < uVar14) {
      do {
        uVar11 = uVar11 * 2;
      } while (uVar11 < uVar14);
    }
    lVar10 = param_1[4];
    lpAddress = (LPVOID)param_1[5];
    uVar16 = param_1[3];
    uVar8 = (ulonglong)((int)lVar10 + (int)lpAddress & 7);
    uVar18 = 8 - uVar8;
    if (uVar8 == 0) {
      uVar18 = uVar12;
    }
    lVar17 = uVar18 + uVar11 * 0x10;
    if (uVar16 < (ulonglong)(lVar10 + lVar17)) {
      if ((ulonglong)param_1[2] < (ulonglong)(lVar10 + lVar17)) {
        FUN_1401b5c50(param_1,lVar17,8);
        lpAddress = (LPVOID)param_1[5];
        lVar10 = param_1[4];
        uVar16 = param_1[3];
        uVar8 = (ulonglong)((int)lVar10 + (int)lpAddress & 7);
        uVar18 = 8 - uVar8;
        if (uVar8 == 0) {
          uVar18 = uVar12;
        }
        lVar17 = uVar11 * 0x10 + uVar18;
      }
      lVar4 = *param_1;
      uVar8 = uVar12;
      if ((lVar4 != 0) && (uVar15 = lVar4 - 1U & lVar10 + lVar17, uVar15 != 0)) {
        uVar8 = lVar4 - uVar15;
      }
      uVar8 = uVar8 + lVar10 + lVar17;
      if (uVar16 < uVar8) {
        param_1[3] = uVar8;
        VirtualAlloc(lpAddress,uVar8,0x1000,4);
      }
    }
    lVar10 = param_1[4] + uVar18 + param_1[5];
    param_1[6] = lVar10;
    param_1[4] = param_1[4] + lVar17;
    param_1[9] = 0;
LAB_140194c0b:
    param_1[10] = uVar11;
  }
  else {
    uVar16 = param_1[10];
    uVar11 = param_1[9] + uVar14;
    if (uVar16 < uVar11) {
      if (uVar11 < uVar16 * 2) {
        uVar11 = uVar16 * 2;
      }
      lVar10 = FUN_1401b5ae0(param_1,lVar10,uVar16 << 4,uVar11 << 4,8,0,
                             "C:\\Users\\Vjekoslav\\Projects\\Voidstar\\File Pilot\\Source\\Program\\Voidstar.h"
                             ,0xa4b,"_ArrayGrow");
      goto LAB_140194c0b;
    }
  }
  param_1[9] = param_1[9] + uVar14;
  param_1[0xb] = lVar10;
  uVar14 = uVar12;
  if (UVar6 != 0) {
    do {
      local_268 = 0;
      local_2a8 = local_248;
      local_278 = 0;
      uStack_270 = 0;
      UVar7 = DragQueryFileW(hDrop,(UINT)uVar14,local_248,0x104);
      local_2a0 = (ulonglong)UVar7;
      pauVar9 = (undefined1 (*) [16])FUN_140188d50(local_260,param_1,&local_2a8,&local_278);
      auVar2 = *pauVar9;
      uStack_2d0 = auVar2._8_4_;
      uStack_2cc = auVar2._12_4_;
      lVar10 = auVar2._8_8_;
      local_2d8 = auVar2._0_8_;
      if (lVar10 != 0) {
        do {
          cVar3 = *(char *)(local_2d8 + -1 + lVar10);
          if ((cVar3 != '\\') && (cVar3 != '/')) break;
          lVar10 = lVar10 + -1;
        } while (lVar10 != 0);
        uStack_2d0 = (undefined4)lVar10;
        uStack_2cc = (undefined4)((ulonglong)lVar10 >> 0x20);
      }
      uVar13 = (UINT)uVar14 + 1;
      puVar1 = (undefined4 *)(param_1[0xb] + uVar12);
      *puVar1 = auVar2._0_4_;
      puVar1[1] = auVar2._4_4_;
      puVar1[2] = uStack_2d0;
      puVar1[3] = uStack_2cc;
      uVar12 = uVar12 + 0x10;
      uVar14 = (ulonglong)uVar13;
    } while (uVar13 < UVar6);
  }
  *(undefined4 *)((longlong)param_1 + 0x44) = 1;
  GlobalUnlock(local_2c8.u.hGlobal);
LAB_140194cf6:
  ReleaseStgMedium(&local_2c8);
  return;
}

