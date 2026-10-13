
HGLOBAL FUN_14018c3e0(ulonglong *param_1)

{
  undefined4 *puVar1;
  longlong *plVar2;
  undefined8 *puVar3;
  ulonglong uVar4;
  longlong lVar5;
  ulonglong uVar6;
  HGLOBAL hMem;
  longlong lVar7;
  ulonglong uVar8;
  longlong lVar9;
  longlong *plVar10;
  undefined4 *puVar11;
  ulonglong uVar12;
  LPVOID lpAddress;
  longlong lVar13;
  longlong lVar14;
  longlong lVar15;
  longlong *local_e8;
  ulonglong uStack_e0;
  undefined8 local_d8;
  undefined4 uStack_d0;
  undefined4 uStack_cc;
  undefined4 local_c8;
  undefined4 uStack_c4;
  undefined4 uStack_c0;
  undefined4 uStack_bc;
  undefined8 local_b8;
  undefined8 uStack_b0;
  undefined4 local_a8;
  undefined4 uStack_a4;
  undefined4 uStack_a0;
  undefined4 uStack_9c;
  longlong *local_98;
  undefined8 uStack_90;
  undefined8 local_88;
  undefined8 uStack_80;
  undefined4 local_78;
  undefined4 uStack_74;
  undefined4 uStack_70;
  undefined4 uStack_6c;
  undefined1 local_68 [16];
  undefined1 local_58 [48];
  
  puVar3 = (undefined8 *)FUN_1400c6b20(local_58,0,0);
  uVar12 = *param_1;
  uVar8 = 1;
  local_88 = puVar3[2];
  uStack_80 = puVar3[3];
  plVar10 = (longlong *)*puVar3;
  uStack_90 = puVar3[1];
  local_78 = *(undefined4 *)(puVar3 + 4);
  uStack_74 = *(undefined4 *)((longlong)puVar3 + 0x24);
  uStack_70 = *(undefined4 *)(puVar3 + 5);
  uStack_6c = *(undefined4 *)((longlong)puVar3 + 0x2c);
  if (1 < uVar12) {
    do {
      uVar8 = uVar8 * 2;
    } while (uVar8 < uVar12);
  }
  lVar13 = 0;
  lVar15 = plVar10[4];
  lpAddress = (LPVOID)plVar10[5];
  uVar6 = plVar10[3];
  uVar4 = (ulonglong)((int)lVar15 + (int)lpAddress & 7);
  lVar9 = 8 - uVar4;
  if (uVar4 == 0) {
    lVar9 = lVar13;
  }
  lVar14 = lVar9 + uVar8 * 0x10;
  local_98 = plVar10;
  if (uVar6 < (ulonglong)(lVar15 + lVar14)) {
    if ((ulonglong)plVar10[2] < (ulonglong)(lVar15 + lVar14)) {
      FUN_1401b5c50(plVar10,lVar14,8);
      lpAddress = (LPVOID)plVar10[5];
      lVar15 = plVar10[4];
      uVar6 = plVar10[3];
      uVar4 = (ulonglong)((int)lVar15 + (int)lpAddress & 7);
      lVar9 = 8 - uVar4;
      if (uVar4 == 0) {
        lVar9 = lVar13;
      }
      lVar14 = lVar9 + uVar8 * 0x10;
    }
    lVar7 = *plVar10;
    lVar5 = lVar13;
    if ((lVar7 != 0) && (uVar8 = lVar7 - 1U & lVar15 + lVar14, uVar8 != 0)) {
      lVar5 = lVar7 - uVar8;
    }
    uVar8 = lVar5 + lVar15 + lVar14;
    if (uVar6 < uVar8) {
      plVar10[3] = uVar8;
      VirtualAlloc(lpAddress,uVar8,0x1000,4);
    }
  }
  lVar15 = 0x14;
  lVar9 = plVar10[5] + plVar10[4] + lVar9;
  plVar10[4] = plVar10[4] + lVar14;
  plVar10[6] = lVar9;
  if (uVar12 != 0) {
    puVar11 = (undefined4 *)param_1[2];
    lVar14 = lVar9 - (longlong)puVar11;
    uVar8 = uVar12;
    do {
      local_e8 = (longlong *)0x0;
      uStack_e0 = 0;
      local_a8 = *puVar11;
      uStack_a4 = puVar11[1];
      uStack_a0 = puVar11[2];
      uStack_9c = puVar11[3];
      local_d8 = 0;
      FUN_140187f00(local_68,plVar10,&local_a8,&local_e8);
      FUN_1401b9a40(plVar10,&local_e8,1);
      puVar11 = puVar11 + 4;
      local_b8 = local_e8;
      plVar2 = local_b8;
      *(undefined2 *)((longlong)local_e8 + uStack_e0 * 2) = 0;
      uVar6 = uStack_e0 + 1;
      if (uVar6 < 2) {
        uStack_e0 = 0;
      }
      lVar7 = uStack_e0;
      if (uStack_e0 == 0) {
        lVar7 = lVar13;
      }
      local_b8._0_4_ = SUB84(local_e8,0);
      local_b8._4_4_ = (undefined4)((ulonglong)local_e8 >> 0x20);
      uStack_b0._0_4_ = (undefined4)lVar7;
      uStack_b0._4_4_ = (undefined4)((ulonglong)lVar7 >> 0x20);
      puVar1 = (undefined4 *)(lVar14 + -0x10 + (longlong)puVar11);
      *puVar1 = (undefined4)local_b8;
      puVar1[1] = local_b8._4_4_;
      puVar1[2] = (undefined4)uStack_b0;
      puVar1[3] = uStack_b0._4_4_;
      lVar15 = lVar15 + *(longlong *)(lVar14 + -8 + (longlong)puVar11) * 2 + 2;
      uVar8 = uVar8 - 1;
      uStack_e0 = uVar6;
      local_b8 = plVar2;
      uStack_b0 = lVar7;
    } while (uVar8 != 0);
  }
  hMem = GlobalAlloc(0x42,lVar15 + 2);
  if (hMem != (HGLOBAL)0x0) {
    puVar3 = GlobalLock(hMem);
    if (puVar3 == (undefined8 *)0x0) {
      GlobalFree(hMem);
      hMem = (HGLOBAL)0x0;
    }
    else {
      *puVar3 = 0x14;
      lVar15 = (longlong)puVar3 + 0x14;
      *(undefined4 *)(puVar3 + 1) = 0;
      *(undefined4 *)((longlong)puVar3 + 0xc) = 1;
      *(undefined4 *)(puVar3 + 2) = 1;
      if (uVar12 != 0) {
        plVar10 = (longlong *)(lVar9 + 8);
        do {
          FUN_1401bacb0(lVar15,plVar10[-1],*plVar10 * 2);
          lVar9 = *plVar10;
          plVar10 = plVar10 + 2;
          lVar15 = lVar15 + lVar9 * 2 + 2;
          uVar12 = uVar12 - 1;
        } while (uVar12 != 0);
      }
      GlobalUnlock(hMem);
    }
  }
  local_e8 = local_98;
  uStack_e0 = uStack_90;
  local_d8 = local_88;
  uStack_d0 = (undefined4)uStack_80;
  uStack_cc = uStack_80._4_4_;
  local_c8 = local_78;
  uStack_c4 = uStack_74;
  uStack_c0 = uStack_70;
  uStack_bc = uStack_6c;
  FUN_140001a90(&local_e8);
  return hMem;
}

