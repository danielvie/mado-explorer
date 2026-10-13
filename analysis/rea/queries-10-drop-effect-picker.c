
void FUN_140194d20(longlong param_1,int param_2,undefined8 *param_3,uint param_4,undefined8 param_5)

{
  undefined8 uVar1;
  undefined8 uVar2;
  bool bVar3;
  int iVar4;
  HRESULT HVar5;
  longlong *plVar6;
  undefined8 *puVar7;
  undefined4 local_res8 [2];
  longlong *local_e8;
  undefined8 uStack_e0;
  longlong *local_d8 [2];
  LPCITEMIDLIST local_c8;
  undefined4 uStack_c0;
  undefined4 uStack_bc;
  LPCITEMIDLIST local_b8;
  undefined8 uStack_b0;
  longlong local_a8;
  longlong lStack_a0;
  longlong local_98;
  longlong lStack_90;
  longlong local_88;
  longlong lStack_80;
  undefined8 local_78;
  undefined8 uStack_70;
  longlong local_68;
  longlong lStack_60;
  undefined4 local_58;
  undefined4 uStack_54;
  undefined4 uStack_50;
  undefined4 uStack_4c;
  undefined1 local_48 [48];
  
  plVar6 = (longlong *)FUN_1400c6b20(local_48,0,0);
  local_a8 = *plVar6;
  lStack_a0 = plVar6[1];
  local_68 = plVar6[2];
  lStack_60 = plVar6[3];
  local_58 = (undefined4)plVar6[4];
  uStack_54 = *(undefined4 *)((longlong)plVar6 + 0x24);
  uStack_50 = (undefined4)plVar6[5];
  uStack_4c = *(undefined4 *)((longlong)plVar6 + 0x2c);
  if (((*(longlong *)(param_1 + 0x80) == 0) && (param_2 == 1)) && (param_3[1] != 0)) {
    bVar3 = false;
    if (((param_4 & 0xc) != 0) || (*(char *)(param_1 + 0x109) != '\0')) {
      bVar3 = true;
    }
    if (*(int *)(param_1 + 0x58) != 0) {
      local_b8 = (LPCITEMIDLIST)*param_3;
      uStack_b0 = param_3[1];
      local_e8 = (longlong *)0x0;
      local_c8 = *(LPCITEMIDLIST *)(param_1 + 0x60);
      uStack_c0 = *(undefined4 *)(param_1 + 0x68);
      uStack_bc = *(undefined4 *)(param_1 + 0x6c);
      uStack_e0 = 0xffffffffffffffff;
      iVar4 = FUN_1401b87a0(&local_c8,&local_b8,&local_e8);
      if ((iVar4 != 0) && (!bVar3)) goto LAB_140194f68;
    }
    local_98 = 0;
    lStack_90 = 0;
    local_e8 = (longlong *)*param_3;
    uStack_e0 = param_3[1];
    local_88 = 0;
    FUN_140187f00(&local_b8,local_a8,&local_e8,&local_98);
    FUN_1401b9a40(local_a8,&local_98,1);
    *(undefined2 *)(local_98 + lStack_90 * 2) = 0;
    local_c8 = (LPCITEMIDLIST)0x0;
    iVar4 = SHParseDisplayName(local_98,0,&local_c8,0,0);
    if (-1 < iVar4) {
      uVar1 = *(undefined8 *)(param_1 + 0x50);
      local_d8[0] = (longlong *)0x0;
      local_b8 = (LPCITEMIDLIST)0x0;
      HVar5 = SHBindToParent(local_c8,(IID *)&DAT_1401d6b38,local_d8,&local_b8);
      if (-1 < HVar5) {
        local_e8 = (longlong *)0x0;
        iVar4 = (**(code **)(*local_d8[0] + 0x50))
                          (local_d8[0],uVar1,1,&local_b8,&DAT_1401d6cb8,0,&local_e8);
        (**(code **)(*local_d8[0] + 0x10))();
        plVar6 = local_e8;
        if ((-1 < iVar4) && (local_e8 != (longlong *)0x0)) {
          local_res8[0] = *(undefined4 *)(param_1 + 0x5c);
          iVar4 = (**(code **)(*local_e8 + 0x18))
                            (local_e8,*(undefined8 *)(param_1 + 0xb8),param_4,param_5,local_res8);
          if (iVar4 < 0) {
            (**(code **)(*plVar6 + 0x10))(plVar6);
          }
          else {
            puVar7 = (undefined8 *)FUN_1401b8530(&local_e8,param_1 + 200,*param_3,param_3[1]);
            uVar1 = *puVar7;
            uVar2 = puVar7[1];
            *(longlong **)(param_1 + 0x80) = plVar6;
            *(undefined8 *)(param_1 + 0x70) = uVar1;
            *(undefined8 *)(param_1 + 0x78) = uVar2;
          }
        }
      }
      CoTaskMemFree(local_c8);
    }
  }
LAB_140194f68:
  local_88 = local_68;
  lStack_80 = lStack_60;
  local_78 = CONCAT44(uStack_54,local_58);
  uStack_70 = CONCAT44(uStack_4c,uStack_50);
  local_98 = local_a8;
  lStack_90 = lStack_a0;
  FUN_140001a90(&local_98);
  return;
}

