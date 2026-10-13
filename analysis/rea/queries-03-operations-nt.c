
bool FUN_1400ca3b0(undefined8 param_1,uint *param_2)

{
  undefined4 *puVar1;
  short sVar2;
  wchar_t wVar3;
  uint uVar4;
  uint uVar5;
  uint uVar6;
  uint uVar7;
  undefined8 uVar8;
  undefined8 uVar9;
  undefined8 uVar10;
  undefined8 uVar11;
  undefined8 uVar12;
  undefined8 uVar13;
  LPCWSTR pWVar14;
  uint uVar15;
  int iVar16;
  HRESULT HVar17;
  DWORD dwMessageId;
  ulonglong uVar18;
  longlong *plVar19;
  undefined8 *puVar20;
  longlong lVar21;
  ulonglong uVar22;
  ulonglong uVar23;
  ulonglong *puVar24;
  longlong lVar25;
  longlong *plVar26;
  short *psVar27;
  wchar_t *pwVar28;
  ulonglong uVar29;
  PCWSTR pWVar30;
  longlong *plVar31;
  LPCWSTR pWVar32;
  longlong *plVar33;
  PCWSTR pWVar34;
  bool bVar35;
  undefined4 uVar36;
  undefined4 uVar37;
  undefined4 uVar38;
  undefined4 uVar39;
  uint local_res20 [2];
  ulonglong in_stack_fffffffffffffc50;
  longlong *local_398;
  LPCWSTR local_390;
  ulonglong uStack_388;
  undefined8 local_380;
  undefined8 local_378;
  undefined8 uStack_370;
  longlong *local_368;
  longlong *local_360;
  PCWSTR local_358;
  ulonglong uStack_350;
  undefined8 local_348;
  PCWSTR local_338;
  ulonglong uStack_330;
  ulonglong local_328;
  LPCWSTR local_318;
  ulonglong uStack_310;
  PCWSTR local_308;
  longlong *plStack_300;
  undefined8 local_2f8;
  PCWSTR local_2e8;
  ulonglong uStack_2e0;
  longlong *local_2d8;
  LPCWSTR local_2d0;
  ulonglong uStack_2c8;
  undefined8 local_2c0;
  longlong local_2b8;
  ulonglong uStack_2b0;
  uint local_2a8;
  LPCWSTR local_298;
  ulonglong uStack_290;
  LPCWSTR local_288;
  ulonglong uStack_280;
  LPCWSTR local_278;
  ulonglong uStack_270;
  undefined8 local_268;
  uint local_260;
  longlong *local_258;
  undefined1 local_250 [24];
  uint local_238;
  uint uStack_234;
  uint uStack_230;
  uint uStack_22c;
  PCWSTR local_228;
  longlong lStack_220;
  undefined8 local_218;
  undefined8 uStack_210;
  undefined8 local_208;
  undefined8 uStack_200;
  undefined8 local_1f8;
  undefined8 uStack_1f0;
  longlong local_188;
  undefined1 local_178 [48];
  undefined1 local_148 [104];
  undefined1 local_e0 [168];
  
  plVar31 = (longlong *)0x0;
  uVar15 = 0x200;
  if (*param_2 != 3) {
    uVar15 = 0x240;
  }
  local_res20[0] = 0;
  local_360 = (longlong *)0x0;
  local_398 = (longlong *)0x0;
  uVar5 = uVar15 | 0x10000000;
  if (param_2[0x14] == 0) {
    uVar5 = uVar15;
  }
  local_2d8 = (longlong *)0x0;
  local_258 = (longlong *)0x0;
  local_260 = uVar5 | 0x10;
  if (param_2[0x13] == 0) {
    local_260 = uVar5;
  }
  plVar33 = plVar31;
  if (*param_2 - 8 < 4) {
LAB_1400ca488:
    uVar15 = (uint)plVar33;
    uVar5 = param_2[2];
    uVar6 = param_2[3];
    local_378 = *(LPCWSTR *)(param_2 + 2);
    uVar18 = *(ulonglong *)(param_2 + 4);
    uVar7 = param_2[5];
    uStack_370 = *(ulonglong *)(param_2 + 4);
    local_2f8 = 0;
    local_308 = (PCWSTR)0x0;
    plStack_300 = (longlong *)0x0;
    FUN_140187f00(&local_228,param_1,&local_378,&local_308);
    uVar29 = 1;
    FUN_1401b9a40(param_1,&local_308,1);
    local_308[(longlong)plStack_300] = L'\0';
    plVar26 = plStack_300;
    if ((longlong)plStack_300 + 1U < 2) {
      plVar26 = (longlong *)0x0;
    }
    if (plVar26 == (longlong *)0x0) {
      plVar26 = plVar31;
    }
    uVar4 = *param_2;
    local_368 = plVar26;
    if (uVar4 < 2) {
      uVar15 = SHCreateItemFromParsingName
                         (local_308,(IBindCtx *)0x0,(IID *)&DAT_1401d6d18,&local_2d8);
      if ((int)uVar15 < 0) goto LAB_1400cb84c;
      if (*(longlong *)(param_2 + 6) != 0) {
        local_2b8 = 0;
        local_328 = 0;
        uStack_2b0 = 0xffffffffffffffff;
        local_368 = (longlong *)0x0;
        local_238 = uVar5;
        uStack_234 = uVar6;
        uStack_230 = (uint)uVar18;
        uStack_22c = uVar7;
        do {
          plVar33 = local_368;
          pWVar30 = *(PCWSTR *)(local_328 + *(longlong *)(param_2 + 10));
          uVar8 = ((undefined8 *)(local_328 + *(longlong *)(param_2 + 10)))[1];
          local_338 = pWVar30;
          uStack_330 = uVar8;
          puVar24 = (ulonglong *)FUN_140067a00(&local_278,&local_338);
          local_338 = (PCWSTR)*puVar24;
          uStack_330 = puVar24[1];
          iVar16 = FUN_1401b87a0(&local_238,&local_338,&local_2b8);
          if ((iVar16 == 0) || (*param_2 != 1)) {
            local_2f8 = 0;
            local_308 = (PCWSTR)0x0;
            plStack_300 = (longlong *)0x0;
            local_338 = pWVar30;
            uStack_330 = uVar8;
            FUN_140187f00(local_250,param_1,&local_338,&local_308);
            FUN_1401b9a40(param_1,&local_308,1);
            pWVar30 = local_308;
            local_338 = (PCWSTR)0x0;
            uStack_330 = 0;
            local_308[(longlong)plStack_300] = L'\0';
            plVar26 = (longlong *)((longlong)plStack_300 + 1);
            if (plVar26 < (longlong *)0x2) {
              plStack_300 = (longlong *)0x0;
            }
            uVar18 = (ulonglong)plStack_300;
            if (plStack_300 == (longlong *)0x0) {
              uVar18 = 0;
            }
            plStack_300 = plVar26;
            if (plVar31 < *(longlong **)(param_2 + 0xc)) {
              local_358 = (PCWSTR)0x0;
              uStack_350 = 0;
              local_348 = 0;
              puVar24 = (ulonglong *)(*(longlong *)(param_2 + 0x10) + (longlong)plVar33);
              local_338 = (PCWSTR)*puVar24;
              uStack_330 = puVar24[1];
              FUN_140187f00(&local_2d0,param_1,&local_338,&local_358);
              FUN_1401b9a40(param_1,&local_358,1);
              local_358[uStack_350] = L'\0';
              uVar22 = uStack_350 + 1;
              if (uVar22 < 2) {
                uStack_350 = 0;
              }
              bVar35 = uStack_350 == 0;
              uVar29 = uStack_350;
              pWVar34 = local_358;
              uStack_350 = uVar22;
              if (bVar35) {
                uVar29 = 0;
              }
            }
            else {
              uVar29 = 0;
              pWVar34 = (PCWSTR)0x0;
            }
            if (((iVar16 == 0) || (param_2[0x12] != 0)) || (uVar29 != 0)) {
LAB_1400cb70b:
              uVar15 = SHCreateItemFromParsingName
                                 (pWVar30,(IBindCtx *)0x0,(IID *)&DAT_1401d6d18,&local_398);
              if ((int)uVar15 < 0) goto LAB_1400cb868;
              if (*param_2 == 0) {
                uVar15 = (**(code **)(*local_360 + 0x80))(local_360,local_398,local_2d8,pWVar34);
              }
              else {
                uVar15 = (**(code **)(*local_360 + 0x70))(local_360,local_398,local_2d8,0,0);
              }
              if ((int)uVar15 < 0) goto LAB_1400cb868;
              if (local_398 != (longlong *)0x0) {
                (**(code **)(*local_398 + 0x10))();
              }
              local_398 = (longlong *)0x0;
              plVar33 = local_368;
            }
            else {
              local_res20[0] = 0;
              local_298 = pWVar30;
              uStack_290 = uVar18;
              FUN_1401a0210(&local_298,local_res20);
              local_288 = L" - Copy";
              local_2e8 = (PCWSTR)CONCAT44(local_2e8._4_4_,1);
              in_stack_fffffffffffffc50 = in_stack_fffffffffffffc50 & 0xffffffff00000000;
              uStack_280 = 7;
              local_378 = pWVar30;
              uStack_370 = uVar18;
              puVar20 = (undefined8 *)
                        FUN_14019b870(&local_390,param_1,&local_378,local_res20[0] >> 1 & 1,
                                      &local_288,in_stack_fffffffffffffc50,&local_2e8);
              pWVar34 = (PCWSTR)*puVar20;
              lVar21 = puVar20[1];
              plVar33 = local_368;
              if (lVar21 != 0) {
                local_338 = pWVar34;
                uStack_330 = lVar21;
                lVar25 = FUN_1401ac340(local_148,0,&local_338);
                local_318 = (LPCWSTR)0x0;
                uStack_310 = 0xffffffffffffffff;
                local_338 = *(PCWSTR *)(lVar25 + 0x18);
                uStack_330 = *(ulonglong *)(lVar25 + 0x20);
                local_2e8 = local_338;
                uStack_2e0 = uStack_330;
                local_228 = pWVar34;
                lStack_220 = lVar21;
                iVar16 = FUN_1401b9dd0(&local_228,&local_2e8,&local_318);
                pWVar34 = local_338;
                plVar33 = local_368;
                if (iVar16 == 0) goto LAB_1400cb70b;
              }
            }
          }
          local_368 = plVar33 + 3;
          local_328 = local_328 + 0x10;
          plVar31 = (longlong *)((longlong)plVar31 + 1);
        } while (plVar31 < *(longlong **)(param_2 + 6));
      }
      if (local_2d8 != (longlong *)0x0) {
        (**(code **)(*local_2d8 + 0x10))();
      }
      local_2d8 = (longlong *)0x0;
    }
    else if (uVar4 - 2 < 2) {
      plVar33 = plVar31;
      if (*(longlong *)(param_2 + 6) != 0) {
        do {
          local_348 = 0;
          local_358 = (PCWSTR)0x0;
          uStack_350 = 0;
          local_378 = *(LPCWSTR *)((longlong)plVar33 + *(longlong *)(param_2 + 10));
          uStack_370 = ((longlong *)((longlong)plVar33 + *(longlong *)(param_2 + 10)))[1];
          FUN_140187f00(&local_278,param_1,&local_378,&local_358);
          FUN_1401b9a40(param_1,&local_358,1);
          local_358[uStack_350] = L'\0';
          uVar15 = SHCreateItemFromParsingName
                             (local_358,(IBindCtx *)0x0,(IID *)&DAT_1401d6d18,&local_398);
          if (((int)uVar15 < 0) ||
             (uVar15 = (**(code **)(*local_360 + 0x90))(local_360,local_398,0), (int)uVar15 < 0))
          goto LAB_1400cb868;
          if (local_398 != (longlong *)0x0) {
            (**(code **)(*local_398 + 0x10))();
          }
          plVar31 = (longlong *)((longlong)plVar31 + 1);
          local_398 = (longlong *)0x0;
          plVar33 = plVar33 + 2;
        } while (plVar31 < *(longlong **)(param_2 + 6));
      }
    }
    else if (uVar4 == 4) {
      uVar18 = *(ulonglong *)(param_2 + 0xc);
      plVar31 = *(longlong **)(param_2 + 0x10);
      local_2e8 = (PCWSTR)CONCAT44(local_2e8._4_4_,1);
      local_2a8 = (uint)(1 < uVar18);
      plVar26 = plVar31;
      if (1 < uVar18) {
        do {
          if (local_2a8 == 0) break;
          local_378 = (LPCWSTR)plVar26[3];
          uStack_370 = plVar26[4];
          local_318 = (LPCWSTR)0x0;
          local_298 = (LPCWSTR)*plVar31;
          uStack_290 = plVar31[1];
          uStack_310 = 0xffffffffffffffff;
          local_2a8 = FUN_1401b87a0(&local_298,&local_378,&local_318);
          uVar29 = uVar29 + 1;
          plVar26 = plVar26 + 3;
        } while (uVar29 < uVar18);
        plVar33 = (longlong *)(ulonglong)local_res20[0];
      }
      local_338 = (PCWSTR)0x0;
      uVar15 = (uint)plVar33;
      if (*(longlong *)(param_2 + 6) != 0) {
        local_368 = (longlong *)0x0;
        local_328 = 0;
        do {
          pWVar30 = local_338;
          plVar31 = local_368;
          local_268 = 0;
          local_278 = (LPCWSTR)0x0;
          uStack_270 = 0;
          local_378 = *(LPCWSTR *)(local_328 + *(longlong *)(param_2 + 10));
          uStack_370 = ((undefined8 *)(local_328 + *(longlong *)(param_2 + 10)))[1];
          FUN_140187f00(&local_228,param_1,&local_378,&local_278);
          FUN_1401b9a40(param_1,&local_278,1);
          local_318 = local_278;
          local_308 = (LPCWSTR)0x0;
          plStack_300 = (longlong *)0x0;
          local_278[uStack_270] = L'\0';
          uVar18 = uStack_270 + 1;
          if (uVar18 < 2) {
            uStack_270 = 0;
          }
          plVar26 = (longlong *)(*(longlong *)(param_2 + 0x10) + (longlong)plVar31);
          local_378 = (LPCWSTR)*plVar26;
          uStack_370 = plVar26[1];
          uVar29 = uStack_270;
          if (uStack_270 == 0) {
            uVar29 = 0;
          }
          local_348 = *(undefined8 *)(*(longlong *)(param_2 + 0x10) + 0x10 + (longlong)plVar31);
          local_2f8 = 0;
          uStack_310 = uVar29;
          uStack_270 = uVar18;
          FUN_140187f00(&local_238,param_1,&local_378,&local_308);
          FUN_1401b9a40(param_1,&local_308,1);
          pWVar32 = local_308;
          local_308[(longlong)plStack_300] = L'\0';
          plVar19 = (longlong *)((longlong)plStack_300 + 1);
          plVar26 = plStack_300;
          if (plVar19 < (longlong *)0x2) {
            plVar26 = (longlong *)0x0;
          }
          if (plVar26 == (longlong *)0x0) {
            plVar26 = (longlong *)0x0;
          }
          plStack_300 = plVar19;
          if ((int)local_348 == 0) {
            if (local_2a8 != 0) {
              local_2b8 = 0;
              uStack_2b0 = 0;
              puVar20 = (undefined8 *)FUN_1400c6b20(local_178,0,0);
              uVar8 = *puVar20;
              uVar9 = puVar20[1];
              uVar10 = puVar20[2];
              uVar11 = puVar20[3];
              uVar12 = puVar20[4];
              uVar13 = puVar20[5];
              local_288 = local_278;
              uStack_280 = uVar29;
              lVar21 = FUN_1401ac340(local_e0,uVar8);
              local_188 = *(longlong *)(lVar21 + 0x60);
              if (1 < *(ulonglong *)(lVar21 + 0x50)) {
                plVar31 = (longlong *)(local_188 + (*(ulonglong *)(lVar21 + 0x50) - 2) * 0x10);
                local_2b8 = *plVar31;
                uStack_2b0 = plVar31[1];
              }
              local_218 = uVar8;
              uStack_210 = uVar9;
              local_208 = uVar10;
              uStack_200 = uVar11;
              local_1f8 = uVar12;
              uStack_1f0 = uVar13;
              FUN_140001a90(&local_218);
              uVar18 = uStack_2b0;
              uVar36 = (undefined4)(in_stack_fffffffffffffc50 >> 0x20);
              plVar31 = local_368;
              if (uStack_2b0 < uVar29) {
                local_380 = 0;
                local_390 = (LPCWSTR)0x0;
                uStack_388 = 0;
                if (uStack_2b0 != 0) {
                  FUN_1401b9a40(param_1,&local_390,uStack_2b0);
                  uVar29 = uStack_388;
                  FUN_1401bacb0(local_390 + uStack_388,local_2b8);
                  uStack_388 = uVar29 + uVar18;
                }
                psVar27 = &DAT_1401d3f40;
                uVar18 = 0;
                do {
                  sVar2 = *psVar27;
                  psVar27 = psVar27 + 1;
                  if (sVar2 == 0) break;
                  uVar18 = uVar18 + 1;
                } while (psVar27 != (short *)0x0);
                if ((longlong)uVar18 < 0) {
                  uVar29 = (uVar18 ^ (longlong)uVar18 >> 0x3f) - ((longlong)uVar18 >> 0x3f);
                  if (uVar29 < uVar18) {
                    uVar18 = uVar18 - uVar29;
                    goto LAB_1400ca88f;
                  }
                }
                else {
LAB_1400ca88f:
                  if (uVar18 == 0) {
                    uVar18 = 0;
                  }
                  if (uVar18 != 0) {
                    FUN_1401b9a40(param_1,&local_390,uVar18);
                    uVar29 = uStack_388;
                    FUN_1401bacb0(local_390 + uStack_388,&DAT_1401d3f40,uVar18 * 2);
                    uStack_388 = uVar29 + uVar18;
                  }
                }
                if (plVar26 != (longlong *)0x0) {
                  FUN_1401b9a40(param_1,&local_390,plVar26);
                  uVar18 = uStack_388;
                  FUN_1401bacb0(local_390 + uStack_388,pWVar32,(longlong)plVar26 * 2);
                  uStack_388 = uVar18 + (longlong)plVar26;
                }
                FUN_1401b9a40(param_1,&local_390,1);
                pWVar14 = local_390;
                local_res20[0] = 0;
                local_378 = local_318;
                uStack_370 = uStack_310;
                local_390[uStack_388] = L'\0';
                uVar18 = uStack_388;
                if (uStack_388 + 1 < 2) {
                  uVar18 = 0;
                }
                uStack_388 = uStack_388 + 1;
                FUN_1401a0210(&local_378,local_res20);
                in_stack_fffffffffffffc50 = CONCAT44(uVar36,1);
                local_378 = (LPCWSTR)0x0;
                uStack_370 = 0;
                local_298 = pWVar14;
                uStack_290 = uVar18;
                if (uVar18 == 0) {
                  uStack_290 = 0;
                }
                puVar20 = (undefined8 *)
                          FUN_14019b870(local_250,param_1,&local_298,local_res20[0] >> 1 & 1,
                                        &local_378,in_stack_fffffffffffffc50,&local_2e8);
                pWVar30 = local_338;
                plVar31 = local_368;
                if (puVar20[1] != 0) {
                  local_378 = (LPCWSTR)*puVar20;
                  uStack_370 = puVar20[1];
                  lVar21 = FUN_1401ac340(local_148,0,&local_378);
                  pWVar32 = *(LPCWSTR *)(lVar21 + 0x18);
                  uStack_370 = *(ulonglong *)(lVar21 + 0x20);
                  pWVar30 = local_338;
                  plVar31 = local_368;
                  local_378 = pWVar32;
                }
              }
            }
            uVar15 = SHCreateItemFromParsingName
                               (local_278,(IBindCtx *)0x0,(IID *)&DAT_1401d6d18,&local_398);
            if ((int)uVar15 < 0) goto LAB_1400cb868;
            uVar18 = (**(code **)(*local_360 + 0x60))(local_360,local_398,pWVar32,0);
            uVar15 = (uint)uVar18;
            plVar33 = (longlong *)(uVar18 & 0xffffffff);
            if ((int)uVar15 < 0) goto LAB_1400cb868;
            if (local_398 != (longlong *)0x0) {
              (**(code **)(*local_398 + 0x10))();
            }
            local_398 = (longlong *)0x0;
          }
          else {
            local_2c0 = 0;
            local_2d0 = (LPCWSTR)0x0;
            uStack_2c8 = 0;
            if (uVar29 != 0) {
              FUN_1401b9a40(param_1,&local_2d0,uVar29);
              uVar18 = uStack_2c8;
              FUN_1401bacb0(local_2d0 + uStack_2c8,local_278,uVar29 * 2);
              uStack_2c8 = uVar18 + uVar29;
              pWVar30 = local_338;
            }
            FUN_1401b9a40(param_1,&local_2d0,1);
            local_2d0[uStack_2c8] = L'\\';
            uStack_2c8 = uStack_2c8 + 1;
            FUN_1401b9a40(param_1,&local_2d0,1);
            local_318 = local_2d0;
            local_2d0[uStack_2c8] = L'\0';
            uStack_310 = uStack_2c8;
            if (uStack_2c8 + 1 < 2) {
              uStack_310 = 0;
            }
            if (uStack_310 == 0) {
              uStack_310 = 0;
            }
            SetVolumeLabelW(local_2d0,pWVar32);
          }
          uVar15 = (uint)plVar33;
          local_338 = (PCWSTR)((longlong)pWVar30 + 1);
          local_328 = local_328 + 0x10;
          local_368 = plVar31 + 3;
        } while (local_338 < *(PCWSTR *)(param_2 + 6));
      }
    }
    else if (uVar4 == 5) {
      uVar15 = SHCreateItemFromParsingName
                         (local_308,(IBindCtx *)0x0,(IID *)&DAT_1401d6d18,&local_398);
      if ((int)uVar15 < 0) goto LAB_1400cb84c;
      plVar33 = plVar31;
      if (*(longlong *)(param_2 + 0xc) != 0) {
        do {
          local_358 = (PCWSTR)0x0;
          uStack_350 = 0;
          local_378 = *(LPCWSTR *)((longlong)plVar33 + *(longlong *)(param_2 + 0x10));
          uStack_370 = ((longlong *)((longlong)plVar33 + *(longlong *)(param_2 + 0x10)))[1];
          local_348 = 0;
          FUN_140187f00(local_250,param_1,&local_378,&local_358);
          FUN_1401b9a40(param_1,&local_358,1);
          local_358[uStack_350] = L'\0';
          uVar15 = (**(code **)(*local_360 + 0xa0))(local_360,local_398,0x10,local_358,0,0);
          if ((int)uVar15 < 0) goto LAB_1400cb868;
          if (local_398 != (longlong *)0x0) {
            (**(code **)(*local_398 + 0x10))();
          }
          plVar31 = (longlong *)((longlong)plVar31 + 1);
          local_398 = (longlong *)0x0;
          plVar33 = plVar33 + 3;
        } while (plVar31 < *(longlong **)(param_2 + 0xc));
      }
    }
    else if (uVar4 == 6) {
      uVar15 = SHCreateItemFromParsingName
                         (local_308,(IBindCtx *)0x0,(IID *)&DAT_1401d6d18,&local_398);
      if ((int)uVar15 < 0) goto LAB_1400cb84c;
      plVar33 = plVar31;
      if (*(longlong *)(param_2 + 0xc) != 0) {
        do {
          local_358 = (PCWSTR)0x0;
          uStack_350 = 0;
          local_378 = *(LPCWSTR *)((longlong)plVar33 + *(longlong *)(param_2 + 0x10));
          uStack_370 = ((longlong *)((longlong)plVar33 + *(longlong *)(param_2 + 0x10)))[1];
          local_348 = 0;
          FUN_140187f00(local_250,param_1,&local_378,&local_358);
          FUN_1401b9a40(param_1,&local_358,1);
          local_358[uStack_350] = L'\0';
          uVar15 = (**(code **)(*local_360 + 0xa0))(local_360,local_398,0x80,local_358,0,0);
          if ((int)uVar15 < 0) goto LAB_1400cb868;
          if (local_398 != (longlong *)0x0) {
            (**(code **)(*local_398 + 0x10))();
          }
          plVar31 = (longlong *)((longlong)plVar31 + 1);
          local_398 = (longlong *)0x0;
          plVar33 = plVar33 + 3;
        } while (plVar31 < *(longlong **)(param_2 + 0xc));
      }
    }
    else if (uVar4 == 7) {
      local_328 = 0;
      if (*(longlong *)(param_2 + 6) != 0) {
        local_338 = (PCWSTR)0x0;
        do {
          pWVar30 = local_308;
          uVar18 = 0;
          local_2c0 = 0;
          local_2d0 = (LPCWSTR)0x0;
          uStack_2c8 = 0;
          local_378 = *(LPCWSTR *)((longlong)local_338 + *(longlong *)(param_2 + 10));
          uStack_370 = ((undefined8 *)((longlong)local_338 + *(longlong *)(param_2 + 10)))[1];
          FUN_140187f00(local_250,param_1,&local_378,&local_2d0);
          FUN_1401b9a40(param_1,&local_2d0,1);
          local_288 = local_2d0;
          local_298 = local_2d0;
          local_2d0[uStack_2c8] = L'\0';
          uStack_290 = uStack_2c8;
          if (uStack_2c8 + 1 < 2) {
            uStack_290 = uVar18;
          }
          if (uStack_290 == 0) {
            uStack_290 = uVar18;
          }
          uStack_2c8 = uStack_2c8 + 1;
          uStack_280 = uStack_290;
          lVar21 = FUN_1401ac340(local_148,0,&local_298);
          local_2b8 = 0;
          uStack_2b0 = 0;
          local_378 = *(LPCWSTR *)(lVar21 + 0x18);
          uStack_370 = *(ulonglong *)(lVar21 + 0x20);
          local_318 = local_378;
          uStack_310 = uStack_370;
          puVar20 = (undefined8 *)FUN_1401b0770(&local_238,&local_378,&local_2b8);
          psVar27 = &DAT_1401d4828;
          uVar29 = uVar18;
          if (uStack_2b0 != 0) {
            do {
              if (*psVar27 == 0) break;
              if (*psVar27 != *(short *)(local_2b8 + uVar29 * 2)) goto LAB_1400cadb3;
              uVar29 = uVar29 + 1;
              psVar27 = psVar27 + 1;
            } while (uVar29 < uStack_2b0);
          }
          if ((uVar29 == uStack_2b0) && (*psVar27 == 0)) {
            local_318 = (LPCWSTR)*puVar20;
            uStack_310 = puVar20[1];
            uVar18 = 1;
          }
LAB_1400cadb3:
          local_380 = 0;
          local_390 = (LPCWSTR)0x0;
          uStack_388 = 0;
          if (plVar26 != (longlong *)0x0) {
            FUN_1401b9a40(param_1,&local_390,plVar26);
            uVar29 = uStack_388;
            FUN_1401bacb0(local_390 + uStack_388,pWVar30,(longlong)plVar26 * 2);
            uStack_388 = uVar29 + (longlong)plVar26;
          }
          FUN_1401b9a40(param_1,&local_390,1);
          uVar29 = uStack_310;
          local_390[uStack_388] = L'\\';
          uStack_388 = uStack_388 + 1;
          if (uStack_310 != 0) {
            FUN_1401b9a40(param_1,&local_390,uStack_310);
            uVar22 = uStack_388;
            FUN_1401bacb0(local_390 + uStack_388,local_318,uVar29 * 2);
            uStack_388 = uVar22 + uVar29;
          }
          uVar29 = 0;
          if ((int)uVar18 == 0) {
            pwVar28 = L" - Shortcut";
            uVar18 = uVar29;
            do {
              wVar3 = *pwVar28;
              pwVar28 = pwVar28 + 1;
              if (wVar3 == L'\0') break;
              uVar18 = uVar18 + 1;
            } while (pwVar28 != (wchar_t *)0x0);
            if ((longlong)uVar18 < 0) {
              uVar22 = (uVar18 ^ (longlong)uVar18 >> 0x3f) - ((longlong)uVar18 >> 0x3f);
              if (uVar18 <= uVar22) goto LAB_1400caeee;
              uVar18 = uVar18 - uVar22;
            }
            if (uVar18 == 0) {
              uVar18 = uVar29;
            }
            if (uVar18 != 0) {
              FUN_1401b9a40(param_1,&local_390,uVar18);
              uVar22 = uStack_388;
              FUN_1401bacb0(local_390 + uStack_388,L" - Shortcut",uVar18 * 2);
              uStack_388 = uVar22 + uVar18;
            }
          }
LAB_1400caeee:
          uVar18 = uStack_388;
          psVar27 = &DAT_1401d4848;
          uVar22 = uVar29;
          do {
            sVar2 = *psVar27;
            psVar27 = psVar27 + 1;
            if (sVar2 == 0) break;
            uVar22 = uVar22 + 1;
          } while (psVar27 != (short *)0x0);
          if ((longlong)uVar22 < 0) {
            uVar23 = (uVar22 ^ (longlong)uVar22 >> 0x3f) - ((longlong)uVar22 >> 0x3f);
            if (uVar23 < uVar22) {
              uVar22 = uVar22 - uVar23;
              goto LAB_1400caf2f;
            }
          }
          else {
LAB_1400caf2f:
            if (uVar22 == 0) {
              uVar22 = uVar29;
            }
            if (uVar22 != 0) {
              FUN_1401b9a40(param_1,&local_390,uVar22);
              uVar29 = uStack_388;
              FUN_1401bacb0(local_390 + uStack_388,&DAT_1401d4848,uVar22 * 2);
              uStack_388 = uVar29 + uVar22;
            }
          }
          uVar15 = 1;
          FUN_1401b9a40(param_1,&local_390,1);
          pWVar32 = local_390;
          local_390[uStack_388] = L'\0';
          uStack_388 = uStack_388 + 1;
          local_378 = local_390;
          uStack_370 = uStack_388;
          iVar16 = FUN_140198cf0(&local_378,&local_2e8);
          do {
            uVar29 = 0;
            if (iVar16 == 0) goto LAB_1400cb149;
            psVar27 = &DAT_1401d4854;
            do {
              sVar2 = *psVar27;
              psVar27 = psVar27 + 1;
              if (sVar2 == 0) break;
              uVar29 = uVar29 + 1;
            } while (psVar27 != (short *)0x0);
            uStack_388 = uVar18;
            if ((longlong)uVar29 < 0) {
              uVar22 = (uVar29 ^ (longlong)uVar29 >> 0x3f) - ((longlong)uVar29 >> 0x3f);
              if (uVar22 < uVar29) {
                uVar29 = uVar29 - uVar22;
                goto LAB_1400cafff;
              }
            }
            else {
LAB_1400cafff:
              if (uVar29 == 0) {
                uVar29 = 0;
              }
              if (uVar29 != 0) {
                FUN_1401b9a40(param_1,&local_390,uVar29);
                uVar22 = uStack_388;
                FUN_1401bacb0(local_390 + uStack_388,&DAT_1401d4854,uVar29 * 2);
                uStack_388 = uVar22 + uVar29;
              }
            }
            FUN_1401b9a40(param_1,&local_390,1);
            local_390[uStack_388] = (short)uVar15 + L'0';
            uStack_388 = uStack_388 + 1;
            psVar27 = &DAT_1401d4860;
            uVar29 = 0;
            do {
              sVar2 = *psVar27;
              psVar27 = psVar27 + 1;
              if (sVar2 == 0) break;
              uVar29 = uVar29 + 1;
            } while (psVar27 != (short *)0x0);
            if ((longlong)uVar29 < 0) {
              uVar22 = (uVar29 ^ (longlong)uVar29 >> 0x3f) - ((longlong)uVar29 >> 0x3f);
              if (uVar22 < uVar29) {
                uVar29 = uVar29 - uVar22;
                goto LAB_1400cb0af;
              }
            }
            else {
LAB_1400cb0af:
              if (uVar29 == 0) {
                uVar29 = 0;
              }
              if (uVar29 != 0) {
                FUN_1401b9a40(param_1,&local_390,uVar29);
                uVar22 = uStack_388;
                FUN_1401bacb0(local_390 + uStack_388,&DAT_1401d4860,uVar29 * 2);
                uStack_388 = uVar22 + uVar29;
              }
            }
            FUN_1401b9a40(param_1,&local_390,1);
            pWVar32 = local_390;
            local_390[uStack_388] = L'\0';
            uStack_388 = uStack_388 + 1;
            local_378 = local_390;
            uStack_370 = uStack_388;
            iVar16 = FUN_140198cf0(&local_378,&local_2e8);
            uVar15 = uVar15 + 1;
          } while (uVar15 < 10);
          if (iVar16 == 0) {
LAB_1400cb149:
            uVar36 = 0;
            uVar37 = 0;
            uVar38 = 0;
            uVar39 = 0;
            puVar20 = (undefined8 *)FUN_1400c6b20(local_178,0,0);
            local_378 = local_288;
            uStack_370 = uStack_280;
            uVar8 = *puVar20;
            uVar9 = puVar20[1];
            uVar10 = puVar20[2];
            uVar11 = puVar20[3];
            uVar12 = puVar20[4];
            uVar13 = puVar20[5];
            lVar21 = FUN_1401ac340(local_148,uVar8,&local_378);
            local_188 = *(longlong *)(lVar21 + 0x60);
            if (1 < *(ulonglong *)(lVar21 + 0x50)) {
              puVar1 = (undefined4 *)(local_188 + (*(ulonglong *)(lVar21 + 0x50) - 2) * 0x10);
              uVar36 = *puVar1;
              uVar37 = puVar1[1];
              uVar38 = puVar1[2];
              uVar39 = puVar1[3];
            }
            local_218 = uVar8;
            uStack_210 = uVar9;
            local_208 = uVar10;
            uStack_200 = uVar11;
            local_1f8 = uVar12;
            uStack_1f0 = uVar13;
            FUN_140001a90(&local_218);
            local_378 = (LPCWSTR)CONCAT44(uVar37,uVar36);
            uStack_370 = CONCAT44(uVar39,uVar38);
            plVar31 = (longlong *)FUN_1401b9c70(&local_278,param_1,&local_378);
            local_2e8 = (PCWSTR)0x0;
            local_368 = (longlong *)0x0;
            local_2b8 = *plVar31;
            uStack_2b0 = plVar31[1];
            local_378 = (LPCWSTR)0x0;
            uStack_370 = 0;
            HVar17 = CoCreateInstance((IID *)&DAT_1401d6d88,(LPUNKNOWN)0x0,1,(IID *)&DAT_1401d6bd8,
                                      &local_2e8);
            if ((((-1 < HVar17) &&
                 (HVar17 = (**(code **)(*(longlong *)local_2e8 + 0xa0))(local_2e8,local_2d0),
                 -1 < HVar17)) &&
                (HVar17 = (**(code **)(*(longlong *)local_2e8 + 0x38))(local_2e8,local_378),
                -1 < HVar17)) &&
               ((HVar17 = (**(code **)(*(longlong *)local_2e8 + 0x48))(local_2e8,local_2b8),
                -1 < HVar17 &&
                (HVar17 = (*(code *)**(undefined8 **)local_2e8)(local_2e8,&DAT_1401d6c98,&local_368)
                , -1 < HVar17)))) {
              HVar17 = (**(code **)(*local_368 + 0x30))(local_368,pWVar32,1);
            }
            FUN_1401b4010(HVar17,"Win_CreateShortcut");
            if (local_2e8 != (PCWSTR)0x0) {
              (**(code **)(*(longlong *)local_2e8 + 0x10))();
            }
            local_2e8 = (PCWSTR)0x0;
            if (local_368 != (longlong *)0x0) {
              (**(code **)(*local_368 + 0x10))();
            }
          }
          local_328 = local_328 + 1;
          local_338 = local_338 + 8;
          uVar15 = local_res20[0];
        } while (local_328 < *(ulonglong *)(param_2 + 6));
      }
    }
    else if (uVar4 == 8) {
      local_358 = *(PCWSTR *)(param_2 + 6);
      uStack_350 = *(ulonglong *)(param_2 + 8);
      local_348 = *(undefined8 *)(param_2 + 10);
      FUN_1401af9f0(&local_358,1);
    }
    else if (uVar4 == 9) {
      SHEmptyRecycleBinW(0,0,0);
    }
    else if (uVar4 == 10) {
      local_358 = *(PCWSTR *)(param_2 + 6);
      uStack_350 = *(ulonglong *)(param_2 + 8);
      local_348 = *(undefined8 *)(param_2 + 10);
      FUN_1401af9f0(&local_358,0);
    }
    else if (uVar4 == 0xb) {
      local_358 = (PCWSTR)0x0;
      uStack_350 = 0;
      local_2f8 = 0;
      local_348 = 0;
      FUN_1401af9f0(&local_358,0);
    }
    if (local_360 == (longlong *)0x0) {
      bVar35 = false;
    }
    else {
      uVar15 = (**(code **)(*local_360 + 0x28))(local_360,local_260);
      if ((((int)uVar15 < 0) ||
          (uVar15 = CoCreateInstance((IID *)&DAT_1401d6b18,(LPUNKNOWN)0x0,0x17,(IID *)&DAT_1401d6d58
                                     ,&local_258), (int)uVar15 < 0)) ||
         (uVar15 = (**(code **)(*local_360 + 0x38))(local_360,local_258), (int)uVar15 < 0))
      goto LAB_1400cb84c;
      uVar15 = (**(code **)(*local_360 + 0xa8))();
      bVar35 = -1 < (int)uVar15;
    }
  }
  else {
    uVar15 = CoCreateInstance((IID *)&DAT_1401d6d98,(LPUNKNOWN)0x0,0x17,(IID *)&DAT_1401d6d68,
                              &local_360);
    plVar33 = (longlong *)(ulonglong)uVar15;
    local_res20[0] = uVar15;
    if (-1 < (int)uVar15) goto LAB_1400ca488;
LAB_1400cb84c:
    bVar35 = false;
  }
  if (uVar15 == 0) goto LAB_1400cb8d4;
LAB_1400cb86a:
  dwMessageId = GetLastError();
  FormatMessageA(0x1300,(LPCVOID)0x0,dwMessageId,0,(LPSTR)0x0,0,(va_list *)0x0);
  FUN_1400c3130("C:\\Users\\Vjekoslav\\Projects\\Voidstar\\File Pilot\\Source\\Win\\WinMain.c",
                "Win__CheckLogHREUSLT",0x1024,"source: %s: HR: 0x%x, msg: %s\n",
                "P_PerformFilesOperationNT",uVar15,0);
  LocalFree((HLOCAL)0x0);
LAB_1400cb8d4:
  if (local_360 != (longlong *)0x0) {
    (**(code **)(*local_360 + 0x10))();
  }
  local_360 = (longlong *)0x0;
  if (local_398 != (longlong *)0x0) {
    (**(code **)(*local_398 + 0x10))();
  }
  local_398 = (longlong *)0x0;
  if (local_2d8 != (longlong *)0x0) {
    (**(code **)(*local_2d8 + 0x10))();
  }
  local_2d8 = (longlong *)0x0;
  if (local_258 != (longlong *)0x0) {
    (**(code **)(*local_258 + 0x10))();
  }
  return bVar35;
LAB_1400cb868:
  bVar35 = false;
  goto LAB_1400cb86a;
}

