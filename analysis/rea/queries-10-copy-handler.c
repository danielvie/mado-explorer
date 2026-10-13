
bool FUN_1400739b0(longlong param_1,longlong *param_2,int param_3)

{
  longlong lVar1;
  undefined8 uVar2;
  undefined8 *puVar3;
  longlong *plVar4;
  longlong lVar5;
  undefined4 local_d8;
  undefined4 uStack_d4;
  undefined4 uStack_d0;
  undefined4 uStack_cc;
  longlong local_c8;
  undefined8 local_b8;
  undefined8 uStack_b0;
  undefined8 local_a8;
  undefined8 uStack_a0;
  undefined4 local_98;
  undefined4 uStack_94;
  undefined4 uStack_90;
  undefined4 uStack_8c;
  undefined8 local_88;
  undefined8 uStack_80;
  undefined4 local_78;
  undefined4 uStack_74;
  undefined4 uStack_70;
  undefined4 uStack_6c;
  undefined4 local_68;
  undefined4 uStack_64;
  undefined4 uStack_60;
  undefined4 uStack_5c;
  undefined8 local_48;
  undefined8 uStack_40;
  undefined8 local_38;
  undefined8 uStack_30;
  undefined4 local_28;
  undefined4 uStack_24;
  undefined4 uStack_20;
  undefined4 uStack_1c;
  undefined4 local_18;
  undefined4 uStack_14;
  undefined4 uStack_10;
  undefined4 uStack_c;
  
  lVar5 = *param_2;
  puVar3 = (undefined8 *)FUN_1400c6b20(&local_48,0,0);
  local_a8 = puVar3[2];
  uStack_a0 = puVar3[3];
  uVar2 = *puVar3;
  uStack_b0 = puVar3[1];
  local_98 = *(undefined4 *)(puVar3 + 4);
  uStack_94 = *(undefined4 *)((longlong)puVar3 + 0x24);
  uStack_90 = *(undefined4 *)(puVar3 + 5);
  uStack_8c = *(undefined4 *)((longlong)puVar3 + 0x2c);
  if (lVar5 == 0) {
    lVar5 = *(longlong *)(param_1 + 0x920);
  }
  local_b8 = uVar2;
  plVar4 = (longlong *)FUN_1400cfff0(&local_88,uVar2,lVar5,0,0xc0);
  lVar1 = *plVar4;
  local_c8 = plVar4[2];
  if ((lVar1 != 0) && (param_3 == 2)) {
    local_d8 = (undefined4)*plVar4;
    uStack_d4 = *(undefined4 *)((longlong)plVar4 + 4);
    uStack_d0 = (undefined4)plVar4[1];
    uStack_cc = *(undefined4 *)((longlong)plVar4 + 0xc);
    local_c8 = plVar4[2];
    puVar3 = (undefined8 *)FUN_1400cdd40(&local_88,uVar2,lVar5,&local_d8,3);
    local_48 = *puVar3;
    uStack_40 = puVar3[1];
    local_38 = puVar3[2];
    uStack_30 = puVar3[3];
    local_28 = *(undefined4 *)(puVar3 + 4);
    uStack_24 = *(undefined4 *)((longlong)puVar3 + 0x24);
    uStack_20 = *(undefined4 *)(puVar3 + 5);
    uStack_1c = *(undefined4 *)((longlong)puVar3 + 0x2c);
    local_18 = *(undefined4 *)(puVar3 + 6);
    uStack_14 = *(undefined4 *)((longlong)puVar3 + 0x34);
    uStack_10 = *(undefined4 *)(puVar3 + 7);
    uStack_c = *(undefined4 *)((longlong)puVar3 + 0x3c);
    FUN_1400ccf10(&local_48);
  }
  local_88 = local_b8;
  uStack_80 = uStack_b0;
  local_78 = (undefined4)local_a8;
  uStack_74 = local_a8._4_4_;
  uStack_70 = (undefined4)uStack_a0;
  uStack_6c = uStack_a0._4_4_;
  local_68 = local_98;
  uStack_64 = uStack_94;
  uStack_60 = uStack_90;
  uStack_5c = uStack_8c;
  FUN_140001a90(&local_88);
  return lVar1 == 0;
}

