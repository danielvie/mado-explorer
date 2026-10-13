
/* WARNING: Removing unreachable block (ram,0x0001400d2398) */
/* WARNING: Removing unreachable block (ram,0x0001400d239d) */

void FUN_1400d20e0(longlong param_1,longlong *param_2,int param_3)

{
  longlong lVar1;
  undefined8 *puVar2;
  char *pcVar3;
  short sVar4;
  uint uVar5;
  undefined8 uVar6;
  undefined1 auVar7 [12];
  undefined1 auVar8 [16];
  bool bVar9;
  int iVar10;
  undefined1 (*pauVar11) [16];
  float *pfVar12;
  longlong lVar13;
  undefined8 uVar14;
  uint uVar15;
  uint uVar16;
  uint uVar17;
  ulonglong uVar18;
  longlong lVar19;
  ulonglong uVar20;
  undefined **ppuVar21;
  ulonglong uVar22;
  undefined2 uVar23;
  uint uVar24;
  longlong lVar25;
  undefined2 uVar26;
  int iVar27;
  undefined8 uVar28;
  int iVar29;
  uint uVar30;
  ulonglong uVar31;
  bool bVar32;
  float fVar33;
  undefined4 uVar34;
  float fVar35;
  ulonglong extraout_XMM0_Qa;
  undefined1 auVar36 [16];
  undefined1 auVar37 [16];
  undefined1 auVar38 [16];
  float fVar39;
  undefined1 auVar40 [16];
  undefined1 auVar41 [16];
  undefined1 auVar42 [12];
  uint local_168;
  undefined4 uStack_164;
  uint uStack_160;
  undefined4 uStack_15c;
  uint local_158;
  uint local_148;
  uint uStack_144;
  int iStack_140;
  int iStack_13c;
  undefined2 local_138;
  undefined2 local_136;
  float local_134;
  undefined8 local_130;
  float local_128;
  float local_118;
  undefined4 uStack_114;
  int local_110;
  undefined2 local_108;
  undefined2 local_106;
  float local_104;
  undefined8 local_100;
  float local_f8;
  undefined8 local_e8;
  undefined8 local_e0;
  undefined8 local_d8;
  undefined8 local_d0;
  undefined1 local_c8 [16];
  undefined8 local_b8;
  undefined8 uStack_b0;
  undefined1 auVar43 [16];
  
  uVar20 = param_2[1];
  if (uVar20 == 0) {
    return;
  }
  uVar22 = 0;
  uVar24 = 0;
  do {
    uVar23 = 0;
    uVar31 = uVar20;
    if (*(short *)(uVar20 + 0x548) == 0) break;
    uVar20 = *(ulonglong *)(uVar20 + 0x540);
    uVar31 = uVar22;
  } while (uVar20 != 0);
  lVar25 = *param_2;
  uVar20 = uVar22;
  do {
    pcVar3 = &DAT_1401cf6cc + uVar20;
    uVar20 = uVar20 + 1;
    lVar25 = lVar25 * 0x21 + (longlong)*pcVar3;
  } while (uVar20 < 4);
  sVar4 = *(short *)(DAT_1401e8bc8 + 0x272);
  uVar15 = 0;
  if (sVar4 != 0) {
    if (sVar4 != 1) {
      if (sVar4 != 2) {
        return;
      }
      FUN_1400fa420(param_1,param_2,uVar31);
      uVar14 = FUN_140178230(lVar25);
      lVar25 = FUN_140183800(local_c8,uVar14,0);
      if ((*(ulonglong *)(lVar25 + 8) >> 0x18 & 2) == 0) {
        return;
      }
      FUN_1400d1ff0(param_1,uVar31);
      return;
    }
    FUN_1400fa420(param_1,param_2,uVar31);
    FUN_1400edb40(param_1,uVar31);
    FUN_1400f3360(param_1,uVar31);
    FUN_1400d9c20(param_1,uVar31);
    if (*(float *)(*(longlong *)(uVar31 + 0x680) + 0x50) <= 1.0) {
      return;
    }
    pauVar11 = (undefined1 (*) [16])FUN_1400cf8d0(local_c8,param_1,param_2);
    iVar27 = *(int *)(*(longlong *)(uVar31 + 0x680) + 0x48);
    if (iVar27 == 0) {
      auVar38 = *pauVar11;
      uVar16 = auVar38._8_4_ - auVar38._0_4_;
      uVar30 = auVar38._12_4_ - auVar38._4_4_;
      if ((int)uVar30 < 0) {
        uVar30 = uVar24;
      }
      uVar30 = uVar30 - *(int *)(param_1 + 0x848);
      if ((int)uVar16 < 0) {
        uVar16 = uVar24;
      }
      if ((int)uVar30 < 0) {
        uVar30 = uVar15;
      }
      FUN_14007c570(param_1,uVar31,(int)uVar30 < (int)uVar16);
      return;
    }
    FUN_14007c570(param_1,uVar31,iVar27 == 1);
    return;
  }
  for (lVar25 = *(longlong *)(param_1 + 0x980); (lVar25 != 0 && (*(int *)(lVar25 + 0x70) != 1));
      lVar25 = *(longlong *)(lVar25 + 0x88)) {
  }
  local_158 = *(uint *)(DAT_1401e8bc8 + 0x1760);
  uStack_164 = 0;
  uStack_15c = 0;
  uStack_160 = local_158;
  if ((*(float *)((longlong)param_2 + 0x34) <= 0.05 && *(float *)((longlong)param_2 + 0x34) != 0.05)
     && (*(char *)(param_1 + 0x813) != '\0')) {
    uStack_160 = uVar24;
  }
  uVar30 = uStack_160;
  if (((*(float *)(param_2 + 6) <= 0.05 && *(float *)(param_2 + 6) != 0.05) &&
      (*(char *)(param_1 + 0x813) != '\0')) && (*(int *)(lVar25 + 0x908) != 0)) {
    local_158 = uVar15;
  }
  local_168 = local_158;
  pauVar11 = (undefined1 (*) [16])FUN_1400cf8d0(local_c8,param_1,param_2);
  auVar38 = *pauVar11;
  local_148 = (uint)*(undefined8 *)*pauVar11;
  uStack_144 = (uint)((ulonglong)*(undefined8 *)*pauVar11 >> 0x20);
  iStack_140 = (int)*(undefined8 *)(*pauVar11 + 8);
  iStack_13c = (int)((ulonglong)*(undefined8 *)(*pauVar11 + 8) >> 0x20);
  iVar29 = iStack_140 - local_148;
  iVar27 = iStack_13c - uStack_144;
  if (((param_3 == 0) && (iVar10 = FUN_140157e30(param_1), iVar10 == 0)) &&
     ((*(char *)(param_1 + 0x811) == '\0' && (*(short *)(param_1 + 0x85a) == 0)))) {
    bVar9 = false;
    uVar20 = uVar22;
  }
  else {
    uVar20 = 1;
    bVar9 = true;
  }
  uVar26 = (undefined2)uVar20;
  ppuVar21 = &PTR_s_FoldersFirst_1401e7220;
  fVar33 = 0.0;
  pfVar12 = (float *)&DAT_1401e720c;
  fVar39 = (float)iVar29 * 0.25;
  uVar20 = uVar22;
  do {
    if (*(float *)(*(longlong *)(uVar31 + 0x98) + 0x138) < *pfVar12) {
      if (2 < (int)uVar20 - 3U) goto LAB_1400d23a9;
      break;
    }
    uVar20 = (ulonglong)((int)uVar20 + 1);
    pfVar12 = pfVar12 + 1;
  } while ((longlong)pfVar12 < 0x1401e7220);
  local_118 = (float)FUN_14017ed10();
  uStack_114 = 0;
  fVar33 = local_118 * 2.0;
  if (fVar33 < 0.0) {
    fVar33 = fVar33 - 0.5;
  }
  else {
    fVar33 = fVar33 + 0.5;
  }
  uVar16 = (uint)fVar33;
  uVar20 = (ulonglong)uVar16;
  ppuVar21 = (undefined **)(ulonglong)uVar16;
  fVar33 = (float)(int)uVar16;
LAB_1400d23a9:
  local_108 = 0;
  local_100 = 0x3c23d70a;
  local_106 = uVar26;
  local_104 = fVar33;
  local_f8 = fVar39;
  uVar34 = FUN_140176440(uVar20,ppuVar21,&local_108);
  *(undefined4 *)(param_2 + 5) = uVar34;
  local_138 = 0;
  local_130 = 0x3c23d70a;
  local_136 = uVar26;
  local_134 = fVar33;
  local_128 = (float)iVar27 * 0.25;
  fVar35 = (float)FUN_140176440();
  fVar39 = *(float *)(param_2 + 5);
  *(float *)((longlong)param_2 + 0x2c) = fVar35;
  if (fVar39 < 0.0) {
    fVar39 = fVar39 - 0.5;
  }
  else {
    fVar39 = fVar39 + 0.5;
  }
  if (fVar35 < 0.0) {
    fVar35 = fVar35 - 0.5;
  }
  else {
    fVar35 = fVar35 + 0.5;
  }
  uVar14 = CONCAT44(uStack_164,local_168);
  uVar6 = CONCAT44(uStack_15c,uStack_160);
  local_110 = (int)fVar39;
  local_b8 = uVar14;
  uStack_b0 = uVar6;
  local_118 = (float)FUN_14007d000(param_1,uVar31,&local_b8);
  lVar25 = DAT_1401e8bc8;
  uStack_144 = 0;
  iStack_140 = 0;
  local_148 = auVar38._0_4_;
  uVar20 = (ulonglong)*(uint *)(DAT_1401e8bc8 + 0x9d8);
  *(uint *)(DAT_1401e8bc8 + 0x9d8) = *(uint *)(DAT_1401e8bc8 + 0x9d8) + 1;
  *(ulonglong *)(lVar25 + 0x9e0 + uVar20 * 0x18) = (ulonglong)local_148;
  *(undefined4 *)(lVar25 + 0x9e8 + uVar20 * 0x18) = 0;
  local_168 = auVar38._4_4_;
  *(undefined4 *)(lVar25 + 0x9f0 + uVar20 * 0x18) = 1;
  lVar25 = DAT_1401e8bc8;
  uStack_164 = 0;
  uStack_160 = 0;
  uVar20 = (ulonglong)*(uint *)(DAT_1401e8bc8 + 0xa40);
  *(uint *)(DAT_1401e8bc8 + 0xa40) = *(uint *)(DAT_1401e8bc8 + 0xa40) + 1;
  *(ulonglong *)(lVar25 + 0xa48 + uVar20 * 0x18) = (ulonglong)local_168;
  *(undefined4 *)(lVar25 + 0xa50 + uVar20 * 0x18) = 0;
  *(undefined4 *)(lVar25 + 0xa58 + uVar20 * 0x18) = 1;
  lVar19 = DAT_1401e8bc8;
  auVar8._4_4_ = 0x3f800000;
  auVar8._0_4_ = (int)fVar39;
  auVar8._8_8_ = 0;
  lVar25 = (ulonglong)*(uint *)(DAT_1401e8bc8 + 0xaa8) + 0x72;
  *(uint *)(DAT_1401e8bc8 + 0xaa8) = *(uint *)(DAT_1401e8bc8 + 0xaa8) + 1;
  *(undefined1 (*) [16])(lVar19 + lVar25 * 0x18) = auVar8 << 0x20;
  *(undefined4 *)(lVar19 + 0x10 + lVar25 * 0x18) = 1;
  lVar25 = DAT_1401e8bc8;
  uVar16 = *(uint *)(DAT_1401e8bc8 + 0xb70);
  auVar37._0_8_ = (ulonglong)(uint)(int)fVar35 << 0x20;
  auVar37._8_4_ = 0x3f800000;
  auVar37._12_4_ = 0;
  *(uint *)(DAT_1401e8bc8 + 0xb70) = uVar16 + 1;
  *(undefined1 (*) [16])(lVar25 + 0xb78 + (ulonglong)uVar16 * 0x18) = auVar37;
  *(undefined4 *)(lVar25 + 0xb88 + (ulonglong)uVar16 * 0x18) = 1;
  lVar19 = DAT_1401e8bc8;
  lVar25 = (ulonglong)*(uint *)(DAT_1401e8bc8 + 0x1318) + 0xcc;
  *(uint *)(DAT_1401e8bc8 + 0x1318) = *(uint *)(DAT_1401e8bc8 + 0x1318) + 1;
  uVar28 = 0x4330830400001;
  *(undefined4 *)(lVar19 + lVar25 * 0x18) = 0;
  *(undefined4 *)(lVar19 + 0x10 + lVar25 * 0x18) = 1;
  lVar19 = DAT_1401e8bc8;
  lVar25 = (ulonglong)*(uint *)(DAT_1401e8bc8 + 0x2e0) + 0x1f;
  *(uint *)(DAT_1401e8bc8 + 0x2e0) = *(uint *)(DAT_1401e8bc8 + 0x2e0) + 1;
  if (bVar9) {
    uVar28 = 0x4300800400001;
  }
  *(undefined4 *)(lVar19 + lVar25 * 0x18) = 0x1b;
  *(undefined4 *)(lVar19 + 0x10 + lVar25 * 0x18) = 1;
  lVar19 = DAT_1401e8bc8;
  uVar20 = uVar22;
  for (lVar25 = param_2[10]; lVar25 != 0; lVar25 = *(longlong *)(lVar25 + 0x50)) {
    uVar20 = (ulonglong)((int)uVar20 + 1);
  }
  uVar18 = uVar22;
  uVar16 = uVar15;
  for (lVar25 = param_2[0xb]; lVar25 != 0; lVar25 = *(longlong *)(lVar25 + 0x58)) {
    uVar16 = (int)uVar18 + 1;
    uVar18 = (ulonglong)uVar16;
  }
  uVar16 = uVar16 + (int)uVar20;
  uVar18 = uVar22;
  if (uVar16 != 0xffffffff && uVar16 != 0) {
    uVar17 = *(uint *)(DAT_1401e8bc8 + 0x1380);
    *(uint *)(DAT_1401e8bc8 + 0x1380) = uVar17 + 1;
    *(undefined4 *)(lVar19 + 5000 + (ulonglong)uVar17 * 0x18) = 2;
    *(undefined4 *)(lVar19 + 0x1398 + (ulonglong)uVar17 * 0x18) = 1;
    if (*(int *)(param_2[0xc] + 0x44) == 0) {
      auVar7._8_4_ = 0x3f800000;
      auVar7._0_8_ = (ulonglong)(uint)((float)uVar20 / (float)uVar16) << 0x20;
      auVar40._12_4_ = 0;
      auVar40._4_8_ = auVar7._4_8_;
      auVar40._0_4_ = 0x3f000000;
    }
    else {
      auVar40._4_4_ = 0x3f000000;
      auVar40._0_4_ = (float)uVar20 / (float)uVar16;
      auVar40._8_4_ = 0x3f800000;
      auVar40._12_4_ = 0;
    }
    local_d8 = auVar40._0_8_;
    auVar37._8_8_ = auVar40._8_8_;
    local_e0 = 0x3f4000003f400000;
    auVar37._0_8_ = 0x3f4000003f400000;
    local_e8 = 0;
    local_d0 = 0x3f800000;
    lVar25 = FUN_140178c20(&local_e8);
    *(undefined4 *)(lVar25 + 0x10) = 1;
    uVar18 = extraout_XMM0_Qa;
  }
  lVar13 = FUN_1401757b0(uVar18,uVar28);
  lVar1 = DAT_1401e8bc8;
  lVar25 = (ulonglong)*(uint *)(DAT_1401e8bc8 + 0x5f8) + 0x40;
  *(uint *)(DAT_1401e8bc8 + 0x5f8) = *(uint *)(DAT_1401e8bc8 + 0x5f8) + 1;
  puVar2 = (undefined8 *)(lVar1 + lVar25 * 0x18);
  *puVar2 = uVar14;
  puVar2[1] = uVar6;
  *(undefined4 *)(lVar1 + 0x10 + lVar25 * 0x18) = 1;
  lVar19 = DAT_1401e8bc8 + 0x1e0;
  if (*(int *)(DAT_1401e8bc8 + 0xdc) != 0) {
    lVar19 = ((ulonglong)(*(int *)(DAT_1401e8bc8 + 0xdc) - 1) + 7) * 0x20 + DAT_1401e8bc8;
  }
  FUN_1401794a0(lVar19,lVar13,lVar25 * 3,lVar1,auVar37);
  FUN_1400fa420(param_1,param_2,uVar31);
  lVar19 = DAT_1401e8bc8;
  lVar25 = (ulonglong)*(uint *)(DAT_1401e8bc8 + 0xaa8) + 0x72;
  *(uint *)(DAT_1401e8bc8 + 0xaa8) = *(uint *)(DAT_1401e8bc8 + 0xaa8) + 1;
  *(undefined1 (*) [16])(lVar19 + lVar25 * 0x18) = ZEXT816(0x3f80000000000002);
  *(undefined4 *)(lVar19 + 0x10 + lVar25 * 0x18) = 1;
  lVar25 = DAT_1401e8bc8;
  auVar41._4_8_ = 0x3f800000;
  auVar41._0_4_ = 2;
  auVar41._12_4_ = 0;
  uVar16 = *(uint *)(DAT_1401e8bc8 + 0xb70);
  *(uint *)(DAT_1401e8bc8 + 0xb70) = uVar16 + 1;
  *(undefined1 (*) [16])(lVar25 + 0xb78 + (ulonglong)uVar16 * 0x18) = ZEXT816(0x3f80000000000002);
  *(undefined4 *)(lVar25 + 0xb88 + (ulonglong)uVar16 * 0x18) = 1;
  lVar25 = *(longlong *)(uVar31 + 0x78);
  uVar20 = uVar22;
  do {
    pcVar3 = "Panel" + uVar20;
    uVar20 = uVar20 + 1;
    lVar25 = lVar25 * 0x21 + (longlong)*pcVar3;
    uVar18 = uVar22;
  } while (uVar20 < 5);
  do {
    uVar20 = uVar18 + 1;
    lVar25 = (longlong)"Canvas"[uVar18] + lVar25 * 0x21;
    uVar18 = uVar20;
  } while (uVar20 < 6);
  lVar19 = FUN_1401757b0(lVar25,2,&IMAGE_DOS_HEADER_140000000,2,auVar41);
  lVar25 = DAT_1401e8bc8;
  iVar27 = *(int *)(*(longlong *)(uVar31 + 0x680) + 0x48);
  if (iVar27 == 0) {
    uVar17 = auVar38._8_4_ - auVar38._0_4_;
    uVar16 = auVar38._12_4_ - auVar38._4_4_;
    if ((int)uVar16 < 0) {
      uVar16 = uVar24;
    }
    uVar16 = uVar16 - *(int *)(param_1 + 0x848);
    if ((int)uVar17 < 0) {
      uVar17 = uVar24;
    }
    if ((int)uVar16 < 0) {
      uVar16 = uVar15;
    }
    bVar32 = (int)uVar16 < (int)uVar17;
  }
  else {
    bVar32 = iVar27 == 1;
  }
  lVar1 = (ulonglong)*(uint *)(DAT_1401e8bc8 + 0x5f8) + 0x40;
  *(uint *)(DAT_1401e8bc8 + 0x5f8) = *(uint *)(DAT_1401e8bc8 + 0x5f8) + 1;
  *(undefined8 *)(lVar25 + lVar1 * 0x18) = 0;
  *(undefined8 *)(lVar25 + 8 + lVar1 * 0x18) = 0;
  *(undefined4 *)(lVar25 + 0x10 + lVar1 * 0x18) = 1;
  lVar25 = DAT_1401e8bc8;
  uVar16 = *(uint *)(DAT_1401e8bc8 + 0x530);
  *(uint *)(DAT_1401e8bc8 + 0x530) = uVar16 + 1;
  uVar17 = (uint)bVar32;
  *(uint *)(lVar25 + 0x538 + (ulonglong)uVar16 * 0x18) = uVar17;
  *(undefined4 *)(lVar25 + 0x548 + (ulonglong)uVar16 * 0x18) = 1;
  lVar25 = DAT_1401e8bc8 + 0x1e0;
  if (*(int *)(DAT_1401e8bc8 + 0xdc) != 0) {
    lVar25 = ((ulonglong)(*(int *)(DAT_1401e8bc8 + 0xdc) - 1) + 7) * 0x20 + DAT_1401e8bc8;
  }
  FUN_1401794a0(lVar25,lVar19);
  uVar16 = local_110 - local_158;
  if ((int)(local_110 - local_158) < 0) {
    uVar16 = uVar24;
  }
  uVar30 = ((int)fVar35 - *(int *)(param_1 + 0x848)) - uVar30;
  if ((int)uVar30 < 0) {
    uVar30 = uVar24;
  }
  LOCK();
  if (DAT_1401e8bb8 == 0) {
    DAT_1401e8bb8 = 0;
  }
  UNLOCK();
  if ((!bVar9) && (*(int *)(*(longlong *)(uVar31 + 0x680) + 0x4c) != DAT_1401e8bb8)) {
    LOCK();
    if (DAT_1401e8bb8 == 0) {
      DAT_1401e8bb8 = 0;
    }
    UNLOCK();
    if ((((DAT_1401e8bb8 != *(int *)(lVar19 + 0x298)) && (*(short *)(lVar13 + 0x288) == 0)) &&
        (*(short *)(lVar13 + 0x28a) == 0)) && (local_118 == 0.0)) goto LAB_1400d2a08;
  }
  uVar23 = 1;
LAB_1400d2a08:
  uVar5 = uVar30;
  if (uVar17 == 1) {
    uVar5 = uVar16;
  }
  fVar39 = (float)FUN_140065160((float)(int)uVar5 * *(float *)(*(longlong *)(uVar31 + 0x680) + 0x40)
                               );
  fVar39 = (float)(int)fVar39 * 0.5;
  iVar27 = FUN_14007b450(uVar31);
  if (iVar27 == 0) {
    fVar39 = (float)FUN_140065160();
    fVar39 = (float)(int)fVar39 * 0.5;
  }
  local_138 = 0;
  local_130 = 0x3c23d70a;
  local_136 = uVar23;
  local_134 = fVar33;
  local_128 = fVar39;
  uVar34 = FUN_140176440();
  *(undefined4 *)(*(longlong *)(uVar31 + 0x680) + 0x50) = uVar34;
  lVar25 = DAT_1401e8bc8;
  fVar39 = *(float *)(*(longlong *)(uVar31 + 0x680) + 0x50);
  if (fVar39 < 0.0) {
    fVar39 = fVar39 - 0.5;
  }
  else {
    fVar39 = fVar39 + 0.5;
  }
  uVar5 = *(uint *)(DAT_1401e8bc8 + 0xaa8);
  if (uVar17 == 1) {
    lVar19 = (ulonglong)uVar5 + 0x72;
    *(uint *)(DAT_1401e8bc8 + 0xaa8) = uVar5 + 1;
    *(undefined4 *)(lVar25 + 0x10 + lVar19 * 0x18) = 1;
    auVar36._0_8_ = (ulonglong)(uVar16 - (int)fVar39) << 0x20;
    auVar36._8_4_ = 0x3f800000;
    auVar36._12_4_ = 0;
    *(undefined1 (*) [16])(lVar25 + lVar19 * 0x18) = auVar36;
    lVar25 = DAT_1401e8bc8;
    auVar42 = ZEXT812(0x3f800000) << 0x20;
    uVar20 = (ulonglong)*(uint *)(DAT_1401e8bc8 + 0xb70);
    *(uint *)(DAT_1401e8bc8 + 0xb70) = *(uint *)(DAT_1401e8bc8 + 0xb70) + 1;
    *(undefined1 (*) [16])(lVar25 + 0xb78 + uVar20 * 0x18) = ZEXT816(0x3f80000000000002);
  }
  else {
    uVar24 = uVar30 - (int)fVar39;
    lVar19 = (ulonglong)uVar5 + 0x72;
    *(uint *)(DAT_1401e8bc8 + 0xaa8) = uVar5 + 1;
    *(undefined4 *)(lVar25 + 0x10 + lVar19 * 0x18) = 1;
    *(undefined1 (*) [16])(lVar25 + lVar19 * 0x18) = ZEXT816(0x3f80000000000002);
    lVar25 = DAT_1401e8bc8;
    auVar42._8_4_ = 0x3f800000;
    auVar42._0_8_ = 0x3f80000000000002;
    uVar20 = (ulonglong)*(uint *)(DAT_1401e8bc8 + 0xb70);
    *(uint *)(DAT_1401e8bc8 + 0xb70) = *(uint *)(DAT_1401e8bc8 + 0xb70) + 1;
    auVar38._0_8_ = (ulonglong)uVar24 << 0x20;
    auVar38._8_4_ = 0x3f800000;
    auVar38._12_4_ = 0;
    *(undefined1 (*) [16])(lVar25 + 0xb78 + uVar20 * 0x18) = auVar38;
  }
  auVar43._12_4_ = 0;
  auVar43._0_12_ = auVar42;
  *(undefined4 *)(lVar25 + 0xb88 + uVar20 * 0x18) = 1;
  lVar19 = DAT_1401e8bc8;
  lVar25 = (ulonglong)*(uint *)(DAT_1401e8bc8 + 0x5f8) + 0x40;
  *(uint *)(DAT_1401e8bc8 + 0x5f8) = *(uint *)(DAT_1401e8bc8 + 0x5f8) + 1;
  *(undefined8 *)(lVar19 + lVar25 * 0x18) = 0;
  *(undefined8 *)(lVar19 + 8 + lVar25 * 0x18) = 0;
  *(undefined4 *)(lVar19 + 0x10 + lVar25 * 0x18) = 1;
  lVar25 = DAT_1401e8bc8 + 0x1e0;
  if (*(int *)(DAT_1401e8bc8 + 0xdc) != 0) {
    lVar25 = ((ulonglong)(*(int *)(DAT_1401e8bc8 + 0xdc) - 1) + 7) * 0x20 + DAT_1401e8bc8;
  }
  for (lVar25 = *(longlong *)(lVar25 + 0x10);
      (uVar20 = uVar22, lVar25 != 0 && (uVar20 = *(ulonglong *)(lVar25 + 8), uVar20 == 0));
      lVar25 = *(longlong *)(lVar25 + 0x1c8)) {
  }
  do {
    pcVar3 = "TopBlock" + uVar22;
    uVar22 = uVar22 + 1;
    uVar20 = (longlong)*pcVar3 + uVar20 * 0x21;
  } while (uVar22 < 8);
  uVar14 = FUN_1401757b0(uVar20,0x4000000000000,uVar24,&IMAGE_DOS_HEADER_140000000,auVar41,auVar43);
  lVar25 = DAT_1401e8bc8 + 0x1e0;
  if (*(int *)(DAT_1401e8bc8 + 0xdc) != 0) {
    lVar25 = ((ulonglong)(*(int *)(DAT_1401e8bc8 + 0xdc) - 1) + 7) * 0x20 + DAT_1401e8bc8;
  }
  FUN_1401794a0(lVar25,uVar14);
  FUN_1400edb40(param_1,uVar31);
  FUN_1400f3360(param_1,uVar31);
  lVar25 = *(longlong *)(uVar31 + 0x78);
  uVar24 = *(int *)(param_1 + 0x850) + 2;
  lVar19 = *(longlong *)(*(longlong *)(param_1 + 0x920) + 0x78);
  FUN_1400ea040(param_1,uVar31,uVar24);
  FUN_1400d9c20(param_1,uVar31);
  local_168 = 0;
  uStack_160 = 0x3f800000;
  uStack_15c = 0;
  uStack_164 = 1;
  local_148 = 0;
  iStack_140 = 0x3f800000;
  iStack_13c = 0;
  uStack_144 = uVar15;
  if (lVar25 == lVar19) {
    uStack_144 = uVar24;
  }
  FUN_14017b480(0,&local_148,&local_168);
  lVar25 = DAT_1401e8bc8 + 0x1e0;
  if (*(int *)(DAT_1401e8bc8 + 0xdc) != 0) {
    lVar25 = ((ulonglong)(*(int *)(DAT_1401e8bc8 + 0xdc) - 1) + 7) * 0x20 + DAT_1401e8bc8;
  }
  lVar19 = *(longlong *)(lVar25 + 0x10);
  *(undefined8 *)(lVar25 + 0x10) = *(undefined8 *)(lVar19 + 0x1d0);
  *(undefined8 *)(lVar19 + 0x1d0) = 0;
  if (1.0 < *(float *)(*(longlong *)(uVar31 + 0x680) + 0x50)) {
    FUN_14007c570(param_1,uVar31,bVar32);
  }
  lVar25 = DAT_1401e8bc8 + 0x1e0;
  if (*(int *)(DAT_1401e8bc8 + 0xdc) != 0) {
    lVar25 = ((ulonglong)(*(int *)(DAT_1401e8bc8 + 0xdc) - 1) + 7) * 0x20 + DAT_1401e8bc8;
  }
  lVar19 = *(longlong *)(lVar25 + 0x10);
  *(undefined8 *)(lVar25 + 0x10) = *(undefined8 *)(lVar19 + 0x1d0);
  *(undefined8 *)(lVar19 + 0x1d0) = 0;
  lVar25 = DAT_1401e8bc8 + 0x1e0;
  if (*(int *)(DAT_1401e8bc8 + 0xdc) != 0) {
    lVar25 = ((ulonglong)(*(int *)(DAT_1401e8bc8 + 0xdc) - 1) + 7) * 0x20 + DAT_1401e8bc8;
  }
  lVar19 = *(longlong *)(lVar25 + 0x10);
  *(undefined8 *)(lVar25 + 0x10) = *(undefined8 *)(lVar19 + 0x1d0);
  *(undefined8 *)(lVar19 + 0x1d0) = 0;
  return;
}

