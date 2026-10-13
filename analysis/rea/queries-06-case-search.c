
ulonglong FUN_140124360(longlong *param_1,ulonglong *param_2,ulonglong *param_3,ulonglong *param_4,
                       longlong *param_5,longlong *param_6,longlong *param_7,int *param_8,
                       int *param_9)

{
  longlong *plVar1;
  bool bVar2;
  bool bVar3;
  int iVar4;
  int iVar5;
  uint uVar6;
  int iVar7;
  uint uVar8;
  longlong lVar9;
  longlong lVar10;
  ulonglong uVar11;
  undefined4 *puVar12;
  uint *puVar13;
  longlong lVar14;
  ulonglong uVar15;
  ulonglong uVar16;
  ulonglong uVar17;
  ulonglong uVar18;
  uint uVar19;
  LPVOID pvVar20;
  uint *puVar21;
  uint uVar22;
  longlong lVar23;
  undefined4 *puVar24;
  uint uVar25;
  uint uVar26;
  longlong lVar27;
  longlong lVar28;
  ulonglong uVar29;
  float fVar30;
  int local_res10;
  uint local_res18;
  uint local_res20;
  uint *in_stack_fffffffffffffea0;
  uint local_118;
  ulonglong local_108;
  ulonglong local_100;
  uint local_f8;
  uint local_f4;
  uint local_f0;
  uint *local_e8;
  ulonglong local_e0;
  ulonglong local_d8;
  uint local_d0;
  undefined4 local_cc;
  int local_c8;
  uint local_c4;
  undefined4 *local_c0;
  longlong local_b8;
  longlong lStack_b0;
  undefined4 *local_a8;
  undefined4 *local_a0;
  ulonglong local_98;
  ulonglong uStack_90;
  ulonglong local_88;
  
  local_98 = *param_2;
  uStack_90 = param_2[1];
  uVar18 = 0;
  local_d0 = 0;
  local_d8 = 0;
  local_a8 = (undefined4 *)FUN_14013f5d0(param_1,&local_98,&local_d8);
  local_98 = *param_3;
  uStack_90 = param_3[1];
  local_e0 = 0;
  local_a0 = (undefined4 *)FUN_14013f5d0(param_1,&local_98,&local_e0);
  uVar11 = local_d8;
  pvVar20 = (LPVOID)param_1[5];
  uVar15 = param_1[3];
  local_f0 = (uint)local_e0;
  if (local_e0 < local_d8) {
    local_f0 = (uint)local_d8;
  }
  lVar27 = (ulonglong)local_f0 * 4;
  lVar23 = param_1[4];
  uVar16 = (ulonglong)((int)lVar23 + (int)pvVar20 & 3);
  uVar17 = 4 - uVar16;
  if (uVar16 == 0) {
    uVar17 = uVar18;
  }
  lVar28 = uVar17 + lVar27;
  if (uVar15 < (ulonglong)(lVar28 + lVar23)) {
    if ((ulonglong)param_1[2] < (ulonglong)(lVar28 + lVar23)) {
      FUN_1401b5c50(param_1,lVar28,4);
      pvVar20 = (LPVOID)param_1[5];
      lVar23 = param_1[4];
      uVar15 = param_1[3];
      uVar16 = (ulonglong)((int)lVar23 + (int)pvVar20 & 3);
      uVar17 = 4 - uVar16;
      if (uVar16 == 0) {
        uVar17 = uVar18;
      }
      lVar28 = uVar17 + lVar27;
    }
    lVar9 = 0;
    lVar14 = *param_1;
    if ((lVar14 != 0) && (uVar16 = lVar14 - 1U & lVar23 + lVar28, uVar16 != 0)) {
      lVar9 = lVar14 - uVar16;
    }
    uVar16 = lVar9 + lVar23 + lVar28;
    if (uVar15 < uVar16) {
      param_1[3] = uVar16;
      VirtualAlloc(pvVar20,uVar16,0x1000,4);
    }
  }
  uVar15 = 0;
  iVar5 = 0;
  local_res10 = 0;
  iVar4 = 0;
  local_c8 = 0;
  local_c4 = 0;
  local_e8 = (uint *)(uVar17 + param_1[4] + param_1[5]);
  param_1[4] = param_1[4] + lVar28;
  local_88 = *param_4;
  param_1[6] = (longlong)local_e8;
  if (local_88 != 0) {
    local_98 = param_4[2];
    uVar18 = uVar15;
    do {
      plVar1 = (longlong *)(local_98 + uVar15 * 0x10);
      local_b8 = *plVar1;
      lStack_b0 = plVar1[1];
      local_100 = uVar18;
      local_c0 = (undefined4 *)FUN_14013f5d0(param_1,&local_b8,&local_100);
      uVar18 = 0;
      uVar22 = 0;
      local_118 = 0;
      local_f8 = 0;
      local_cc = 1;
      uVar15 = uVar18;
      if ((local_100 != 0) && (uVar15 = 0, uVar11 != 0)) {
        lVar23 = param_1[4];
        pvVar20 = (LPVOID)param_1[5];
        bVar3 = false;
        uVar11 = (ulonglong)((int)lVar23 + (int)pvVar20 & 3);
        uVar15 = 4 - uVar11;
        if (uVar11 == 0) {
          uVar15 = uVar18;
        }
        uVar11 = param_1[3];
        lVar28 = uVar15 + lVar27;
        if (uVar11 < (ulonglong)(lVar28 + lVar23)) {
          if ((ulonglong)param_1[2] < (ulonglong)(lVar28 + lVar23)) {
            FUN_1401b5c50(param_1,lVar28,4);
            pvVar20 = (LPVOID)param_1[5];
            lVar23 = param_1[4];
            uVar11 = param_1[3];
            uVar17 = (ulonglong)((int)lVar23 + (int)pvVar20 & 3);
            uVar15 = 4 - uVar17;
            if (uVar17 == 0) {
              uVar15 = uVar18;
            }
            lVar28 = uVar15 + lVar27;
          }
          lVar9 = 0;
          lVar14 = *param_1;
          if ((lVar14 != 0) && (uVar17 = lVar14 - 1U & lVar23 + lVar28, uVar17 != 0)) {
            lVar9 = lVar14 - uVar17;
          }
          uVar17 = lVar9 + lVar23 + lVar28;
          if (uVar11 < uVar17) {
            param_1[3] = uVar17;
            VirtualAlloc(pvVar20,uVar17,0x1000,4);
          }
        }
        lVar23 = param_1[4] + uVar15 + param_1[5];
        local_b8 = lVar23;
        FUN_1401bace0(lVar23,0,lVar27);
        param_1[4] = param_1[4] + lVar28;
        uVar17 = 0;
        param_1[6] = lVar23;
        local_res18 = 0;
        local_108 = 0;
        puVar12 = local_a8;
        uVar15 = uVar18;
        uVar11 = local_d8;
        puVar24 = local_c0;
        if (local_100 != 0) {
          do {
            uVar22 = (uint)uVar15;
            lVar23 = local_b8;
            if (uVar11 <= uVar17) break;
            iVar4 = FUN_140169dc0(*puVar24);
            iVar5 = FUN_140169dc0(*puVar12);
            if (iVar4 == iVar5) {
              uVar15 = uVar18;
              if (local_f0 <= uVar22) goto LAB_140124d0c;
              lVar23 = param_1[4];
              pvVar20 = (LPVOID)param_1[5];
              uVar15 = param_1[3];
              uVar16 = (ulonglong)((int)lVar23 + (int)pvVar20 & 3);
              uVar11 = 4 - uVar16;
              if (uVar16 == 0) {
                uVar11 = uVar18;
              }
              lVar28 = lVar27 + uVar11;
              if (uVar15 < (ulonglong)(lVar28 + lVar23)) {
                if ((ulonglong)param_1[2] < (ulonglong)(lVar28 + lVar23)) {
                  FUN_1401b5c50(param_1,lVar28,4);
                  pvVar20 = (LPVOID)param_1[5];
                  lVar23 = param_1[4];
                  uVar15 = param_1[3];
                  uVar16 = (ulonglong)((int)lVar23 + (int)pvVar20 & 3);
                  uVar11 = 4 - uVar16;
                  if (uVar16 == 0) {
                    uVar11 = uVar18;
                  }
                  lVar28 = lVar27 + uVar11;
                }
                lVar14 = *param_1;
                lVar9 = 0;
                if ((lVar14 != 0) && (uVar16 = lVar14 - 1U & lVar23 + lVar28, uVar16 != 0)) {
                  lVar9 = lVar14 - uVar16;
                }
                uVar16 = lVar9 + lVar23 + lVar28;
                if (uVar15 < uVar16) {
                  param_1[3] = uVar16;
                  VirtualAlloc(pvVar20,uVar16,0x1000,4);
                }
              }
              lVar23 = param_1[4] + uVar11 + param_1[5];
              FUN_1401bace0(lVar23,0,lVar27);
              param_1[4] = param_1[4] + lVar28;
              in_stack_fffffffffffffea0 = &local_f4;
              local_f4 = 0;
              param_1[6] = lVar23;
              iVar4 = FUN_140065880(param_1,puVar24,local_100 - local_108,puVar12 + 1,
                                    (local_d8 - uVar17) + -1,in_stack_fffffffffffffea0,local_a8,
                                    local_d8,local_e8,lVar23,local_f0,local_f8,&local_cc,10);
              uVar22 = local_f4;
              if (iVar4 != 0) {
                if ((!bVar3) || ((int)local_res18 < (int)local_f4)) {
                  FUN_1401bacb0(local_b8,lVar23,lVar27);
                  local_res18 = uVar22;
                }
                bVar3 = true;
              }
              local_108 = local_108 + 1;
              local_e8[local_f8] = (uint)((longlong)puVar12 - (longlong)local_a8 >> 2);
              local_f8 = local_f8 + 1;
              uVar15 = (ulonglong)local_f8;
              puVar24 = puVar24 + 1;
              uVar11 = local_d8;
            }
            uVar22 = (uint)uVar15;
            uVar17 = uVar17 + 1;
            puVar12 = puVar12 + 1;
            lVar23 = local_b8;
          } while (local_108 < local_100);
        }
        uVar19 = 0;
        if (local_108 == local_100) {
          local_118 = ((uVar22 - (int)((longlong)(uVar11 * 4) >> 2)) * 3 - *local_e8) + 100;
          uVar18 = (ulonglong)local_118;
          puVar13 = local_e8;
          if (uVar22 != 0) {
            do {
              uVar25 = (uint)uVar18;
              uVar26 = *puVar13;
              if (uVar19 != 0) {
                iVar4 = uVar26 - local_e8[uVar19 - 1];
                if (5 < iVar4) {
                  uVar25 = (5 - iVar4) * 0x32 + uVar25;
                }
                if (uVar26 == local_e8[uVar19 - 1] + 1) {
                  uVar25 = uVar25 + 200;
                }
              }
              if (uVar26 == 0) {
                local_118 = uVar25 + 0x32;
              }
              else {
                uVar8 = local_a8[uVar26];
                uVar26 = local_a8[uVar26 - 1];
                if ((uVar26 - 0x61 < 0x1a) ||
                   ((uVar26 - 0xe0 < 0x20 && ((0xff7fffffU >> (uVar26 - 0xe0 & 0x1f) & 1) != 0)))) {
LAB_140124a1c:
                  if (((((uVar8 - 0x41 < 0x1a) ||
                        ((uVar8 - 0xc0 < 0x1f && ((0x7f7fffffU >> (uVar8 - 0xc0 & 0x1f) & 1) != 0)))
                        ) || ((uVar8 - 0x100 < 0x80 && ((uVar8 & 1) == 0)))) ||
                      ((uVar6 = uVar8 & 1, uVar8 - 0x400 < 0x30 ||
                       ((uVar8 - 0x460 < 0xa0 && (uVar6 == 0)))))) ||
                     (((uVar8 - 0x500 < 0x30 && (uVar6 == 0)) ||
                      (((uVar8 - 0xa640 < 0x2e && (uVar6 == 0)) ||
                       ((uVar8 - 0xa680 < 0x1c && (uVar6 == 0)))))))) {
                    uVar25 = uVar25 + 0x1e;
                  }
                }
                else {
                  if (uVar26 - 0x100 < 0x80) {
                    uVar6 = uVar26 & 0x80000001;
                    if ((int)uVar6 < 0) {
                      uVar6 = (uVar6 - 1 | 0xfffffffe) + 1;
                    }
                    if (uVar6 == 1) goto LAB_140124a1c;
                  }
                  if (uVar26 - 0x430 < 0x30) goto LAB_140124a1c;
                  if (uVar26 - 0x460 < 0xa0) {
                    uVar6 = uVar26 & 0x80000001;
                    if ((int)uVar6 < 0) {
                      uVar6 = (uVar6 - 1 | 0xfffffffe) + 1;
                    }
                    if (uVar6 == 1) goto LAB_140124a1c;
                  }
                  if (uVar26 - 0x500 < 0x30) {
                    uVar6 = uVar26 & 0x80000001;
                    if ((int)uVar6 < 0) {
                      uVar6 = (uVar6 - 1 | 0xfffffffe) + 1;
                    }
                    if (uVar6 == 1) goto LAB_140124a1c;
                  }
                  if (uVar26 - 0xa640 < 0x2e) {
                    uVar6 = uVar26 & 0x80000001;
                    if ((int)uVar6 < 0) {
                      uVar6 = (uVar6 - 1 | 0xfffffffe) + 1;
                    }
                    if (uVar6 == 1) goto LAB_140124a1c;
                  }
                  if (uVar26 - 0xa680 < 0x1c) {
                    uVar6 = uVar26 & 0x80000001;
                    if ((int)uVar6 < 0) {
                      uVar6 = (uVar6 - 1 | 0xfffffffe) + 1;
                    }
                    if (uVar6 == 1) goto LAB_140124a1c;
                  }
                }
                if ((uVar26 == 0x5f) || (bVar2 = false, uVar26 == 0x20)) {
                  bVar2 = true;
                }
                local_118 = uVar25 + 0x19;
                if (!bVar2) {
                  local_118 = uVar25;
                }
              }
              uVar18 = (ulonglong)local_118;
              uVar19 = uVar19 + 1;
              puVar13 = puVar13 + 1;
              lVar23 = local_b8;
            } while (uVar19 < uVar22);
          }
        }
        if ((bVar3) && ((local_108 != local_100 || ((int)uVar18 < (int)local_res18)))) {
          FUN_1401bacb0(local_e8,lVar23,lVar27);
          uVar18 = (ulonglong)local_res18;
          local_118 = local_res18;
          uVar19 = local_f8;
        }
        else {
          uVar15 = uVar18;
          uVar19 = uVar22;
          if (local_108 != local_100) goto LAB_140124d0c;
        }
        local_f8 = uVar19;
        uVar15 = uVar18;
        if ((uVar22 != 0) && (local_d0 = 1, param_6 != (longlong *)0x0)) {
          if (*param_6 == 0) {
            lVar23 = param_6[2];
            if (lVar23 == 0) {
              uVar11 = 1;
              if (1 < local_d8) {
                do {
                  uVar11 = uVar11 * 2;
                } while (uVar11 < local_d8);
              }
              lVar23 = param_5[4];
              pvVar20 = (LPVOID)param_5[5];
              uVar15 = param_5[3];
              uVar18 = (ulonglong)((int)lVar23 + (int)pvVar20 & 7);
              lVar28 = 8 - uVar18;
              if (uVar18 == 0) {
                lVar28 = 0;
              }
              lVar14 = lVar28 + uVar11 * 4;
              if (uVar15 < (ulonglong)(lVar14 + lVar23)) {
                if ((ulonglong)param_5[2] < (ulonglong)(lVar14 + lVar23)) {
                  FUN_1401b5c50(param_5,lVar14,8);
                  pvVar20 = (LPVOID)param_5[5];
                  lVar23 = param_5[4];
                  uVar15 = param_5[3];
                  uVar18 = (ulonglong)((int)lVar23 + (int)pvVar20 & 7);
                  lVar28 = 8 - uVar18;
                  if (uVar18 == 0) {
                    lVar28 = 0;
                  }
                  lVar14 = lVar28 + uVar11 * 4;
                }
                lVar9 = *param_5;
                lVar10 = 0;
                if ((lVar9 != 0) && (uVar18 = lVar9 - 1U & lVar14 + lVar23, uVar18 != 0)) {
                  lVar10 = lVar9 - uVar18;
                }
                uVar18 = lVar10 + lVar14 + lVar23;
                if (uVar15 < uVar18) {
                  param_5[3] = uVar18;
                  VirtualAlloc(pvVar20,uVar18,0x1000,4);
                }
              }
              lVar23 = param_5[5] + lVar28 + param_5[4];
              param_5[6] = lVar23;
              param_5[4] = lVar14 + param_5[4];
              *param_6 = 0;
              uVar22 = local_f8;
LAB_140124cc2:
              param_6[1] = uVar11;
            }
            else {
              uVar15 = param_6[1];
              if (uVar15 < local_d8) {
                uVar11 = local_d8;
                if (local_d8 < uVar15 * 2) {
                  uVar11 = uVar15 * 2;
                }
                in_stack_fffffffffffffea0 =
                     (uint *)((ulonglong)in_stack_fffffffffffffea0 & 0xffffffff00000000);
                lVar23 = FUN_1401b5ae0(param_5,lVar23,uVar15 * 4,uVar11 * 4,8,
                                       in_stack_fffffffffffffea0,
                                       "C:\\Users\\Vjekoslav\\Projects\\Voidstar\\File Pilot\\Source\\Program\\Voidstar.h"
                                       ,0xa4b,"_ArrayGrow");
                goto LAB_140124cc2;
              }
            }
            param_6[2] = lVar23;
          }
          uVar15 = (ulonglong)uVar22;
          puVar13 = local_e8;
          do {
            FUN_14012c1d0(param_5,param_6,*puVar13);
            puVar13 = puVar13 + 1;
            uVar15 = uVar15 - 1;
          } while (uVar15 != 0);
          uVar15 = (ulonglong)local_118;
        }
      }
LAB_140124d0c:
      uVar11 = 0;
      iVar4 = 0;
      iVar5 = local_res10 + (int)uVar15;
      local_res20 = 0;
      local_cc = 1;
      uVar18 = uVar11;
      if ((local_100 != 0) && (local_e0 != 0)) {
        lVar23 = param_1[4];
        pvVar20 = (LPVOID)param_1[5];
        local_118 = 0;
        uVar18 = (ulonglong)((int)lVar23 + (int)pvVar20 & 3);
        uVar15 = 4 - uVar18;
        if (uVar18 == 0) {
          uVar15 = uVar11;
        }
        uVar18 = param_1[3];
        lVar28 = uVar15 + lVar27;
        if (uVar18 < (ulonglong)(lVar28 + lVar23)) {
          if ((ulonglong)param_1[2] < (ulonglong)(lVar28 + lVar23)) {
            FUN_1401b5c50(param_1,lVar28,4);
            pvVar20 = (LPVOID)param_1[5];
            lVar23 = param_1[4];
            uVar18 = param_1[3];
            uVar17 = (ulonglong)((int)lVar23 + (int)pvVar20 & 3);
            uVar15 = 4 - uVar17;
            if (uVar17 == 0) {
              uVar15 = 0;
            }
            lVar28 = uVar15 + lVar27;
          }
          lVar9 = 0;
          lVar14 = *param_1;
          if ((lVar14 != 0) && (uVar17 = lVar14 - 1U & lVar23 + lVar28, uVar17 != 0)) {
            lVar9 = lVar14 - uVar17;
          }
          uVar17 = lVar9 + lVar23 + lVar28;
          if (uVar18 < uVar17) {
            param_1[3] = uVar17;
            VirtualAlloc(pvVar20,uVar17,0x1000,4);
          }
        }
        lVar23 = param_1[4] + uVar15 + param_1[5];
        local_b8 = lVar23;
        FUN_1401bace0(lVar23,0,lVar27);
        param_1[4] = param_1[4] + lVar28;
        uVar18 = 0;
        param_1[6] = lVar23;
        local_res18 = 0;
        local_108 = 0;
        puVar12 = local_a0;
        uVar15 = uVar11;
        uVar17 = local_e0;
        uVar16 = uVar11;
        uVar29 = uVar18;
        if (local_100 != 0) {
          do {
            lVar23 = local_b8;
            if (uVar17 <= uVar29) break;
            iVar4 = FUN_140169dc0(*local_c0);
            iVar7 = FUN_140169dc0(*puVar12);
            if (iVar4 == iVar7) {
              uVar22 = (uint)uVar16;
              if (local_f0 <= uVar22) goto LAB_1401254e2;
              lVar23 = param_1[4];
              pvVar20 = (LPVOID)param_1[5];
              uVar17 = (ulonglong)((int)lVar23 + (int)pvVar20 & 3);
              uVar15 = 4 - uVar17;
              if (uVar17 == 0) {
                uVar15 = uVar18;
              }
              uVar18 = param_1[3];
              lVar28 = lVar27 + uVar15;
              if (uVar18 < (ulonglong)(lVar23 + lVar28)) {
                if ((ulonglong)param_1[2] < (ulonglong)(lVar23 + lVar28)) {
                  FUN_1401b5c50(param_1,lVar28,4);
                  pvVar20 = (LPVOID)param_1[5];
                  lVar23 = param_1[4];
                  uVar18 = param_1[3];
                  uVar17 = (ulonglong)((int)lVar23 + (int)pvVar20 & 3);
                  uVar15 = 4 - uVar17;
                  if (uVar17 == 0) {
                    uVar15 = 0;
                  }
                  lVar28 = lVar27 + uVar15;
                }
                lVar14 = *param_1;
                lVar9 = 0;
                if ((lVar14 != 0) && (uVar17 = lVar14 - 1U & lVar23 + lVar28, uVar17 != 0)) {
                  lVar9 = lVar14 - uVar17;
                }
                uVar17 = lVar9 + lVar23 + lVar28;
                if (uVar18 < uVar17) {
                  param_1[3] = uVar17;
                  VirtualAlloc(pvVar20,uVar17,0x1000,4);
                }
              }
              lVar23 = param_1[4] + uVar15 + param_1[5];
              FUN_1401bace0(lVar23,0,lVar27);
              param_1[4] = param_1[4] + lVar28;
              in_stack_fffffffffffffea0 = &local_f4;
              local_f4 = 0;
              param_1[6] = lVar23;
              iVar4 = FUN_140065880(param_1,local_c0,local_100 - local_108,puVar12 + 1,
                                    (local_e0 - uVar29) + -1,in_stack_fffffffffffffea0,local_a0,
                                    local_e0,local_e8,lVar23,local_f0,uVar22,&local_cc,10);
              uVar19 = local_f4;
              if (iVar4 == 0) {
                uVar15 = (ulonglong)local_118;
              }
              else {
                if ((local_118 == 0) || ((int)local_res18 < (int)local_f4)) {
                  FUN_1401bacb0(local_b8,lVar23,lVar27);
                  local_res18 = uVar19;
                }
                uVar15 = 1;
                local_118 = 1;
              }
              local_c0 = local_c0 + 1;
              local_e8[uVar16] = (uint)((longlong)puVar12 - (longlong)local_a0 >> 2);
              local_108 = local_108 + 1;
              uVar18 = 0;
              uVar17 = local_e0;
              uVar16 = (ulonglong)(uVar22 + 1);
            }
            puVar12 = puVar12 + 1;
            lVar23 = local_b8;
            uVar29 = uVar29 + 1;
          } while (local_108 < local_100);
        }
        uVar17 = local_e0;
        puVar13 = local_e8;
        uVar19 = (uint)uVar16;
        uVar22 = (uint)uVar15;
        if (local_108 == local_100) {
          uVar18 = uVar18 & 0xffffffff;
          local_res20 = ((uVar19 - (int)((longlong)(local_e0 * 4) >> 2)) * 3 - *local_e8) + 100;
          uVar11 = (ulonglong)local_res20;
          puVar21 = local_e8;
          if (uVar19 != 0) {
            do {
              uVar26 = (uint)uVar11;
              uVar22 = *puVar21;
              iVar4 = (int)uVar18;
              if (iVar4 != 0) {
                iVar7 = uVar22 - local_e8[iVar4 - 1];
                if (5 < iVar7) {
                  uVar26 = (5 - iVar7) * 0x32 + uVar26;
                }
                if (uVar22 == local_e8[iVar4 - 1] + 1) {
                  uVar26 = uVar26 + 200;
                }
              }
              if (uVar22 == 0) {
                local_res20 = uVar26 + 0x32;
              }
              else {
                uVar25 = local_a0[uVar22];
                uVar22 = local_a0[uVar22 - 1];
                if ((uVar22 - 0x61 < 0x1a) ||
                   ((uVar22 - 0xe0 < 0x20 && ((0xff7fffffU >> (uVar22 - 0xe0 & 0x1f) & 1) != 0)))) {
LAB_1401251fb:
                  if (((((uVar25 - 0x41 < 0x1a) ||
                        ((uVar25 - 0xc0 < 0x1f && ((0x7f7fffffU >> (uVar25 - 0xc0 & 0x1f) & 1) != 0)
                         ))) || ((uVar25 - 0x100 < 0x80 && ((uVar25 & 1) == 0)))) ||
                      (((uVar8 = uVar25 & 1, uVar25 - 0x400 < 0x30 ||
                        ((uVar25 - 0x460 < 0xa0 && (uVar8 == 0)))) ||
                       ((uVar25 - 0x500 < 0x30 && (uVar8 == 0)))))) ||
                     (((uVar25 - 0xa640 < 0x2e && (uVar8 == 0)) ||
                      ((uVar25 - 0xa680 < 0x1c && (uVar8 == 0)))))) {
                    uVar26 = uVar26 + 0x1e;
                  }
                }
                else {
                  if (uVar22 - 0x100 < 0x80) {
                    uVar8 = uVar22 & 0x80000001;
                    if ((int)uVar8 < 0) {
                      uVar8 = (uVar8 - 1 | 0xfffffffe) + 1;
                    }
                    if (uVar8 == 1) goto LAB_1401251fb;
                  }
                  if (uVar22 - 0x430 < 0x30) goto LAB_1401251fb;
                  if (uVar22 - 0x460 < 0xa0) {
                    uVar8 = uVar22 & 0x80000001;
                    if ((int)uVar8 < 0) {
                      uVar8 = (uVar8 - 1 | 0xfffffffe) + 1;
                    }
                    if (uVar8 == 1) goto LAB_1401251fb;
                  }
                  if (uVar22 - 0x500 < 0x30) {
                    uVar8 = uVar22 & 0x80000001;
                    if ((int)uVar8 < 0) {
                      uVar8 = (uVar8 - 1 | 0xfffffffe) + 1;
                    }
                    if (uVar8 == 1) goto LAB_1401251fb;
                  }
                  if (uVar22 - 0xa640 < 0x2e) {
                    uVar8 = uVar22 & 0x80000001;
                    if ((int)uVar8 < 0) {
                      uVar8 = (uVar8 - 1 | 0xfffffffe) + 1;
                    }
                    if (uVar8 == 1) goto LAB_1401251fb;
                  }
                  if (uVar22 - 0xa680 < 0x1c) {
                    uVar8 = uVar22 & 0x80000001;
                    if ((int)uVar8 < 0) {
                      uVar8 = (uVar8 - 1 | 0xfffffffe) + 1;
                    }
                    if (uVar8 == 1) goto LAB_1401251fb;
                  }
                }
                if ((uVar22 == 0x5f) || (bVar3 = false, uVar22 == 0x20)) {
                  bVar3 = true;
                }
                local_res20 = uVar26 + 0x19;
                if (!bVar3) {
                  local_res20 = uVar26;
                }
              }
              uVar11 = (ulonglong)local_res20;
              uVar18 = (ulonglong)(iVar4 + 1U);
              puVar21 = puVar21 + 1;
              lVar23 = local_b8;
              uVar22 = local_118;
            } while (iVar4 + 1U < uVar19);
          }
        }
        if ((uVar22 == 0) || ((local_108 == local_100 && ((int)local_res18 <= (int)uVar11)))) {
          if (local_108 == local_100) goto LAB_140125308;
        }
        else {
          FUN_1401bacb0(local_e8,lVar23,lVar27);
          uVar11 = (ulonglong)local_res18;
          local_res20 = local_res18;
LAB_140125308:
          if ((uVar19 != 0) && (local_d0 = 1, param_7 != (longlong *)0x0)) {
            if (*param_7 == 0) {
              lVar23 = param_7[2];
              if (lVar23 == 0) {
                uVar11 = 1;
                if (1 < uVar17) {
                  do {
                    uVar11 = uVar11 * 2;
                  } while (uVar11 < uVar17);
                }
                lVar28 = param_5[4];
                pvVar20 = (LPVOID)param_5[5];
                uVar15 = param_5[3];
                uVar18 = (ulonglong)((int)lVar28 + (int)pvVar20 & 7);
                lVar23 = 8 - uVar18;
                if (uVar18 == 0) {
                  lVar23 = 0;
                }
                lVar14 = lVar23 + uVar11 * 4;
                if (uVar15 < (ulonglong)(lVar28 + lVar14)) {
                  if ((ulonglong)param_5[2] < (ulonglong)(lVar28 + lVar14)) {
                    FUN_1401b5c50(param_5,lVar14,8);
                    pvVar20 = (LPVOID)param_5[5];
                    lVar28 = param_5[4];
                    uVar15 = param_5[3];
                    uVar18 = (ulonglong)((int)lVar28 + (int)pvVar20 & 7);
                    lVar23 = 8 - uVar18;
                    if (uVar18 == 0) {
                      lVar23 = 0;
                    }
                    lVar14 = lVar23 + uVar11 * 4;
                  }
                  lVar10 = 0;
                  lVar9 = *param_5;
                  if ((lVar9 != 0) && (uVar18 = lVar9 - 1U & lVar28 + lVar14, uVar18 != 0)) {
                    lVar10 = lVar9 - uVar18;
                  }
                  uVar18 = lVar10 + lVar28 + lVar14;
                  if (uVar15 < uVar18) {
                    param_5[3] = uVar18;
                    VirtualAlloc(pvVar20,uVar18,0x1000,4);
                  }
                }
                lVar23 = param_5[5] + param_5[4] + lVar23;
                param_5[4] = param_5[4] + lVar14;
                param_5[6] = lVar23;
                *param_7 = 0;
LAB_1401254a9:
                param_7[1] = uVar11;
              }
              else {
                uVar15 = param_7[1];
                if (uVar15 < uVar17) {
                  uVar11 = uVar17;
                  if (uVar17 < uVar15 * 2) {
                    uVar11 = uVar15 * 2;
                  }
                  in_stack_fffffffffffffea0 =
                       (uint *)((ulonglong)in_stack_fffffffffffffea0 & 0xffffffff00000000);
                  lVar23 = FUN_1401b5ae0(param_5,lVar23,uVar15 * 4,uVar11 * 4,8,
                                         in_stack_fffffffffffffea0,
                                         "C:\\Users\\Vjekoslav\\Projects\\Voidstar\\File Pilot\\Source\\Program\\Voidstar.h"
                                         ,0xa4b,"_ArrayGrow");
                  goto LAB_1401254a9;
                }
              }
              param_7[2] = lVar23;
            }
            do {
              FUN_14012c1d0(param_5,param_7,*puVar13);
              puVar13 = puVar13 + 1;
              uVar16 = uVar16 - 1;
            } while (uVar16 != 0);
            uVar11 = (ulonglong)local_res20;
          }
        }
        uVar18 = 0;
LAB_1401254e2:
        iVar4 = (int)uVar11;
      }
      fVar30 = (float)iVar4 * 0.75;
      if (fVar30 < 0.0) {
        fVar30 = fVar30 - 0.5;
      }
      else {
        fVar30 = fVar30 + 0.5;
      }
      local_c4 = local_c4 + 1;
      uVar15 = (ulonglong)local_c4;
      iVar4 = local_c8 + (int)fVar30;
      uVar11 = local_d8;
      local_res10 = iVar5;
      local_c8 = iVar4;
    } while (uVar15 < local_88);
    uVar18 = (ulonglong)local_d0;
  }
  *param_8 = iVar5;
  *param_9 = iVar4;
  return uVar18;
}

