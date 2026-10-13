
void FUN_140169320(longlong param_1,ulonglong param_2)

{
  longlong *plVar1;
  int iVar2;
  undefined8 *puVar3;
  undefined1 auVar4 [16];
  undefined8 uVar5;
  undefined8 uVar6;
  undefined8 uVar7;
  undefined8 uVar8;
  undefined8 uVar9;
  undefined8 uVar10;
  bool bVar11;
  uint uVar12;
  undefined8 *puVar13;
  ulonglong uVar14;
  longlong lVar15;
  longlong lVar16;
  ulonglong uVar17;
  ulonglong **ppuVar18;
  undefined4 *puVar19;
  undefined8 *puVar20;
  undefined8 uVar21;
  ulonglong uVar22;
  ulonglong *puVar23;
  undefined8 *puVar24;
  ulonglong *puVar25;
  ulonglong *puVar26;
  ulonglong uVar27;
  LPVOID lpAddress;
  undefined2 uVar28;
  longlong lVar29;
  ulonglong *puVar30;
  undefined4 uVar31;
  undefined4 uVar32;
  undefined4 uVar33;
  undefined4 uVar34;
  undefined8 in_stack_fffffffffffffe18;
  ulonglong *local_1c8 [2];
  undefined8 local_1b8;
  undefined8 uStack_1b0;
  ulonglong local_1a8;
  ulonglong *puStack_1a0;
  ulonglong local_198;
  code *pcStack_190;
  undefined8 local_188;
  undefined8 *puStack_180;
  ulonglong local_178;
  undefined8 uStack_170;
  uint local_168;
  longlong local_160;
  longlong local_158;
  longlong local_148;
  ulonglong local_138;
  ulonglong uStack_130;
  longlong local_128;
  ulonglong **local_118;
  undefined8 local_110;
  undefined8 local_108;
  undefined8 uStack_100;
  undefined8 local_f8;
  undefined8 uStack_f0;
  undefined8 local_e8;
  undefined8 uStack_e0;
  undefined8 local_d8;
  undefined8 uStack_d0;
  undefined1 local_c8 [16];
  undefined1 local_b8 [16];
  undefined1 local_a8 [104];
  
  uVar31 = (undefined4)((ulonglong)in_stack_fffffffffffffe18 >> 0x20);
  puVar13 = (undefined8 *)FUN_1400c6b20(&local_f8,0,0);
  lVar15 = 0;
  local_198 = 0;
  pcStack_190 = (code *)0x0;
  uVar5 = *puVar13;
  uVar6 = puVar13[1];
  local_188 = 0;
  uVar7 = puVar13[2];
  uVar8 = puVar13[3];
  puStack_180 = (undefined8 *)0x0;
  uVar9 = puVar13[4];
  uVar10 = puVar13[5];
  uVar14 = 1;
  local_178 = 0;
  uStack_170 = (ulonglong)(uint)uStack_170;
  do {
    uVar14 = uVar14 * 2;
  } while (uVar14 < 0x120);
  puStack_1a0 = (ulonglong *)uVar14;
  if (uVar14 < 0x100000) {
    puStack_1a0 = (ulonglong *)0x100000;
  }
  uVar17 = 0x10000;
  if (puStack_1a0 < 0x10000) {
    uVar17 = (ulonglong)puStack_1a0;
  }
  uVar27 = (ulonglong)puStack_1a0;
  if (puStack_1a0 < uVar14) {
    uVar27 = uVar14;
  }
  lVar29 = lVar15;
  if ((uVar17 != 0) && (uVar12 = (int)uVar17 - 1U & 0x28, uVar12 != 0)) {
    lVar29 = uVar17 - uVar12;
  }
  uVar14 = lVar29 + 0x28;
  local_1a8 = uVar17;
  if (uVar27 == uVar14) {
    puVar13 = VirtualAlloc((LPVOID)0x0,uVar27,0x3000,4);
    uStack_170 = CONCAT44(uStack_170._4_4_,1);
    local_198 = uVar27;
    pcStack_190 = (code *)uVar14;
    puStack_180 = puVar13;
  }
  else {
    puVar13 = VirtualAlloc((LPVOID)0x0,uVar27,0x2000,4);
    uStack_170 = CONCAT44(uStack_170._4_4_,1);
    local_198 = uVar27;
    pcStack_190 = (code *)uVar14;
    puStack_180 = puVar13;
    VirtualAlloc(puVar13,uVar14,0x1000,4);
  }
  uVar27 = (ulonglong)((uint)puVar13 & 7);
  *puVar13 = 0;
  puVar13[1] = 0;
  puVar13[2] = 0;
  puVar13[3] = 0;
  lVar29 = 8 - uVar27;
  if (uVar27 == 0) {
    lVar29 = lVar15;
  }
  puVar13[4] = local_178;
  uVar27 = lVar29 + 0x120;
  if ((uVar17 != 0) && (uVar22 = uVar17 - 1 & uVar27, uVar22 != 0)) {
    lVar15 = uVar17 - uVar22;
  }
  uVar17 = lVar15 + uVar27;
  if (uVar14 < uVar17) {
    pcStack_190 = (code *)uVar17;
    VirtualAlloc(puVar13,uVar17,0x1000,4);
  }
  puVar25 = (ulonglong *)((longlong)puVar13 + lVar29 + 0x28);
  FUN_1401bace0(lVar29 + 0x68 + (longlong)puVar13,0,0xb8);
  *puVar25 = local_1a8;
  puVar25[1] = (ulonglong)puStack_1a0;
  puVar25[2] = local_198;
  puVar25[3] = (ulonglong)pcStack_190;
  puVar25[4] = uVar27;
  puVar25[5] = (ulonglong)puStack_180;
  puVar25[6] = (ulonglong)puVar25;
  *(uint *)(puVar25 + 7) = (uint)uStack_170;
  *(undefined4 *)((longlong)puVar25 + 0x3c) = uStack_170._4_4_;
  *(ulonglong **)(param_1 + 0xcf8) = puVar25;
  puVar25[8] = param_2;
  puVar25[9] = 0;
  puVar25[10] = 0;
  puVar25[0x13] = (ulonglong)puVar25;
  puVar25[0xb] = 0;
  puVar25[0xc] = 0;
  puVar25[0xd] = 0;
  puVar25[0xe] = 0;
  puVar25[0xf] = 0;
  puVar25[0x10] = 0;
  local_1a8 = 0;
  puStack_1a0 = (ulonglong *)0x0;
  local_198 = 0;
  pcStack_190 = (code *)0x0;
  local_188 = 0;
  puStack_180 = (undefined8 *)0x0;
  local_178 = 0;
  uStack_170 = 0;
  uVar14 = FUN_1401b6980(puVar25,puVar25 + 9,puVar25[0x10],0x10,
                         *(longlong *)(param_2 + 0x2e8) + puVar25[9]);
  puVar25[0x10] = uVar14;
  lVar15 = *(longlong *)(param_2 + 0x90);
  iVar2 = *(int *)(*(longlong *)(DAT_1401e88b8 + 0x48) + 0x3c0);
  local_168 = (uint)(*(longlong *)(lVar15 + 0x170) == 0);
  lVar29 = 0x88;
  if (*(longlong *)(*(longlong *)(param_2 + 0x98) + 0x10) == 0) {
    lVar29 = 0x118;
  }
  plVar1 = (longlong *)(lVar29 + *(longlong *)(param_2 + 0x98));
  local_108 = *(undefined8 *)(lVar15 + 0x168);
  uStack_100 = *(undefined8 *)(lVar15 + 0x170);
  lVar29 = plVar1[1] + *plVar1;
  LOCK();
  if (DAT_1401e8bb8 == 0) {
    DAT_1401e8bb8 = 0;
  }
  UNLOCK();
  puVar13 = &local_108;
  local_158 = lVar29;
  FUN_1400c3130("C:\\Users\\Vjekoslav\\Projects\\Voidstar\\File Pilot\\Source\\Program\\Program.c",
                "Rename_Start",0xbe49,
                "%lu - Rename_Start in \'%S\', expanded: %lu, selection: %llu, dir count: %llu\n",
                DAT_1401e8bb8,puVar13,CONCAT44(uVar31,*(undefined4 *)(lVar15 + 0x200)),
                *(undefined8 *)(param_2 + 0x2e8),*(undefined8 *)(lVar15 + 0x248));
  lVar15 = *(longlong *)(param_2 + 0x98);
  lVar16 = 0x60;
  if (*(longlong *)(lVar15 + 0x10) == 0) {
    lVar16 = 0xf0;
  }
  local_128 = *(longlong *)(lVar16 + 0x10 + lVar15);
  local_138 = *(ulonglong *)(lVar16 + lVar15);
  uStack_130 = ((ulonglong *)(lVar16 + lVar15))[1];
  if (lVar29 != 0) {
    local_160 = 0;
    local_148 = local_128;
    do {
      lVar15 = *(longlong *)(param_2 + 800);
      puVar30 = *(ulonglong **)(local_160 + local_148);
      local_1c8[0] = puVar30;
      if (lVar15 == 0) {
        local_1b8 = 0;
        uStack_1b0 = 0;
        local_158 = lVar29;
      }
      else {
        uVar17 = FUN_14016d7b0(local_1c8,8);
        uVar14 = uVar17 + 2;
        if (1 < uVar17) {
          uVar14 = uVar17;
        }
        uVar17 = *(longlong *)(param_2 + 0x2f0) * 2 - 1;
        local_1b8 = uVar14 & uVar17;
        puVar23 = (ulonglong *)0x0;
        uVar27 = local_1b8;
        do {
          puVar26 = (ulonglong *)(uVar27 * 0x10 + *(longlong *)(param_2 + 0x318));
          uVar22 = *puVar26;
          if (uVar22 == uVar14) {
            ppuVar18 = local_1c8;
            lVar29 = 8;
            do {
              lVar29 = lVar29 + -1;
              if (*(char *)ppuVar18 !=
                  *(char *)((longlong)ppuVar18 + (puVar26[1] * 0x10 - (longlong)local_1c8) + lVar15)
                 ) goto LAB_14016976c;
              ppuVar18 = (ulonglong **)((longlong)ppuVar18 + 1);
            } while (lVar29 != 0);
          }
          else {
LAB_14016976c:
            puVar26 = puVar23;
            if (uVar22 == 0) break;
          }
          uVar27 = uVar27 + 1 & uVar17;
          if (puVar26 != (ulonglong *)0x0) goto LAB_14016978f;
          puVar23 = puVar26;
        } while (uVar27 != local_1b8);
        if (puVar26 == (ulonglong *)0x0) {
          bVar11 = false;
        }
        else {
LAB_14016978f:
          local_1b8 = puVar26[1];
          uStack_130 = CONCAT44(uStack_130._4_4_,1);
          uStack_1b0 = uStack_130;
          *(ulonglong *)(param_2 + 0x2f8) = local_1b8;
          bVar11 = true;
          local_138 = local_1b8;
        }
        if (bVar11) {
          uVar12 = (uint)puVar30[4];
          uVar31 = (undefined4)*puVar30;
          uVar32 = *(undefined4 *)((longlong)puVar30 + 4);
          uVar33 = (undefined4)puVar30[1];
          uVar34 = *(undefined4 *)((longlong)puVar30 + 0xc);
          if ((char)uVar12 < '\0') {
            puVar3 = (undefined8 *)puVar30[0xe];
            puVar20 = (undefined8 *)0x0;
            while ((puVar3 != (undefined8 *)0x0 && (puVar20 == (undefined8 *)0x0))) {
              if (*(int *)(puVar3 + 1) == 1) {
                puVar20 = puVar3;
              }
              puVar3 = (undefined8 *)*puVar3;
            }
            uVar31 = *(undefined4 *)(puVar20 + 3);
            uVar32 = *(undefined4 *)((longlong)puVar20 + 0x1c);
            uVar33 = *(undefined4 *)(puVar20 + 4);
            uVar34 = *(undefined4 *)((longlong)puVar20 + 0x24);
          }
          else if (((iVar2 == 0) && ((uVar12 & 2) == 0)) || ((uVar12 & 0x20) != 0)) {
            local_1b8 = *puVar30;
            uStack_1b0 = puVar30[1];
            puVar19 = (undefined4 *)FUN_14016da00(local_c8,&local_1b8,0);
            uVar31 = *puVar19;
            uVar32 = puVar19[1];
            uVar33 = puVar19[2];
            uVar34 = puVar19[3];
            puVar30 = local_1c8[0];
          }
          lVar15 = 0;
          auVar4._4_4_ = uVar32;
          auVar4._0_4_ = uVar31;
          auVar4._8_4_ = uVar33;
          auVar4._12_4_ = uVar34;
          uVar28 = (undefined2)uVar33;
          if ((iVar2 != 0) && ((puVar30[4] & 0xa2) == 0)) {
            local_1b8 = *puVar30;
            uStack_1b0 = puVar30[1];
            lVar29 = FUN_14016da00(local_b8,&local_1b8,0);
            uVar28 = (undefined2)*(undefined4 *)(lVar29 + 8);
          }
          uVar14 = puVar25[4];
          lpAddress = (LPVOID)puVar25[5];
          uVar17 = puVar25[3];
          uVar27 = (ulonglong)((int)uVar14 + (int)lpAddress & 7);
          lVar29 = 8 - uVar27;
          if (uVar27 == 0) {
            lVar29 = lVar15;
          }
          lVar16 = lVar29 + 0x78;
          if (uVar17 < lVar16 + uVar14) {
            if (puVar25[2] < lVar16 + uVar14) {
              FUN_1401b5c50(puVar25,lVar16,8);
              lpAddress = (LPVOID)puVar25[5];
              uVar14 = puVar25[4];
              uVar17 = puVar25[3];
              uVar27 = (ulonglong)((int)uVar14 + (int)lpAddress & 7);
              lVar29 = 8 - uVar27;
              if (uVar27 == 0) {
                lVar29 = lVar15;
              }
              lVar16 = lVar29 + 0x78;
            }
            uVar27 = *puVar25;
            if ((uVar27 != 0) && (uVar22 = uVar27 - 1 & uVar14 + lVar16, uVar22 != 0)) {
              lVar15 = uVar27 - uVar22;
            }
            uVar14 = lVar15 + uVar14 + lVar16;
            if (uVar17 < uVar14) {
              puVar25[3] = uVar14;
              VirtualAlloc(lpAddress,uVar14,0x1000,4);
            }
          }
          puVar24 = (undefined8 *)(puVar25[5] + puVar25[4] + lVar29);
          *puVar24 = 0;
          puVar24[1] = 0;
          puVar24[2] = 0;
          puVar24[3] = 0;
          puVar24[4] = 0;
          puVar24[5] = 0;
          puVar24[6] = 0;
          puVar24[7] = 0;
          puVar24[8] = 0;
          puVar24[9] = 0;
          puVar24[10] = 0;
          puVar24[0xb] = 0;
          puVar24[0xc] = 0;
          puVar24[0xd] = 0;
          puVar24[0xe] = 0;
          puVar25[4] = puVar25[4] + lVar16;
          puVar25[6] = (ulonglong)puVar24;
          puVar20 = (undefined8 *)
                    FUN_1401b8530(local_a8,puVar25,CONCAT44(uVar32,uVar31),auVar4._8_8_);
          puVar3 = puVar24 + 2;
          uVar21 = puVar20[1];
          *puVar24 = *puVar20;
          puVar24[1] = uVar21;
          uVar21 = FUN_1401786d0(*(undefined8 *)(param_2 + 0x4d8),"Rename%llu",local_1c8[0]);
          puVar30 = puVar25 + 0x13;
          *puVar3 = uVar21;
          FUN_14017e400(puVar30,puVar3,0x100);
          local_1b8 = CONCAT44(uVar32,uVar31);
          uStack_1b0 = CONCAT44(uVar34,uVar33);
          FUN_14017dab0(puVar30,puVar3,&local_1b8,0);
          FUN_14017df60(puVar30,puVar3);
          puVar20 = (undefined8 *)0x0;
          *(undefined2 *)(puVar24 + 6) = uVar28;
          if (puVar25[0x11] == 0) {
            puVar25[0x12] = (ulonglong)puVar3;
            puVar25[0x11] = (ulonglong)puVar3;
            puVar24[0xd] = 0;
          }
          else {
            *(undefined8 **)(puVar25[0x12] + 0x60) = puVar3;
            puVar24[0xd] = puVar25[0x12];
            puVar25[0x12] = (ulonglong)puVar3;
          }
          puVar24[0xe] = 0;
          if (local_168 != 0) {
            puVar3 = (undefined8 *)local_1c8[0][0xe];
            while ((puVar3 != (undefined8 *)0x0 && (puVar20 == (undefined8 *)0x0))) {
              if (*(int *)(puVar3 + 1) == 1) {
                puVar20 = puVar3;
              }
              puVar3 = (undefined8 *)*puVar3;
            }
            *(undefined4 *)(puVar24 + 0xb) = *(undefined4 *)(puVar20 + 0xc);
          }
          uVar21 = FUN_1401b6980(puVar25,puVar25 + 9,puVar25[0x10],0x10,0x10);
          uVar17 = FUN_14016d7b0(local_1c8,8);
          local_110 = 8;
          local_118 = local_1c8;
          uVar14 = uVar17 + 2;
          if (1 < uVar17) {
            uVar14 = uVar17;
          }
          puVar13 = (undefined8 *)((ulonglong)puVar13 & 0xffffffff00000000);
          uVar14 = FUN_1401b67b0(puVar25,puVar25 + 9,uVar21,0x10,&local_118,puVar13,uVar14);
          *(ulonglong **)(uVar14 + puVar25[0xb] * 0x10) = local_1c8[0];
          puVar25[0x10] = uVar14;
          *(undefined8 **)(uVar14 + 8 + puVar25[0xb] * 0x10) = puVar24;
        }
      }
      local_160 = local_160 + 0x18;
      lVar29 = local_158 + -1;
      local_158 = lVar29;
    } while (lVar29 != 0);
    local_158 = 0;
  }
  puStack_1a0 = puVar25 + 0x13;
  *(ulonglong **)(param_1 + 0xcf8) = puVar25;
  local_198 = puVar25[0x11];
  pcStack_190 = FUN_140169c00;
  lVar15 = (ulonglong)*(uint *)(param_1 + 0xa18) * 0x38;
  local_1a8 = 0x100000030;
  puVar13 = (undefined8 *)(lVar15 + 0xa20 + param_1);
  *puVar13 = 0x100000030;
  puVar13[1] = puStack_1a0;
  local_188 = 0;
  puVar25 = (ulonglong *)(lVar15 + 0xa30 + param_1);
  *puVar25 = local_198;
  puVar25[1] = (ulonglong)FUN_140169c00;
  puStack_180 = (undefined8 *)0x0;
  puVar13 = (undefined8 *)(lVar15 + 0xa40 + param_1);
  *puVar13 = 0;
  puVar13[1] = 0;
  *(ulonglong *)(lVar15 + 0xa50 + param_1) = param_2;
  *(int *)(param_1 + 0xa18) = *(int *)(param_1 + 0xa18) + 1;
  local_178 = param_2;
  local_f8 = uVar5;
  uStack_f0 = uVar6;
  local_e8 = uVar7;
  uStack_e0 = uVar8;
  local_d8 = uVar9;
  uStack_d0 = uVar10;
  FUN_140001a90(&local_f8);
  return;
}

