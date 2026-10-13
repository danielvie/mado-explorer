
void FUN_14018e300(longlong *param_1,HMENU param_2,ulonglong param_3,int param_4,int param_5)

{
  longlong lVar1;
  undefined1 auVar2 [16];
  HBITMAP pHVar3;
  char cVar4;
  undefined1 uVar5;
  BOOL BVar6;
  int iVar7;
  longlong lVar8;
  char *pcVar9;
  undefined4 *puVar10;
  ulonglong uVar11;
  undefined8 *puVar12;
  ulonglong uVar13;
  ulonglong *puVar14;
  ulonglong uVar15;
  char *pcVar16;
  short *psVar17;
  short *psVar18;
  char *pcVar19;
  longlong lVar20;
  longlong lVar21;
  ulonglong *puVar22;
  LPWSTR pWVar23;
  longlong lVar24;
  char *pcVar25;
  short *psVar26;
  ulonglong uVar27;
  ulonglong uVar28;
  ulonglong uVar29;
  LPVOID pvVar30;
  char *pcVar31;
  LPWSTR lpString;
  undefined4 uVar32;
  undefined4 uVar33;
  undefined4 uVar34;
  undefined4 uVar35;
  undefined4 uVar36;
  undefined4 uVar37;
  undefined8 local_338;
  undefined8 uStack_330;
  uint local_328;
  uint local_324;
  char *local_318;
  undefined8 uStack_310;
  int local_308;
  uint local_304;
  ulonglong *local_300;
  undefined8 local_2f8;
  undefined8 local_2f0;
  undefined8 local_2e8;
  undefined8 local_2e0;
  undefined8 local_2d8;
  undefined8 local_2d0;
  undefined8 local_2c8;
  undefined8 local_2c0;
  undefined8 local_2b8;
  undefined8 local_2b0;
  undefined8 local_2a8;
  undefined8 local_2a0;
  undefined8 local_298;
  undefined8 local_290;
  undefined8 local_288;
  undefined8 local_280;
  undefined8 local_278;
  undefined8 local_270;
  undefined8 local_268;
  undefined8 local_260;
  undefined8 local_258;
  undefined8 local_250;
  undefined8 local_248;
  undefined8 local_240;
  undefined8 local_238;
  ulonglong local_230;
  undefined8 local_228;
  undefined8 local_220;
  tagMENUITEMINFOW local_218;
  char *local_1c8;
  undefined4 uStack_1c0;
  undefined4 uStack_1bc;
  LPWSTR local_1b8;
  char *local_1b0;
  short *local_1a8;
  ulonglong local_1a0;
  char *local_198;
  undefined4 uStack_190;
  undefined4 uStack_18c;
  undefined8 local_188;
  undefined8 uStack_180;
  undefined8 local_178;
  undefined8 uStack_170;
  undefined8 local_168;
  undefined8 uStack_160;
  undefined8 local_158;
  undefined8 local_150;
  undefined8 uStack_148;
  undefined8 local_140;
  undefined1 local_138 [4];
  undefined4 local_134;
  short local_128;
  short local_126;
  undefined1 local_118 [16];
  undefined1 local_108 [16];
  undefined1 local_f8 [16];
  undefined1 local_e8 [16];
  undefined8 local_d8;
  undefined8 uStack_d0;
  undefined1 local_98 [96];
  
  local_304 = GetMenuItemCount(param_2);
  if (-1 < (int)local_304) {
    if (*(char *)(param_3 + 0x65) != '\0') {
      FUN_1400c7550(param_1,param_3,param_5);
      return;
    }
    puVar22 = (ulonglong *)0x0;
    local_324 = 0;
    local_300 = (ulonglong *)0x0;
    puVar14 = puVar22;
    if (local_304 != 0) {
      do {
        local_218.cbSize = 0x50;
        local_218.hbmpChecked = (HBITMAP)0x0;
        local_218.hbmpUnchecked = (HBITMAP)0x0;
        local_218.fMask = 0x187;
        local_218.fType = 0;
        local_218.fState = 0;
        local_218.wID = 0;
        local_218.hSubMenu = (HMENU)0x0;
        local_218.dwItemData = 0;
        local_218.dwTypeData = (LPWSTR)0x0;
        local_218.cch = 0;
        local_218.hbmpItem = (HBITMAP)0x0;
        BVar6 = GetMenuItemInfoW(param_2,(UINT)puVar22,1,&local_218);
        puVar22 = local_300;
        if (BVar6 != 0) {
          uVar28 = (ulonglong)local_218.wID;
          local_328 = local_218.wID;
          if (local_218.fType == 0x800) {
            *(int *)(param_1 + 0x13) = (int)param_1[0x13] + 1;
          }
          else {
            lVar20 = param_1[4];
            pvVar30 = (LPVOID)param_1[5];
            uVar34 = 0;
            uVar35 = 0;
            uVar36 = 0;
            uVar37 = 0;
            uVar13 = param_1[3];
            uVar27 = (ulonglong)((int)lVar20 + (int)pvVar30 & 7);
            lVar21 = 8 - uVar27;
            if (uVar27 == 0) {
              lVar21 = 0;
            }
            lVar24 = lVar21 + 0x200;
            if (uVar13 < (ulonglong)(lVar24 + lVar20)) {
              if ((ulonglong)param_1[2] < (ulonglong)(lVar24 + lVar20)) {
                FUN_1401b5c50(param_1,lVar24,8);
                pvVar30 = (LPVOID)param_1[5];
                lVar20 = param_1[4];
                uVar13 = param_1[3];
                uVar27 = (ulonglong)((int)lVar20 + (int)pvVar30 & 7);
                lVar21 = 8 - uVar27;
                if (uVar27 == 0) {
                  lVar21 = 0;
                }
                lVar24 = lVar21 + 0x200;
              }
              lVar8 = 0;
              lVar1 = *param_1;
              if ((lVar1 != 0) && (uVar27 = lVar1 - 1U & lVar20 + lVar24, uVar27 != 0)) {
                lVar8 = lVar1 - uVar27;
              }
              uVar27 = lVar8 + lVar20 + lVar24;
              if (uVar13 < uVar27) {
                param_1[3] = uVar27;
                VirtualAlloc(pvVar30,uVar27,0x1000,4);
              }
            }
            lpString = (LPWSTR)(param_1[5] + lVar21 + param_1[4]);
            FUN_1401bace0(lpString,0,0x200);
            param_1[4] = param_1[4] + lVar24;
            param_1[6] = (longlong)lpString;
            local_308 = GetMenuStringW(param_2,local_324,lpString,0x100,0x400);
            if (0 < local_308) {
              pcVar9 = (char *)0x0;
              pcVar31 = (char *)(longlong)local_308;
              local_318 = (char *)0x0;
              uStack_310 = 0;
              pcVar16 = pcVar9;
              pcVar19 = pcVar9;
              if (local_308 != 0) {
                do {
                  if (lpString[(longlong)pcVar16] == L'&') {
                    if ((int)pcVar19 == 0) {
                      uStack_190 = 1;
                      uStack_310 = CONCAT44(uStack_18c,1);
                      local_318 = pcVar16;
                      local_198 = pcVar16;
                      break;
                    }
                    pcVar19 = (char *)(ulonglong)((int)pcVar19 - 1);
                  }
                  pcVar16 = pcVar16 + 1;
                } while (pcVar16 < pcVar31);
              }
              pWVar23 = lpString;
              if ((int)uStack_310 != 0) {
                lVar20 = param_1[4];
                lVar21 = (longlong)pcVar31 * 2;
                pvVar30 = (LPVOID)param_1[5];
                pcVar16 = (char *)param_1[3];
                uVar28 = (ulonglong)((int)lVar20 + (int)pvVar30 & 7);
                pcVar19 = (char *)(8 - uVar28);
                if (uVar28 == 0) {
                  pcVar19 = pcVar9;
                }
                pcVar25 = pcVar19 + lVar21;
                if (pcVar16 < pcVar25 + lVar20) {
                  if ((char *)param_1[2] < pcVar25 + lVar20) {
                    FUN_1401b5c50(param_1,pcVar25,8);
                    pvVar30 = (LPVOID)param_1[5];
                    lVar20 = param_1[4];
                    pcVar16 = (char *)param_1[3];
                    uVar28 = (ulonglong)((int)lVar20 + (int)pvVar30 & 7);
                    pcVar19 = (char *)(8 - uVar28);
                    if (uVar28 == 0) {
                      pcVar19 = pcVar9;
                    }
                    pcVar25 = pcVar19 + lVar21;
                  }
                  lVar24 = *param_1;
                  if ((lVar24 != 0) &&
                     (uVar28 = lVar24 - 1U & (ulonglong)(pcVar25 + lVar20), uVar28 != 0)) {
                    pcVar9 = (char *)(lVar24 - uVar28);
                  }
                  pcVar9 = pcVar9 + (longlong)(pcVar25 + lVar20);
                  if (pcVar16 < pcVar9) {
                    param_1[3] = (longlong)pcVar9;
                    VirtualAlloc(pvVar30,(SIZE_T)pcVar9,0x1000,4);
                  }
                }
                pWVar23 = (LPWSTR)(pcVar19 + param_1[4] + param_1[5]);
                FUN_1401bace0(pWVar23,0,lVar21);
                pcVar9 = local_318;
                param_1[4] = (longlong)(pcVar25 + param_1[4]);
                param_1[6] = (longlong)pWVar23;
                FUN_1401bacb0(pWVar23,lpString,(longlong)local_318 * 2);
                FUN_1401bacb0(pWVar23 + (longlong)pcVar9,lpString + (longlong)(pcVar9 + 1),
                              ((longlong)pcVar31 - (longlong)pcVar9) * 2 + -2);
                uVar28 = (ulonglong)local_328;
                pcVar31 = (char *)(longlong)(local_308 + -1);
              }
              pcVar16 = (char *)0x0;
              local_318 = pcVar16;
              pcVar9 = pcVar16;
              if (pcVar31 != (char *)0x0) {
                do {
                  if (pWVar23[(longlong)local_318] == L'\t') {
                    if ((int)pcVar9 == 0) {
                      uStack_1c0 = 1;
                      uStack_310 = CONCAT44(uStack_1bc,1);
                      local_1c8 = local_318;
                      goto LAB_14018e6e0;
                    }
                    pcVar9 = (char *)(ulonglong)((int)pcVar9 - 1);
                  }
                  local_318 = local_318 + 1;
                } while (local_318 < pcVar31);
              }
              uStack_310 = 0;
              local_318 = pcVar31;
LAB_14018e6e0:
              local_1b0 = local_318;
              if (((longlong)local_318 < 0) &&
                 (pcVar9 = (char *)(((ulonglong)local_318 ^ (longlong)local_318 >> 0x3f) -
                                   ((longlong)local_318 >> 0x3f)), local_1b0 = pcVar16,
                 pcVar9 < pcVar31)) {
                local_1b0 = pcVar31 + -(longlong)pcVar9;
              }
              local_158 = 0;
              if (local_1b0 == (char *)0x0) {
                local_1b0 = (char *)0x0;
              }
              local_168 = 0;
              uStack_160 = 0;
              local_1b8 = pWVar23;
              puVar10 = (undefined4 *)FUN_140188d50(local_118,param_1,&local_1b8,&local_168);
              uVar34 = *puVar10;
              uVar35 = puVar10[1];
              uVar36 = puVar10[2];
              uVar37 = puVar10[3];
              puVar14 = local_300;
            }
            uVar13 = 0;
            uVar32 = 0;
            uVar33 = 0;
            local_318 = (char *)0x0;
            uStack_310 = 0;
            pcVar9 = (char *)0x0;
            if ((uint)uVar28 < 0x7fff) {
              lVar20 = param_1[4];
              pvVar30 = (LPVOID)param_1[5];
              uVar27 = param_1[3];
              uVar11 = (ulonglong)((int)lVar20 + (int)pvVar30 & 7);
              uVar15 = 8 - uVar11;
              if (uVar11 == 0) {
                uVar15 = uVar13;
              }
              lVar21 = uVar15 + 0x200;
              if (uVar27 < (ulonglong)(lVar21 + lVar20)) {
                if ((ulonglong)param_1[2] < (ulonglong)(lVar21 + lVar20)) {
                  FUN_1401b5c50(param_1,lVar21,8);
                  pvVar30 = (LPVOID)param_1[5];
                  lVar20 = param_1[4];
                  uVar27 = param_1[3];
                  uVar11 = (ulonglong)((int)lVar20 + (int)pvVar30 & 7);
                  uVar15 = 8 - uVar11;
                  if (uVar11 == 0) {
                    uVar15 = uVar13;
                  }
                  lVar21 = uVar15 + 0x200;
                }
                lVar24 = *param_1;
                uVar11 = uVar13;
                if ((lVar24 != 0) && (uVar29 = lVar24 - 1U & lVar21 + lVar20, uVar29 != 0)) {
                  uVar11 = lVar24 - uVar29;
                }
                uVar11 = uVar11 + lVar21 + lVar20;
                if (uVar27 < uVar11) {
                  param_1[3] = uVar11;
                  VirtualAlloc(pvVar30,uVar11,0x1000,4);
                }
              }
              psVar26 = (short *)(param_1[5] + uVar15 + param_1[4]);
              FUN_1401bace0(psVar26,0,0x200);
              param_1[4] = param_1[4] + lVar21;
              param_1[6] = (longlong)psVar26;
              iVar7 = (**(code **)(*(longlong *)param_1[0x19] + 0x28))
                                ((longlong *)param_1[0x19],uVar28,4,0,psVar26,0x100);
              if ((iVar7 < 0) || (psVar17 = psVar26, uVar27 = uVar13, *psVar26 == 0)) {
                local_338 = (char *)CONCAT44(uVar35,uVar34);
                uStack_330 = CONCAT44(uVar37,uVar36);
                FUN_1400c3130("C:\\Users\\Vjekoslav\\Projects\\Voidstar\\File Pilot\\Source\\Win\\WinMain.c"
                              ,"Win_ContextMenuBuild",0x35f6,
                              "\tWin_ContextMenuBuild GetCommandString GCS_VERBW failed for item name: %S\n"
                              ,&local_338);
                lVar20 = param_1[4];
                pvVar30 = (LPVOID)param_1[5];
                uVar27 = param_1[3];
                uVar11 = (ulonglong)((int)lVar20 + (int)pvVar30 & 7);
                uVar15 = 8 - uVar11;
                if (uVar11 == 0) {
                  uVar15 = uVar13;
                }
                lVar21 = uVar15 + 0x100;
                if (uVar27 < (ulonglong)(lVar21 + lVar20)) {
                  if ((ulonglong)param_1[2] < (ulonglong)(lVar21 + lVar20)) {
                    FUN_1401b5c50(param_1,lVar21,8);
                    pvVar30 = (LPVOID)param_1[5];
                    lVar20 = param_1[4];
                    uVar27 = param_1[3];
                    uVar11 = (ulonglong)((int)lVar20 + (int)pvVar30 & 7);
                    uVar15 = 8 - uVar11;
                    if (uVar11 == 0) {
                      uVar15 = uVar13;
                    }
                    lVar21 = uVar15 + 0x100;
                  }
                  lVar24 = *param_1;
                  uVar11 = uVar13;
                  if ((lVar24 != 0) && (uVar29 = lVar24 - 1U & lVar21 + lVar20, uVar29 != 0)) {
                    uVar11 = lVar24 - uVar29;
                  }
                  uVar11 = uVar11 + lVar21 + lVar20;
                  if (uVar27 < uVar11) {
                    param_1[3] = uVar11;
                    VirtualAlloc(pvVar30,uVar11,0x1000,4);
                  }
                }
                pcVar16 = (char *)(param_1[5] + uVar15 + param_1[4]);
                FUN_1401bace0(pcVar16,0,0x100);
                param_1[4] = param_1[4] + lVar21;
                param_1[6] = (longlong)pcVar16;
                iVar7 = (**(code **)(*(longlong *)param_1[0x19] + 0x28))
                                  ((longlong *)param_1[0x19],uVar28,0,0,pcVar16,0x100);
                pcVar9 = (char *)0x0;
                if ((-1 < iVar7) &&
                   (pcVar19 = pcVar16, uVar28 = uVar13, pcVar9 = (char *)0x0, *pcVar16 != '\0')) {
                  do {
                    pcVar9 = pcVar19 + 1;
                    if (*pcVar19 == '\0') break;
                    uVar28 = uVar28 + 1;
                    pcVar19 = pcVar9;
                  } while (pcVar9 != (char *)0x0);
                  uVar27 = uVar28;
                  if (((longlong)uVar28 < 0) &&
                     (uVar15 = (uVar28 ^ (longlong)uVar28 >> 0x3f) - ((longlong)uVar28 >> 0x3f),
                     uVar27 = uVar13, uVar15 < uVar28)) {
                    uVar27 = uVar28 - uVar15;
                  }
                  if (uVar27 == 0) {
                    uVar27 = uVar13;
                  }
                  uStack_310._0_4_ = (int)uVar27;
                  uStack_310._4_4_ = (undefined4)(uVar27 >> 0x20);
                  uVar32 = (int)uStack_310;
                  uVar33 = uStack_310._4_4_;
                  local_318 = pcVar16;
                  uStack_310 = uVar27;
                  pcVar9 = pcVar16;
                }
              }
              else {
                do {
                  psVar18 = psVar17 + 1;
                  if (*psVar17 == 0) break;
                  uVar27 = uVar27 + 1;
                  psVar17 = psVar18;
                } while (psVar18 != (short *)0x0);
                local_1a0 = uVar27;
                if (((longlong)uVar27 < 0) &&
                   (uVar28 = (uVar27 ^ (longlong)uVar27 >> 0x3f) - ((longlong)uVar27 >> 0x3f),
                   local_1a0 = uVar13, uVar28 < uVar27)) {
                  local_1a0 = uVar27 - uVar28;
                }
                local_140 = 0;
                if (local_1a0 == 0) {
                  local_1a0 = uVar13;
                }
                local_150 = 0;
                uStack_148 = 0;
                local_1a8 = psVar26;
                puVar12 = (undefined8 *)FUN_140188d50(local_108,param_1,&local_1a8,&local_150);
                uVar32 = *(undefined4 *)(puVar12 + 1);
                uVar33 = *(undefined4 *)((longlong)puVar12 + 0xc);
                pcVar9 = (char *)*puVar12;
              }
              uVar28 = (ulonglong)local_328;
            }
            if (param_4 == 0) {
              local_228 = 1;
              local_220 = 0xffffffffffffffff;
              uStack_330 = CONCAT44(uVar33,uVar32);
              local_338 = pcVar9;
              iVar7 = FUN_1401b8970(&local_338,"groupby",&local_228);
              puVar22 = local_300;
              if (iVar7 == 0) {
                local_2f8 = 1;
                local_2f0 = 0xffffffffffffffff;
                uStack_330 = CONCAT44(uVar33,uVar32);
                local_338 = pcVar9;
                iVar7 = FUN_1401b8970(&local_338,"pintohome",&local_2f8);
                puVar22 = local_300;
                if (iVar7 == 0) {
                  local_2e8 = 1;
                  local_2e0 = 0xffffffffffffffff;
                  uStack_330 = CONCAT44(uVar33,uVar32);
                  local_338 = pcVar9;
                  iVar7 = FUN_1401b8970(&local_338,"viewcustomwizard",&local_2e8);
                  puVar22 = local_300;
                  if (iVar7 == 0) {
                    local_2d8 = 1;
                    local_2d0 = 0xffffffffffffffff;
                    uStack_330 = CONCAT44(uVar33,uVar32);
                    local_338 = pcVar9;
                    iVar7 = FUN_1401b8970(&local_338,&DAT_1401d501c,&local_2d8);
                    puVar22 = local_300;
                    if (iVar7 == 0) {
                      local_2c8 = 1;
                      local_2c0 = 0xffffffffffffffff;
                      uStack_330 = CONCAT44(uVar33,uVar32);
                      local_338 = pcVar9;
                      iVar7 = FUN_1401b8970(&local_338,&DAT_1401d5024,&local_2c8);
                      puVar22 = local_300;
                      if (iVar7 == 0) {
                        local_2b8 = 1;
                        local_2b0 = 0xffffffffffffffff;
                        uStack_330 = CONCAT44(uVar33,uVar32);
                        local_338 = pcVar9;
                        iVar7 = FUN_1401b8970(&local_338,"windows.share",&local_2b8);
                        if ((iVar7 == 0) || (puVar22 = local_300, param_1[0xe] != 0))
                        goto LAB_14018ec07;
                      }
                    }
                  }
                }
              }
            }
            else {
LAB_14018ec07:
              lVar20 = param_1[4];
              pvVar30 = (LPVOID)param_1[5];
              uVar27 = param_1[3];
              uVar11 = (ulonglong)((int)lVar20 + (int)pvVar30 & 7);
              uVar15 = 8 - uVar11;
              if (uVar11 == 0) {
                uVar15 = uVar13;
              }
              lVar21 = uVar15 + 0xe0;
              if (uVar27 < (ulonglong)(lVar21 + lVar20)) {
                if ((ulonglong)param_1[2] < (ulonglong)(lVar21 + lVar20)) {
                  FUN_1401b5c50(param_1,lVar21,8);
                  pvVar30 = (LPVOID)param_1[5];
                  lVar20 = param_1[4];
                  uVar27 = param_1[3];
                  uVar11 = (ulonglong)((int)lVar20 + (int)pvVar30 & 7);
                  uVar15 = 8 - uVar11;
                  if (uVar11 == 0) {
                    uVar15 = uVar13;
                  }
                  lVar21 = uVar15 + 0xe0;
                }
                lVar24 = *param_1;
                if ((lVar24 != 0) && (uVar11 = lVar24 - 1U & lVar20 + lVar21, uVar11 != 0)) {
                  uVar13 = lVar24 - uVar11;
                }
                uVar13 = uVar13 + lVar20 + lVar21;
                if (uVar27 < uVar13) {
                  param_1[3] = uVar13;
                  VirtualAlloc(pvVar30,uVar13,0x1000,4);
                }
              }
              puVar22 = (ulonglong *)(uVar15 + param_1[5] + param_1[4]);
              param_1[6] = (longlong)puVar22;
              param_1[4] = lVar21 + param_1[4];
              puVar22[0xe] = param_3;
              if (*(longlong *)(param_3 + 0x78) == 0) {
                *(ulonglong **)(param_3 + 0x78) = puVar22;
              }
              if (puVar14 != (ulonglong *)0x0) {
                puVar14[0x11] = (ulonglong)puVar22;
              }
              *(undefined4 *)(puVar22 + 1) = *(undefined4 *)((longlong)param_1 + 0x94);
              *(int *)((longlong)param_1 + 0x94) = *(int *)((longlong)param_1 + 0x94) + 1;
              uVar13 = 0;
              *(uint *)(puVar22 + 0x12) = local_324;
              puVar22[0x13] = (ulonglong)param_2;
              local_2a8 = 1;
              local_2a0 = 0xffffffffffffffff;
              uStack_330 = CONCAT44(uVar33,uVar32);
              local_338 = pcVar9;
              iVar7 = FUN_1401b8970(&local_338,"newfolder",&local_2a8);
              if (iVar7 == 0) {
                local_298 = 1;
                local_290 = 0xffffffffffffffff;
                uStack_330 = CONCAT44(uVar33,uVar32);
                local_338 = pcVar9;
                iVar7 = FUN_1401b8970(&local_338,&DAT_1401d504c,&local_298);
                if (iVar7 != 0) goto LAB_14018ed81;
              }
              else {
LAB_14018ed81:
                *(undefined4 *)(param_3 + 0x38) = 10;
              }
              local_288 = 1;
              local_280 = 0xffffffffffffffff;
              uStack_330 = CONCAT44(uVar33,uVar32);
              local_338 = pcVar9;
              cVar4 = FUN_1401b8970(&local_338,&DAT_1401d5054,&local_288);
              *(char *)((longlong)puVar22 + 99) = cVar4;
              if (cVar4 != '\0') {
                param_1[0x17] = (longlong)puVar22;
              }
              local_278 = 1;
              local_270 = 0xffffffffffffffff;
              uStack_330 = CONCAT44(uVar33,uVar32);
              local_338 = pcVar9;
              cVar4 = FUN_1401b8970(&local_338,"arrange",&local_278);
              *(char *)((longlong)puVar22 + 100) = cVar4;
              if (cVar4 != '\0') {
                param_1[0x18] = (longlong)puVar22;
              }
              if (param_1[0x15] == 0) {
                param_1[0x15] = (longlong)puVar22;
              }
              else {
                *(ulonglong **)(param_1[0x16] + 0x80) = puVar22;
              }
              param_1[0x16] = (longlong)puVar22;
              puVar22[0x10] = 0;
              *(short *)((longlong)puVar22 + 0x66) = (short)param_1[0x13];
              *(short *)(puVar22 + 0xd) = (short)param_4;
              local_268 = 0;
              local_260 = 0xffffffffffffffff;
              uStack_330 = CONCAT44(uVar33,uVar32);
              *(bool *)((longlong)puVar22 + 0x5c) = ((byte)local_218.fState & 3) == 3;
              local_338 = pcVar9;
              iVar7 = FUN_1401b8970(&local_338,"paste",&local_268);
              if (iVar7 == 0) {
                local_258 = 0;
                local_250 = 0xffffffffffffffff;
                uStack_330 = CONCAT44(uVar33,uVar32);
                local_338 = pcVar9;
                iVar7 = FUN_1401b8970(&local_338,"pastelink",&local_258);
                if (iVar7 != 0) goto LAB_14018ee9d;
              }
              else {
LAB_14018ee9d:
                puVar12 = (undefined8 *)FUN_1400c36b0(local_98,param_1,2);
                local_248 = 0;
                local_240 = 0xffffffffffffffff;
                uStack_330 = CONCAT44(uVar33,uVar32);
                lVar20 = puVar12[3];
                local_d8 = *puVar12;
                uStack_d0 = puVar12[1];
                *(bool *)((longlong)puVar22 + 0x5c) = lVar20 == 0;
                local_338 = pcVar9;
                iVar7 = FUN_1401b8970(&local_338,"pastelink",&local_248);
                if (iVar7 != 0) {
                  if ((lVar20 == 0) || ((int)local_d8 != 0)) {
                    uVar5 = 1;
                  }
                  else {
                    uVar5 = 0;
                  }
                  *(undefined1 *)((longlong)puVar22 + 0x5c) = uVar5;
                }
              }
              auVar2._4_4_ = uVar35;
              auVar2._0_4_ = uVar34;
              auVar2._8_4_ = uVar36;
              auVar2._12_4_ = uVar37;
              if (auVar2._8_8_ != 0) {
                uVar27 = puVar22[0xe];
                *(undefined4 *)(puVar22 + 2) = uVar34;
                *(undefined4 *)((longlong)puVar22 + 0x14) = uVar35;
                *(undefined4 *)(puVar22 + 3) = uVar36;
                *(undefined4 *)((longlong)puVar22 + 0x1c) = uVar37;
                if (*(longlong *)(uVar27 + 0x18) == 0) {
                  puVar12 = &local_178;
                  local_178 = 0;
                  uStack_170 = 0;
                }
                else {
                  local_188 = *(undefined8 *)(uVar27 + 0x20);
                  uStack_180 = *(undefined8 *)(uVar27 + 0x28);
                  puVar12 = (undefined8 *)FUN_1401b8b00(local_f8,param_1,&DAT_1401d507c,&local_188);
                }
                local_318 = (char *)*puVar12;
                uStack_310 = puVar12[1];
                local_338 = (char *)puVar22[2];
                uStack_330 = puVar22[3];
                puVar14 = (ulonglong *)
                          FUN_1401b8b00(local_e8,param_1,&DAT_1401c0724,&local_318,&local_338);
                uVar27 = puVar14[1];
                puVar22[4] = *puVar14;
                puVar22[5] = uVar27;
              }
              if ((uint)uVar28 < 0x7fff) {
                *puVar22 = uVar28;
                puVar22[8] = (ulonglong)pcVar9;
                *(undefined4 *)(puVar22 + 9) = uVar32;
                *(undefined4 *)((longlong)puVar22 + 0x4c) = uVar33;
                uStack_330 = CONCAT44(uVar33,uVar32);
                local_338 = pcVar9;
                uVar34 = FUN_14018f230(&local_338);
                *(undefined4 *)(puVar22 + 7) = uVar34;
              }
              pHVar3 = local_218.hbmpItem;
              iVar7 = GetObjectW(local_218.hbmpItem,0x20,local_138);
              if (((iVar7 != 0) && (local_128 == 1)) && (local_126 == 0x20)) {
                lVar20 = param_1[4];
                pvVar30 = (LPVOID)param_1[5];
                uVar28 = param_1[3];
                uVar15 = (ulonglong)((int)lVar20 + (int)pvVar30 & 7);
                uVar27 = 8 - uVar15;
                if (uVar15 == 0) {
                  uVar27 = uVar13;
                }
                lVar21 = uVar27 + 0x20;
                if (uVar28 < (ulonglong)(lVar21 + lVar20)) {
                  if ((ulonglong)param_1[2] < (ulonglong)(lVar21 + lVar20)) {
                    FUN_1401b5c50(param_1,lVar21,8);
                    pvVar30 = (LPVOID)param_1[5];
                    lVar20 = param_1[4];
                    uVar28 = param_1[3];
                    uVar15 = (ulonglong)((int)lVar20 + (int)pvVar30 & 7);
                    uVar27 = 8 - uVar15;
                    if (uVar15 == 0) {
                      uVar27 = uVar13;
                    }
                    lVar21 = uVar27 + 0x20;
                  }
                  lVar24 = *param_1;
                  uVar15 = uVar13;
                  if ((lVar24 != 0) && (uVar11 = lVar24 - 1U & lVar21 + lVar20, uVar11 != 0)) {
                    uVar15 = lVar24 - uVar11;
                  }
                  uVar15 = uVar15 + lVar21 + lVar20;
                  if (uVar28 < uVar15) {
                    param_1[3] = uVar15;
                    VirtualAlloc(pvVar30,uVar15,0x1000,4);
                  }
                }
                lVar20 = DAT_1401e8b90;
                uVar28 = param_1[5] + uVar27 + param_1[4];
                param_1[4] = param_1[4] + lVar21;
                param_1[6] = uVar28;
                puVar22[10] = uVar28;
                FUN_1401b0ce0(param_1,pHVar3,uVar28,local_134,local_134,0,
                              *(undefined4 *)(lVar20 + 0x94),*(undefined4 *)(lVar20 + 0x98));
                *(undefined1 *)(puVar22[0xe] + 0x62) = 1;
              }
              *(bool *)(puVar22 + 0xc) = local_218.hSubMenu != (HMENU)0x0;
              puVar14 = puVar22;
              if (local_218.hSubMenu != (HMENU)0x0) {
                iVar7 = param_5;
                if (param_5 == 0) {
                  uVar28 = param_1[0xf];
                  iVar7 = 0;
                  if (uVar28 != 0) {
                    do {
                      if (iVar7 != 0) goto LAB_14018f1a4;
                      local_230 = puVar22[5];
                      local_338 = (char *)puVar22[4];
                      uStack_330 = puVar22[5];
                      local_238 = 0;
                      puVar12 = (undefined8 *)(param_1[0x11] + uVar13 * 0x10);
                      local_318 = (char *)*puVar12;
                      uStack_310 = puVar12[1];
                      iVar7 = FUN_1401b87a0(&local_318,&local_338,&local_238);
                      uVar13 = (ulonglong)((int)uVar13 + 1);
                    } while (uVar13 < uVar28);
                    if (iVar7 != 0) goto LAB_14018f1a4;
                  }
                }
                else {
LAB_14018f1a4:
                  FUN_1400c7550(param_1,puVar22,iVar7);
                }
              }
            }
          }
        }
        local_300 = puVar22;
        local_324 = local_324 + 1;
        puVar22 = (ulonglong *)(ulonglong)local_324;
      } while (local_324 < local_304);
    }
    *(undefined1 *)(param_3 + 0x65) = 1;
  }
  return;
}

