
void FUN_140195340(longlong param_1,ulonglong *param_2,ulonglong *param_3)

{
  longlong lVar1;
  undefined8 uVar2;
  longlong *plVar3;
  int iVar4;
  undefined8 *puVar5;
  ulonglong uVar6;
  longlong lVar7;
  longlong lVar8;
  ulonglong uVar9;
  LPVOID lpAddress;
  ulonglong uVar10;
  longlong lVar11;
  longlong lVar12;
  DWORD local_res8 [2];
  IDataObject *local_res10;
  longlong *local_res18;
  longlong *local_res20;
  ulonglong local_108;
  ulonglong uStack_100;
  longlong *local_f8;
  ulonglong uStack_f0;
  ulonglong local_e8;
  undefined4 uStack_e0;
  undefined4 uStack_dc;
  undefined8 local_d8;
  undefined8 uStack_d0;
  longlong *local_c8;
  ulonglong uStack_c0;
  ulonglong local_b8;
  longlong *local_a8;
  undefined8 uStack_a0;
  undefined8 local_98;
  undefined8 uStack_90;
  undefined4 local_88;
  undefined4 uStack_84;
  undefined4 uStack_80;
  undefined4 uStack_7c;
  undefined1 local_78 [56];
  
  puVar5 = (undefined8 *)FUN_1400c6b20(local_78,0,0);
  uVar9 = 1;
  local_98 = puVar5[2];
  uStack_90 = puVar5[3];
  plVar3 = (longlong *)*puVar5;
  uStack_a0 = puVar5[1];
  local_88 = *(undefined4 *)(puVar5 + 4);
  uStack_84 = *(undefined4 *)((longlong)puVar5 + 0x24);
  uStack_80 = *(undefined4 *)(puVar5 + 5);
  uStack_7c = *(undefined4 *)((longlong)puVar5 + 0x2c);
  if (1 < *param_3) {
    do {
      uVar9 = uVar9 * 2;
    } while (uVar9 < *param_3);
  }
  lVar8 = plVar3[4];
  lpAddress = (LPVOID)plVar3[5];
  uVar10 = plVar3[3];
  uVar6 = (ulonglong)((int)lVar8 + (int)lpAddress & 7);
  lVar11 = 8 - uVar6;
  if (uVar6 == 0) {
    lVar11 = 0;
  }
  lVar12 = lVar11 + uVar9 * 0x10;
  local_a8 = plVar3;
  if (uVar10 < (ulonglong)(lVar8 + lVar12)) {
    if ((ulonglong)plVar3[2] < (ulonglong)(lVar8 + lVar12)) {
      FUN_1401b5c50(plVar3,lVar12,8);
      lpAddress = (LPVOID)plVar3[5];
      lVar8 = plVar3[4];
      uVar10 = plVar3[3];
      uVar6 = (ulonglong)((int)lVar8 + (int)lpAddress & 7);
      lVar11 = 8 - uVar6;
      if (uVar6 == 0) {
        lVar11 = 0;
      }
      lVar12 = lVar11 + uVar9 * 0x10;
    }
    lVar7 = 0;
    lVar1 = *plVar3;
    if ((lVar1 != 0) && (uVar9 = lVar1 - 1U & lVar8 + lVar12, uVar9 != 0)) {
      lVar7 = lVar1 - uVar9;
    }
    uVar9 = lVar7 + lVar8 + lVar12;
    if (uVar10 < uVar9) {
      plVar3[3] = uVar9;
      VirtualAlloc(lpAddress,uVar9,0x1000,4);
    }
  }
  lVar8 = plVar3[4];
  local_e8 = param_3[2];
  plVar3[4] = lVar8 + lVar12;
  plVar3[6] = lVar8 + lVar11 + plVar3[5];
  uVar2 = *(undefined8 *)(param_1 + 0x50);
  local_res10 = (IDataObject *)0x0;
  if (*param_3 == 0) {
    local_108 = *param_2;
    uStack_100 = param_2[1];
    local_res18 = (longlong *)0x0;
    iVar4 = FUN_14019ef30(plVar3,uVar2,&local_108,&local_res18);
    if (-1 < iVar4) {
      local_res20 = (longlong *)0x0;
      iVar4 = (**(code **)(*local_res18 + 0x40))(local_res18,uVar2,&DAT_1401d6b78,&local_res20);
      if (-1 < iVar4) {
        iVar4 = (**(code **)(*local_res20 + 0x78))(local_res20,0,&DAT_1401d6ca8,&local_res10);
      }
      (**(code **)(*local_res18 + 0x10))();
    }
  }
  else {
    local_f8 = (longlong *)*param_2;
    uStack_f0 = param_2[1];
    local_108 = 0;
    uStack_100 = 0xffffffffffffffff;
    local_c8 = *(longlong **)(DAT_1401e8b90 + 0x1300);
    uStack_c0 = *(ulonglong *)(DAT_1401e8b90 + 0x1308);
    iVar4 = FUN_1401b87a0(&local_f8,&local_c8,&local_108);
    if (iVar4 == 0) {
      local_f8 = (longlong *)*param_2;
      uStack_f0 = param_2[1];
      local_c8 = (longlong *)*param_3;
      uStack_c0 = param_3[1];
      local_b8 = param_3[2];
      iVar4 = FUN_1401a16e0(plVar3,uVar2,&local_f8,&local_c8,&DAT_1401d6ca8,&local_res10);
    }
    else {
      local_f8 = (longlong *)*param_3;
      uStack_f0 = param_3[1];
      local_e8 = param_3[2];
      iVar4 = FUN_1401a1be0(plVar3,&local_f8,&DAT_1401d6ca8,&local_res10);
    }
  }
  if (-1 < iVar4) {
    uVar9 = *param_2;
    uVar10 = param_2[1];
    *(undefined4 *)(param_1 + 0x58) = 1;
    puVar5 = (undefined8 *)FUN_1401b8530(&local_f8,param_1 + 200,uVar9,uVar10);
    uVar2 = puVar5[1];
    local_res8[0] = 0;
    *(undefined8 *)(param_1 + 0x60) = *puVar5;
    *(undefined8 *)(param_1 + 0x68) = uVar2;
    iVar4 = SHDoDragDrop((HWND)0x0,local_res10,(IDropSource *)0x0,7,local_res8);
  }
  FUN_1401b4010(iVar4,"Win_DragDropStart");
  if (local_res10 != (IDataObject *)0x0) {
    (*local_res10->lpVtbl->Release)(local_res10);
  }
  local_res10 = (IDataObject *)0x0;
  *(undefined4 *)(param_1 + 0x58) = 0;
  local_f8 = local_a8;
  uStack_f0 = uStack_a0;
  local_d8 = CONCAT44(uStack_84,local_88);
  uStack_d0 = CONCAT44(uStack_7c,uStack_80);
  local_e8 = local_98;
  uStack_e0 = (undefined4)uStack_90;
  uStack_dc = uStack_90._4_4_;
  FUN_140001a90(&local_f8);
  return;
}

