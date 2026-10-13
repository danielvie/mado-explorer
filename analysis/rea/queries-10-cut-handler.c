
bool FUN_140073b80(longlong param_1,longlong *param_2,int param_3)

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
  undefined4 local_b8;
  undefined4 uStack_b4;
  undefined8 uStack_b0;
  undefined4 local_a8;
  undefined4 uStack_a4;
  undefined4 uStack_a0;
  undefined4 uStack_9c;
  undefined4 local_98;
  undefined4 uStack_94;
  undefined4 uStack_90;
  undefined4 uStack_8c;
  undefined4 local_88;
  undefined4 uStack_84;
  undefined4 uStack_80;
  undefined4 uStack_7c;
  undefined8 local_78;
  undefined8 uStack_70;
  undefined8 local_68;
  undefined8 uStack_60;
  undefined4 local_58;
  undefined4 uStack_54;
  undefined4 uStack_50;
  undefined4 uStack_4c;
  undefined8 local_48;
  undefined8 uStack_40;
  undefined4 local_38;
  undefined4 uStack_34;
  undefined4 uStack_30;
  undefined4 uStack_2c;
  undefined4 local_28;
  undefined4 uStack_24;
  undefined4 uStack_20;
  undefined4 uStack_1c;
  
  lVar5 = *param_2;
  puVar3 = (undefined8 *)FUN_1400c6b20(&local_b8,0,0);
  local_68 = puVar3[2];
  uStack_60 = puVar3[3];
  uVar2 = *puVar3;
  uStack_70 = puVar3[1];
  local_58 = *(undefined4 *)(puVar3 + 4);
  uStack_54 = *(undefined4 *)((longlong)puVar3 + 0x24);
  uStack_50 = *(undefined4 *)(puVar3 + 5);
  uStack_4c = *(undefined4 *)((longlong)puVar3 + 0x2c);
  if (lVar5 == 0) {
    lVar5 = *(longlong *)(param_1 + 0x920);
  }
  local_78 = uVar2;
  plVar4 = (longlong *)FUN_1400cfff0(&local_48,uVar2,lVar5,0,0x80);
  lVar1 = *plVar4;
  local_c8 = plVar4[2];
  if ((lVar1 != 0) && (param_3 == 2)) {
    local_d8 = (undefined4)*plVar4;
    uStack_d4 = *(undefined4 *)((longlong)plVar4 + 4);
    uStack_d0 = (undefined4)plVar4[1];
    uStack_cc = *(undefined4 *)((longlong)plVar4 + 0xc);
    local_c8 = plVar4[2];
    puVar3 = (undefined8 *)FUN_1400cdd40(&local_48,uVar2,lVar5,&local_d8,3);
    uStack_b0 = puVar3[1];
    local_a8 = *(undefined4 *)(puVar3 + 2);
    uStack_a4 = *(undefined4 *)((longlong)puVar3 + 0x14);
    uStack_a0 = *(undefined4 *)(puVar3 + 3);
    uStack_9c = *(undefined4 *)((longlong)puVar3 + 0x1c);
    local_98 = *(undefined4 *)(puVar3 + 4);
    uStack_94 = *(undefined4 *)((longlong)puVar3 + 0x24);
    uStack_90 = *(undefined4 *)(puVar3 + 5);
    uStack_8c = *(undefined4 *)((longlong)puVar3 + 0x2c);
    local_88 = *(undefined4 *)(puVar3 + 6);
    uStack_84 = *(undefined4 *)((longlong)puVar3 + 0x34);
    uStack_80 = *(undefined4 *)(puVar3 + 7);
    uStack_7c = *(undefined4 *)((longlong)puVar3 + 0x3c);
    _local_b8 = CONCAT44((int)((ulonglong)*puVar3 >> 0x20),1);
    FUN_1400ccf10(&local_b8);
  }
  local_48 = local_78;
  uStack_40 = uStack_70;
  local_38 = (undefined4)local_68;
  uStack_34 = local_68._4_4_;
  uStack_30 = (undefined4)uStack_60;
  uStack_2c = uStack_60._4_4_;
  local_28 = local_58;
  uStack_24 = uStack_54;
  uStack_20 = uStack_50;
  uStack_1c = uStack_4c;
  FUN_140001a90(&local_48);
  return lVar1 == 0;
}

