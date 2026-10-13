
uint * FUN_1400c36b0(uint *param_1,longlong *param_2,ulonglong param_3)

{
  undefined4 *puVar1;
  char cVar2;
  short sVar3;
  longlong lVar4;
  uint uVar5;
  uint uVar6;
  uint uVar7;
  BOOL BVar8;
  UINT uFormat;
  HANDLE pvVar9;
  short *psVar10;
  uint *puVar11;
  char *pcVar12;
  ulonglong uVar13;
  ulonglong uVar14;
  undefined8 *puVar15;
  ulonglong uVar16;
  short *psVar17;
  int *piVar18;
  longlong lVar19;
  char *pcVar20;
  ulonglong uVar21;
  ulonglong uVar22;
  ulonglong uVar23;
  LPVOID pvVar24;
  longlong lVar25;
  short *psVar26;
  undefined4 uVar27;
  undefined4 uVar28;
  undefined4 uVar29;
  undefined4 uVar30;
  short *local_78;
  ulonglong local_70;
  undefined8 local_68;
  undefined8 uStack_60;
  undefined8 local_58;
  undefined8 uStack_50;
  undefined8 local_48;
  undefined8 local_40;
  undefined8 uStack_38;
  undefined8 local_30;
  
  param_1[0] = 0;
  param_1[1] = 0;
  param_1[2] = 0;
  param_1[3] = 0;
  param_1[4] = 0;
  param_1[5] = 0;
  param_1[6] = 0;
  param_1[7] = 0;
  param_1[8] = 0;
  param_1[9] = 0;
  param_1[10] = 0;
  param_1[0xb] = 0;
  param_1[0xc] = 0;
  param_1[0xd] = 0;
  param_1[0xe] = 0;
  param_1[0xf] = 0;
  BVar8 = OpenClipboard((HWND)0x0);
  if (BVar8 == 0) {
    return param_1;
  }
  uVar22 = 0;
  if ((param_3 & 1) != 0) {
    BVar8 = IsClipboardFormatAvailable(0xd);
    if (BVar8 == 0) {
      BVar8 = IsClipboardFormatAvailable(1);
      if (((BVar8 != 0) && (pvVar9 = GetClipboardData(1), pvVar9 != (HANDLE)0x0)) &&
         (pcVar12 = GlobalLock(pvVar9), pcVar20 = pcVar12, pcVar12 != (char *)0x0)) {
        do {
          cVar2 = *pcVar20;
          pcVar20 = pcVar20 + 1;
        } while (cVar2 != '\0');
        lVar19 = (longlong)pcVar20 - (longlong)pcVar12;
        uVar13 = lVar19 - 1;
        if (lVar19 == 0) {
          uVar13 = uVar22;
        }
        puVar11 = (uint *)FUN_1401b8530(&local_68,param_2,pcVar12,uVar13);
        goto LAB_1400c37ea;
      }
    }
    else {
      pvVar9 = GetClipboardData(0xd);
      if ((pvVar9 != (HANDLE)0x0) &&
         (psVar10 = GlobalLock(pvVar9), psVar26 = psVar10, psVar10 != (short *)0x0)) {
        do {
          sVar3 = *psVar26;
          psVar26 = psVar26 + 1;
        } while (sVar3 != 0);
        lVar19 = (longlong)psVar26 - (longlong)psVar10 >> 1;
        local_48 = 0;
        local_70 = lVar19 - 1;
        if (lVar19 == 0) {
          local_70 = uVar22;
        }
        local_58 = 0;
        uStack_50 = 0;
        local_78 = psVar10;
        puVar11 = (uint *)FUN_140188d50(&local_68,param_2,&local_78,&local_58);
LAB_1400c37ea:
        uVar5 = puVar11[1];
        uVar6 = puVar11[2];
        uVar7 = puVar11[3];
        param_1[2] = *puVar11;
        param_1[3] = uVar5;
        param_1[4] = uVar6;
        param_1[5] = uVar7;
        GlobalUnlock(pvVar9);
      }
    }
  }
  if ((((param_3 & 2) == 0) || (BVar8 = IsClipboardFormatAvailable(0xf), BVar8 == 0)) ||
     ((pvVar9 = GetClipboardData(0xf), pvVar9 == (HANDLE)0x0 ||
      (puVar11 = GlobalLock(pvVar9), puVar11 == (uint *)0x0)))) goto LAB_1400c3ddc;
  lVar19 = *(longlong *)(param_1 + 10);
  if (lVar19 == 0) {
    uVar21 = 1;
    do {
      uVar13 = uVar21;
      uVar21 = uVar13 * 2;
    } while (uVar21 < 0x10);
    lVar19 = param_2[4];
    pvVar24 = (LPVOID)param_2[5];
    uVar14 = param_2[3];
    uVar23 = (ulonglong)((int)lVar19 + (int)pvVar24 & 7);
    uVar16 = 8 - uVar23;
    if (uVar23 == 0) {
      uVar16 = uVar22;
    }
    lVar25 = uVar16 + uVar13 * 0x20;
    if (uVar14 < (ulonglong)(lVar25 + lVar19)) {
      if ((ulonglong)param_2[2] < (ulonglong)(lVar25 + lVar19)) {
        FUN_1401b5c50(param_2,lVar25,8);
        pvVar24 = (LPVOID)param_2[5];
        lVar19 = param_2[4];
        uVar14 = param_2[3];
        uVar23 = (ulonglong)((int)lVar19 + (int)pvVar24 & 7);
        uVar16 = 8 - uVar23;
        if (uVar23 == 0) {
          uVar16 = uVar22;
        }
        lVar25 = uVar16 + uVar13 * 0x20;
      }
      lVar4 = *param_2;
      uVar13 = uVar22;
      if ((lVar4 != 0) && (uVar23 = lVar4 - 1U & lVar25 + lVar19, uVar23 != 0)) {
        uVar13 = lVar4 - uVar23;
      }
      uVar13 = uVar13 + lVar25 + lVar19;
      if (uVar14 < uVar13) {
        param_2[3] = uVar13;
        VirtualAlloc(pvVar24,uVar13,0x1000,4);
      }
    }
    lVar19 = param_2[5] + uVar16 + param_2[4];
    param_2[6] = lVar19;
    param_2[4] = param_2[4] + lVar25;
    param_1[6] = 0;
    param_1[7] = 0;
LAB_1400c39c2:
    *(ulonglong *)(param_1 + 8) = uVar21;
  }
  else {
    uVar13 = *(ulonglong *)(param_1 + 8);
    uVar21 = *(longlong *)(param_1 + 6) + 0x10;
    if (uVar13 < uVar21) {
      if (uVar21 < uVar13 * 2) {
        uVar21 = uVar13 * 2;
      }
      lVar19 = FUN_1401b5ae0(param_2,lVar19,uVar13 << 4,uVar21 << 4,8,0,
                             "C:\\Users\\Vjekoslav\\Projects\\Voidstar\\File Pilot\\Source\\Program\\Voidstar.h"
                             ,0xa4b,"_ArrayGrow");
      goto LAB_1400c39c2;
    }
  }
  *(longlong *)(param_1 + 10) = lVar19;
  psVar26 = (short *)((ulonglong)*puVar11 + (longlong)puVar11);
  if (puVar11[4] == 0) {
    cVar2 = (char)*psVar26;
    while (psVar10 = psVar26, uVar13 = uVar22, cVar2 != '\0') {
      do {
        psVar17 = (short *)((longlong)psVar10 + 1);
        if ((char)*psVar10 == '\0') break;
        uVar13 = uVar13 + 1;
        psVar10 = psVar17;
      } while (psVar17 != (short *)0x0);
      uVar21 = uVar13;
      if (((longlong)uVar13 < 0) &&
         (uVar14 = (uVar13 ^ (longlong)uVar13 >> 0x3f) - ((longlong)uVar13 >> 0x3f), uVar21 = uVar22
         , uVar14 < uVar13)) {
        uVar21 = uVar13 - uVar14;
      }
      lVar19 = *(longlong *)(param_1 + 10);
      if (uVar21 == 0) {
        uVar21 = uVar22;
      }
      local_68 = psVar26;
      uStack_60 = uVar21;
      if (lVar19 == 0) {
        lVar19 = param_2[4];
        pvVar24 = (LPVOID)param_2[5];
        uVar13 = param_2[3];
        uVar16 = (ulonglong)((int)lVar19 + (int)pvVar24 & 7);
        uVar14 = 8 - uVar16;
        if (uVar16 == 0) {
          uVar14 = uVar22;
        }
        lVar25 = uVar14 + 0x10;
        if (uVar13 < (ulonglong)(lVar25 + lVar19)) {
          if ((ulonglong)param_2[2] < (ulonglong)(lVar25 + lVar19)) {
            FUN_1401b5c50(param_2,lVar25,8);
            pvVar24 = (LPVOID)param_2[5];
            lVar19 = param_2[4];
            uVar13 = param_2[3];
            uVar16 = (ulonglong)((int)lVar19 + (int)pvVar24 & 7);
            uVar14 = 8 - uVar16;
            if (uVar16 == 0) {
              uVar14 = uVar22;
            }
            lVar25 = uVar14 + 0x10;
          }
          lVar4 = *param_2;
          uVar16 = uVar22;
          if ((lVar4 != 0) && (uVar23 = lVar4 - 1U & lVar25 + lVar19, uVar23 != 0)) {
            uVar16 = lVar4 - uVar23;
          }
          uVar16 = uVar16 + lVar25 + lVar19;
          if (uVar13 < uVar16) {
            param_2[3] = uVar16;
            VirtualAlloc(pvVar24,uVar16,0x1000,4);
          }
        }
        lVar19 = param_2[5] + uVar14 + param_2[4];
        param_2[6] = lVar19;
        param_2[4] = param_2[4] + lVar25;
        param_1[6] = 0;
        param_1[7] = 0;
        param_1[8] = 1;
        param_1[9] = 0;
      }
      else {
        uVar13 = *(ulonglong *)(param_1 + 8);
        uVar14 = *(longlong *)(param_1 + 6) + 1;
        if (uVar13 < uVar14) {
          if (uVar14 < uVar13 * 2) {
            uVar14 = uVar13 * 2;
          }
          lVar19 = FUN_1401b5ae0(param_2,lVar19,uVar13 << 4,uVar14 << 4,8,0,
                                 "C:\\Users\\Vjekoslav\\Projects\\Voidstar\\File Pilot\\Source\\Program\\Voidstar.h"
                                 ,0xa4b,"_ArrayGrow");
          *(ulonglong *)(param_1 + 8) = uVar14;
        }
      }
      *(longlong *)(param_1 + 10) = lVar19;
      psVar26 = (short *)((longlong)psVar26 + uVar21 + 1);
      puVar1 = (undefined4 *)(lVar19 + *(longlong *)(param_1 + 6) * 0x10);
      *puVar1 = (undefined4)local_68;
      puVar1[1] = local_68._4_4_;
      puVar1[2] = (undefined4)uStack_60;
      puVar1[3] = uStack_60._4_4_;
      *(longlong *)(param_1 + 6) = *(longlong *)(param_1 + 6) + 1;
      cVar2 = (char)*psVar26;
    }
  }
  else {
    sVar3 = *psVar26;
    while (psVar10 = psVar26, uVar13 = uVar22, sVar3 != 0) {
      do {
        psVar17 = psVar10 + 1;
        if (*psVar10 == 0) break;
        uVar13 = uVar13 + 1;
        psVar10 = psVar17;
      } while (psVar17 != (short *)0x0);
      uVar21 = uVar13;
      if (((longlong)uVar13 < 0) &&
         (uVar14 = (uVar13 ^ (longlong)uVar13 >> 0x3f) - ((longlong)uVar13 >> 0x3f), uVar21 = uVar22
         , uVar14 < uVar13)) {
        uVar21 = uVar13 - uVar14;
      }
      if (uVar21 == 0) {
        uVar21 = uVar22;
      }
      local_30 = 0;
      local_40 = 0;
      uStack_38 = 0;
      local_78 = psVar26;
      local_70 = uVar21;
      puVar15 = (undefined8 *)FUN_140188d50(&local_58,param_2,&local_78,&local_40);
      lVar19 = *(longlong *)(param_1 + 10);
      uVar27 = *(undefined4 *)puVar15;
      uVar28 = *(undefined4 *)((longlong)puVar15 + 4);
      local_68 = (short *)*puVar15;
      uVar29 = (undefined4)puVar15[1];
      uVar30 = *(undefined4 *)((longlong)puVar15 + 0xc);
      uStack_60 = puVar15[1];
      if (lVar19 == 0) {
        lVar19 = param_2[4];
        pvVar24 = (LPVOID)param_2[5];
        uVar13 = param_2[3];
        uVar16 = (ulonglong)((int)lVar19 + (int)pvVar24 & 7);
        uVar14 = 8 - uVar16;
        if (uVar16 == 0) {
          uVar14 = uVar22;
        }
        lVar25 = uVar14 + 0x10;
        if (uVar13 < (ulonglong)(lVar25 + lVar19)) {
          if ((ulonglong)param_2[2] < (ulonglong)(lVar25 + lVar19)) {
            FUN_1401b5c50(param_2,lVar25,8);
            pvVar24 = (LPVOID)param_2[5];
            lVar19 = param_2[4];
            uVar13 = param_2[3];
            uVar16 = (ulonglong)((int)lVar19 + (int)pvVar24 & 7);
            uVar14 = 8 - uVar16;
            if (uVar16 == 0) {
              uVar14 = uVar22;
            }
            lVar25 = uVar14 + 0x10;
          }
          lVar4 = *param_2;
          uVar16 = uVar22;
          if ((lVar4 != 0) && (uVar23 = lVar4 - 1U & lVar19 + lVar25, uVar23 != 0)) {
            uVar16 = lVar4 - uVar23;
          }
          uVar16 = uVar16 + lVar19 + lVar25;
          if (uVar13 < uVar16) {
            param_2[3] = uVar16;
            VirtualAlloc(pvVar24,uVar16,0x1000,4);
          }
        }
        lVar19 = uVar14 + param_2[4] + param_2[5];
        param_2[6] = lVar19;
        param_2[4] = param_2[4] + lVar25;
        param_1[6] = 0;
        param_1[7] = 0;
        param_1[8] = 1;
        param_1[9] = 0;
LAB_1400c3bb6:
        uVar27 = (undefined4)local_68;
        uVar28 = local_68._4_4_;
        uVar29 = (undefined4)uStack_60;
        uVar30 = uStack_60._4_4_;
      }
      else {
        uVar13 = *(ulonglong *)(param_1 + 8);
        uVar14 = *(longlong *)(param_1 + 6) + 1;
        if (uVar13 < uVar14) {
          if (uVar14 < uVar13 * 2) {
            uVar14 = uVar13 * 2;
          }
          lVar19 = FUN_1401b5ae0(param_2,lVar19,uVar13 << 4,uVar14 << 4,8,0,
                                 "C:\\Users\\Vjekoslav\\Projects\\Voidstar\\File Pilot\\Source\\Program\\Voidstar.h"
                                 ,0xa4b,"_ArrayGrow");
          *(ulonglong *)(param_1 + 8) = uVar14;
          goto LAB_1400c3bb6;
        }
      }
      *(longlong *)(param_1 + 10) = lVar19;
      psVar26 = psVar26 + uVar21 + 1;
      puVar1 = (undefined4 *)(lVar19 + *(longlong *)(param_1 + 6) * 0x10);
      *puVar1 = uVar27;
      puVar1[1] = uVar28;
      puVar1[2] = uVar29;
      puVar1[3] = uVar30;
      *(longlong *)(param_1 + 6) = *(longlong *)(param_1 + 6) + 1;
      sVar3 = *psVar26;
    }
  }
  GlobalUnlock(pvVar9);
LAB_1400c3ddc:
  uFormat = RegisterClipboardFormatW(L"Preferred DropEffect");
  pvVar9 = GetClipboardData(uFormat);
  if ((pvVar9 != (HANDLE)0x0) && (piVar18 = GlobalLock(pvVar9), piVar18 != (int *)0x0)) {
    *param_1 = (uint)(*piVar18 == 2);
    GlobalUnlock(pvVar9);
  }
  CloseClipboard();
  return param_1;
}

