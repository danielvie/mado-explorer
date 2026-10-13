
void FUN_140040560(longlong param_1,longlong *param_2,int param_3)

{
  longlong *plVar1;
  char *pcVar2;
  longlong lVar3;
  longlong lVar4;
  longlong *plVar5;
  undefined8 uVar6;
  undefined8 uVar7;
  undefined8 uVar8;
  undefined8 uVar9;
  undefined8 uVar10;
  longlong lVar11;
  longlong lVar12;
  longlong *plVar13;
  longlong lVar14;
  undefined1 uVar15;
  int iVar16;
  undefined8 *puVar17;
  longlong lVar18;
  longlong lVar19;
  longlong lVar20;
  longlong lVar21;
  int iVar22;
  longlong lVar23;
  longlong lVar24;
  ulonglong uVar25;
  longlong lVar26;
  ulonglong uVar27;
  LPVOID lpAddress;
  int local_res10 [2];
  int local_res18;
  int local_res20 [2];
  longlong *in_stack_fffffffffffffde8;
  longlong *plVar28;
  int *piVar29;
  int local_1e8 [4];
  char *local_1d8;
  undefined8 uStack_1d0;
  undefined1 local_1c8 [8];
  ulonglong local_1c0;
  longlong local_1b8;
  undefined8 local_1a8;
  undefined8 uStack_1a0;
  undefined8 local_198;
  undefined8 uStack_190;
  longlong *local_188;
  longlong lStack_180;
  longlong lStack_178;
  longlong local_170;
  longlong lStack_168;
  longlong local_160;
  longlong *local_158;
  longlong lStack_150;
  longlong local_148;
  undefined8 uStack_140;
  undefined8 local_138;
  undefined1 local_130;
  undefined4 local_12c;
  longlong *local_128;
  longlong lStack_120;
  longlong local_118;
  longlong lStack_110;
  longlong local_108;
  longlong lStack_100;
  undefined1 local_f8 [184];
  undefined4 uVar30;
  
  local_res18 = param_3;
  puVar17 = (undefined8 *)FUN_1400c6b20(local_f8,0,0);
  lVar18 = param_2[0x121];
  plVar5 = (longlong *)*puVar17;
  uVar6 = puVar17[1];
  uVar7 = puVar17[2];
  uVar8 = puVar17[3];
  uVar9 = puVar17[4];
  uVar10 = puVar17[5];
  *(undefined4 *)(param_2 + 0x122) = 0;
  local_1b8 = lVar18;
  if (param_2[0x125] != 0) {
    FUN_1401bace0(param_2[0x125],0,param_2[0x123] << 4);
    param_2[0x123] = 0;
  }
  uVar30 = (undefined4)((ulonglong)in_stack_fffffffffffffde8 >> 0x20);
  pcVar2 = (char *)param_2[0x7c];
  lVar26 = param_2[0x7d];
  if (lVar26 == 0) {
    FUN_14003f880(param_1,param_2,*(undefined8 *)(lVar18 + 0xa0),1);
    FUN_14003f880(param_1,param_2,*(undefined8 *)(lVar18 + 0xa0),0);
  }
  else {
    if (*(int *)(lVar18 + 0x9c) == 0) {
      FUN_1400c7550(lVar18,*(undefined8 *)(lVar18 + 0xa0),1);
      *(undefined4 *)(lVar18 + 0x9c) = 1;
    }
    *(undefined4 *)(param_2 + 0x126) = 0;
    local_158 = (longlong *)0x0;
    lStack_150 = 0;
    local_138 = 0;
    local_130 = 0x20;
    local_12c = 0;
    local_148 = 0;
    uStack_140 = 0;
    local_1d8 = pcVar2;
    uStack_1d0 = lVar26;
    puVar17 = (undefined8 *)FUN_1401b9110(&local_188,plVar5,&local_1d8,&local_158);
    uVar30 = (undefined4)((ulonglong)in_stack_fffffffffffffde8 >> 0x20);
    lVar20 = plVar5[2];
    lVar19 = plVar5[3];
    lVar11 = plVar5[4];
    lVar12 = plVar5[5];
    plVar13 = (longlong *)*puVar17;
    lVar14 = puVar17[1];
    lVar26 = puVar17[2];
    *(int *)((longlong)plVar5 + 0x3c) = *(int *)((longlong)plVar5 + 0x3c) + 1;
    local_1c0 = ((ulonglong)plVar5[2] >> 2) + ((ulonglong)plVar5[2] >> 1);
    lVar4 = plVar5[6];
    local_160 = lVar4;
    local_188 = plVar5;
    lStack_180 = lVar20;
    lStack_178 = lVar19;
    local_170 = lVar11;
    lStack_168 = lVar12;
    for (pcVar2 = *(char **)(lVar18 + 0xa8); pcVar2 != (char *)0x0;
        pcVar2 = *(char **)(pcVar2 + 0x80)) {
      if (local_1c0 < (ulonglong)plVar5[4]) {
        local_128 = plVar5;
        lStack_120 = lVar20;
        local_118 = lVar19;
        lStack_110 = lVar11;
        local_108 = lVar12;
        lStack_100 = lVar4;
        FUN_140001a90(&local_128);
      }
      if (pcVar2[0x60] == '\0') {
        plVar28 = (longlong *)(pcVar2 + 0xa0);
        iVar22 = 0;
        local_1e8[0] = 0;
        local_res20[0] = 0;
        if (*(longlong *)(pcVar2 + 0xb0) != 0) {
          FUN_1401bace0(*(longlong *)(pcVar2 + 0xb0),0,*plVar28 << 2);
          *plVar28 = 0;
        }
        plVar1 = (longlong *)(pcVar2 + 0xb8);
        if (*(longlong *)(pcVar2 + 200) != 0) {
          FUN_1401bace0(*(longlong *)(pcVar2 + 200),0,*plVar1 << 2);
          *plVar1 = 0;
        }
        local_1a8 = *(undefined8 *)(pcVar2 + 0x20);
        uStack_1a0 = *(undefined8 *)(pcVar2 + 0x28);
        local_198 = *(undefined8 *)(pcVar2 + 0x10);
        uStack_190 = *(undefined8 *)(pcVar2 + 0x18);
        piVar29 = local_1e8;
        in_stack_fffffffffffffde8 = param_2;
        local_158 = plVar13;
        lStack_150 = lVar14;
        local_148 = lVar26;
        FUN_140123060(plVar5,&local_198,&local_1a8,&local_158,param_2,plVar28,plVar1,piVar29,
                      local_res20,1);
        uVar30 = (undefined4)((ulonglong)piVar29 >> 0x20);
        lVar18 = 0;
        for (puVar17 = *(undefined8 **)(pcVar2 + 0x30); puVar17 != (undefined8 *)0x0;
            puVar17 = (undefined8 *)puVar17[3]) {
          local_res10[0] = 0;
          piVar29 = local_res10;
          local_198 = 0;
          uStack_190 = 0;
          local_1a8 = *puVar17;
          uStack_1a0 = puVar17[1];
          plVar28 = (longlong *)0x0;
          in_stack_fffffffffffffde8 = plVar5;
          local_188 = plVar13;
          lStack_180 = lVar14;
          lStack_178 = lVar26;
          iVar16 = FUN_140123060(plVar5,&local_1a8,&local_198,&local_188,plVar5,0,0,piVar29,
                                 local_1c8,1);
          uVar30 = (undefined4)((ulonglong)piVar29 >> 0x20);
          if (iVar16 != 0) {
            iVar22 = iVar22 + local_res10[0];
          }
        }
        iVar22 = local_res20[0] + iVar22 + local_1e8[0];
        if (0 < iVar22) {
          lVar21 = param_2[0x125];
          uVar27 = (ulonglong)uStack_1d0 >> 0x20;
          uStack_1d0 = CONCAT44((int)uVar27,iVar22);
          local_1d8 = pcVar2;
          if (lVar21 == 0) {
            lVar21 = param_2[4];
            lpAddress = (LPVOID)param_2[5];
            uVar27 = param_2[3];
            uVar25 = (ulonglong)((int)lVar21 + (int)lpAddress & 7);
            lVar24 = 8 - uVar25;
            if (uVar25 == 0) {
              lVar24 = lVar18;
            }
            lVar23 = lVar24 + 0x10;
            if (uVar27 < (ulonglong)(lVar23 + lVar21)) {
              if ((ulonglong)param_2[2] < (ulonglong)(lVar23 + lVar21)) {
                FUN_1401b5c50(param_2,lVar23,8);
                lpAddress = (LPVOID)param_2[5];
                lVar21 = param_2[4];
                uVar27 = param_2[3];
                uVar25 = (ulonglong)((int)lVar21 + (int)lpAddress & 7);
                lVar24 = 8 - uVar25;
                if (uVar25 == 0) {
                  lVar24 = lVar18;
                }
                lVar23 = lVar24 + 0x10;
              }
              lVar3 = *param_2;
              if ((lVar3 != 0) && (uVar25 = lVar3 - 1U & lVar23 + lVar21, uVar25 != 0)) {
                lVar18 = lVar3 - uVar25;
              }
              uVar25 = lVar18 + lVar23 + lVar21;
              if (uVar27 < uVar25) {
                param_2[3] = uVar25;
                VirtualAlloc(lpAddress,uVar25,0x1000,4);
              }
            }
            lVar21 = param_2[5] + lVar24 + param_2[4];
            param_2[6] = lVar21;
            param_2[4] = lVar23 + param_2[4];
            param_2[0x123] = 0;
            param_2[0x124] = 1;
          }
          else {
            uVar27 = param_2[0x124];
            uVar25 = param_2[0x123] + 1;
            if (uVar27 < uVar25) {
              if (uVar25 < uVar27 * 2) {
                uVar25 = uVar27 * 2;
              }
              in_stack_fffffffffffffde8 = (longlong *)0x0;
              lVar21 = FUN_1401b5ae0(param_2,lVar21,uVar27 << 4,uVar25 << 4,8,
                                     (ulonglong)plVar28 & 0xffffffff00000000,
                                     "C:\\Users\\Vjekoslav\\Projects\\Voidstar\\File Pilot\\Source\\Program\\Voidstar.h"
                                     ,CONCAT44(uVar30,0xa4b),"_ArrayGrow");
              param_2[0x124] = uVar25;
            }
          }
          param_2[0x125] = lVar21;
          plVar28 = (longlong *)(lVar21 + param_2[0x123] * 0x10);
          *plVar28 = (longlong)local_1d8;
          plVar28[1] = uStack_1d0;
          param_2[0x123] = param_2[0x123] + 1;
          if (*(longlong *)(pcVar2 + 0x50) != 0) {
            *(undefined4 *)(param_2 + 0x126) = 1;
          }
        }
      }
      uVar30 = (undefined4)((ulonglong)in_stack_fffffffffffffde8 >> 0x20);
    }
    FUN_140165a20(param_2[0x125],param_2[0x123],0x10,&LAB_1400178a0);
    lVar18 = local_1b8;
    param_3 = local_res18;
  }
  lVar26 = *(longlong *)(param_1 + 0x130);
  lVar4 = *(longlong *)(lVar26 + 0xa0);
  if (lVar4 == 0) {
    lVar26 = 0;
  }
  else {
    uStack_1d0 = 0xf;
    local_1d8 = "HK_CreateFolder";
    lVar19 = FUN_1401b65d0(lVar26 + 0x68,lVar4,0x18,&local_1d8,CONCAT44(uVar30,1));
    lVar20 = *(longlong *)(lVar26 + 0x70);
    if (lVar19 != 0) {
      lVar20 = *(longlong *)(lVar19 + 8);
      *(longlong *)(lVar26 + 0x78) = lVar20;
    }
    lVar26 = *(longlong *)(lVar4 + 0x10 + lVar20 * 0x18);
  }
  uStack_1d0 = 0;
  local_1d8 = (char *)param_2[0x47];
  iVar16 = *(int *)(param_1 + 0xa18);
  iVar22 = iVar16 + -1;
  if (iVar22 == 0) {
    iVar22 = 1;
  }
  *(int *)(param_1 + 0xa18) = iVar22;
  uVar15 = (**(code **)(lVar26 + 0x30))(param_1,&local_1d8,0);
  *(int *)(param_1 + 0xa18) = iVar16;
  for (lVar18 = *(longlong *)(lVar18 + 0xa8); lVar18 != 0; lVar18 = *(longlong *)(lVar18 + 0x80)) {
    if (*(int *)(*(longlong *)(lVar18 + 0x70) + 0x38) == 10) {
      *(undefined1 *)(lVar18 + 0x5c) = uVar15;
    }
  }
  if ((param_3 == 4) && (param_2[0x123] != 0)) {
    param_2[0x53] = (ulonglong)*(uint *)(*(longlong *)param_2[0x125] + 8);
  }
  *(undefined4 *)(param_2 + 0x65) = 0;
  local_128 = plVar5;
  lStack_120 = uVar6;
  local_118 = uVar7;
  lStack_110 = uVar8;
  local_108 = uVar9;
  lStack_100 = uVar10;
  FUN_140001a90(&local_128);
  return;
}

