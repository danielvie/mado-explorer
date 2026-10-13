
/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

void FUN_1400356f0(longlong param_1)

{
  undefined8 uVar1;
  undefined8 uVar2;
  longlong lVar3;
  undefined8 uVar4;
  undefined8 uVar5;
  undefined8 uVar6;
  undefined8 uVar7;
  longlong lVar8;
  undefined8 uVar9;
  BOOL BVar10;
  longlong lVar11;
  undefined8 *puVar12;
  longlong *plVar13;
  undefined8 uVar14;
  LARGE_INTEGER local_res8;
  longlong local_e8;
  longlong lStack_e0;
  longlong local_d8;
  undefined8 uStack_d0;
  undefined8 local_c8;
  undefined8 uStack_c0;
  undefined8 local_b8;
  undefined8 uStack_b0;
  undefined8 local_a8;
  undefined8 uStack_a0;
  undefined1 local_98 [16];
  undefined1 local_88 [128];
  
  local_e8 = *(longlong *)(param_1 + 0x180);
  lStack_e0 = *(undefined8 *)(param_1 + 0x188);
  *(undefined4 *)(param_1 + 0x15c) = 1;
  lVar11 = 0;
  BVar10 = QueryPerformanceCounter(&local_res8);
  if (BVar10 != 0) {
    lVar11 = ((local_res8.QuadPart - DAT_1401e8ba0) * 1000000) / DAT_1401e8b98;
  }
  LOCK();
  if (DAT_1401e8bb8 == 0) {
    DAT_1401e8bb8 = 0;
  }
  UNLOCK();
  if ((99999 < (ulonglong)(lVar11 - local_e8)) && (9 < (uint)(DAT_1401e8bb8 - (int)lStack_e0))) {
    puVar12 = (undefined8 *)FUN_1400c6b20(local_88,0,0);
    uVar9 = uRam00000001401e8d50;
    lVar8 = _DAT_1401e8d48;
    uVar1 = *puVar12;
    uVar2 = puVar12[1];
    uVar4 = puVar12[2];
    uVar5 = puVar12[3];
    uVar6 = puVar12[4];
    uVar7 = puVar12[5];
    local_e8 = _DAT_1401e8d48;
    lStack_e0 = uRam00000001401e8d50;
    plVar13 = (longlong *)FUN_140067a00(&local_d8,&local_e8);
    local_e8 = *plVar13;
    lStack_e0 = plVar13[1];
    FUN_1400c2db0(&local_e8);
    plVar13 = (longlong *)FUN_140032680(local_98,param_1,*(undefined8 *)(param_1 + 0x130),uVar1);
    lVar11 = *plVar13;
    lVar3 = plVar13[1];
    uVar14 = FUN_1401b40a0(lVar11,lVar3);
    local_d8 = lVar8;
    uStack_d0 = uVar9;
    *(undefined8 *)(*(longlong *)(param_1 + 0x130) + 0x40) = uVar14;
    local_e8 = lVar11;
    lStack_e0 = lVar3;
    FUN_1400cd6e0(&local_d8,0,&local_e8);
    local_c8 = uVar1;
    uStack_c0 = uVar2;
    local_b8 = uVar4;
    uStack_b0 = uVar5;
    local_a8 = uVar6;
    uStack_a0 = uVar7;
    FUN_140001a90(&local_c8);
    *(undefined4 *)(param_1 + 0x15c) = 0;
    FUN_1400018e0(param_1 + 0x180);
  }
  return;
}

