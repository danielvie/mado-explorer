
void FUN_14005b8e0(longlong param_1,longlong *param_2)

{
  undefined4 uVar1;
  longlong lVar2;
  longlong *plVar3;
  undefined1 auVar4 [16];
  undefined8 uVar5;
  undefined8 uVar6;
  undefined8 uVar7;
  bool bVar8;
  int iVar9;
  uint uVar10;
  BOOL BVar11;
  int iVar12;
  int iVar13;
  undefined8 *puVar14;
  undefined8 uVar15;
  longlong lVar16;
  longlong lVar17;
  ulonglong uVar18;
  longlong *plVar19;
  longlong *plVar20;
  undefined8 uVar21;
  longlong lVar22;
  longlong *plVar23;
  uint uVar24;
  longlong lVar25;
  ulonglong uVar26;
  int iVar27;
  undefined8 *puVar28;
  undefined8 *puVar29;
  undefined8 *puVar30;
  ulonglong *puVar31;
  longlong *plVar32;
  ulonglong uVar33;
  ulonglong uVar34;
  bool bVar35;
  uint local_res20;
  ulonglong local_4b8;
  longlong local_4a8;
  longlong local_4a0;
  undefined4 *local_490;
  longlong local_488;
  longlong local_480;
  undefined8 uStack_478;
  undefined4 local_448;
  undefined4 uStack_444;
  undefined4 uStack_440;
  undefined4 uStack_43c;
  ulonglong local_438;
  undefined8 uStack_430;
  longlong local_428;
  undefined8 *local_420;
  undefined8 uStack_418;
  longlong local_410;
  undefined4 local_3f8;
  undefined4 uStack_3f4;
  undefined8 uStack_3f0;
  undefined8 uStack_3e8;
  LARGE_INTEGER local_3e0;
  LARGE_INTEGER local_3d8;
  LARGE_INTEGER local_3d0;
  LARGE_INTEGER local_3c8;
  LARGE_INTEGER local_3c0;
  LARGE_INTEGER local_3b8;
  longlong local_3b0;
  undefined8 local_3a8;
  undefined8 local_3a0;
  longlong local_398;
  longlong lStack_390;
  undefined8 local_388;
  undefined8 uStack_380;
  undefined8 local_378;
  undefined8 uStack_370;
  undefined8 local_368;
  undefined8 uStack_360;
  undefined8 local_358;
  undefined8 local_348;
  undefined8 uStack_340;
  undefined8 local_338;
  undefined8 uStack_330;
  undefined8 uStack_328;
  longlong local_318;
  longlong lStack_310;
  longlong local_308;
  longlong lStack_300;
  longlong local_2f8;
  longlong lStack_2f0;
  longlong local_2e8;
  longlong local_2d8;
  longlong lStack_2d0;
  longlong local_2c8;
  longlong lStack_2c0;
  undefined4 local_2b8;
  undefined4 uStack_2b4;
  undefined4 uStack_2b0;
  undefined4 uStack_2ac;
  longlong local_2a8;
  longlong local_298;
  longlong lStack_290;
  undefined8 local_288;
  undefined8 uStack_280;
  undefined8 local_278;
  undefined8 uStack_270;
  undefined8 local_268;
  undefined8 uStack_260;
  undefined4 local_258;
  undefined4 uStack_254;
  undefined4 uStack_250;
  undefined4 uStack_24c;
  undefined8 local_248;
  undefined8 local_238;
  undefined8 uStack_230;
  undefined8 local_228;
  undefined8 uStack_220;
  undefined8 local_218;
  undefined8 local_208;
  undefined8 uStack_200;
  undefined8 local_1f8;
  undefined8 uStack_1f0;
  undefined8 local_1e8;
  undefined8 local_1d8;
  undefined8 uStack_1d0;
  undefined4 local_1c8;
  undefined4 uStack_1c4;
  undefined4 uStack_1c0;
  undefined4 uStack_1bc;
  undefined8 local_1b8;
  undefined8 local_1a8;
  undefined8 uStack_1a0;
  undefined8 local_198;
  undefined8 uStack_190;
  undefined4 local_188;
  undefined4 uStack_184;
  undefined8 local_178;
  undefined8 uStack_170;
  undefined8 local_168;
  undefined8 uStack_160;
  undefined4 local_158;
  undefined4 uStack_154;
  undefined8 local_148;
  undefined8 uStack_140;
  undefined4 local_138;
  undefined4 uStack_134;
  undefined4 uStack_130;
  undefined4 uStack_12c;
  undefined8 local_128;
  undefined8 uStack_120;
  undefined8 local_118;
  undefined8 uStack_110;
  undefined4 local_108;
  undefined4 uStack_104;
  undefined4 uStack_100;
  undefined4 uStack_fc;
  longlong local_f8;
  longlong lStack_f0;
  undefined1 local_e8 [16];
  undefined8 local_d8;
  undefined8 uStack_d0;
  undefined1 local_c8 [24];
  undefined1 local_b0 [48];
  undefined1 local_80 [64];
  
  lVar22 = *param_2;
  lVar2 = param_2[4];
  uVar18 = 0;
  LOCK();
  bVar35 = *(longlong *)(lVar2 + 0xc0) == 0;
  if (bVar35) {
    *(longlong *)(lVar2 + 0xc0) = 0;
  }
  UNLOCK();
  if (bVar35) {
    LOCK();
    bVar35 = *(longlong *)(param_1 + 0x4c0) == 0;
    if (bVar35) {
      *(longlong *)(param_1 + 0x4c0) = 0;
    }
    UNLOCK();
    iVar12 = 0;
    iVar9 = FUN_14005b250(param_1,lVar2);
    if ((bVar35) || (iVar9 == 0)) {
      if ((*(int *)(lVar22 + 0x44) == 0) ||
         (uVar33 = 1, *(int *)(lVar22 + 0x44) != *(int *)(lVar22 + 0x40))) {
        uVar33 = uVar18;
      }
      uVar10 = *(uint *)(lVar22 + 0x68) >> 1 & 1;
      iVar27 = (int)uVar33;
      puVar14 = (undefined8 *)FUN_140068010(local_c8,param_1);
      local_4b8 = 0;
      bVar35 = false;
      bVar8 = false;
      puVar30 = (undefined8 *)*puVar14;
      uStack_418 = puVar14[1];
      local_res20 = 0;
      local_410 = puVar14[2];
      local_420 = puVar30;
      local_3b0 = local_410;
      local_288 = puVar30;
      uStack_280 = uStack_418;
      if (uVar10 == 0) {
        uVar26 = uVar18;
        uVar33 = uVar18;
        uVar34 = uVar18;
        if (*(int *)(lVar22 + 0xe8) != 0) {
          do {
            puVar31 = (ulonglong *)(uVar18 * 0x140 + *(longlong *)(lVar22 + 0xf8));
            uVar24 = (uint)uVar18;
            if (puVar31[0x10] != 0) {
              while( true ) {
                LOCK();
                uVar18 = *puVar31;
                if (uVar18 == 0) {
                  *puVar31 = 0;
                  uVar18 = 0;
                }
                UNLOCK();
                uVar24 = local_res20;
                if (uVar18 <= puVar31[8]) {
                  LOCK();
                  uVar18 = *puVar31;
                  if (uVar18 == 0) {
                    *puVar31 = 0;
                    uVar18 = 0;
                  }
                  UNLOCK();
                  if ((puVar31[8] != uVar18) || (iVar27 == 0)) goto LAB_14005bfb7;
                }
                if (2000 < uVar26) break;
                local_480 = 0;
                BVar11 = QueryPerformanceCounter(&local_3e0);
                if (BVar11 != 0) {
                  local_480 = ((local_3e0.QuadPart - DAT_1401e8ba0) * 1000000) / DAT_1401e8b98;
                }
                uVar18 = puVar31[0x11];
                if (*(longlong *)(uVar18 + 0x78) + *(longlong *)(uVar18 + 0x70) != 0) {
                  LOCK();
                  if (DAT_1401e8bb8 == 0) {
                    DAT_1401e8bb8 = 0;
                  }
                  uVar24 = DAT_1401e8bb8;
                  UNLOCK();
                  if (*(uint *)(lVar2 + 0x674) < DAT_1401e8bb8) {
                    *(undefined4 *)(lVar2 + 0x678) = 0;
                  }
                  *(uint *)(lVar2 + 0x678) = *(uint *)(lVar2 + 0x678) | 1;
                  *(uint *)(lVar2 + 0x674) = uVar24;
                  if (*(longlong *)(uVar18 + 0x80) == 0) {
                    *(undefined8 *)(uVar18 + 0x80) = *(undefined8 *)(uVar18 + 0x60);
                    uVar15 = FUN_1401b5dc0(lVar2 + 0x208,lVar2 + 0x248,*(undefined8 *)(lVar2 + 600),
                                           0x18,*(longlong *)(uVar18 + 0x78) +
                                                *(longlong *)(uVar18 + 0x70),0);
                    *(undefined8 *)(lVar2 + 600) = uVar15;
                  }
                  if (*(char *)(uVar18 + 0x48) != '\0') {
                    *(int *)(lVar2 + 0x1e8) = (int)*(char *)(uVar18 + 0x49);
                    *(undefined4 *)(lVar2 + 0x1ec) = *(undefined4 *)(uVar18 + 0x4c);
                    puVar14 = (undefined8 *)
                              FUN_1401b8530(local_e8,lVar2,*(undefined8 *)(uVar18 + 0x50),
                                            *(undefined8 *)(uVar18 + 0x58));
                    uVar15 = puVar14[1];
                    *(undefined8 *)(lVar2 + 0x1f0) = *puVar14;
                    *(undefined8 *)(lVar2 + 0x1f8) = uVar15;
                    *(int *)(lVar2 + 0x204) = (int)*(char *)(uVar18 + 0x4a);
                  }
                  lVar17 = *(longlong *)(uVar18 + 0x80);
                  while (lVar17 != 0) {
                    lVar17 = 0;
                    auVar4._8_8_ = 0;
                    auVar4._0_8_ = uVar34;
                    lVar25 = SUB168(ZEXT816(0x624dd2f1a9fbe77) * auVar4,8);
                    if (uVar34 == ((uVar34 - lVar25 >> 1) + lVar25 >> 8) * 500) {
                      BVar11 = QueryPerformanceCounter(&local_3d8);
                      lVar25 = lVar17;
                      if (BVar11 != 0) {
                        lVar25 = ((local_3d8.QuadPart - DAT_1401e8ba0) * 1000000) / DAT_1401e8b98;
                      }
                      local_4b8 = local_4b8 + (lVar25 - local_480);
                      local_480 = lVar25;
                      if (2000 < local_4b8) {
                        bVar35 = true;
                        *(uint *)(lVar2 + 0x660) = *(uint *)(lVar2 + 0x660) | (uint)puVar31[0x1c];
                        goto LAB_14005bf59;
                      }
                    }
                    uVar34 = uVar34 + 1;
                    *(undefined8 *)(uVar18 + 0x80) =
                         *(undefined8 *)(*(longlong *)(uVar18 + 0x80) + 0x68);
                    uVar15 = FUN_140052ce0(lVar2);
                    if (iVar9 != 0) {
                      lVar25 = FUN_1400574e0(lVar2,uVar15);
                      local_3f8 = *(undefined4 *)(lVar2 + 0x1e8);
                      uVar5 = *(undefined8 *)(lVar2 + 0x1f0);
                      uVar6 = *(undefined8 *)(lVar2 + 0x1f8);
                      uVar7 = *(undefined8 *)(lVar25 + 0x10);
                      uVar21 = *(undefined8 *)(lVar25 + 0x18);
                      uStack_3f4 = *(undefined4 *)(lVar2 + 0x1ec);
                      uStack_3f0._0_4_ = (undefined4)uVar5;
                      uStack_3f0._4_4_ = (undefined4)((ulonglong)uVar5 >> 0x20);
                      uStack_250 = (undefined4)uStack_3f0;
                      uStack_24c = uStack_3f0._4_4_;
                      uStack_3f0 = uVar5;
                      uStack_3e8 = uVar6;
                      local_268 = uVar7;
                      uStack_260 = uVar21;
                      local_258 = local_3f8;
                      uStack_254 = uStack_3f4;
                      local_248 = uVar6;
                      lVar25 = FUN_140128850(param_1,&local_268);
                      if ((lVar25 != 0) && ((*(byte *)(lVar25 + 8) & 0xf) != 0)) {
                        local_490 = &DAT_1401e75b8;
                        local_488 = 4;
                        do {
                          lVar17 = local_410;
                          puVar14 = (undefined8 *)0x0;
                          uVar1 = *local_490;
                          if (puVar30 != (undefined8 *)0x0) {
                            do {
                              local_228 = CONCAT44(uStack_3f4,local_3f8);
                              uStack_220 = uStack_3f0;
                              local_238 = uVar7;
                              uStack_230 = uVar21;
                              local_218 = uVar6;
                              lVar16 = FUN_140067e70(*(undefined8 *)(lVar17 + (longlong)puVar14 * 8)
                                                     ,&local_238,uVar1);
                              if (lVar16 != 0) {
                                FUN_14006a4c0(param_1,*(undefined8 *)
                                                       (lVar17 + (longlong)puVar14 * 8),lVar16,
                                              lVar25);
                              }
                              puVar14 = (undefined8 *)(ulonglong)((int)puVar14 + 1);
                            } while (puVar14 < puVar30);
                          }
                          local_490 = local_490 + 1;
                          local_488 = local_488 + -1;
                        } while (local_488 != 0);
                        uVar33 = 1;
                        lVar17 = lVar25;
                      }
                      puVar14 = (undefined8 *)0x0;
                      if (*(longlong *)(param_1 + 0x600) == 0) {
                        local_d8 = 0;
                        uStack_d0 = 0;
                      }
                      else {
                        local_278 = *(undefined8 *)(lVar2 + 0x168);
                        uStack_270 = *(undefined8 *)(lVar2 + 0x170);
                        lVar25 = FUN_1401b65d0(param_1 + 0x5c8,*(longlong *)(param_1 + 0x600),0x18,
                                               &local_278,1);
                        if (lVar25 != 0) {
                          *(undefined8 *)(param_1 + 0x5d8) = *(undefined8 *)(lVar25 + 8);
                          iVar12 = FUN_14005b5a0(uVar15);
                          if (iVar12 == 0) {
                            if (lVar17 == 0) {
                              local_1f8 = CONCAT44(uStack_3f4,local_3f8);
                              uStack_1f0 = uStack_3f0;
                              local_208 = uVar7;
                              uStack_200 = uVar21;
                              local_1e8 = uVar6;
                              lVar17 = FUN_140128670(param_1,&local_208);
                              *(byte *)(lVar17 + 8) = *(byte *)(lVar17 + 8) | 8;
                            }
                            else if ((*(byte *)(lVar17 + 8) & 8) != 0) goto LAB_14005bee3;
                            lVar25 = local_410;
                            *(byte *)(lVar17 + 8) = *(byte *)(lVar17 + 8) | 8;
                            if (puVar30 != (undefined8 *)0x0) {
                              do {
                                FUN_140067b50(param_1,*(undefined8 *)
                                                       (lVar25 + (longlong)puVar14 * 8),lVar17,8);
                                puVar14 = (undefined8 *)(ulonglong)((int)puVar14 + 1);
                              } while (puVar14 < puVar30);
                            }
                            uVar33 = 1;
                          }
                        }
                      }
                    }
LAB_14005bee3:
                    lVar17 = *(longlong *)(uVar18 + 0x80);
                  }
                }
                *(uint *)(lVar2 + 0x660) = *(uint *)(lVar2 + 0x660) | (uint)puVar31[0x1c];
                if (!bVar35) {
                  puVar31[0x11] = *(ulonglong *)(uVar18 + 0x88);
                  FUN_140001a10(uVar18);
                  puVar31[8] = puVar31[8] + 1;
                }
LAB_14005bf59:
                lVar17 = 0;
                BVar11 = QueryPerformanceCounter(&local_3d0);
                if (BVar11 != 0) {
                  lVar17 = ((local_3d0.QuadPart - DAT_1401e8ba0) * 1000000) / DAT_1401e8b98;
                }
                uVar26 = local_4b8 + (lVar17 - local_480);
                local_4b8 = uVar26;
              }
              bVar35 = true;
            }
LAB_14005bfb7:
            iVar12 = (int)uVar33;
            local_res20 = uVar24 + 1;
            uVar18 = (ulonglong)local_res20;
          } while (local_res20 < *(uint *)(lVar22 + 0xe8));
        }
      }
      else {
        uVar33 = uVar18;
        iVar13 = 0;
        if (*(int *)(lVar22 + 0xe8) != 0) {
          do {
            iVar12 = iVar13;
            puVar31 = (ulonglong *)(uVar18 * 0x140 + *(longlong *)(lVar22 + 0xf8));
            uVar24 = (uint)uVar18;
            if (puVar31[0x10] != 0) {
              while( true ) {
                LOCK();
                uVar18 = *puVar31;
                if (uVar18 == 0) {
                  *puVar31 = 0;
                  uVar18 = 0;
                }
                UNLOCK();
                uVar24 = local_res20;
                if (uVar18 <= puVar31[8]) {
                  LOCK();
                  uVar18 = *puVar31;
                  if (uVar18 == 0) {
                    *puVar31 = 0;
                    uVar18 = 0;
                  }
                  UNLOCK();
                  if ((puVar31[8] != uVar18) || (iVar27 == 0)) goto LAB_14005c5b2;
                }
                if (2000 < uVar33) break;
                local_4a8 = 0;
                BVar11 = QueryPerformanceCounter(&local_3c8);
                if (BVar11 != 0) {
                  local_4a8 = ((local_3c8.QuadPart - DAT_1401e8ba0) * 1000000) / DAT_1401e8b98;
                }
                uVar18 = puVar31[0x11];
                if (*(longlong *)(uVar18 + 0x78) + *(longlong *)(uVar18 + 0x70) != 0) {
                  puVar14 = (undefined8 *)FUN_1400c6b20(local_b0,0);
                  local_428 = 0;
                  uVar15 = *puVar14;
                  uVar5 = puVar14[1];
                  uVar6 = puVar14[2];
                  uVar7 = puVar14[3];
                  local_388 = puVar14[4];
                  uStack_380 = puVar14[5];
                  local_438 = 0;
                  uStack_430 = 0;
                  lVar17 = FUN_1401b5dc0(uVar15,&local_438,0,8,
                                         *(longlong *)(uVar18 + 0x78) + *(longlong *)(uVar18 + 0x70)
                                         ,0);
                  plVar23 = *(longlong **)(uVar18 + 0x60);
                  local_428 = lVar17;
                  uVar33 = local_438;
joined_r0x00014005c0f2:
                  local_438 = uVar33;
                  if (plVar23 != (longlong *)0x0) {
                    plVar32 = (longlong *)0x0;
                    plVar3 = (longlong *)plVar23[0xd];
                    plVar20 = plVar23;
                    if ((*(byte *)(plVar23 + 4) & 0x40) != 0) {
                      plVar20 = (longlong *)plVar23[0xe];
                      local_2e8 = 0;
                      local_318 = 0;
                      lStack_310 = 0;
                      local_308 = 0;
                      lStack_300 = 0;
                      local_2f8 = 0;
                      lStack_2f0 = 0;
                      plVar19 = plVar32;
                      if (plVar20 != (longlong *)0x0) {
                        do {
                          if (plVar19 != (longlong *)0x0) goto LAB_14005c15a;
                          if ((int)plVar20[1] == 2) {
                            plVar19 = plVar20;
                          }
                          plVar20 = (longlong *)*plVar20;
                        } while (plVar20 != (longlong *)0x0);
                        if (plVar19 != (longlong *)0x0) {
LAB_14005c15a:
                          local_318 = *plVar19;
                          lStack_310 = plVar19[1];
                          local_308 = plVar19[2];
                          lStack_300 = plVar19[3];
                          local_2f8 = plVar19[4];
                          lStack_2f0 = plVar19[5];
                          local_2e8 = plVar19[6];
                        }
                      }
                      plVar20 = &local_2f8;
                    }
                    local_2c8 = plVar23[2];
                    lStack_2c0 = plVar23[3];
                    lVar25 = FUN_140060e50(lVar2,&local_2c8);
                    plVar19 = plVar32;
                    if (lVar25 != 0) {
                      local_298 = *plVar20;
                      lStack_290 = plVar20[1];
                      plVar19 = (longlong *)FUN_140060d10(lVar25,&local_298);
                    }
                    if (plVar19 != (longlong *)0x0) {
                      local_2d8 = *plVar23;
                      lStack_2d0 = plVar23[1];
                      local_3a8 = 0;
                      local_398 = *plVar19;
                      lStack_390 = plVar19[1];
                      local_3a0 = 0xffffffffffffffff;
                      iVar13 = FUN_1401b87a0(&local_398,&local_2d8,&local_3a8);
                      if (iVar13 == 0) {
                        plVar19[0xb] = 0;
                        plVar19[0xc] = 0;
                      }
                      local_358 = 0;
                      local_480 = 0;
                      uStack_478 = 0;
                      local_378 = 0;
                      uStack_370 = 0;
                      local_368 = 0;
                      uStack_360 = 0;
                      if ((iVar13 == 0) && (iVar9 != 0)) {
                        lVar25 = FUN_1400574e0(lVar2,plVar19);
                        uStack_3f0 = *(undefined8 *)(lVar2 + 0x1f0);
                        uStack_3e8 = *(undefined8 *)(lVar2 + 0x1f8);
                        local_480 = *(undefined8 *)(lVar25 + 0x10);
                        uStack_478 = *(undefined8 *)(lVar25 + 0x18);
                        local_3f8 = *(undefined4 *)(lVar2 + 0x1e8);
                        uStack_3f4 = *(undefined4 *)(lVar2 + 0x1ec);
                        local_368 = *(undefined8 *)(lVar2 + 0x1e8);
                        local_378 = local_480;
                        uStack_370 = uStack_478;
                        uStack_360 = uStack_3f0;
                        local_358 = uStack_3e8;
                        FUN_140060110(lVar2,plVar19);
                      }
                      else {
                        FUN_140060110(lVar2,plVar19);
                        plVar23 = plVar3;
                        uVar33 = local_438;
                        if ((iVar13 != 0) || (iVar9 == 0)) goto joined_r0x00014005c0f2;
                      }
                      local_1d8 = local_480;
                      uStack_1d0 = uStack_478;
                      local_1b8 = local_358;
                      local_1c8 = (undefined4)local_368;
                      uStack_1c4 = local_368._4_4_;
                      uStack_1c0 = (undefined4)uStack_360;
                      uStack_1bc = uStack_360._4_4_;
                      plVar20 = (longlong *)FUN_140128850(param_1,&local_1d8);
                      if ((plVar20 == (longlong *)0x0) || ((*(byte *)(plVar20 + 1) & 0xf) == 0)) {
                        plVar20 = plVar32;
                      }
                      plVar23 = plVar3;
                      uVar33 = local_438;
                      if (plVar20 != (longlong *)0x0) {
                        lVar17 = FUN_1400574e0(lVar2,plVar19);
                        uStack_3f0 = *(undefined8 *)(lVar2 + 0x1f0);
                        local_188 = *(undefined4 *)(lVar2 + 0x1f8);
                        uStack_184 = *(undefined4 *)(lVar2 + 0x1fc);
                        uStack_3e8 = *(undefined8 *)(lVar2 + 0x1f8);
                        local_1a8 = *(undefined8 *)(lVar17 + 0x10);
                        uStack_1a0 = *(undefined8 *)(lVar17 + 0x18);
                        local_3f8 = *(undefined4 *)(lVar2 + 0x1e8);
                        uStack_3f4 = *(undefined4 *)(lVar2 + 0x1ec);
                        local_198 = *(undefined8 *)(lVar2 + 0x1e8);
                        uStack_190 = uStack_3f0;
                        FUN_1401281d0(param_1,plVar20);
                        local_4a0 = 4;
                        do {
                          lVar17 = local_410;
                          puVar14 = (undefined8 *)0x0;
                          if (puVar30 != (undefined8 *)0x0) {
                            do {
                              puVar28 = (undefined8 *)(lVar17 + (longlong)puVar14 * 8);
                              local_348 = local_480;
                              uStack_340 = uStack_478;
                              local_338 = local_368;
                              uStack_330 = uStack_360;
                              uStack_328 = local_358;
                              lVar25 = FUN_140067e70(*puVar28,&local_348);
                              if (lVar25 != 0) {
                                FUN_14006a4c0(param_1,*puVar28);
                              }
                              puVar14 = (undefined8 *)(ulonglong)((int)puVar14 + 1);
                            } while (puVar14 < puVar30);
                          }
                          local_4a0 = local_4a0 + -1;
                        } while (local_4a0 != 0);
                        iVar12 = 1;
                        bVar8 = true;
                        lVar17 = local_428;
                        uVar33 = local_438;
                      }
                      goto joined_r0x00014005c0f2;
                    }
                    lVar17 = FUN_1401b5dc0(uVar15,&local_438,lVar17,8,1,0);
                    *(longlong **)(lVar17 + local_438 * 8) = plVar23;
                    uVar33 = local_438 + 1;
                    plVar23 = plVar3;
                    local_428 = lVar17;
                    goto joined_r0x00014005c0f2;
                  }
                  uVar34 = 0;
                  if (uVar33 != 0) {
                    do {
                      uVar21 = FUN_140052ce0(lVar2,*(undefined8 *)(lVar17 + uVar34 * 8));
                      FUN_14005f660(lVar2,uVar21);
                      uVar34 = uVar34 + 1;
                      puVar30 = local_420;
                    } while (uVar34 < uVar33);
                  }
                  local_448 = (undefined4)uVar6;
                  uStack_444 = (undefined4)((ulonglong)uVar6 >> 0x20);
                  uStack_440 = (undefined4)uVar7;
                  uStack_43c = (undefined4)((ulonglong)uVar7 >> 0x20);
                  local_138 = local_448;
                  uStack_134 = uStack_444;
                  uStack_130 = uStack_440;
                  uStack_12c = uStack_43c;
                  local_128 = local_388;
                  uStack_120 = uStack_380;
                  local_148 = uVar15;
                  uStack_140 = uVar5;
                  FUN_140001a90(&local_148);
                }
                lVar17 = 0;
                *(uint *)(lVar2 + 0x660) = *(uint *)(lVar2 + 0x660) | (uint)puVar31[0x1c];
                if (!bVar35) {
                  uVar33 = *(ulonglong *)(uVar18 + 0x88);
                  puVar31[8] = puVar31[8] + 1;
                  puVar31[0x11] = uVar33;
                  FUN_140001a10(uVar18);
                }
                BVar11 = QueryPerformanceCounter(&local_3c0);
                if (BVar11 != 0) {
                  lVar17 = ((local_3c0.QuadPart - DAT_1401e8ba0) * 1000000) / DAT_1401e8b98;
                }
                uVar33 = local_4b8 + (lVar17 - local_4a8);
                local_4b8 = uVar33;
              }
              bVar35 = true;
            }
LAB_14005c5b2:
            local_res20 = uVar24 + 1;
            uVar18 = (ulonglong)local_res20;
            iVar13 = iVar12;
          } while (local_res20 < *(uint *)(lVar22 + 0xe8));
        }
      }
      puVar28 = (undefined8 *)0x0;
      BVar11 = QueryPerformanceCounter(&local_3b8);
      puVar14 = puVar28;
      if (BVar11 != 0) {
        puVar14 = (undefined8 *)(((local_3b8.QuadPart - DAT_1401e8ba0) * 1000000) / DAT_1401e8b98);
      }
      *(longlong *)(lVar2 + 0x290) = (longlong)puVar14 - param_2[5];
      if ((iVar27 != 0) && (!bVar35)) {
        if (uVar10 != 0) {
          lVar22 = FUN_14005df20(lVar2);
          if (lVar22 != 0) {
            LOCK();
            if (DAT_1401e8bb8 == 0) {
              DAT_1401e8bb8 = 0;
            }
            uVar10 = DAT_1401e8bb8;
            UNLOCK();
            if (*(uint *)(lVar2 + 0x674) < DAT_1401e8bb8) {
              *(undefined4 *)(lVar2 + 0x678) = 0;
            }
            *(uint *)(lVar2 + 0x678) = *(uint *)(lVar2 + 0x678) | 4;
            *(uint *)(lVar2 + 0x674) = uVar10;
          }
          if (*(char *)(lVar2 + 0x690) != '\0') {
            LOCK();
            if (DAT_1401e8bb8 == 0) {
              DAT_1401e8bb8 = 0;
            }
            uVar10 = DAT_1401e8bb8;
            UNLOCK();
            if (*(uint *)(lVar2 + 0x674) < DAT_1401e8bb8) {
              *(undefined4 *)(lVar2 + 0x678) = 0;
            }
            *(uint *)(lVar2 + 0x678) = *(uint *)(lVar2 + 0x678) | 0x10;
            *(uint *)(lVar2 + 0x674) = uVar10;
            *(undefined1 *)(lVar2 + 0x690) = 0;
          }
        }
        if (bVar8) {
          *(uint *)(param_1 + 0x158) = *(uint *)(param_1 + 0x158) | 0x2000;
          FUN_1401557d0(param_1);
        }
        lVar22 = local_410;
        if ((iVar12 != 0) && (puVar30 != (undefined8 *)0x0)) {
          do {
            FUN_140003ab0(param_1,*(undefined8 *)(lVar22 + (longlong)puVar28 * 8),0x20);
            puVar28 = (undefined8 *)(ulonglong)((int)puVar28 + 1);
          } while (puVar28 < puVar30);
        }
        FUN_140156ee0(lVar2 + 0x640,param_2);
        if (*(longlong *)(lVar2 + 0x170) == 0) {
          puVar14 = (undefined8 *)FUN_1400c6b20(local_80,0,0);
          uVar15 = *puVar14;
          uVar5 = puVar14[1];
          local_2b8 = (undefined4)local_288;
          uStack_2b4 = local_288._4_4_;
          uStack_2b0 = (undefined4)uStack_280;
          uStack_2ac = uStack_280._4_4_;
          local_388 = puVar14[2];
          uStack_380 = puVar14[3];
          local_398 = puVar14[4];
          lStack_390 = puVar14[5];
          local_2a8 = local_3b0;
          FUN_140127730(param_1,&local_2b8,4);
          lVar22 = *(longlong *)(lVar2 + 0x260) + *(longlong *)(lVar2 + 0x268);
          if (lVar22 != 0) {
            plVar23 = (longlong *)
                      FUN_1401b59c0(uVar15,lVar22 * 0x18,8,1,
                                    "C:\\Users\\Vjekoslav\\Projects\\Voidstar\\File Pilot\\Source\\Program\\Program.c"
                                    ,0xa908,"Dir_LoadWorkUpdate");
            lVar17 = lVar22;
            lVar25 = 0;
            do {
              *(undefined8 *)(lVar25 + (longlong)plVar23) =
                   *(undefined8 *)(lVar25 + *(longlong *)(lVar2 + 600));
              puVar28 = *(undefined8 **)(lVar25 + *(longlong *)(lVar2 + 600));
              uVar6 = puVar28[1];
              puVar14 = (undefined8 *)(lVar25 + 8 + (longlong)plVar23);
              *puVar14 = *puVar28;
              puVar14[1] = uVar6;
              lVar17 = lVar17 + -1;
              lVar25 = lVar25 + 0x18;
            } while (lVar17 != 0);
            FUN_140165a20(plVar23,lVar22,0x18,FUN_140001220);
            do {
              puVar29 = (undefined8 *)0x0;
              puVar14 = *(undefined8 **)(*plVar23 + 0x70);
              puVar28 = puVar29;
              while ((puVar14 != (undefined8 *)0x0 && (puVar28 == (undefined8 *)0x0))) {
                if (*(int *)(puVar14 + 1) == 1) {
                  puVar28 = puVar14;
                }
                puVar14 = (undefined8 *)*puVar14;
              }
              lVar17 = FUN_1400574e0(lVar2);
              uStack_330 = puVar28[10];
              local_158 = *(undefined4 *)(puVar28 + 0xb);
              uStack_154 = *(undefined4 *)((longlong)puVar28 + 0x5c);
              uStack_328 = puVar28[0xb];
              local_178 = *(undefined8 *)(lVar17 + 0x10);
              uStack_170 = *(undefined8 *)(lVar17 + 0x18);
              local_338 = puVar28[9];
              local_168 = local_338;
              uStack_160 = uStack_330;
              lVar25 = FUN_140128670(param_1,&local_178);
              lVar17 = local_410;
              *(byte *)(lVar25 + 8) = *(byte *)(lVar25 + 8) | 4;
              if (puVar30 != (undefined8 *)0x0) {
                do {
                  FUN_140067b50(param_1,*(undefined8 *)(lVar17 + (longlong)puVar29 * 8),lVar25,4);
                  puVar29 = (undefined8 *)(ulonglong)((int)puVar29 + 1);
                } while (puVar29 < puVar30);
              }
              puVar14 = (undefined8 *)0x0;
              plVar23 = plVar23 + 3;
              lVar22 = lVar22 + -1;
            } while (lVar22 != 0);
            if (puVar30 != (undefined8 *)0x0) {
              do {
                FUN_140003ab0(param_1,*(undefined8 *)(local_410 + (longlong)puVar14 * 8),0x20);
                puVar14 = (undefined8 *)(ulonglong)((int)puVar14 + 1);
              } while (puVar14 < puVar30);
            }
          }
          local_108 = (undefined4)local_388;
          uStack_104 = local_388._4_4_;
          uStack_100 = (undefined4)uStack_380;
          uStack_fc = uStack_380._4_4_;
          local_f8 = local_398;
          lStack_f0 = lStack_390;
          local_118 = uVar15;
          uStack_110 = uVar5;
          FUN_140001a90(&local_118);
        }
        FUN_140053cf0(param_1,lVar2);
      }
      if (*(longlong *)(lVar2 + 0x650) != 0) {
        *(uint *)(DAT_1401e8958 + 0x1a4) = *(uint *)(DAT_1401e8958 + 0x1a4) | 0x20;
      }
    }
  }
  return;
}

