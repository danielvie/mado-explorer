
void FUN_1400c2bf0(longlong param_1,ulonglong param_2,longlong *param_3,tagPOINT param_4)

{
  longlong lVar1;
  undefined4 uVar2;
  longlong *plVar3;
  undefined8 *puVar4;
  tagPOINT local_res8;
  longlong local_138;
  longlong lStack_130;
  longlong local_128;
  longlong lStack_120;
  longlong local_118;
  undefined4 uStack_110;
  undefined4 uStack_10c;
  undefined4 local_108;
  undefined4 uStack_104;
  undefined4 uStack_100;
  undefined4 uStack_fc;
  undefined8 local_f8;
  undefined8 uStack_f0;
  ulonglong local_e8;
  undefined8 uStack_e0;
  undefined8 local_d8;
  undefined8 uStack_d0;
  undefined8 local_c8;
  undefined8 uStack_c0;
  ulonglong local_b8;
  undefined8 uStack_b0;
  longlong local_a8;
  undefined8 uStack_a0;
  tagPOINT local_98;
  longlong local_88;
  longlong lStack_80;
  longlong local_78;
  undefined8 uStack_70;
  undefined8 local_68;
  undefined8 uStack_60;
  undefined1 local_58 [16];
  undefined1 local_48 [48];
  
  plVar3 = (longlong *)FUN_1400c6b20(local_48,0,0);
  local_78 = plVar3[2];
  uStack_70 = plVar3[3];
  lVar1 = *plVar3;
  lStack_80 = plVar3[1];
  local_68 = plVar3[4];
  uStack_60 = plVar3[5];
  local_118 = 0;
  local_128 = 0;
  lStack_120 = 0;
  local_138 = *param_3;
  lStack_130 = param_3[1];
  local_88 = lVar1;
  FUN_140187f00(local_58,lVar1,&local_138,&local_128);
  FUN_1401b9a40(lVar1,&local_128,1);
  local_a8 = local_128;
  local_138 = local_128;
  *(undefined2 *)(local_128 + lStack_120 * 2) = 0;
  lStack_130 = lStack_120;
  if (lStack_120 + 1U < 2) {
    lStack_130 = 0;
  }
  if (lStack_130 == 0) {
    lStack_130 = 0;
  }
  puVar4 = (undefined8 *)FUN_1401b2d10(local_58,lVar1,&local_138);
  local_d8 = *puVar4;
  local_98.x = 0;
  local_98.y = 0;
  uStack_e0 = 0;
  local_f8 = 0x2010400000000068;
  uStack_b0 = 0;
  uStack_a0 = 0;
  local_c8 = 0;
  uStack_c0 = 0;
  uStack_f0 = *(undefined8 *)(DAT_1401e8b90 + 0x9c8);
  local_e8 = param_2 & 0xffff;
  uStack_d0 = 1;
  local_res8 = param_4;
  local_b8 = local_e8;
  ClientToScreen(*(HWND *)(DAT_1401e8b90 + 0x9c8),&local_res8);
  local_98 = local_res8;
  uVar2 = (**(code **)(**(longlong **)(param_1 + 200) + 0x20))
                    (*(longlong **)(param_1 + 200),&local_f8);
  FUN_1401b4010(uVar2,"P_ContextMenuInvoke");
  local_128 = local_88;
  lStack_120 = lStack_80;
  local_118 = local_78;
  uStack_110 = (undefined4)uStack_70;
  uStack_10c = uStack_70._4_4_;
  local_108 = (undefined4)local_68;
  uStack_104 = local_68._4_4_;
  uStack_100 = (undefined4)uStack_60;
  uStack_fc = uStack_60._4_4_;
  FUN_140001a90(&local_128);
  return;
}

