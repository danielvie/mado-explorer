
ulonglong FUN_140123060(longlong *param_1,undefined1 (*param_2) [16],undefined1 (*param_3) [16],
                       longlong *param_4,longlong *param_5,longlong *param_6,longlong *param_7,
                       uint *param_8,int *param_9,int param_10)

{
  undefined8 *puVar1;
  undefined8 uVar2;
  undefined1 auVar3 [16];
  uint uVar4;
  longlong lVar5;
  ulonglong uVar6;
  longlong lVar7;
  longlong lVar8;
  int iVar9;
  uint uVar10;
  undefined4 *puVar11;
  undefined4 *puVar12;
  longlong lVar13;
  ulonglong uVar14;
  ulonglong uVar15;
  ulonglong uVar16;
  ulonglong uVar17;
  LPVOID pvVar18;
  undefined8 uVar19;
  longlong lVar20;
  float fVar21;
  undefined4 uVar22;
  undefined4 uVar23;
  undefined4 uVar24;
  undefined4 uVar25;
  undefined4 uVar26;
  undefined4 uVar27;
  undefined4 uVar28;
  undefined4 uVar29;
  uint local_res10;
  int local_res18 [2];
  ulonglong uVar30;
  undefined8 in_stack_fffffffffffffeb0;
  undefined4 uVar32;
  undefined8 uVar31;
  char *in_stack_fffffffffffffeb8;
  char *pcVar33;
  uint local_128;
  undefined4 local_124;
  int local_120;
  uint local_11c;
  undefined1 local_118 [16];
  undefined4 *local_108;
  undefined1 local_f8 [16];
  ulonglong local_e8;
  ulonglong local_e0;
  undefined8 local_d8;
  undefined8 uStack_d0;
  longlong local_c8;
  undefined8 local_b8;
  undefined8 uStack_b0;
  undefined4 local_a8;
  undefined4 uStack_a4;
  undefined4 uStack_a0;
  undefined4 uStack_9c;
  
  uVar6 = 0;
  local_128 = 0;
  local_118 = *param_2;
  uVar17 = local_118._8_8_;
  uVar26 = local_118._0_4_;
  uVar27 = local_118._4_4_;
  uVar28 = local_118._8_4_;
  uVar29 = local_118._12_4_;
  if (0x400 < uVar17) {
    local_f8._0_8_ = local_118._0_8_;
    uVar14 = uVar6;
    uVar16 = uVar6;
    uVar19 = local_f8._0_8_;
    local_f8 = local_118;
    if (uVar17 != 0) {
      do {
        if ((0x3ff < uVar14) || (lVar5 = FUN_140187e70(uVar19), lVar5 == 0)) break;
        uVar14 = uVar14 + lVar5;
        uVar16 = uVar16 + 1;
      } while (uVar16 < uVar17);
    }
    local_118._0_4_ = (undefined4)uVar19;
    local_118._4_4_ = (undefined4)((ulonglong)uVar19 >> 0x20);
    local_118._8_4_ = (undefined4)uVar14;
    local_118._12_4_ = (undefined4)(uVar14 >> 0x20);
    uVar26 = local_118._0_4_;
    uVar27 = local_118._4_4_;
    uVar28 = local_118._8_4_;
    uVar29 = local_118._12_4_;
  }
  uVar17 = 0;
  local_118 = *param_3;
  *(undefined4 *)*param_2 = uVar26;
  *(undefined4 *)(*param_2 + 4) = uVar27;
  *(undefined4 *)(*param_2 + 8) = uVar28;
  *(undefined4 *)(*param_2 + 0xc) = uVar29;
  uVar14 = local_118._8_8_;
  uVar22 = local_118._0_4_;
  uVar23 = local_118._4_4_;
  uVar24 = local_118._8_4_;
  uVar25 = local_118._12_4_;
  local_f8 = local_118;
  if (0x400 < uVar14) {
    local_f8._0_8_ = local_118._0_8_;
    uVar16 = uVar17;
    uVar19 = local_f8._0_8_;
    if (uVar14 != 0) {
      do {
        if ((0x3ff < uVar17) || (lVar5 = FUN_140187e70(uVar19), lVar5 == 0)) break;
        uVar17 = uVar17 + lVar5;
        uVar16 = uVar16 + 1;
      } while (uVar16 < uVar14);
    }
    local_118._8_8_ = uVar17;
    local_118._0_8_ = uVar19;
    auVar3 = local_118;
    local_118._0_4_ = (undefined4)uVar19;
    local_118._4_4_ = (undefined4)((ulonglong)uVar19 >> 0x20);
    local_118._8_4_ = (undefined4)uVar17;
    local_118._12_4_ = (undefined4)(uVar17 >> 0x20);
    uVar22 = local_118._0_4_;
    uVar23 = local_118._4_4_;
    uVar24 = local_118._8_4_;
    uVar25 = local_118._12_4_;
    local_118 = auVar3;
  }
  *(undefined4 *)*param_3 = uVar22;
  *(undefined4 *)(*param_3 + 4) = uVar23;
  *(undefined4 *)(*param_3 + 8) = uVar24;
  *(undefined4 *)(*param_3 + 0xc) = uVar25;
  if (param_10 == 0) {
    local_e8 = *(ulonglong *)(*param_2 + 8);
    lVar5 = param_1[4];
    pvVar18 = (LPVOID)param_1[5];
    uVar16 = param_1[3];
    uVar17 = *(ulonglong *)(*param_3 + 8);
    uVar14 = uVar17;
    if (uVar17 < local_e8) {
      uVar14 = local_e8;
    }
    uVar15 = (ulonglong)((int)lVar5 + (int)pvVar18 & 3);
    uVar30 = 4 - uVar15;
    if (uVar15 == 0) {
      uVar30 = 0;
    }
    lVar13 = uVar30 + (uVar14 & 0xffffffff) * 4;
    local_e0 = uVar17;
    if (uVar16 < (ulonglong)(lVar5 + lVar13)) {
      if ((ulonglong)param_1[2] < (ulonglong)(lVar5 + lVar13)) {
        FUN_1401b5c50(param_1,lVar13,4);
        pvVar18 = (LPVOID)param_1[5];
        lVar5 = param_1[4];
        uVar16 = param_1[3];
        uVar15 = (ulonglong)((int)lVar5 + (int)pvVar18 & 3);
        uVar30 = 4 - uVar15;
        if (uVar15 == 0) {
          uVar30 = uVar6;
        }
        lVar13 = uVar30 + (uVar14 & 0xffffffff) * 4;
      }
      lVar7 = 0;
      lVar20 = *param_1;
      if ((lVar20 != 0) && (uVar15 = lVar20 - 1U & lVar5 + lVar13, uVar15 != 0)) {
        lVar7 = lVar20 - uVar15;
      }
      uVar15 = lVar7 + lVar5 + lVar13;
      if (uVar16 < uVar15) {
        param_1[3] = uVar15;
        VirtualAlloc(pvVar18,uVar15,0x1000,4);
      }
    }
    uVar16 = 0;
    iVar9 = 0;
    local_120 = 0;
    puVar12 = (undefined4 *)(param_1[5] + param_1[4] + uVar30);
    local_11c = 0;
    param_1[4] = param_1[4] + lVar13;
    lVar5 = *param_4;
    param_1[6] = (longlong)puVar12;
    uVar10 = 0;
    local_res10 = 0;
    local_f8._0_8_ = lVar5;
    if (lVar5 != 0) {
      local_118._0_8_ = param_4[2];
      lVar5 = param_4[2];
      uVar6 = uVar16;
      local_108 = puVar12;
      do {
        local_res18[0] = 0;
        local_124 = 0;
        local_b8 = CONCAT44(uVar27,uVar26);
        uStack_b0 = CONCAT44(uVar29,uVar28);
        puVar1 = (undefined8 *)(lVar5 + uVar16 * 0x10);
        uVar19 = *puVar1;
        uVar2 = puVar1[1];
        pcVar33 = (char *)((ulonglong)in_stack_fffffffffffffeb8 & 0xffffffff00000000);
        uVar31 = CONCAT44((int)((ulonglong)in_stack_fffffffffffffeb0 >> 0x20),(int)uVar14);
        uVar30 = 0;
        local_d8 = uVar19;
        uStack_d0 = uVar2;
        local_a8 = uVar26;
        uStack_a4 = uVar27;
        uStack_a0 = uVar28;
        uStack_9c = uVar29;
        uVar4 = FUN_140065230(param_1,&local_d8,&local_a8,local_res18,&local_b8,0,puVar12,uVar31,
                              pcVar33,&local_124,10);
        uVar32 = (undefined4)((ulonglong)uVar31 >> 0x20);
        uVar16 = (ulonglong)uVar4;
        puVar11 = puVar12;
        uVar10 = (uint)uVar6;
        if ((uVar4 != 0) && (local_128 = 1, param_6 != (longlong *)0x0)) {
          if (*param_6 == 0) {
            lVar5 = param_6[2];
            if (lVar5 == 0) {
              uVar15 = 1;
              if (1 < local_e8) {
                do {
                  uVar15 = uVar15 * 2;
                } while (uVar15 < local_e8);
              }
              lVar5 = param_5[4];
              pvVar18 = (LPVOID)param_5[5];
              uVar6 = param_5[3];
              uVar17 = (ulonglong)((int)lVar5 + (int)pvVar18 & 7);
              lVar13 = 8 - uVar17;
              if (uVar17 == 0) {
                lVar13 = 0;
              }
              lVar20 = lVar13 + uVar15 * 4;
              if (uVar6 < (ulonglong)(lVar5 + lVar20)) {
                if ((ulonglong)param_5[2] < (ulonglong)(lVar5 + lVar20)) {
                  FUN_1401b5c50(param_5,lVar20,8);
                  pvVar18 = (LPVOID)param_5[5];
                  lVar5 = param_5[4];
                  uVar6 = param_5[3];
                  uVar17 = (ulonglong)((int)lVar5 + (int)pvVar18 & 7);
                  lVar13 = 8 - uVar17;
                  if (uVar17 == 0) {
                    lVar13 = 0;
                  }
                  lVar20 = lVar13 + uVar15 * 4;
                }
                lVar8 = 0;
                lVar7 = *param_5;
                if ((lVar7 != 0) && (uVar17 = lVar7 - 1U & lVar20 + lVar5, uVar17 != 0)) {
                  lVar8 = lVar7 - uVar17;
                }
                uVar17 = lVar8 + lVar20 + lVar5;
                if (uVar6 < uVar17) {
                  param_5[3] = uVar17;
                  VirtualAlloc(pvVar18,uVar17,0x1000,4);
                }
              }
              lVar5 = param_5[4] + lVar13 + param_5[5];
              param_5[4] = param_5[4] + lVar20;
              param_5[6] = lVar5;
              *param_6 = 0;
              puVar12 = local_108;
              uVar17 = local_e0;
LAB_140123593:
              param_6[1] = uVar15;
            }
            else {
              uVar6 = param_6[1];
              if (uVar6 < local_e8) {
                uVar15 = local_e8;
                if (local_e8 < uVar6 * 2) {
                  uVar15 = uVar6 * 2;
                }
                pcVar33 = "_ArrayGrow";
                uVar31 = CONCAT44(uVar32,0xa4b);
                lVar5 = FUN_1401b5ae0(param_5,lVar5,uVar6 * 4,uVar15 * 4,8,
                                      uVar30 & 0xffffffff00000000,
                                      "C:\\Users\\Vjekoslav\\Projects\\Voidstar\\File Pilot\\Source\\Program\\Voidstar.h"
                                      ,uVar31,"_ArrayGrow");
                goto LAB_140123593;
              }
            }
            param_6[2] = lVar5;
          }
          do {
            FUN_14012c1d0(param_5,param_6,*puVar12);
            uVar32 = (undefined4)((ulonglong)uVar31 >> 0x20);
            puVar12 = puVar12 + 1;
            uVar16 = uVar16 - 1;
            puVar11 = local_108;
            uVar10 = local_res10;
          } while (uVar16 != 0);
        }
        uVar10 = uVar10 + local_res18[0];
        in_stack_fffffffffffffeb8 = (char *)((ulonglong)pcVar33 & 0xffffffff00000000);
        in_stack_fffffffffffffeb0 = CONCAT44(uVar32,(int)uVar14);
        uVar16 = 0;
        local_res18[0] = 0;
        local_124 = 0;
        local_d8 = CONCAT44(uVar23,uVar22);
        uStack_d0 = CONCAT44(uVar25,uVar24);
        local_b8 = uVar19;
        uStack_b0 = uVar2;
        local_a8 = uVar22;
        uStack_a4 = uVar23;
        uStack_a0 = uVar24;
        uStack_9c = uVar25;
        uVar4 = FUN_140065230(param_1,&local_b8,&local_a8,local_res18,&local_d8,0,puVar11,
                              in_stack_fffffffffffffeb0,in_stack_fffffffffffffeb8,&local_124,10);
        uVar6 = (ulonglong)uVar4;
        puVar12 = puVar11;
        if ((uVar4 != 0) && (local_128 = 1, param_7 != (longlong *)0x0)) {
          if (*param_7 == 0) {
            lVar5 = param_7[2];
            if (lVar5 == 0) {
              uVar15 = 1;
              if (1 < uVar17) {
                do {
                  uVar15 = uVar15 * 2;
                } while (uVar15 < uVar17);
              }
              lVar5 = param_5[4];
              pvVar18 = (LPVOID)param_5[5];
              uVar17 = param_5[3];
              uVar16 = (ulonglong)((int)lVar5 + (int)pvVar18 & 7);
              lVar13 = 8 - uVar16;
              if (uVar16 == 0) {
                lVar13 = 0;
              }
              lVar20 = lVar13 + uVar15 * 4;
              if (uVar17 < (ulonglong)(lVar5 + lVar20)) {
                if ((ulonglong)param_5[2] < (ulonglong)(lVar5 + lVar20)) {
                  FUN_1401b5c50(param_5,lVar20,8);
                  pvVar18 = (LPVOID)param_5[5];
                  lVar5 = param_5[4];
                  uVar17 = param_5[3];
                  uVar16 = (ulonglong)((int)lVar5 + (int)pvVar18 & 7);
                  lVar13 = 8 - uVar16;
                  if (uVar16 == 0) {
                    lVar13 = 0;
                  }
                  lVar20 = lVar13 + uVar15 * 4;
                }
                lVar8 = 0;
                lVar7 = *param_5;
                if ((lVar7 != 0) && (uVar16 = lVar7 - 1U & lVar5 + lVar20, uVar16 != 0)) {
                  lVar8 = lVar7 - uVar16;
                }
                uVar16 = lVar8 + lVar5 + lVar20;
                if (uVar17 < uVar16) {
                  param_5[3] = uVar16;
                  VirtualAlloc(pvVar18,uVar16,0x1000,4);
                }
              }
              lVar5 = param_5[4] + lVar13 + param_5[5];
              param_5[4] = param_5[4] + lVar20;
              param_5[6] = lVar5;
              *param_7 = 0;
              puVar11 = local_108;
              uVar17 = local_e0;
LAB_1401237d1:
              param_7[1] = uVar15;
            }
            else {
              uVar30 = param_7[1];
              if (uVar30 < uVar17) {
                uVar15 = uVar17;
                if (uVar17 < uVar30 * 2) {
                  uVar15 = uVar30 * 2;
                }
                in_stack_fffffffffffffeb8 = "_ArrayGrow";
                in_stack_fffffffffffffeb0 =
                     CONCAT44((int)((ulonglong)in_stack_fffffffffffffeb0 >> 0x20),0xa4b);
                lVar5 = FUN_1401b5ae0(param_5,lVar5,uVar30 * 4,uVar15 * 4,8,
                                      uVar16 & 0xffffffff00000000,
                                      "C:\\Users\\Vjekoslav\\Projects\\Voidstar\\File Pilot\\Source\\Program\\Voidstar.h"
                                      ,in_stack_fffffffffffffeb0,"_ArrayGrow");
                goto LAB_1401237d1;
              }
            }
            param_7[2] = lVar5;
          }
          do {
            FUN_14012c1d0(param_5,param_7,*puVar11);
            puVar11 = puVar11 + 1;
            uVar6 = uVar6 - 1;
            puVar12 = local_108;
          } while (uVar6 != 0);
        }
        fVar21 = (float)local_res18[0] * 0.75;
        if (fVar21 < 0.0) {
          fVar21 = fVar21 - 0.5;
        }
        else {
          fVar21 = fVar21 + 0.5;
        }
        local_11c = local_11c + 1;
        uVar16 = (ulonglong)local_11c;
        uVar6 = (ulonglong)uVar10;
        iVar9 = local_120 + (int)fVar21;
        lVar5 = local_118._0_8_;
        local_res10 = uVar10;
        local_120 = iVar9;
      } while (uVar16 < (ulonglong)local_f8._0_8_);
      uVar6 = (ulonglong)local_128;
    }
    *param_8 = uVar10;
    *param_9 = iVar9;
  }
  else {
    local_d8 = *param_4;
    uStack_d0 = param_4[1];
    local_c8 = param_4[2];
    local_118._4_4_ = uVar23;
    local_118._0_4_ = uVar22;
    local_118._8_4_ = uVar24;
    local_118._12_4_ = uVar25;
    local_f8._4_4_ = uVar27;
    local_f8._0_4_ = uVar26;
    local_f8._8_4_ = uVar28;
    local_f8._12_4_ = uVar29;
    uVar6 = FUN_140124360(param_1,local_f8,local_118,&local_d8,param_5,param_6,param_7,param_8,
                          param_9);
  }
  return uVar6;
}

