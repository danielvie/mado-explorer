
/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

char FUN_1400741d0(longlong param_1,longlong *param_2,int param_3)

{
  int iVar1;
  longlong lVar2;
  char cVar3;
  int iVar4;
  BOOL BVar5;
  undefined8 *puVar6;
  int *piVar7;
  longlong lVar8;
  undefined8 local_168;
  undefined8 local_160;
  uint local_158;
  undefined4 uStack_154;
  undefined8 uStack_150;
  undefined8 uStack_148;
  undefined8 uStack_140;
  undefined8 uStack_138;
  undefined8 local_128;
  undefined8 uStack_120;
  undefined8 local_118;
  undefined8 uStack_110;
  undefined4 local_108;
  uint uStack_104;
  undefined8 local_f8;
  undefined8 uStack_f0;
  undefined8 local_e8;
  undefined8 uStack_e0;
  int local_d8;
  int iStack_d4;
  int iStack_d0;
  int iStack_cc;
  undefined8 local_c8;
  undefined8 uStack_c0;
  undefined4 local_b8;
  undefined4 uStack_b4;
  undefined4 uStack_b0;
  undefined4 uStack_ac;
  longlong local_a8;
  int local_88;
  int iStack_84;
  int iStack_80;
  int iStack_7c;
  int local_78;
  int iStack_74;
  int iStack_70;
  int iStack_6c;
  undefined4 local_58;
  undefined4 uStack_54;
  undefined4 uStack_50;
  undefined4 uStack_4c;
  undefined8 local_48;
  undefined8 uStack_40;
  undefined8 local_38;
  undefined8 uStack_30;
  undefined4 local_28;
  undefined4 uStack_24;
  undefined4 uStack_20;
  undefined4 uStack_1c;
  
  lVar8 = *param_2;
  puVar6 = (undefined8 *)FUN_1400c6b20(&local_f8,0,0);
  local_38 = puVar6[2];
  uStack_30 = puVar6[3];
  local_48 = *puVar6;
  uStack_40 = puVar6[1];
  local_28 = *(undefined4 *)(puVar6 + 4);
  uStack_24 = *(undefined4 *)((longlong)puVar6 + 0x24);
  uStack_20 = *(undefined4 *)(puVar6 + 5);
  uStack_1c = *(undefined4 *)((longlong)puVar6 + 0x2c);
  if (lVar8 == 0) {
    lVar8 = *(longlong *)(param_1 + 0x920);
  }
  piVar7 = (int *)FUN_1400c36b0(&local_158,local_48,2);
  local_88 = piVar7[4];
  iStack_84 = piVar7[5];
  iStack_80 = piVar7[6];
  iStack_7c = piVar7[7];
  iVar1 = *piVar7;
  local_78 = piVar7[8];
  iStack_74 = piVar7[9];
  iStack_70 = piVar7[10];
  iStack_6c = piVar7[0xb];
  iVar4 = FUN_1400d06d0(lVar8,iVar1);
  if ((iVar4 == 0) || (CONCAT44(iStack_7c,iStack_80) == 0)) {
    cVar3 = '\x01';
  }
  else {
    cVar3 = '\0';
  }
  if ((cVar3 == '\0') && (param_3 == 2)) {
    lVar2 = *(longlong *)(lVar8 + 0x90);
    local_160 = 0xffffffffffffffff;
    local_168 = 0;
    local_58 = *(undefined4 *)(lVar2 + 0x168);
    uStack_54 = *(undefined4 *)(lVar2 + 0x16c);
    uStack_50 = *(undefined4 *)(lVar2 + 0x170);
    uStack_4c = *(undefined4 *)(lVar2 + 0x174);
    local_f8 = CONCAT44(uRam00000001401e8c6c,_DAT_1401e8c68);
    uStack_f0 = CONCAT44(DAT_1401e8c70._4_4_,(undefined4)DAT_1401e8c70);
    iVar4 = FUN_1401b87a0(&local_f8,&local_58,&local_168);
    if (iVar4 == 0) {
      local_158 = (uint)(iVar1 != 0);
    }
    else {
      local_158 = 2;
    }
    uStack_150 = *(undefined8 *)(lVar2 + 0x168);
    uStack_148 = *(undefined8 *)(lVar2 + 0x170);
    local_128 = 0;
    uStack_120 = 0;
    uStack_140 = CONCAT44(iStack_7c,iStack_80);
    uStack_138 = CONCAT44(iStack_74,local_78);
    local_118 = 0;
    uStack_110 = 0;
    local_108 = 0;
    local_f8 = CONCAT44(uStack_154,local_158);
    local_d8 = local_78;
    iStack_d4 = iStack_74;
    iStack_d0 = iStack_70;
    iStack_cc = iStack_6c;
    local_c8 = 0;
    uStack_c0 = 0;
    local_a8 = (ulonglong)uStack_104 << 0x20;
    local_b8 = 0;
    uStack_b4 = 0;
    uStack_b0 = 0;
    uStack_ac = 0;
    uStack_f0 = uStack_150;
    local_e8 = uStack_148;
    uStack_e0 = uStack_140;
    FUN_1400c9cd0(&local_f8);
    local_f8 = CONCAT44(iStack_7c,iStack_80);
    uStack_f0 = CONCAT44(iStack_74,local_78);
    local_e8 = CONCAT44(iStack_6c,iStack_70);
    FUN_14004ba90(lVar8,1,CONCAT44(iStack_7c,iStack_80),&local_f8);
    if (iVar1 != 0) {
      BVar5 = OpenClipboard((HWND)0x0);
      if (BVar5 != 0) {
        EmptyClipboard();
      }
      CloseClipboard();
    }
  }
  local_f8 = local_48;
  uStack_f0 = uStack_40;
  local_e8 = local_38;
  uStack_e0 = uStack_30;
  local_d8 = local_28;
  iStack_d4 = uStack_24;
  iStack_d0 = uStack_20;
  iStack_cc = uStack_1c;
  FUN_140001a90(&local_f8);
  return cVar3;
}

