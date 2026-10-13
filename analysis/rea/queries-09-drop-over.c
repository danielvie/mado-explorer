
undefined8 FUN_140195c90(longlong param_1,uint param_2,uint *param_3,uint *param_4)

{
  longlong lVar1;
  longlong *plVar2;
  longlong *plVar3;
  undefined4 uVar4;
  undefined4 uVar5;
  undefined4 uVar6;
  longlong lVar7;
  ulonglong uVar8;
  undefined4 uVar9;
  undefined4 uVar10;
  undefined4 uVar11;
  undefined4 uVar12;
  undefined4 uVar13;
  byte bVar14;
  int iVar15;
  int iVar16;
  int iVar17;
  undefined4 *puVar18;
  byte bVar19;
  uint uVar20;
  uint uVar21;
  uint uVar22;
  uint local_res10 [4];
  uint *local_res20;
  undefined8 local_248;
  undefined8 uStack_240;
  undefined8 local_238;
  undefined8 local_230;
  undefined8 local_228;
  undefined8 uStack_220;
  undefined1 local_218 [304];
  int local_e8;
  undefined4 local_e0;
  undefined4 uStack_dc;
  undefined4 uStack_d8;
  undefined4 uStack_d4;
  uint local_a8;
  int iStack_a4;
  ulonglong local_a0;
  undefined4 local_98;
  undefined4 uStack_94;
  undefined4 uStack_90;
  undefined4 uStack_8c;
  undefined4 local_88;
  undefined4 uStack_84;
  undefined4 uStack_80;
  undefined4 uStack_7c;
  
  lVar7 = DAT_1401e8b90;
  plVar2 = *(longlong **)(param_1 + 0x10);
  lVar1 = DAT_1401e8b90 + 0x8a0;
  bVar14 = (byte)(param_2 >> 2) & 2;
  bVar19 = bVar14 | 1;
  if ((param_2 & 4) == 0) {
    bVar19 = bVar14;
  }
  bVar14 = bVar19 | 4;
  if ((param_2 & 0x20) == 0) {
    bVar14 = bVar19;
  }
  uVar22 = 0;
  *(byte *)(DAT_1401e8b90 + 0x9aa) = bVar14;
  local_res20 = param_4;
  FUN_1401bace0(local_218,0,0x1b8);
  FUN_1401a9d60(lVar7,3,local_218);
  iVar17 = local_e8;
  plVar3 = *(longlong **)(lVar7 + 0x920);
  if (plVar3 != (longlong *)0x0) {
    if (local_e8 == 1) {
      local_238 = 0;
      local_248 = *(undefined8 *)(lVar7 + 0x910);
      uStack_240 = *(undefined8 *)(lVar7 + 0x918);
      local_230 = 0xffffffffffffffff;
      local_228 = CONCAT44(uStack_dc,local_e0);
      uStack_220 = CONCAT44(uStack_d4,uStack_d8);
      iVar15 = FUN_1401b87a0(&local_248,&local_228,&local_238);
      if (iVar15 != 0) goto LAB_140195daf;
    }
    (**(code **)(*plVar3 + 0x28))(plVar3);
    (**(code **)(**(longlong **)(lVar7 + 0x920) + 0x10))();
    *(undefined8 *)(lVar7 + 0x910) = 0;
    *(undefined8 *)(lVar7 + 0x918) = 0;
    *(undefined8 *)(lVar7 + 0x920) = 0;
    iVar17 = local_e8;
  }
LAB_140195daf:
  if ((*(int *)(lVar7 + 0x950) == 2) && (iVar17 != 2)) {
    local_248 = 0;
    uStack_240 = 0;
    local_228 = 0;
    uStack_220 = 0;
    FUN_1401950b0(*(undefined8 *)(lVar7 + 0x958),0,&local_228,&local_248);
    iVar17 = local_e8;
  }
  uVar9 = uStack_8c;
  uVar6 = uStack_90;
  uVar5 = uStack_94;
  uVar4 = local_98;
  uVar8 = local_a0;
  *(int *)(lVar7 + 0x950) = iVar17;
  iVar15 = (int)local_a0;
  if (*(int *)(lVar7 + 0x92c) == iVar15) {
    local_228 = *(undefined8 *)(lVar7 + 0x930);
    uStack_220 = *(undefined8 *)(lVar7 + 0x938);
    local_238 = 0;
    local_230 = 0xffffffffffffffff;
    local_248 = CONCAT44(uStack_94,local_98);
    uStack_240 = CONCAT44(uStack_8c,uStack_90);
    iVar16 = FUN_1401b87a0(&local_228,&local_248,&local_238);
    uVar13 = uStack_7c;
    uVar12 = uStack_80;
    uVar11 = uStack_84;
    uVar10 = local_88;
    if (iVar16 != 0) {
      local_228 = *(undefined8 *)(lVar7 + 0x940);
      uStack_220 = *(undefined8 *)(lVar7 + 0x948);
      local_238 = 0;
      local_230 = 0xffffffffffffffff;
      local_248 = CONCAT44(uStack_84,local_88);
      uStack_240 = CONCAT44(uStack_7c,uStack_80);
      iVar16 = FUN_1401b87a0(&local_228,&local_248,&local_238);
      if (iVar16 != 0) {
        local_a0 = uVar8;
        local_88 = uVar10;
        uStack_84 = uVar11;
        uStack_80 = uVar12;
        uStack_7c = uVar13;
        local_98 = uVar4;
        uStack_94 = uVar5;
        uStack_90 = uVar6;
        uStack_8c = uVar9;
        local_e8 = iVar17;
        uVar20 = 0;
        goto LAB_140195f1f;
      }
    }
  }
  local_res10[0] = 1;
  *(int *)(lVar7 + 0x92c) = iVar15;
  puVar18 = (undefined4 *)
            FUN_1401b8530(&local_248,lVar7 + 0x968,CONCAT44(uStack_94,local_98),
                          CONCAT44(uStack_8c,uStack_90));
  uVar4 = puVar18[1];
  uVar5 = puVar18[2];
  uVar6 = puVar18[3];
  *(undefined4 *)(lVar7 + 0x930) = *puVar18;
  *(undefined4 *)(lVar7 + 0x934) = uVar4;
  *(undefined4 *)(lVar7 + 0x938) = uVar5;
  *(undefined4 *)(lVar7 + 0x93c) = uVar6;
  puVar18 = (undefined4 *)
            FUN_1401b8530(&local_248,lVar7 + 0x968,CONCAT44(uStack_84,local_88),
                          CONCAT44(uStack_7c,uStack_80));
  uVar4 = puVar18[1];
  uVar5 = puVar18[2];
  uVar6 = puVar18[3];
  *(undefined4 *)(lVar7 + 0x940) = *puVar18;
  *(undefined4 *)(lVar7 + 0x944) = uVar4;
  *(undefined4 *)(lVar7 + 0x948) = uVar5;
  *(undefined4 *)(lVar7 + 0x94c) = uVar6;
  uVar20 = local_res10[0];
LAB_140195f1f:
  if (local_e8 == 1) {
    *(uint *)(lVar7 + 0x928) = param_2;
    if (iStack_a4 == 1) {
      param_2 = param_2 & 0xfffffffe | 2;
      *(uint *)(lVar7 + 0x928) = param_2;
    }
    local_248 = CONCAT44(uStack_dc,local_e0);
    uStack_240 = CONCAT44(uStack_d4,uStack_d8);
    FUN_140194d20(lVar1,1,&local_248,param_2,param_3);
    plVar3 = *(longlong **)(lVar7 + 0x920);
    uVar22 = 0;
    if (plVar3 != (longlong *)0x0) {
      local_res10[0] = *(uint *)(lVar7 + 0x8fc);
      iVar17 = (**(code **)(*plVar3 + 0x20))
                         (plVar3,*(undefined4 *)(lVar7 + 0x928),param_3,local_res10);
      if (-1 < iVar17) {
        uVar22 = local_res10[0];
      }
    }
  }
  else if (local_e8 == 2) {
    uVar21 = local_a8 & 2 | 1;
    if ((local_a8 & 1) == 0) {
      uVar21 = local_a8 & 2;
    }
    uVar22 = uVar21 | 4;
    if ((local_a8 & 4) == 0) {
      uVar22 = uVar21;
    }
    if (uVar20 != 0) {
      local_248 = CONCAT44(uStack_84,local_88);
      uStack_240 = CONCAT44(uStack_7c,uStack_80);
      local_228 = CONCAT44(uStack_94,local_98);
      uStack_220 = CONCAT44(uStack_8c,uStack_90);
      FUN_1401950b0(*(undefined8 *)(lVar7 + 0x958),local_a0 & 0xffffffff,&local_228,&local_248);
    }
  }
  *local_res20 = uVar22;
  if (plVar2 != (longlong *)0x0) {
    local_res20 = param_3;
    (**(code **)(*plVar2 + 0x28))(plVar2,&local_res20,uVar22);
  }
  return 0;
}

