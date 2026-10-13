
void FUN_1401a86b0(longlong param_1,longlong param_2,ulonglong param_3)

{
  char *pcVar1;
  char cVar2;
  longlong lVar3;
  undefined8 uVar4;
  undefined8 uVar5;
  undefined8 uVar6;
  bool bVar7;
  bool bVar8;
  int iVar9;
  int iVar10;
  uint uVar11;
  BOOL BVar12;
  longlong lVar13;
  longlong lVar14;
  undefined8 *puVar15;
  longlong *plVar16;
  undefined4 *puVar17;
  ulonglong uVar18;
  undefined8 uVar19;
  short sVar20;
  longlong lVar21;
  undefined8 uVar22;
  longlong lVar24;
  undefined8 uVar25;
  short sVar26;
  short sVar27;
  short sVar28;
  short sVar29;
  ulonglong uVar30;
  LPVOID lpAddress;
  short sVar31;
  longlong *plVar32;
  uint uVar33;
  undefined8 *puVar34;
  undefined4 uVar35;
  undefined4 uVar36;
  undefined4 uVar37;
  undefined4 uVar38;
  int local_res18 [2];
  int local_res20;
  longlong *local_228;
  longlong *local_220;
  SYSTEMTIME local_218;
  longlong *local_208;
  undefined4 local_200;
  undefined4 local_1fc;
  undefined4 local_1f8;
  undefined4 local_1f4;
  undefined4 local_1f0;
  undefined4 local_1e8;
  undefined4 local_1e4;
  undefined4 local_1e0;
  undefined4 local_1dc;
  undefined4 local_1d8;
  longlong local_1d0;
  ulonglong uStack_1c8;
  undefined8 local_1c0;
  undefined8 uStack_1b8;
  undefined8 local_1b0;
  uint local_1a8;
  uint local_1a4;
  _FILETIME local_1a0;
  longlong *local_198;
  undefined4 local_190;
  undefined4 uStack_18c;
  undefined4 local_188;
  undefined4 uStack_184;
  undefined4 local_180;
  undefined4 uStack_17c;
  char *local_178;
  ulonglong uStack_170;
  undefined8 local_168;
  undefined8 uStack_160;
  undefined8 local_158;
  undefined8 uStack_150;
  undefined8 local_148;
  undefined8 uStack_140;
  undefined8 local_138;
  undefined8 uStack_130;
  undefined8 local_128;
  undefined8 uStack_120;
  undefined4 local_118;
  undefined4 uStack_114;
  undefined4 uStack_110;
  undefined4 uStack_10c;
  undefined8 local_108;
  undefined8 local_f8;
  undefined8 uStack_f0;
  undefined1 local_e8 [16];
  undefined1 local_d8 [16];
  undefined1 local_c8 [16];
  undefined1 local_b8 [16];
  undefined1 local_a8 [16];
  undefined1 local_98 [16];
  undefined1 local_88 [72];
  ulonglong uVar23;
  
  bVar7 = true;
  uVar35 = 0;
  uVar36 = 0;
  uVar37 = 0;
  uVar38 = 0;
  bVar8 = true;
  local_res20 = 1;
  local_f8 = 0;
  uStack_f0 = 0;
  local_1a4 = *(uint *)(param_1 + 0x68) & 1;
  plVar32 = (longlong *)((param_3 & 0xffffffff) * 0x140 + *(longlong *)(param_1 + 0xf8));
  if (plVar32[0x12] == 0) {
    lVar13 = FUN_14018c760(1);
    plVar32[0x10] = lVar13;
    plVar32[0x11] = lVar13;
    plVar32[0x12] = 0x100000;
    local_168 = *(undefined8 *)(param_1 + 0x80);
    uStack_160 = *(undefined8 *)(param_1 + 0x88);
    local_158 = *(undefined8 *)(param_1 + 0x90);
    uStack_150 = *(undefined8 *)(param_1 + 0x98);
    local_148 = *(undefined8 *)(param_1 + 0xa0);
    uStack_140 = *(undefined8 *)(param_1 + 0xa8);
    local_138 = *(undefined8 *)(param_1 + 0xb0);
    uStack_130 = *(undefined8 *)(param_1 + 0xb8);
    local_128 = *(undefined8 *)(param_1 + 0xc0);
    uStack_120 = *(undefined8 *)(param_1 + 200);
    local_118 = *(undefined4 *)(param_1 + 0xd0);
    uStack_114 = *(undefined4 *)(param_1 + 0xd4);
    uStack_110 = *(undefined4 *)(param_1 + 0xd8);
    uStack_10c = *(undefined4 *)(param_1 + 0xdc);
    local_108 = *(undefined8 *)(param_1 + 0xe0);
    FUN_1401a76d0(plVar32 + 0x12,&local_168,lVar13);
  }
  local_198 = (longlong *)0x0;
  iVar9 = SHGetKnownFolderItem(&DAT_1401d6bb8,0,0,&DAT_1401d6d18,&local_198);
  if (-1 < iVar9) {
    local_208 = (longlong *)0x0;
    iVar9 = (**(code **)(*local_198 + 0x18))(local_198,0,&DAT_1401d6bc8,&DAT_1401d6d38,&local_208);
    if (-1 < iVar9) {
      local_200 = 0x9b174b33;
      local_1fc = 0x11d240ff;
      local_1f8 = 0xc0007ea2;
      local_1f4 = 0x7108c34f;
      local_1f0 = 2;
      local_1e8 = 0x9b174b33;
      local_1e4 = 0x11d240ff;
      local_1e0 = 0xc0007ea2;
      local_1dc = 0x7108c34f;
      local_1d8 = 3;
      local_220 = (longlong *)0x0;
      iVar10 = (**(code **)(*local_208 + 0x18))(local_208,1,&local_220,0);
      while (iVar10 == 0) {
        local_228 = (longlong *)0x0;
        iVar10 = (**(code **)*local_220)(local_220,&DAT_1401d6d28,&local_228);
        if (-1 < iVar10) {
          if ((bVar7) &&
             (uVar23 = *(ulonglong *)(plVar32[0x10] + 0x40), bVar8 = false,
             (uVar23 >> 2) + (uVar23 >> 1) < *(ulonglong *)(plVar32[0x10] + 0x20))) {
            lVar13 = FUN_14018c760(0);
            *(longlong *)(plVar32[0x10] + 0x88) = lVar13;
            plVar32[0x10] = lVar13;
            LOCK();
            *plVar32 = *plVar32 + 1;
            UNLOCK();
          }
          bVar7 = bVar8;
          plVar16 = (longlong *)plVar32[0x10];
          lVar13 = plVar16[4];
          lpAddress = (LPVOID)plVar16[5];
          uVar23 = plVar16[3];
          uVar30 = (ulonglong)((int)lVar13 + (int)lpAddress & 7);
          lVar24 = 8 - uVar30;
          if (uVar30 == 0) {
            lVar24 = 0;
          }
          lVar21 = lVar24 + 0x78;
          if (uVar23 < (ulonglong)(lVar13 + lVar21)) {
            if ((ulonglong)plVar16[2] < (ulonglong)(lVar13 + lVar21)) {
              FUN_1401b5c50(plVar16,lVar21,8);
              lpAddress = (LPVOID)plVar16[5];
              lVar13 = plVar16[4];
              uVar23 = plVar16[3];
              uVar30 = (ulonglong)((int)lVar13 + (int)lpAddress & 7);
              lVar24 = 8 - uVar30;
              if (uVar30 == 0) {
                lVar24 = 0;
              }
              lVar21 = lVar24 + 0x78;
            }
            lVar14 = 0;
            lVar3 = *plVar16;
            if ((lVar3 != 0) && (uVar30 = lVar3 - 1U & lVar21 + lVar13, uVar30 != 0)) {
              lVar14 = lVar3 - uVar30;
            }
            uVar30 = lVar14 + lVar21 + lVar13;
            if (uVar23 < uVar30) {
              plVar16[3] = uVar30;
              VirtualAlloc(lpAddress,uVar30,0x1000,4);
            }
          }
          puVar34 = (undefined8 *)(plVar16[5] + plVar16[4] + lVar24);
          plVar16[6] = (longlong)puVar34;
          plVar16[4] = plVar16[4] + lVar21;
          lVar13 = plVar32[0x10];
          if (*(longlong *)(lVar13 + 0x60) == 0) {
            *(undefined8 **)(lVar13 + 0x68) = puVar34;
            *(undefined8 **)(plVar32[0x10] + 0x60) = puVar34;
          }
          else {
            *(undefined8 **)(*(longlong *)(lVar13 + 0x68) + 0x68) = puVar34;
            *(undefined8 **)(plVar32[0x10] + 0x68) = puVar34;
          }
          uVar22 = 0;
          puVar34[0xd] = 0;
          local_res18[0] = 0;
          puVar15 = (undefined8 *)
                    FUN_140197060(local_e8,plVar32[0x10],local_220,0x80080001,local_res18);
          uVar4 = *puVar15;
          uVar5 = puVar15[1];
          plVar16 = (longlong *)FUN_1401971a0(local_d8,plVar32 + 0x12,local_228,&DAT_1401d7000);
          uVar33 = 0x40;
          local_1d0 = *plVar16;
          uStack_1c8 = plVar16[1];
          iVar9 = (**(code **)(*local_220 + 0x30))(local_220,0x20091000,&local_1a8);
          if (-1 < iVar9) {
            uVar33 = local_1a8 >> 0x1c & 2;
            uVar11 = uVar33 | 0x40;
            if (((local_1a8 >> 0x13 & 1) != 0) && ((local_1a8 & 0x1000) == 0)) {
              uVar11 = uVar33 | 0x44;
            }
            uVar33 = uVar11 | 8;
            if ((local_1a8 & 0x1000) == 0) {
              uVar33 = uVar11;
            }
            if ((local_1a8 >> 0x10 & 1) != 0) {
              uVar33 = uVar33 | 0x10;
            }
          }
          if (local_res18[0] != 0) {
            uVar33 = uVar33 | 0x200;
          }
          uVar33 = uVar33 | *(uint *)(param_2 + 0x24);
          if (local_res20 != 0) {
            if (local_1a4 != 0) {
              puVar17 = (undefined4 *)
                        FUN_1401b8530(local_c8,plVar32[0x10],*(undefined8 *)(param_2 + 0x10),
                                      *(undefined8 *)(param_2 + 0x18));
              uVar35 = *puVar17;
              uVar36 = puVar17[1];
              uVar37 = puVar17[2];
              uVar38 = puVar17[3];
            }
            local_res20 = 0;
          }
          local_1b0 = 0;
          local_1c0 = 0;
          uStack_1b8 = 0;
          iVar10 = (**(code **)(*local_228 + 0x68))(local_228,&DAT_1401d6e38,&local_1c0);
          uVar6 = uStack_1b8;
          uVar19 = uVar22;
          if (-1 < iVar10) {
            PropVariantClear((PROPVARIANT *)&local_1c0);
            uVar19 = uVar6;
          }
          iVar10 = (**(code **)(*local_228 + 0x68))(local_228,&DAT_1401d6e50,&local_1c0);
          uVar6 = uStack_1b8;
          uVar25 = uVar22;
          if (-1 < iVar10) {
            PropVariantClear((PROPVARIANT *)&local_1c0);
            uVar25 = uVar6;
          }
          iVar10 = (**(code **)(*local_228 + 0x68))(local_228,&DAT_1401d6e20,&local_1c0);
          uVar6 = uStack_1b8;
          if (-1 < iVar10) {
            PropVariantClear((PROPVARIANT *)&local_1c0);
            uVar22 = uVar6;
          }
          *(undefined4 *)((longlong)puVar34 + 0x24) = 0;
          puVar34[5] = 0;
          local_180 = (undefined4)uVar22;
          local_190 = (undefined4)uVar19;
          local_188 = (undefined4)uVar25;
          uStack_17c = (undefined4)((ulonglong)uVar22 >> 0x20);
          uVar23 = 0;
          sVar20 = 0;
          uStack_18c = (undefined4)((ulonglong)uVar19 >> 0x20);
          lVar13 = 0;
          uVar30 = 0;
          puVar34[6] = uVar19;
          uStack_184 = (undefined4)((ulonglong)uVar25 >> 0x20);
          puVar34[7] = uVar25;
          puVar34[8] = uVar22;
          *(uint *)(puVar34 + 4) = uVar33;
          *puVar34 = uVar4;
          puVar34[1] = uVar5;
          *(undefined4 *)(puVar34 + 2) = uVar35;
          *(undefined4 *)((longlong)puVar34 + 0x14) = uVar36;
          *(undefined4 *)(puVar34 + 3) = uVar37;
          *(undefined4 *)((longlong)puVar34 + 0x1c) = uVar38;
          if (uStack_1c8 != 0) {
            do {
              pcVar1 = (char *)(local_1d0 + uVar30);
              if (9 < (byte)(*pcVar1 - 0x30U)) break;
              uVar30 = uVar30 + 1;
              lVar13 = (longlong)(*pcVar1 + -0x30) + lVar13 * 10;
            } while (uVar30 < uStack_1c8);
          }
          puVar34[5] = lVar13;
          iVar10 = (**(code **)(*local_228 + 0x68))(local_228,&DAT_1401d6fb8,&local_1c0);
          if (-1 < iVar10) {
            puVar15 = (undefined8 *)FUN_1401b8680(local_b8,plVar32[0x10],uStack_1b8);
            uVar4 = puVar15[1];
            puVar34[9] = *puVar15;
            puVar34[10] = uVar4;
            PropVariantClear((PROPVARIANT *)&local_1c0);
          }
          puVar15 = (undefined8 *)FUN_1401971a0(local_a8,plVar32[0x10],local_228,&local_200);
          uVar4 = *puVar15;
          uVar5 = puVar15[1];
          puVar15 = (undefined8 *)FUN_140197060(local_98,plVar32[0x10],local_220,0x80058000,0);
          uVar22 = *puVar15;
          uVar6 = puVar15[1];
          puVar15 = (undefined8 *)FUN_1401971a0(local_88,plVar32[0x10],local_228,&local_1e8);
          lVar13 = 1;
          sVar26 = 1;
          local_178 = (char *)*puVar15;
          uStack_170 = puVar15[1];
          uVar18 = uVar23;
          uVar30 = uVar23;
          if (*local_178 == '-') {
            sVar26 = -1;
            uVar18 = 1;
          }
          do {
            sVar29 = (short)uVar30;
            pcVar1 = local_178 + uVar18;
            if (9 < (byte)(*pcVar1 - 0x30U)) break;
            uVar18 = uVar18 + 1;
            uVar30 = (longlong)(*pcVar1 + -0x30) + uVar30 * 10;
            sVar29 = (short)uVar30;
          } while (uVar18 < 4);
          sVar27 = 1;
          uVar18 = uVar23;
          uVar30 = uVar23;
          if (local_178[5] == '-') {
            sVar27 = -1;
            uVar18 = 1;
          }
          do {
            sVar31 = (short)uVar30;
            cVar2 = (local_178 + 5)[uVar18];
            if (9 < (byte)(cVar2 - 0x30U)) break;
            uVar18 = uVar18 + 1;
            uVar30 = (longlong)(cVar2 + -0x30) + uVar30 * 10;
            sVar31 = (short)uVar30;
          } while (uVar18 < 2);
          sVar28 = 1;
          local_218.wMonth = sVar31 * sVar27;
          local_218.wYear = sVar29 * sVar26;
          uVar18 = uVar23;
          uVar30 = uVar23;
          if (local_178[8] == '-') {
            sVar28 = -1;
            uVar18 = 1;
          }
          do {
            sVar26 = (short)uVar30;
            cVar2 = (local_178 + 8)[uVar18];
            if (9 < (byte)(cVar2 - 0x30U)) break;
            uVar18 = uVar18 + 1;
            uVar30 = (longlong)(cVar2 + -0x30) + uVar30 * 10;
            sVar26 = (short)uVar30;
          } while (uVar18 < 2);
          sVar29 = 1;
          local_218.wDayOfWeek = 0;
          local_218.wDay = sVar26 * sVar28;
          uVar18 = uVar23;
          uVar30 = uVar23;
          if (local_178[0xb] == '-') {
            sVar29 = -1;
            uVar18 = 1;
          }
          do {
            sVar26 = (short)uVar30;
            cVar2 = (local_178 + 0xb)[uVar18];
            if (9 < (byte)(cVar2 - 0x30U)) break;
            uVar18 = uVar18 + 1;
            uVar30 = (longlong)(cVar2 + -0x30) + uVar30 * 10;
            sVar26 = (short)uVar30;
          } while (uVar18 < 2);
          sVar27 = 1;
          uVar18 = uVar23;
          uVar30 = uVar23;
          if (local_178[0xe] == '-') {
            sVar27 = -1;
            uVar18 = 1;
          }
          do {
            sVar31 = (short)uVar30;
            cVar2 = (local_178 + 0xe)[uVar18];
            if (9 < (byte)(cVar2 - 0x30U)) break;
            uVar18 = uVar18 + 1;
            uVar30 = (longlong)(cVar2 + -0x30) + uVar30 * 10;
            sVar31 = (short)uVar30;
          } while (uVar18 < 2);
          lVar24 = 1;
          local_218.wMinute = sVar31 * sVar27;
          local_218.wHour = sVar26 * sVar29;
          uVar18 = uVar23;
          uVar30 = uVar23;
          if (local_178[0x11] == '-') {
            lVar24 = -1;
            uVar18 = 1;
          }
          do {
            sVar26 = (short)uVar30;
            cVar2 = (local_178 + 0x11)[uVar18];
            if (9 < (byte)(cVar2 - 0x30U)) break;
            uVar18 = uVar18 + 1;
            uVar30 = (longlong)(cVar2 + -0x30) + uVar30 * 10;
            sVar26 = (short)uVar30;
            local_1d0 = lVar24;
          } while (uVar18 < 2);
          local_218.wSecond = sVar26 * (short)lVar24;
          uVar30 = uStack_170;
          if ((longlong)uStack_170 < 0) {
            uVar30 = (uStack_170 ^ (longlong)uStack_170 >> 0x3f) - ((longlong)uStack_170 >> 0x3f);
            if (uVar30 < uStack_170) {
              uVar30 = uStack_170 - uVar30;
              goto LAB_1401a8f2a;
            }
          }
          else {
LAB_1401a8f2a:
            lVar13 = 1;
            uVar18 = uVar30 - 0x14;
            if (uVar30 < 0x15) {
              uVar18 = uVar23;
            }
            if (uVar18 != 0) {
              lVar13 = -1;
              uVar30 = (ulonglong)(local_178[0x14] == '-');
              if (local_178[0x14] != '-') {
                lVar13 = 1;
              }
              for (; uVar30 < uVar18; uVar30 = uVar30 + 1) {
                sVar20 = (short)uVar23;
                if (9 < (byte)(local_178[uVar30 + 0x14] - 0x30U)) break;
                uVar23 = (longlong)(local_178[uVar30 + 0x14] + -0x30) + uVar23 * 10;
                sVar20 = (short)uVar23;
                local_1d0 = lVar13;
              }
            }
          }
          local_218.wMilliseconds = sVar20 * (short)lVar13;
          lVar13 = FUN_1401b7a30(plVar32[0x10],puVar34,2);
          *(undefined8 *)(lVar13 + 0x10) = uVar4;
          *(undefined8 *)(lVar13 + 0x18) = uVar5;
          *(undefined8 *)(lVar13 + 0x20) = uVar22;
          *(undefined8 *)(lVar13 + 0x28) = uVar6;
          local_1a0.dwLowDateTime = 0;
          local_1a0.dwHighDateTime = 0;
          BVar12 = SystemTimeToFileTime(&local_218,&local_1a0);
          if (BVar12 != 0) {
            *(_FILETIME *)(lVar13 + 0x30) = local_1a0;
          }
          (**(code **)(*local_228 + 0x10))();
          bVar8 = bVar7;
          if ((uVar33 & 2) == 0) {
            plVar16 = (longlong *)(plVar32[0x10] + 0x70);
            *plVar16 = *plVar16 + 1;
          }
          else {
            plVar16 = (longlong *)(plVar32[0x10] + 0x78);
            *plVar16 = *plVar16 + 1;
          }
        }
        (**(code **)(*local_220 + 0x10))();
        iVar10 = (**(code **)(*local_208 + 0x18))(local_208,1,&local_220,0);
      }
    }
  }
  FUN_1401b4010(iVar9,"Win_LoadRecycleBinProc");
  if (local_208 != (longlong *)0x0) {
    (**(code **)(*local_208 + 0x10))();
  }
  local_208 = (longlong *)0x0;
  if (local_198 != (longlong *)0x0) {
    (**(code **)(*local_198 + 0x10))();
  }
  return;
}

