
/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

int * FUN_140039820(int *param_1,longlong *param_2,longlong param_3,undefined8 *param_4)

{
  byte *pbVar1;
  int *piVar2;
  ulonglong *puVar3;
  int iVar4;
  int iVar5;
  undefined4 uVar6;
  undefined4 uVar7;
  ulonglong *puVar8;
  uint uVar9;
  undefined4 uVar10;
  int iVar11;
  int iVar12;
  undefined8 uVar13;
  undefined8 uVar14;
  int *piVar15;
  ulonglong uVar16;
  longlong lVar17;
  undefined8 *puVar18;
  ulonglong uVar19;
  uint *puVar20;
  longlong lVar21;
  undefined4 *puVar22;
  longlong lVar23;
  char cVar24;
  longlong lVar25;
  ulonglong uVar26;
  int iVar27;
  longlong lVar28;
  longlong *plVar29;
  longlong *plVar30;
  int iVar31;
  uint uVar32;
  ulonglong uVar33;
  int iVar34;
  ulonglong uVar35;
  ulonglong uVar36;
  uint uVar37;
  char *pcVar38;
  char *pcVar39;
  LPVOID pvVar40;
  uint uVar41;
  char *pcVar42;
  undefined8 *puVar43;
  char *pcVar44;
  bool bVar45;
  int iVar46;
  int iVar47;
  int iVar48;
  undefined4 uVar49;
  undefined8 local_2a8;
  uint local_2a0;
  uint local_298;
  undefined8 local_288;
  undefined8 uStack_280;
  char *local_278;
  undefined8 uStack_270;
  undefined4 local_268;
  longlong local_258;
  int *local_250;
  char *local_248;
  longlong local_240;
  int local_238;
  int iStack_234;
  int iStack_230;
  int iStack_22c;
  int local_228;
  int iStack_224;
  int local_220;
  int iStack_21c;
  int iStack_218;
  int iStack_214;
  char *local_210;
  longlong lStack_208;
  undefined8 local_200;
  undefined8 local_1f8;
  undefined8 uStack_1f0;
  longlong *local_1e0;
  char *local_1d8;
  longlong local_1d0;
  longlong local_1c8 [2];
  char *local_1b8;
  undefined8 local_1b0;
  char *local_1a8;
  ulonglong uStack_1a0;
  char *local_188;
  ulonglong uStack_180;
  char *local_168;
  ulonglong uStack_160;
  undefined1 local_150 [24];
  undefined8 local_138;
  undefined8 uStack_130;
  undefined8 local_128;
  undefined8 uStack_120;
  undefined8 local_118;
  undefined8 uStack_110;
  undefined8 local_108;
  undefined8 uStack_100;
  undefined8 local_f8;
  undefined8 uStack_f0;
  undefined8 local_e8;
  char local_d8 [160];
  
  uVar33 = 0;
  iVar12 = 0;
  piVar15 = param_1 + 0x12;
  param_1[2] = 0;
  param_1[3] = 0;
  param_1[4] = 0;
  param_1[5] = 0;
  *param_1 = 1;
  param_1[6] = 0;
  param_1[7] = 0;
  param_1[8] = 0;
  param_1[9] = 0;
  param_1[10] = 0;
  param_1[0xb] = 0;
  param_1[0xc] = 0;
  param_1[0xe] = 0;
  param_1[0xf] = 0;
  param_1[0x10] = 0;
  param_1[0x11] = 0;
  param_1[0x14] = 0;
  param_1[0x15] = 0;
  param_1[0x16] = 0;
  param_1[0x17] = 0;
  param_1[0x18] = 0;
  param_1[0x19] = 0;
  param_1[0x1a] = 0;
  param_1[0x1b] = 0;
  param_1[0x1c] = 0;
  param_1[0x1d] = 0;
  param_1[0x1e] = 0;
  param_1[0x1f] = 0;
  param_1[0x20] = 0;
  param_1[0x21] = 0;
  param_1[0x22] = 0;
  param_1[0x23] = 0;
  param_1[0x24] = 0;
  param_1[0x25] = 0;
  param_1[0x26] = 0;
  param_1[0x27] = 0;
  param_1[0x28] = 0;
  param_1[0x29] = 0;
  param_1[0x2a] = 0;
  param_1[0x2b] = 0;
  param_1[0x2c] = 0;
  param_1[0x2d] = 0;
  param_1[0x2e] = 0;
  piVar15[0] = 0;
  piVar15[1] = 0;
  local_288 = "options";
  uStack_280 = 7;
  local_250 = piVar15;
  uVar13 = FUN_1401b6980(param_2,piVar15,0,0x18,0x10);
  uVar14 = uStack_280;
  pcVar42 = local_288;
  uVar26 = 0x31415926;
  iVar31 = 0;
  uVar35 = uVar33;
  do {
    pbVar1 = (byte *)("options" + uVar35);
    uVar35 = uVar35 + 1;
    uVar26 = (uVar26 << 9 | uVar26 >> 0x37) + (ulonglong)*pbVar1;
  } while (uVar35 < 7);
  uVar35 = (uVar26 ^ 0x31415926) * 0x40000 + ~(uVar26 ^ 0x31415926);
  lVar28 = 0x31415928;
  uVar35 = (uVar35 >> 0x1f | uVar35 << 0x21) * 0x15;
  uVar35 = (uVar35 >> 0xb | uVar35 << 0x35) * 0x41;
  uVar35 = (uVar35 >> 0x16 | uVar35 << 0x2a) ^ uVar35;
  lVar25 = 0x31415928;
  if (1 < uVar35 + 0x31415926) {
    lVar25 = 0x31415926;
  }
  lVar25 = FUN_1401b67b0(param_2,piVar15,uVar13,0x18,&local_288,1,lVar25 + uVar35);
  uStack_280 = 6;
  puVar18 = (undefined8 *)(lVar25 + *(longlong *)(param_1 + 0x16) * 0x18);
  *puVar18 = pcVar42;
  puVar18[1] = uVar14;
  *(longlong *)(param_1 + 0x20) = lVar25;
  *(undefined4 *)(lVar25 + 0x10 + *(longlong *)(param_1 + 0x16) * 0x18) = 0;
  local_288 = "colors";
  uVar13 = FUN_1401b6980(param_2,piVar15,*(undefined8 *)(param_1 + 0x20),0x18,0x10);
  uVar14 = uStack_280;
  pcVar42 = local_288;
  uVar26 = 0x31415926;
  uVar35 = uVar33;
  do {
    pbVar1 = (byte *)("colors" + uVar35);
    uVar35 = uVar35 + 1;
    uVar26 = (uVar26 << 9 | uVar26 >> 0x37) + (ulonglong)*pbVar1;
  } while (uVar35 < 6);
  uVar35 = (uVar26 ^ 0x31415926) * 0x40000 + ~(uVar26 ^ 0x31415926);
  uVar35 = (uVar35 >> 0x1f | uVar35 << 0x21) * 0x15;
  uVar35 = (uVar35 >> 0xb | uVar35 << 0x35) * 0x41;
  uVar35 = (uVar35 >> 0x16 | uVar35 << 0x2a) ^ uVar35;
  lVar25 = 0x31415928;
  if (1 < uVar35 + 0x31415926) {
    lVar25 = 0x31415926;
  }
  lVar25 = FUN_1401b67b0(param_2,piVar15,uVar13,0x18,&local_288,1,lVar25 + uVar35);
  uStack_280 = 7;
  puVar18 = (undefined8 *)(lVar25 + *(longlong *)(param_1 + 0x16) * 0x18);
  *puVar18 = pcVar42;
  puVar18[1] = uVar14;
  *(longlong *)(param_1 + 0x20) = lVar25;
  *(undefined4 *)(lVar25 + 0x10 + *(longlong *)(param_1 + 0x16) * 0x18) = 0;
  local_288 = &DAT_1401d1898;
  uVar14 = FUN_1401b6980(param_2,piVar15,*(undefined8 *)(param_1 + 0x20),0x18,0x10);
  uVar26 = 0x31415926;
  uVar35 = uVar33;
  do {
    pbVar1 = &DAT_1401d1898 + uVar35;
    uVar35 = uVar35 + 1;
    uVar26 = (uVar26 << 9 | uVar26 >> 0x37) + (ulonglong)*pbVar1;
  } while (uVar35 < 7);
  uVar49 = (undefined4)local_288;
  uVar6 = local_288._4_4_;
  uVar7 = (undefined4)uStack_280;
  uVar10 = uStack_280._4_4_;
  uVar35 = (uVar26 ^ 0x31415926) * 0x40000 + ~(uVar26 ^ 0x31415926);
  uVar35 = (uVar35 >> 0x1f | uVar35 << 0x21) * 0x15;
  uVar35 = (uVar35 >> 0xb | uVar35 << 0x35) * 0x41;
  uVar35 = (uVar35 >> 0x16 | uVar35 << 0x2a) ^ uVar35;
  puVar18 = &local_288;
  if (1 < uVar35 + 0x31415926) {
    lVar28 = 0x31415926;
  }
  lVar25 = FUN_1401b67b0(param_2,piVar15,uVar14,0x18,puVar18,1,lVar28 + uVar35);
  puVar22 = (undefined4 *)(lVar25 + *(longlong *)(param_1 + 0x16) * 0x18);
  *puVar22 = uVar49;
  puVar22[1] = uVar6;
  puVar22[2] = uVar7;
  puVar22[3] = uVar10;
  *(longlong *)(param_1 + 0x20) = lVar25;
  *(undefined4 *)(lVar25 + 0x10 + *(longlong *)(param_1 + 0x16) * 0x18) = 0;
  local_288 = _DAT_1401e8d48;
  uStack_280 = lRam00000001401e8d50;
  piVar15 = (int *)FUN_1400cbdb0(&local_188,param_2,&local_288);
  local_210 = (char *)0x0;
  lStack_208 = 0;
  local_200 = 0;
  local_238 = *piVar15;
  iStack_234 = piVar15[1];
  iVar4 = (int)*(longlong *)(piVar15 + 2);
  iVar5 = piVar15[3];
  lVar25 = *(longlong *)(piVar15 + 2);
  uVar35 = *(ulonglong *)(piVar15 + 4);
  iVar47 = (int)uVar35;
  iVar48 = (int)(uVar35 >> 0x20);
  if (local_238 != 0) {
    uVar32 = (uint)(local_238 != 1);
    goto LAB_14003b060;
  }
  iStack_230 = iVar4;
  iStack_22c = iVar5;
  local_228 = iVar47;
  iStack_224 = iVar48;
  local_220 = iVar4;
  iStack_21c = iVar5;
  iStack_218 = iVar47;
  iStack_214 = iVar48;
  lVar28 = FUN_1401b40a0(lVar25,uVar35);
  *(longlong *)(param_1 + 0xe) = lVar28;
  if (lVar28 == param_3) {
    param_1[0xc] = 3;
    return param_1;
  }
  param_1[0xc] = 2;
  local_2a8._0_4_ = 0;
  local_2a8._4_4_ = 0;
  local_2a0 = -1;
  local_298 = 0;
  if (uVar35 != 0) {
    local_298 = 0;
    uVar26 = uVar33;
    uVar19 = uVar33;
    uVar16 = uVar33;
LAB_140039c70:
    uVar32 = (uint)uVar19;
    if (*(char *)(lVar25 + uVar19) == '\0') goto LAB_140039d3f;
    switch(*(char *)(lVar25 + uVar19)) {
    case '\t':
    case '\n':
    case '\r':
    case ' ':
    case ',':
    case ']':
    case '}':
      break;
    default:
      return param_1;
    case '\"':
      puVar18 = (undefined8 *)0x0;
      iVar27 = FUN_1401baac0(&local_2a8,lVar25,uVar35,0,0);
      if (iVar27 < 0) {
        return param_1;
      }
      uVar16 = (ulonglong)local_2a8._4_4_;
      uVar32 = (uint)local_2a8;
      goto switchD_140039ca1_caseD_5b;
    case '-':
    case '0':
    case '1':
    case '2':
    case '3':
    case '4':
    case '5':
    case '6':
    case '7':
    case '8':
    case '9':
    case 'f':
    case 'n':
    case 't':
      if (uVar19 < uVar35) {
        do {
          cVar24 = *(char *)(uVar19 + lVar25);
          if (cVar24 == '\0') break;
          switch(cVar24) {
          case '\t':
          case '\n':
          case '\r':
          case ' ':
          case ',':
          case ']':
          case '}':
            goto switchD_140039d19_caseD_9;
          default:
            if (0x5e < (byte)(cVar24 - 0x20U)) {
              local_298 = 0xfffffffe;
              goto LAB_140039d3f;
            }
            uVar19 = (ulonglong)((int)uVar19 + 1);
            if (uVar35 <= uVar19) goto LAB_140039d32;
          }
        } while( true );
      }
LAB_140039d32:
      local_298 = 0xfffffffd;
      goto LAB_140039d3f;
    case ':':
      local_2a0 = (int)uVar16 + -1;
      break;
    case '[':
    case '{':
      goto switchD_140039ca1_caseD_5b;
    }
    goto switchD_140039ca1_caseD_9;
  }
LAB_140039d3f:
  if ((int)local_298 < 1) {
    return param_1;
  }
  lVar25 = param_2[4];
  pvVar40 = (LPVOID)param_2[5];
  uVar26 = param_2[3];
  lVar28 = (longlong)(int)local_298 * 0x14;
  local_2a8 = (char *)0x0;
  uVar16 = (ulonglong)((int)lVar25 + (int)pvVar40 & 3);
  local_2a0 = 0xffffffff;
  uVar19 = 4 - uVar16;
  if (uVar16 == 0) {
    uVar19 = uVar33;
  }
  lVar23 = uVar19 + lVar28;
  if (uVar26 < (ulonglong)(lVar25 + lVar23)) {
    if ((ulonglong)param_2[2] < (ulonglong)(lVar25 + lVar23)) {
      FUN_1401b5c50(param_2,lVar23,4);
      pvVar40 = (LPVOID)param_2[5];
      lVar25 = param_2[4];
      uVar26 = param_2[3];
      uVar16 = (ulonglong)((int)lVar25 + (int)pvVar40 & 3);
      uVar19 = 4 - uVar16;
      if (uVar16 == 0) {
        uVar19 = uVar33;
      }
      lVar23 = uVar19 + lVar28;
    }
    lVar21 = *param_2;
    uVar16 = uVar33;
    if ((lVar21 != 0) && (uVar36 = lVar21 - 1U & lVar25 + lVar23, uVar36 != 0)) {
      uVar16 = lVar21 - uVar36;
    }
    uVar16 = uVar16 + lVar25 + lVar23;
    if (uVar26 < uVar16) {
      param_2[3] = uVar16;
      VirtualAlloc(pvVar40,uVar16,0x1000,4);
    }
  }
  local_258 = param_2[5] + uVar19 + param_2[4];
  FUN_1401bace0(local_258,0,lVar28);
  lVar28 = local_258;
  uVar32 = local_298;
  param_2[4] = param_2[4] + lVar23;
  param_2[6] = local_258;
  lVar25 = CONCAT44(iStack_22c,iStack_230);
  lStack_208 = local_258;
  uVar41 = local_2a8._4_4_;
  iVar27 = iVar12;
  if (uVar35 != 0) {
    puVar43 = (undefined8 *)(ulonglong)local_298;
    uVar26 = uVar33;
    uVar19 = uVar33;
    uVar37 = local_2a0;
LAB_140039e60:
    iVar34 = (int)uVar19;
    iVar11 = (int)uVar26;
    cVar24 = *(char *)(uVar26 + lVar25);
    iVar27 = iVar34;
    if (cVar24 == '\0') goto LAB_14003a169;
    uVar9 = local_2a8._4_4_;
    switch(cVar24) {
    case '\t':
    case '\n':
    case '\r':
    case ' ':
      break;
    default:
      goto switchD_140039e99_caseD_b;
    case '\"':
      puVar18 = puVar43;
      iVar27 = FUN_1401baac0(&local_2a8,lVar25,uVar35,lVar28,puVar43);
      if (iVar27 < 0) goto LAB_14003a1a3;
      uVar19 = (ulonglong)(iVar34 + 1);
      if ((local_2a0 != 0xffffffff) && (lVar28 != 0)) {
        piVar15 = (int *)(lVar28 + 0xc + (longlong)(int)local_2a0 * 0x14);
        *piVar15 = *piVar15 + 1;
      }
      uVar41 = local_2a8._4_4_;
      uVar37 = local_2a0;
      iVar11 = (uint)local_2a8;
      break;
    case ',':
      if (((lVar28 != 0) && (uVar37 = local_2a0, local_2a0 != 0xffffffff)) &&
         (1 < *(int *)(lVar28 + (longlong)(int)local_2a0 * 0x14) - 1U)) {
        local_2a0 = *(uint *)(lVar28 + 0x10 + (longlong)(int)local_2a0 * 0x14);
        uVar37 = local_2a0;
      }
      break;
    case '-':
    case '0':
    case '1':
    case '2':
    case '3':
    case '4':
    case '5':
    case '6':
    case '7':
    case '8':
    case '9':
    case 'f':
    case 'n':
    case 't':
      if (((lVar28 == 0) || (local_2a0 == 0xffffffff)) ||
         ((iVar27 = *(int *)(lVar28 + (longlong)(int)local_2a0 * 0x14), iVar27 != 1 &&
          ((iVar27 != 4 || (*(int *)(lVar28 + 0xc + (longlong)(int)local_2a0 * 0x14) == 0)))))) {
        if (uVar26 < uVar35) {
          do {
            iVar27 = (int)uVar26;
            cVar24 = *(char *)(uVar26 + lVar25);
            if (cVar24 == '\0') break;
            switch(cVar24) {
            case '\t':
            case '\n':
            case '\r':
            case ' ':
            case ',':
            case ']':
            case '}':
              goto switchD_14003a0e0_caseD_9;
            default:
              if (0x5e < (byte)(cVar24 - 0x20U)) goto switchD_140039e99_caseD_b;
              uVar26 = (ulonglong)(iVar27 + 1);
              if (uVar35 <= uVar26) goto LAB_14003a0f3;
            }
          } while( true );
        }
        goto LAB_14003a0f3;
      }
switchD_140039e99_caseD_b:
      iVar27 = -2;
      goto LAB_14003a1a3;
    case ':':
      local_2a0 = uVar41 - 1;
      uVar37 = local_2a0;
      break;
    case '[':
    case '{':
      uVar19 = (ulonglong)(iVar34 + 1);
      if (lVar28 == 0) break;
      if (local_2a8._4_4_ < uVar32) {
        uVar41 = local_2a8._4_4_ + 1;
        *(undefined8 *)(lVar28 + 4 + (ulonglong)local_2a8._4_4_ * 0x14) = 0xffffffffffffffff;
        piVar15 = (int *)(lVar28 + (ulonglong)local_2a8._4_4_ * 0x14);
        piVar15[3] = 0;
        piVar15[4] = -1;
        local_2a8 = (char *)CONCAT44(uVar41,(uint)local_2a8);
        if (uVar37 != 0xffffffff) {
          if (*(int *)(lVar28 + (longlong)(int)uVar37 * 0x14) == 1) goto switchD_140039e99_caseD_b;
          piVar2 = (int *)(lVar28 + 0xc + (longlong)(int)uVar37 * 0x14);
          *piVar2 = *piVar2 + 1;
          piVar15[4] = uVar37;
        }
        piVar15[1] = iVar11;
        local_2a0 = uVar9;
        *piVar15 = (cVar24 != '{') + 1;
        uVar37 = uVar9;
        break;
      }
      goto LAB_14003a2d4;
    case ']':
    case '}':
      if (lVar28 != 0) {
        iVar27 = (cVar24 != '}') + 1;
        if (local_2a8._4_4_ != 0) {
          uVar26 = (ulonglong)(local_2a8._4_4_ - 1);
          do {
            piVar15 = (int *)(lVar28 + uVar26 * 0x14);
            uVar41 = local_2a8._4_4_;
            if ((piVar15[1] != -1) && (piVar15[2] == -1)) {
              if (*piVar15 != iVar27) goto switchD_140039e99_caseD_b;
              piVar15[2] = iVar11 + 1;
              local_2a0 = piVar15[4];
              uVar37 = local_2a0;
              goto switchD_140039e99_caseD_9;
            }
            uVar26 = (ulonglong)piVar15[4];
          } while (piVar15[4] != -1);
          if ((*piVar15 == iVar27) && (uVar37 != 0xffffffff)) break;
        }
        goto switchD_140039e99_caseD_b;
      }
    }
    goto switchD_140039e99_caseD_9;
  }
LAB_14003a169:
  if ((lVar28 != 0) && (lVar23 = (longlong)(int)(uVar41 - 1), -1 < (int)(uVar41 - 1))) {
    piVar15 = (int *)(lVar28 + (lVar23 * 5 + 2) * 4);
    do {
      if ((piVar15[-1] != -1) && (*piVar15 == -1)) goto LAB_14003a0f3;
      piVar15 = piVar15 + -5;
      lVar23 = lVar23 + -1;
    } while (-1 < lVar23);
  }
LAB_14003a1a3:
  pcVar42 = (char *)(longlong)iVar27;
  local_138 = *param_4;
  uStack_130 = param_4[1];
  local_128 = param_4[2];
  uStack_120 = param_4[3];
  local_118 = param_4[4];
  uStack_110 = param_4[5];
  local_108 = param_4[6];
  uStack_100 = param_4[7];
  local_f8 = param_4[8];
  uStack_f0 = param_4[9];
  local_e8 = param_4[10];
  local_248 = pcVar42;
  local_210 = pcVar42;
  local_1e0 = (longlong *)FUN_140036170(&local_138);
  uVar49 = (undefined4)((ulonglong)puVar18 >> 0x20);
  local_200 = CONCAT44(1,(undefined4)local_200);
  *(longlong **)(param_1 + 0x10) = local_1e0;
  local_1e0[8] = *(longlong *)(param_1 + 0xe);
  if ((char *)0x1 < pcVar42) {
    piVar15 = (int *)(lVar28 + 0x28);
    iVar27 = 2;
    do {
      local_1f8 = *(undefined8 *)(piVar15 + -5);
      uStack_1f0 = *(undefined8 *)(piVar15 + -3);
      pcVar44 = (char *)(longlong)iVar27;
      if (piVar15[-1] == 0) {
        iVar11 = (int)((ulonglong)local_1f8 >> 0x20);
        iVar34 = (int)uStack_1f0 - iVar11;
        if (((int)local_1f8 == 4) && (iVar34 < 0x80)) {
          lVar28 = (longlong)iVar34;
          if (0 < iVar34) {
            uVar35 = uVar33;
            do {
              pcVar38 = local_d8 + uVar35;
              cVar24 = pcVar38[(iVar11 + lVar25) - (longlong)local_d8];
              if ((byte)(cVar24 + 0xbfU) < 0x1a) {
                cVar24 = cVar24 + ' ';
              }
              uVar35 = uVar35 + 1;
              *pcVar38 = cVar24;
            } while ((longlong)uVar35 < lVar28);
          }
          uVar35 = uVar33;
          if (*(longlong *)(param_1 + 0x20) == 0) {
LAB_14003a327:
            if (((char *)(longlong)iVar27 < pcVar42) && (*piVar15 - 1U < 2)) {
              *(int *)(*(longlong *)(param_1 + 0x20) + 0x10 + (uVar35 & 0xffffffff) * 0x18) = iVar27
              ;
            }
          }
          else {
            local_288 = local_d8;
            puVar18 = (undefined8 *)CONCAT44((int)((ulonglong)puVar18 >> 0x20),1);
            uStack_280 = lVar28;
            lVar23 = FUN_1401b65d0(local_250,*(longlong *)(param_1 + 0x20),0x18,&local_288,puVar18);
            uVar35 = (ulonglong)(uint)local_250[2];
            if (lVar23 != 0) {
              uVar35 = *(ulonglong *)(lVar23 + 8);
              *(ulonglong *)(local_250 + 4) = uVar35;
            }
            if (-1 < (int)uVar35) goto LAB_14003a327;
          }
          local_1c8[0] = 0;
          local_1b8 = "version";
          local_1d8 = local_d8;
          local_1c8[1] = 0xffffffffffffffff;
          local_1b0 = 7;
          local_1d0 = lVar28;
          iVar11 = FUN_1401b87a0(&local_1d8,&local_1b8,local_1c8);
          if ((iVar11 != 0) && (*piVar15 == 8)) {
            pcVar38 = (char *)0x1;
            pcVar39 = (char *)(piVar15[1] + lVar25);
            iVar34 = piVar15[2] - piVar15[1];
            iVar11 = 0;
            if (iVar34 != 0) {
              bVar45 = *pcVar39 == '-';
              pcVar38 = (char *)0xffffffffffffffff;
              uVar26 = (ulonglong)bVar45;
              iVar11 = iVar31;
              uVar35 = uVar33;
              if (!bVar45) {
                pcVar38 = (char *)0x1;
              }
              for (; uVar26 < (ulonglong)(longlong)iVar34; uVar26 = uVar26 + 1) {
                iVar11 = (int)uVar35;
                cVar24 = pcVar39[uVar26];
                if (9 < (byte)(cVar24 - 0x30U)) break;
                uVar35 = (longlong)(cVar24 + -0x30) + uVar35 * 10;
                iVar11 = (int)uVar35;
                local_2a8 = pcVar38;
              }
            }
            iVar11 = iVar11 * (int)pcVar38;
            if (iVar11 < 1) {
              iVar11 = 1;
            }
            *param_1 = iVar11;
          }
        }
      }
      uVar49 = (undefined4)((ulonglong)puVar18 >> 0x20);
      iVar27 = iVar27 + 1;
      piVar15 = piVar15 + 5;
      lVar28 = local_258;
    } while (pcVar44 < pcVar42);
  }
  plVar29 = local_1e0;
  lVar25 = *(longlong *)(param_1 + 0x20);
  lVar23 = CONCAT44(iStack_21c,local_220);
  piVar15 = local_250;
  local_240 = lVar23;
  if (lVar25 != 0) {
    uStack_280 = 6;
    uVar14 = CONCAT44(uVar49,1);
    local_288 = "colors";
    lVar17 = FUN_1401b65d0(param_1 + 0x12,lVar25,0x18,&local_288,uVar14);
    uVar49 = (undefined4)((ulonglong)uVar14 >> 0x20);
    lVar21 = *(longlong *)(param_1 + 0x14);
    if (lVar17 != 0) {
      lVar21 = *(longlong *)(lVar17 + 8);
      *(longlong *)(param_1 + 0x16) = lVar21;
      lVar25 = *(longlong *)(param_1 + 0x20);
    }
    local_298 = *(uint *)(lVar25 + 0x10 + lVar21 * 0x18);
    piVar15 = param_1 + 0x12;
    if (0 < (int)local_298) {
      pcVar44 = pcVar42 + -1;
      iVar27 = local_298 + 1;
      pcVar39 = (char *)(longlong)iVar27;
      pcVar38 = pcVar39;
      lVar23 = local_240;
      piVar15 = local_250;
      local_2a8 = pcVar44;
      local_1d8 = pcVar39;
      uVar32 = local_298;
      if (pcVar39 < pcVar44) {
        do {
          uVar26 = 0;
          local_1c8[0] = (longlong)pcVar38 * 0x14;
          piVar15 = (int *)(local_1c8[0] + lVar28);
          local_1f8 = *(undefined8 *)piVar15;
          uStack_1f0 = *(undefined8 *)(piVar15 + 2);
          uVar35 = uVar33;
          if (((-1 < iVar27) && (uVar35 = uVar26, pcVar39 < pcVar42 + -1)) &&
             (local_268 = *(undefined4 *)(local_1c8[0] + 0x10 + lVar28), piVar15[3] == 1)) {
            puVar3 = (ulonglong *)(lVar28 + 0x14 + (longlong)pcVar38 * 0x14);
            local_168 = (char *)*puVar3;
            uStack_160 = puVar3[1];
            if ((*(int *)(lVar28 + 0x24 + (longlong)pcVar38 * 0x14) == iVar27) &&
               ((int)local_168 == 1)) {
              uVar35 = (ulonglong)local_168 & 0xffffffff;
              local_1a8 = local_168;
              uStack_1a0 = uStack_160;
            }
          }
          if ((*piVar15 == 4) && ((int)uVar35 != 0)) {
            local_1d8 = pcVar38;
            local_1b8 = (char *)FUN_140024540(plVar29);
            lVar25 = local_258;
            lVar28 = 0x21;
            plVar30 = (longlong *)(local_1b8 + 0x948);
            do {
              puVar18 = (undefined8 *)*plVar30;
              iVar46 = 0;
              local_288 = (char *)0x0;
              uStack_280 = 0;
              local_278 = (char *)*puVar18;
              uStack_270 = puVar18[1];
              iVar34 = FUN_1400355d0(&local_220,iVar27 + 1,&local_278);
              pcVar42 = (char *)(longlong)iVar34;
              local_278 = (char *)0x0;
              uVar26 = 0;
              uVar35 = uVar33;
              iVar11 = iVar31;
              if (((-1 < iVar34) && (uVar35 = uVar26, pcVar42 < local_248 + -1)) &&
                 (*(int *)(lVar25 + (longlong)pcVar42 * 0x14 + 0xc) == 1)) {
                puVar43 = (undefined8 *)(lVar25 + 0x14 + (longlong)pcVar42 * 0x14);
                local_188 = (char *)*puVar43;
                uStack_180 = puVar43[1];
                if ((*(int *)(lVar25 + 0x24 + (longlong)pcVar42 * 0x14) == iVar34) &&
                   ((int)local_188 == 4)) {
                  local_268 = *(undefined4 *)(lVar25 + 0x24 + (longlong)pcVar42 * 0x14);
                  uVar35 = uStack_180 & 0xffffffff;
                  iVar11 = 4;
                  local_278 = local_188;
                }
              }
              iVar34 = 0;
              pcVar42 = (char *)0x0;
              if (iVar11 == 4) {
                iVar46 = (int)((ulonglong)local_278 >> 0x20);
                local_288 = (char *)(iVar46 + local_240);
                iVar46 = (int)uVar35 - iVar46;
                uStack_280._4_4_ = iVar46 >> 0x1f;
                iVar34 = uStack_280._4_4_;
                uStack_280 = (longlong)iVar46;
                pcVar42 = local_288;
              }
              local_278 = pcVar42;
              uStack_270 = CONCAT44(iVar34,iVar46);
              FUN_140034a90(&local_278,puVar18);
              plVar29 = local_1e0;
              plVar30 = plVar30 + 1;
              lVar28 = lVar28 + -1;
            } while (lVar28 != 0);
            iVar11 = (int)((ulonglong)local_1f8 >> 0x20);
            puVar18 = (undefined8 *)
                      FUN_1401b8530(local_150,local_1e0,iVar11 + local_240,
                                    (longlong)((int)uStack_1f0 - iVar11));
            pcVar42 = local_1b8;
            pcVar38 = local_1d8;
            uVar13 = puVar18[1];
            *(undefined8 *)(local_1b8 + 0xa50) = *puVar18;
            *(undefined8 *)(local_1b8 + 0xa58) = uVar13;
            lVar25 = plVar29[4];
            pvVar40 = (LPVOID)plVar29[5];
            uVar35 = plVar29[3];
            uVar19 = (ulonglong)((int)lVar25 + (int)pvVar40 & 7);
            uVar26 = 8 - uVar19;
            if (uVar19 == 0) {
              uVar26 = uVar33;
            }
            lVar28 = uVar26 + 0x18;
            if (uVar35 < (ulonglong)(lVar25 + lVar28)) {
              if ((ulonglong)plVar29[2] < (ulonglong)(lVar25 + lVar28)) {
                FUN_1401b5c50(plVar29,lVar28);
                pvVar40 = (LPVOID)plVar29[5];
                lVar25 = plVar29[4];
                uVar35 = plVar29[3];
                uVar19 = (ulonglong)((int)lVar25 + (int)pvVar40 & 7);
                uVar26 = 8 - uVar19;
                if (uVar19 == 0) {
                  uVar26 = uVar33;
                }
                lVar28 = uVar26 + 0x18;
              }
              lVar23 = *plVar29;
              uVar19 = uVar33;
              if ((lVar23 != 0) && (uVar16 = lVar23 - 1U & lVar25 + lVar28, uVar16 != 0)) {
                uVar19 = lVar23 - uVar16;
              }
              uVar19 = uVar19 + lVar25 + lVar28;
              if (uVar35 < uVar19) {
                plVar29[3] = uVar19;
                VirtualAlloc(pvVar40,uVar19,0x1000,4);
              }
            }
            puVar18 = (undefined8 *)(plVar29[4] + uVar26 + plVar29[5]);
            *puVar18 = 0;
            puVar18[1] = 0;
            puVar18[2] = 0;
            plVar29[4] = plVar29[4] + lVar28;
            plVar29[6] = (longlong)puVar18;
            *puVar18 = pcVar42;
            if (plVar29[10] == 0) {
              plVar29[0xb] = (longlong)puVar18;
              plVar29[10] = (longlong)puVar18;
              puVar18[1] = 0;
            }
            else {
              *(undefined8 **)(plVar29[0xb] + 0x10) = puVar18;
              puVar18[1] = plVar29[0xb];
              plVar29[0xb] = (longlong)puVar18;
            }
            puVar18[2] = 0;
            plVar29[0xc] = plVar29[0xc] + 1;
            lVar25 = plVar29[4];
            pvVar40 = (LPVOID)plVar29[5];
            uVar35 = plVar29[3];
            uVar19 = (ulonglong)((int)lVar25 + (int)pvVar40 & 7);
            uVar26 = 8 - uVar19;
            if (uVar19 == 0) {
              uVar26 = uVar33;
            }
            lVar28 = uVar26 + 0x20;
            if (uVar35 < (ulonglong)(lVar25 + lVar28)) {
              if ((ulonglong)plVar29[2] < (ulonglong)(lVar25 + lVar28)) {
                FUN_1401b5c50(plVar29,lVar28);
                pvVar40 = (LPVOID)plVar29[5];
                lVar25 = plVar29[4];
                uVar35 = plVar29[3];
                uVar19 = (ulonglong)((int)lVar25 + (int)pvVar40 & 7);
                uVar26 = 8 - uVar19;
                if (uVar19 == 0) {
                  uVar26 = uVar33;
                }
                lVar28 = uVar26 + 0x20;
              }
              lVar23 = *plVar29;
              uVar19 = uVar33;
              if ((lVar23 != 0) && (uVar16 = lVar23 - 1U & lVar25 + lVar28, uVar16 != 0)) {
                uVar19 = lVar23 - uVar16;
              }
              uVar19 = uVar19 + lVar25 + lVar28;
              if (uVar35 < uVar19) {
                plVar29[3] = uVar19;
                VirtualAlloc(pvVar40,uVar19,0x1000,4);
              }
            }
            puVar18 = (undefined8 *)(plVar29[4] + uVar26 + plVar29[5]);
            *puVar18 = 0;
            puVar18[1] = 0;
            puVar18[2] = 0;
            puVar18[3] = 0;
            plVar29[4] = plVar29[4] + lVar28;
            plVar29[6] = (longlong)puVar18;
            uVar13 = *(undefined8 *)(pcVar42 + 0xa58);
            *puVar18 = *(undefined8 *)(pcVar42 + 0xa50);
            puVar18[1] = uVar13;
            lVar25 = plVar29[9];
            if (*(longlong *)(lVar25 + 0x270) == 0) {
              *(undefined8 **)(lVar25 + 0x278) = puVar18;
              *(undefined8 **)(plVar29[9] + 0x270) = puVar18;
              puVar18[2] = 0;
            }
            else {
              *(undefined8 **)(*(longlong *)(lVar25 + 0x278) + 0x18) = puVar18;
              puVar18[2] = *(undefined8 *)(plVar29[9] + 0x278);
              *(undefined8 **)(plVar29[9] + 0x278) = puVar18;
            }
            puVar18[3] = 0;
            *(longlong *)(plVar29[9] + 0x280) = *(longlong *)(plVar29[9] + 0x280) + 1;
            lVar25 = plVar29[4];
            pvVar40 = (LPVOID)plVar29[5];
            uVar35 = plVar29[3];
            uVar19 = (ulonglong)((int)lVar25 + (int)pvVar40 & 7);
            uVar26 = 8 - uVar19;
            if (uVar19 == 0) {
              uVar26 = uVar33;
            }
            lVar28 = uVar26 + 0x20;
            if (uVar35 < (ulonglong)(lVar28 + lVar25)) {
              if ((ulonglong)plVar29[2] < (ulonglong)(lVar28 + lVar25)) {
                FUN_1401b5c50(plVar29,lVar28);
                pvVar40 = (LPVOID)plVar29[5];
                lVar25 = plVar29[4];
                uVar35 = plVar29[3];
                uVar19 = (ulonglong)((int)lVar25 + (int)pvVar40 & 7);
                uVar26 = 8 - uVar19;
                if (uVar19 == 0) {
                  uVar26 = uVar33;
                }
                lVar28 = uVar26 + 0x20;
              }
              lVar23 = *plVar29;
              uVar19 = uVar33;
              if ((lVar23 != 0) && (uVar16 = lVar23 - 1U & lVar25 + lVar28, uVar16 != 0)) {
                uVar19 = lVar23 - uVar16;
              }
              uVar19 = uVar19 + lVar25 + lVar28;
              if (uVar35 < uVar19) {
                plVar29[3] = uVar19;
                VirtualAlloc(pvVar40,uVar19,0x1000,4);
              }
            }
            puVar18 = (undefined8 *)(plVar29[4] + uVar26 + plVar29[5]);
            *puVar18 = 0;
            puVar18[1] = 0;
            puVar18[2] = 0;
            puVar18[3] = 0;
            plVar29[4] = plVar29[4] + lVar28;
            plVar29[6] = (longlong)puVar18;
            uVar13 = *(undefined8 *)(pcVar42 + 0xa58);
            *puVar18 = *(undefined8 *)(pcVar42 + 0xa50);
            puVar18[1] = uVar13;
            lVar25 = plVar29[9];
            if (*(longlong *)(lVar25 + 0x2b8) == 0) {
              *(undefined8 **)(lVar25 + 0x2c0) = puVar18;
              *(undefined8 **)(plVar29[9] + 0x2b8) = puVar18;
              puVar18[2] = 0;
            }
            else {
              *(undefined8 **)(*(longlong *)(lVar25 + 0x2c0) + 0x18) = puVar18;
              puVar18[2] = *(undefined8 *)(plVar29[9] + 0x2c0);
              *(undefined8 **)(plVar29[9] + 0x2c0) = puVar18;
            }
            puVar18[3] = 0;
            *(longlong *)(plVar29[9] + 0x2c8) = *(longlong *)(plVar29[9] + 0x2c8) + 1;
            lVar25 = plVar29[4];
            pvVar40 = (LPVOID)plVar29[5];
            uVar35 = plVar29[3];
            uVar19 = (ulonglong)((int)lVar25 + (int)pvVar40 & 7);
            uVar26 = 8 - uVar19;
            if (uVar19 == 0) {
              uVar26 = uVar33;
            }
            lVar28 = uVar26 + 0x20;
            if (uVar35 < (ulonglong)(lVar28 + lVar25)) {
              if ((ulonglong)plVar29[2] < (ulonglong)(lVar28 + lVar25)) {
                FUN_1401b5c50(plVar29,lVar28);
                pvVar40 = (LPVOID)plVar29[5];
                lVar25 = plVar29[4];
                uVar35 = plVar29[3];
                uVar19 = (ulonglong)((int)lVar25 + (int)pvVar40 & 7);
                uVar26 = 8 - uVar19;
                if (uVar19 == 0) {
                  uVar26 = uVar33;
                }
                lVar28 = uVar26 + 0x20;
              }
              lVar23 = *plVar29;
              uVar19 = uVar33;
              if ((lVar23 != 0) && (uVar16 = lVar23 - 1U & lVar25 + lVar28, uVar16 != 0)) {
                uVar19 = lVar23 - uVar16;
              }
              uVar19 = uVar19 + lVar25 + lVar28;
              if (uVar35 < uVar19) {
                plVar29[3] = uVar19;
                VirtualAlloc(pvVar40,uVar19,0x1000,4);
              }
            }
            puVar18 = (undefined8 *)(plVar29[4] + uVar26 + plVar29[5]);
            *puVar18 = 0;
            puVar18[1] = 0;
            puVar18[2] = 0;
            puVar18[3] = 0;
            plVar29[4] = plVar29[4] + lVar28;
            plVar29[6] = (longlong)puVar18;
            uVar13 = *(undefined8 *)(pcVar42 + 0xa58);
            *puVar18 = *(undefined8 *)(pcVar42 + 0xa50);
            puVar18[1] = uVar13;
            lVar25 = plVar29[9];
            if (*(longlong *)(lVar25 + 0x300) == 0) {
              *(undefined8 **)(lVar25 + 0x308) = puVar18;
              *(undefined8 **)(plVar29[9] + 0x300) = puVar18;
              puVar18[2] = 0;
            }
            else {
              *(undefined8 **)(*(longlong *)(lVar25 + 0x308) + 0x18) = puVar18;
              puVar18[2] = *(undefined8 *)(plVar29[9] + 0x308);
              *(undefined8 **)(plVar29[9] + 0x308) = puVar18;
            }
            puVar18[3] = 0;
            *(longlong *)(plVar29[9] + 0x310) = *(longlong *)(plVar29[9] + 0x310) + 1;
            pcVar44 = local_2a8;
            pcVar42 = local_248;
            lVar28 = local_258;
            uVar32 = local_298;
          }
          uVar49 = (undefined4)((ulonglong)uVar14 >> 0x20);
          puVar20 = (uint *)(lVar28 + 0x10 + local_1c8[0]);
          do {
            iVar27 = iVar27 + 1;
            puVar20 = puVar20 + 5;
            pcVar39 = (char *)(longlong)iVar27;
            pcVar38 = pcVar38 + 1;
            if (pcVar42 <= pcVar39) break;
          } while (*puVar20 != uVar32);
          lVar23 = local_240;
          piVar15 = local_250;
          local_1d8 = pcVar38;
        } while (pcVar39 < pcVar44);
      }
    }
  }
  lVar25 = *(longlong *)(param_1 + 0x20);
  if (lVar25 != 0) {
    uStack_280 = 7;
    local_288 = "options";
    uVar14 = CONCAT44(uVar49,1);
    lVar21 = FUN_1401b65d0(piVar15,lVar25,0x18,&local_288,uVar14);
    uVar49 = (undefined4)((ulonglong)uVar14 >> 0x20);
    lVar28 = *(longlong *)(piVar15 + 2);
    if (lVar21 != 0) {
      lVar28 = *(longlong *)(lVar21 + 8);
      *(longlong *)(piVar15 + 4) = lVar28;
      lVar25 = *(longlong *)(param_1 + 0x20);
    }
    iVar27 = *(int *)(lVar25 + 0x10 + lVar28 * 0x18);
    if (0 < iVar27) {
      lVar28 = 0xb40;
      lVar25 = 0x28;
      do {
        puVar18 = *(undefined8 **)(lVar28 + plVar29[9]);
        iVar11 = *(int *)(puVar18 + 2);
        if (iVar11 == 1) {
          local_288 = (char *)0x0;
          uStack_280 = 0;
          local_278 = (char *)*puVar18;
          uStack_270 = puVar18[1];
          iVar46 = FUN_1400355d0(&local_220,iVar27,&local_278);
          pcVar42 = (char *)(longlong)iVar46;
          local_278 = (char *)0x0;
          iVar11 = iVar31;
          iVar34 = iVar31;
          if (((-1 < iVar46) && (iVar34 = iVar12, pcVar42 < local_248 + -1)) &&
             (*(int *)(local_258 + (longlong)pcVar42 * 0x14 + 0xc) == 1)) {
            puVar3 = (ulonglong *)(local_258 + 0x14 + (longlong)pcVar42 * 0x14);
            local_1a8 = (char *)*puVar3;
            puVar8 = puVar3 + 1;
            uStack_1a0 = *puVar8;
            if ((*(int *)(local_258 + 0x24 + (longlong)pcVar42 * 0x14) == iVar46) &&
               ((int)*puVar3 == 4)) {
              local_268 = *(undefined4 *)(local_258 + 0x24 + (longlong)pcVar42 * 0x14);
              iVar11 = 4;
              iVar34 = (int)*puVar8;
              local_278 = (char *)*puVar3;
              uStack_270 = *puVar8;
            }
          }
          pcVar42 = local_288;
          uVar35 = uStack_280;
          if (iVar11 == 4) {
            iVar11 = (int)((ulonglong)local_278 >> 0x20);
            pcVar42 = (char *)(iVar11 + lVar23);
            uVar35 = (longlong)(iVar34 - iVar11);
          }
          if (uVar35 != 0) {
            *(undefined4 *)(puVar18 + 2) = 1;
            puVar22 = (undefined4 *)FUN_1401b8530(local_150,plVar29,pcVar42);
            uVar49 = puVar22[1];
            uVar6 = puVar22[2];
            uVar7 = puVar22[3];
            *(undefined4 *)(puVar18 + 3) = *puVar22;
            *(undefined4 *)((longlong)puVar18 + 0x1c) = uVar49;
            *(undefined4 *)(puVar18 + 4) = uVar6;
            *(undefined4 *)((longlong)puVar18 + 0x24) = uVar7;
          }
        }
        else if (iVar11 == 2) {
          local_278 = (char *)*puVar18;
          uStack_270 = puVar18[1];
          iVar46 = 0;
          local_288 = (char *)0x0;
          uStack_280 = 0;
          iVar34 = FUN_1400355d0(&local_220,iVar27,&local_278);
          pcVar42 = (char *)(longlong)iVar34;
          local_278 = (char *)0x0;
          uVar26 = 0;
          uVar35 = uVar33;
          iVar11 = iVar31;
          if (((-1 < iVar34) && (uVar35 = uVar26, pcVar42 < local_248 + -1)) &&
             (*(int *)(local_258 + (longlong)pcVar42 * 0x14 + 0xc) == 1)) {
            puVar3 = (ulonglong *)(local_258 + 0x14 + (longlong)pcVar42 * 0x14);
            local_168 = (char *)*puVar3;
            uStack_160 = puVar3[1];
            if ((*(int *)(local_258 + 0x24 + (longlong)pcVar42 * 0x14) == iVar34) &&
               ((int)local_168 == 8)) {
              local_268 = *(undefined4 *)(local_258 + 0x24 + (longlong)pcVar42 * 0x14);
              uVar35 = uStack_160 & 0xffffffff;
              iVar11 = 8;
              local_278 = local_168;
            }
          }
          iVar34 = 0;
          pcVar42 = (char *)0x0;
          if (iVar11 == 8) {
            iVar46 = (int)((ulonglong)local_278 >> 0x20);
            local_288 = (char *)(iVar46 + lVar23);
            iVar46 = (int)uVar35 - iVar46;
            uStack_280._4_4_ = iVar46 >> 0x1f;
            iVar34 = uStack_280._4_4_;
            uStack_280 = (longlong)iVar46;
            pcVar42 = local_288;
          }
          local_278 = pcVar42;
          uStack_270 = CONCAT44(iVar34,iVar46);
          FUN_1400349d0(&local_278,puVar18);
        }
        else if (iVar11 == 4) {
          FUN_14003bf80(plVar29,&local_220,iVar27,puVar18);
        }
        else if (iVar11 == 5) {
          local_288 = (char *)0x0;
          uStack_280 = 0;
          local_278 = (char *)*puVar18;
          uStack_270 = puVar18[1];
          iVar46 = FUN_1400355d0(&local_220,iVar27,&local_278);
          pcVar42 = (char *)(longlong)iVar46;
          local_278 = (char *)0x0;
          iVar11 = iVar31;
          iVar34 = iVar31;
          if (((-1 < iVar46) && (pcVar42 < local_248 + -1)) &&
             (*(int *)(local_258 + (longlong)pcVar42 * 0x14 + 0xc) == 1)) {
            piVar15 = (int *)(local_258 + 0x14 + (longlong)pcVar42 * 0x14);
            local_188 = *(char **)piVar15;
            puVar3 = (ulonglong *)(piVar15 + 2);
            uStack_180 = *puVar3;
            if ((*(int *)(local_258 + 0x24 + (longlong)pcVar42 * 0x14) == iVar46) && (*piVar15 == 8)
               ) {
              local_268 = *(undefined4 *)(local_258 + 0x24 + (longlong)pcVar42 * 0x14);
              iVar11 = 8;
              iVar34 = (int)*puVar3;
              local_278 = *(char **)piVar15;
              uStack_270 = *puVar3;
            }
          }
          pcVar42 = local_288;
          uVar35 = uStack_280;
          if (iVar11 == 8) {
            iVar11 = (int)((ulonglong)local_278 >> 0x20);
            pcVar42 = (char *)(iVar11 + lVar23);
            uVar35 = (longlong)(iVar34 - iVar11);
          }
          if (uVar35 != 0) {
            *(undefined4 *)(puVar18 + 2) = 5;
            iVar11 = 0;
            uVar26 = uVar33;
            for (uVar19 = (ulonglong)(*pcVar42 == '-'); uVar19 < uVar35; uVar19 = uVar19 + 1) {
              iVar11 = (int)uVar26;
              if (9 < (byte)(pcVar42[uVar19] - 0x30U)) break;
              uVar26 = (longlong)(pcVar42[uVar19] + -0x30) + uVar26 * 10;
              iVar11 = (int)uVar26;
            }
            iVar34 = -iVar11;
            if (*pcVar42 != '-') {
              iVar34 = iVar11;
            }
            *(int *)(puVar18 + 3) = iVar34;
          }
        }
        uVar49 = (undefined4)((ulonglong)uVar14 >> 0x20);
        lVar28 = lVar28 + 8;
        lVar25 = lVar25 + -1;
      } while (lVar25 != 0);
      iVar31 = *(int *)(plVar29[9] + 0x60);
      iVar27 = 8;
      if ((7 < iVar31) && (iVar27 = iVar31, 0x10 < iVar31)) {
        iVar27 = 0x10;
      }
      *(int *)(plVar29[9] + 0x60) = iVar27;
      iVar31 = *(int *)(plVar29[9] + 0xf0);
      iVar27 = 8;
      if ((7 < iVar31) && (iVar27 = iVar31, 0x10 < iVar31)) {
        iVar27 = 0x10;
      }
      *(int *)(plVar29[9] + 0xf0) = iVar27;
      iVar31 = *(int *)(plVar29[9] + 0x138);
      iVar27 = 1;
      if ((0 < iVar31) && (iVar27 = iVar31, 10 < iVar31)) {
        iVar27 = 10;
      }
      *(int *)(plVar29[9] + 0x138) = iVar27;
      iVar31 = *(int *)(plVar29[9] + 0x378);
      if ((-1 < iVar31) && (iVar12 = iVar31, 100 < iVar31)) {
        iVar12 = 100;
      }
      *(int *)(plVar29[9] + 0x378) = iVar12;
    }
  }
  lVar25 = *(longlong *)(param_1 + 0x20);
  if (lVar25 != 0) {
    uStack_280 = 7;
    local_288 = &DAT_1401d1898;
    lVar23 = FUN_1401b65d0(param_1 + 0x12,lVar25,0x18,&local_288,CONCAT44(uVar49,1));
    lVar28 = *(longlong *)(param_1 + 0x14);
    if (lVar23 != 0) {
      lVar28 = *(longlong *)(lVar23 + 8);
      *(longlong *)(param_1 + 0x16) = lVar28;
    }
    if (0 < *(int *)(lVar25 + 0x10 + lVar28 * 0x18)) {
      FUN_140035100(plVar29,&local_220);
    }
  }
  uVar32 = (0 < *param_1) + 4;
  param_1[2] = iVar4;
  param_1[3] = iVar5;
  param_1[4] = iVar47;
  param_1[5] = iVar48;
  *(char **)(param_1 + 6) = local_210;
  *(longlong *)(param_1 + 8) = lStack_208;
  *(undefined8 *)(param_1 + 10) = local_200;
LAB_14003b060:
  param_1[0xc] = uVar32;
  return param_1;
switchD_140039d19_caseD_9:
  uVar32 = (int)uVar19 - 1;
switchD_140039ca1_caseD_5b:
  local_298 = (int)uVar26 + 1;
  uVar26 = (ulonglong)local_298;
switchD_140039ca1_caseD_9:
  local_2a8._0_4_ = uVar32 + 1;
  uVar19 = (ulonglong)(uint)local_2a8;
  if (uVar35 <= uVar19) goto LAB_140039d3f;
  goto LAB_140039c70;
switchD_14003a0e0_caseD_9:
  if (lVar28 != 0) {
    uVar26 = (ulonglong)local_2a8._4_4_;
    if (uVar32 <= local_2a8._4_4_) {
LAB_14003a2d4:
      iVar27 = -1;
      goto LAB_14003a1a3;
    }
    uVar41 = local_2a8._4_4_ + 1;
    local_2a8 = (char *)((ulonglong)uVar41 << 0x20);
    *(undefined4 *)(lVar28 + uVar26 * 0x14) = 8;
    *(int *)(lVar28 + 4 + uVar26 * 0x14) = iVar11;
    *(int *)(lVar28 + 8 + uVar26 * 0x14) = iVar27;
    *(undefined4 *)(lVar28 + 0xc + uVar26 * 0x14) = 0;
    *(uint *)(lVar28 + 0x10 + uVar26 * 0x14) = local_2a0;
  }
  iVar11 = iVar27 + -1;
  uVar19 = (ulonglong)(iVar34 + 1);
  uVar37 = local_2a0;
  if ((local_2a0 != 0xffffffff) && (lVar28 != 0)) {
    piVar15 = (int *)(lVar28 + 0xc + (longlong)(int)local_2a0 * 0x14);
    *piVar15 = *piVar15 + 1;
  }
switchD_140039e99_caseD_9:
  iVar27 = (int)uVar19;
  uVar26 = (ulonglong)(iVar11 + 1U);
  local_2a8 = (char *)CONCAT44(local_2a8._4_4_,iVar11 + 1U);
  if (uVar35 <= uVar26) goto LAB_14003a169;
  goto LAB_140039e60;
LAB_14003a0f3:
  iVar27 = -3;
  goto LAB_14003a1a3;
}

