
void FUN_1401af9f0(undefined8 *param_1,int param_2)

{
  int iVar1;
  undefined8 *puVar2;
  HMENU hMenu;
  longlong *local_res8;
  undefined8 local_d8;
  undefined8 uStack_d0;
  undefined8 local_c8;
  undefined4 uStack_c0;
  undefined4 uStack_bc;
  undefined4 local_b8;
  undefined4 uStack_b4;
  undefined4 uStack_b0;
  undefined4 uStack_ac;
  undefined8 local_a8;
  undefined8 uStack_a0;
  char *local_98;
  undefined8 uStack_90;
  undefined8 local_88;
  undefined8 uStack_80;
  undefined8 local_78;
  undefined8 uStack_70;
  wchar_t *local_68;
  undefined8 uStack_60;
  undefined8 local_58;
  undefined8 uStack_50;
  undefined8 local_48;
  undefined8 local_38;
  undefined8 uStack_30;
  undefined8 local_28;
  undefined8 uStack_20;
  undefined8 local_18;
  undefined8 uStack_10;
  
  puVar2 = (undefined8 *)FUN_1400c6b20(&local_d8,0,0);
  local_c8 = param_1[2];
  hMenu = (HMENU)0x0;
  local_28 = puVar2[2];
  uStack_20 = puVar2[3];
  local_38 = *puVar2;
  uStack_30 = puVar2[1];
  local_18 = puVar2[4];
  uStack_10 = puVar2[5];
  local_res8 = (longlong *)0x0;
  local_d8 = *param_1;
  uStack_d0 = param_1[1];
  iVar1 = FUN_1401a1be0(local_38,&local_d8,&DAT_1401d6aa8,&local_res8);
  if ((-1 < iVar1) && (local_res8 != (longlong *)0x0)) {
    hMenu = CreatePopupMenu();
    if (hMenu != (HMENU)0x0) {
      iVar1 = (**(code **)(*local_res8 + 0x18))(local_res8,hMenu,0,0,0x7fff,0);
      if (-1 < iVar1) {
        local_48 = 0;
        local_a8 = 0x10400000000068;
        local_88 = 0;
        uStack_90 = 0;
        local_78 = 0;
        uStack_70 = 0;
        uStack_60 = 0;
        local_58 = 0;
        uStack_50 = 0;
        uStack_a0 = *(undefined8 *)(DAT_1401e8b90 + 0x9c8);
        uStack_80 = 1;
        if (param_2 == 0) {
          local_98 = "undelete";
          local_68 = L"undelete";
        }
        else {
          local_98 = "delete";
          local_68 = L"delete";
        }
        iVar1 = (**(code **)(*local_res8 + 0x20))(local_res8,&local_a8);
      }
    }
  }
  FUN_1401b4010(iVar1,"Win_RecycleBinPerformAction");
  if (local_res8 != (longlong *)0x0) {
    (**(code **)(*local_res8 + 0x10))();
  }
  local_res8 = (longlong *)0x0;
  if (hMenu != (HMENU)0x0) {
    DestroyMenu(hMenu);
  }
  local_d8 = local_38;
  uStack_d0 = uStack_30;
  local_c8 = local_28;
  uStack_c0 = (undefined4)uStack_20;
  uStack_bc = uStack_20._4_4_;
  local_b8 = (undefined4)local_18;
  uStack_b4 = local_18._4_4_;
  uStack_b0 = (undefined4)uStack_10;
  uStack_ac = uStack_10._4_4_;
  FUN_140001a90(&local_d8);
  return;
}

