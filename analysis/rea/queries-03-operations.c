
void FUN_1400c9cd0(undefined4 *param_1)

{
  int *piVar1;
  undefined8 *puVar2;
  undefined8 uVar3;
  undefined8 uVar4;
  uint uVar5;
  undefined8 *puVar6;
  longlong lVar7;
  ulonglong *puVar8;
  ulonglong uVar9;
  longlong lVar10;
  ulonglong uVar11;
  ulonglong uVar12;
  ulonglong *puVar13;
  ulonglong uVar14;
  longlong lVar15;
  ulonglong uVar16;
  ulonglong uVar17;
  LPVOID pvVar18;
  longlong lVar19;
  ulonglong uVar20;
  undefined4 *puVar21;
  ulonglong uStack_b0;
  undefined *local_88;
  undefined8 uStack_80;
  undefined8 local_78;
  undefined8 uStack_70;
  undefined1 local_68 [16];
  undefined1 local_58 [16];
  undefined1 local_48 [16];
  
  lVar15 = 0;
  uVar14 = 1;
  uVar11 = 1;
  do {
    uVar11 = uVar11 * 2;
  } while (uVar11 < 200);
  uVar12 = uVar11;
  if (uVar11 < 0x100000) {
    uVar12 = 0x100000;
  }
  local_88 = (undefined *)0x0;
  uStack_80 = 0;
  uVar17 = 0x10000;
  if (uVar12 < 0x10000) {
    uVar17 = uVar12;
  }
  uVar20 = uVar12;
  if (uVar12 < uVar11) {
    uVar20 = uVar11;
  }
  local_78 = 0;
  uStack_70 = 0;
  lVar19 = lVar15;
  if ((uVar17 != 0) && (uVar5 = (int)uVar17 - 1U & 0x28, uVar5 != 0)) {
    lVar19 = uVar17 - uVar5;
  }
  uStack_b0 = lVar19 + 0x28;
  if (uVar20 == uStack_b0) {
    puVar6 = VirtualAlloc((LPVOID)0x0,uVar20,0x3000,4);
  }
  else {
    puVar6 = VirtualAlloc((LPVOID)0x0,uVar20,0x2000,4);
    VirtualAlloc(puVar6,uStack_b0,0x1000,4);
  }
  uVar11 = (ulonglong)((uint)puVar6 & 7);
  *puVar6 = local_88;
  puVar6[1] = uStack_80;
  lVar19 = 8 - uVar11;
  if (uVar11 == 0) {
    lVar19 = lVar15;
  }
  *(undefined4 *)(puVar6 + 2) = (undefined4)local_78;
  *(undefined4 *)((longlong)puVar6 + 0x14) = local_78._4_4_;
  *(undefined4 *)(puVar6 + 3) = (undefined4)uStack_70;
  *(undefined4 *)((longlong)puVar6 + 0x1c) = uStack_70._4_4_;
  puVar6[4] = 0;
  lVar7 = lVar15;
  if ((uVar17 != 0) && (uVar11 = uVar17 - 1 & lVar19 + 200U, uVar11 != 0)) {
    lVar7 = uVar17 - uVar11;
  }
  uVar11 = lVar19 + 200U + lVar7;
  if (uStack_b0 < uVar11) {
    VirtualAlloc(puVar6,uVar11,0x1000,4);
    uStack_b0 = uVar11;
  }
  lVar7 = DAT_1401e8b90;
  puVar2 = (undefined8 *)((longlong)puVar6 + lVar19 + 0x68);
  *puVar2 = 0;
  puVar2[1] = 0;
  puVar13 = (ulonglong *)(lVar19 + 0x28 + (longlong)puVar6);
  puVar2 = (undefined8 *)((longlong)puVar6 + lVar19 + 0x78);
  *puVar2 = 0;
  puVar2[1] = 0;
  puVar2 = (undefined8 *)((longlong)puVar6 + lVar19 + 0x88);
  *puVar2 = 0;
  puVar2[1] = 0;
  puVar2 = (undefined8 *)((longlong)puVar6 + lVar19 + 0x98);
  *puVar2 = 0;
  puVar2[1] = 0;
  puVar2 = (undefined8 *)((longlong)puVar6 + lVar19 + 0xa8);
  *puVar2 = 0;
  puVar2[1] = 0;
  puVar2 = (undefined8 *)((longlong)puVar6 + lVar19 + 0xb8);
  *puVar2 = 0;
  puVar2[1] = 0;
  *puVar13 = uVar17;
  puVar13[1] = uVar12;
  puVar13[2] = uVar20;
  puVar13[3] = uStack_b0;
  puVar13[4] = lVar19 + 200;
  puVar13[5] = (ulonglong)puVar6;
  puVar13[6] = (ulonglong)puVar13;
  puVar13[7] = 1;
  puVar13[8] = lVar7 + 0x1a80;
  LOCK();
  piVar1 = (int *)(lVar7 + 0x1b20);
  *piVar1 = *piVar1 + 1;
  UNLOCK();
  uVar3 = *(undefined8 *)(param_1 + 2);
  uVar4 = *(undefined8 *)(param_1 + 4);
  *(undefined4 *)(puVar13 + 9) = *param_1;
  puVar8 = (ulonglong *)FUN_1401b8530(local_58,puVar13,uVar3,uVar4);
  uVar11 = *(ulonglong *)(param_1 + 6);
  uVar12 = puVar8[1];
  puVar13[10] = *puVar8;
  puVar13[0xb] = uVar12;
  local_78 = *puVar8;
  uStack_70 = puVar8[1];
  local_88 = (&PTR_DAT_1401e87c0)[(longlong)(int)puVar13[9] * 2];
  uStack_80 = *(undefined8 *)((longlong)(int)puVar13[9] * 0x10 + 0x1401e87c8);
  puVar6 = &local_78;
  FUN_1400c3130("C:\\Users\\Vjekoslav\\Projects\\Voidstar\\File Pilot\\Source\\Win\\WinMain.c",
                "P_PerformFilesOperation",0x1db3,
                "P_PerformFilesOperation kind: %S, destDirPath: \'%S\', file count: %llu\n",
                &local_88,puVar6,uVar11);
  uVar12 = puVar13[0xe];
  if (uVar12 == 0) {
    uVar20 = 1;
    if (1 < uVar11) {
      do {
        uVar20 = uVar20 * 2;
      } while (uVar20 < uVar11);
    }
    uVar12 = puVar13[4];
    pvVar18 = (LPVOID)puVar13[5];
    uVar17 = puVar13[3];
    uVar9 = (ulonglong)((int)uVar12 + (int)pvVar18 & 7);
    lVar19 = 8 - uVar9;
    if (uVar9 == 0) {
      lVar19 = lVar15;
    }
    lVar7 = lVar19 + uVar20 * 0x10;
    if (uVar17 < uVar12 + lVar7) {
      if (puVar13[2] < uVar12 + lVar7) {
        FUN_1401b5c50(puVar13,lVar7,8);
        pvVar18 = (LPVOID)puVar13[5];
        uVar12 = puVar13[4];
        uVar17 = puVar13[3];
        uVar9 = (ulonglong)((int)uVar12 + (int)pvVar18 & 7);
        lVar19 = 8 - uVar9;
        if (uVar9 == 0) {
          lVar19 = lVar15;
        }
        lVar7 = uVar20 * 0x10 + lVar19;
      }
      uVar9 = *puVar13;
      lVar10 = lVar15;
      if ((uVar9 != 0) && (uVar16 = uVar9 - 1 & uVar12 + lVar7, uVar16 != 0)) {
        lVar10 = uVar9 - uVar16;
      }
      uVar12 = lVar10 + uVar12 + lVar7;
      if (uVar17 < uVar12) {
        puVar13[3] = uVar12;
        VirtualAlloc(pvVar18,uVar12,0x1000,4);
      }
    }
    uVar12 = puVar13[5] + puVar13[4] + lVar19;
    puVar13[6] = uVar12;
    puVar13[4] = puVar13[4] + lVar7;
    puVar13[0xc] = 0;
LAB_1400ca0c5:
    puVar13[0xd] = uVar20;
  }
  else {
    uVar17 = puVar13[0xd];
    uVar20 = puVar13[0xc] + uVar11;
    if (uVar17 < uVar20) {
      if (uVar20 < uVar17 * 2) {
        uVar20 = uVar17 * 2;
      }
      puVar6 = (undefined8 *)((ulonglong)puVar6 & 0xffffffff00000000);
      uVar12 = FUN_1401b5ae0(puVar13,uVar12,uVar17 << 4,uVar20 << 4,8,puVar6,
                             "C:\\Users\\Vjekoslav\\Projects\\Voidstar\\File Pilot\\Source\\Program\\Voidstar.h"
                             ,0xa4b,"_ArrayGrow");
      goto LAB_1400ca0c5;
    }
  }
  puVar13[0xc] = puVar13[0xc] + uVar11;
  uVar17 = *(ulonglong *)(param_1 + 0xc);
  puVar13[0xe] = uVar12;
  if (uVar17 == 0) goto LAB_1400ca257;
  uVar12 = puVar13[0x11];
  if (uVar12 == 0) {
    if (1 < uVar17) {
      do {
        uVar14 = uVar14 * 2;
      } while (uVar14 < uVar17);
    }
    uVar12 = puVar13[4];
    pvVar18 = (LPVOID)puVar13[5];
    uVar20 = puVar13[3];
    uVar9 = (ulonglong)((int)uVar12 + (int)pvVar18 & 7);
    lVar19 = 8 - uVar9;
    if (uVar9 == 0) {
      lVar19 = lVar15;
    }
    lVar7 = lVar19 + uVar14 * 0x18;
    if (uVar20 < uVar12 + lVar7) {
      if (puVar13[2] < uVar12 + lVar7) {
        FUN_1401b5c50(puVar13,lVar7,8);
        pvVar18 = (LPVOID)puVar13[5];
        uVar12 = puVar13[4];
        uVar20 = puVar13[3];
        uVar9 = (ulonglong)((int)uVar12 + (int)pvVar18 & 7);
        lVar19 = 8 - uVar9;
        if (uVar9 == 0) {
          lVar19 = lVar15;
        }
        lVar7 = uVar14 * 0x18 + lVar19;
      }
      uVar9 = *puVar13;
      lVar10 = lVar15;
      if ((uVar9 != 0) && (uVar16 = uVar9 - 1 & uVar12 + lVar7, uVar16 != 0)) {
        lVar10 = uVar9 - uVar16;
      }
      uVar12 = lVar10 + uVar12 + lVar7;
      if (uVar20 < uVar12) {
        puVar13[3] = uVar12;
        VirtualAlloc(pvVar18,uVar12,0x1000,4);
      }
    }
    uVar12 = puVar13[4] + lVar19 + puVar13[5];
    puVar13[6] = uVar12;
    puVar13[4] = puVar13[4] + lVar7;
    puVar13[0xf] = 0;
LAB_1400ca245:
    puVar13[0x10] = uVar14;
  }
  else {
    uVar20 = puVar13[0x10];
    uVar14 = puVar13[0xf] + uVar17;
    if (uVar20 < uVar14) {
      if (uVar14 < uVar20 * 2) {
        uVar14 = uVar20 * 2;
      }
      uVar12 = FUN_1401b5ae0(puVar13,uVar12,uVar20 * 0x18,uVar14 * 0x18,8,
                             (ulonglong)puVar6 & 0xffffffff00000000,
                             "C:\\Users\\Vjekoslav\\Projects\\Voidstar\\File Pilot\\Source\\Program\\Voidstar.h"
                             ,0xa4b,"_ArrayGrow");
      goto LAB_1400ca245;
    }
  }
  puVar13[0xf] = puVar13[0xf] + uVar17;
  puVar13[0x11] = uVar12;
LAB_1400ca257:
  if (uVar11 != 0) {
    lVar19 = *(longlong *)(param_1 + 10);
    if (uVar17 == 0) {
      do {
        puVar6 = (undefined8 *)(lVar15 + lVar19);
        puVar6 = (undefined8 *)FUN_1401b8530(local_68,puVar13,*puVar6,puVar6[1]);
        uVar14 = puVar13[0xe];
        uVar3 = puVar6[1];
        *(undefined8 *)(lVar15 + uVar14) = *puVar6;
        ((undefined8 *)(lVar15 + uVar14))[1] = uVar3;
        uVar11 = uVar11 - 1;
        lVar15 = lVar15 + 0x10;
      } while (uVar11 != 0);
    }
    else {
      lVar7 = *(longlong *)(param_1 + 0x10);
      lVar10 = lVar15;
      puVar21 = (undefined4 *)(lVar7 + 0x10);
      do {
        puVar6 = (undefined8 *)(lVar15 + lVar19);
        puVar6 = (undefined8 *)FUN_1401b8530(local_68,puVar13,*puVar6,puVar6[1]);
        uVar14 = puVar13[0xe];
        uVar3 = puVar6[1];
        *(undefined8 *)(lVar15 + uVar14) = *puVar6;
        ((undefined8 *)(lVar15 + uVar14))[1] = uVar3;
        puVar6 = (undefined8 *)(lVar10 + lVar7);
        puVar6 = (undefined8 *)FUN_1401b8530(local_48,puVar13,*puVar6,puVar6[1]);
        uVar14 = puVar13[0x11];
        lVar15 = lVar15 + 0x10;
        uVar3 = puVar6[1];
        *(undefined8 *)(lVar10 + uVar14) = *puVar6;
        ((undefined8 *)(lVar10 + uVar14))[1] = uVar3;
        *(undefined4 *)(lVar10 + 0x10 + puVar13[0x11]) = *puVar21;
        uVar11 = uVar11 - 1;
        lVar10 = lVar10 + 0x18;
        puVar21 = puVar21 + 6;
      } while (uVar11 != 0);
    }
  }
  if ((((int)puVar13[9] - 8U & 0xfffffffc) == 0) && ((int)puVar13[9] != 9)) {
    uVar11 = puVar13[8];
    FUN_1400ca3b0(puVar13,puVar13 + 9);
    FUN_140001a10(puVar13);
    LOCK();
    *(int *)(uVar11 + 0xa0) = *(int *)(uVar11 + 0xa0) + -1;
    UNLOCK();
  }
  else {
    FUN_1400cb980(FUN_14019b830,puVar13);
  }
  return;
}

