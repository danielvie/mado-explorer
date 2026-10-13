
/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

void FUN_14003efc0(longlong param_1,longlong param_2,undefined8 *param_3)

{
  longlong lVar1;
  undefined8 uVar2;
  undefined8 uVar3;
  longlong *plVar4;
  longlong lVar5;
  undefined8 uVar6;
  undefined8 uVar7;
  undefined8 uVar8;
  undefined8 uVar9;
  int iVar10;
  int iVar11;
  undefined8 *puVar12;
  longlong lVar13;
  longlong *plVar14;
  undefined4 *puVar15;
  longlong lVar16;
  undefined4 uVar17;
  undefined4 uVar18;
  undefined4 uVar19;
  undefined4 uVar20;
  longlong local_168;
  longlong lStack_160;
  undefined8 local_158;
  longlong lStack_150;
  longlong local_148;
  longlong lStack_140;
  longlong local_138;
  longlong lStack_130;
  longlong local_128;
  longlong lStack_120;
  undefined4 local_118;
  undefined4 uStack_114;
  undefined4 uStack_110;
  undefined4 uStack_10c;
  longlong local_108;
  uint local_f8;
  undefined4 uStack_f4;
  undefined4 uStack_f0;
  undefined4 uStack_ec;
  undefined4 uStack_e8;
  undefined4 uStack_e4;
  longlong lStack_e0;
  longlong lStack_d8;
  longlong local_d0;
  longlong local_c8;
  longlong lStack_c0;
  undefined8 local_b8;
  undefined8 uStack_b0;
  undefined4 local_a8;
  uint uStack_a4;
  undefined4 local_98;
  undefined4 uStack_94;
  undefined4 uStack_90;
  undefined4 uStack_8c;
  undefined8 local_88;
  undefined4 local_78;
  undefined4 uStack_74;
  undefined4 uStack_70;
  undefined4 uStack_6c;
  
  if (*(char *)((longlong)param_3 + 0x5c) != '\0') {
    return;
  }
  puVar12 = (undefined8 *)FUN_1400c6b20(&local_158,0,0);
  lVar13 = *(longlong *)(param_2 + 0x238);
  uVar2 = *puVar12;
  uVar3 = puVar12[1];
  uVar6 = puVar12[2];
  uVar7 = puVar12[3];
  uVar8 = puVar12[4];
  uVar9 = puVar12[5];
  if (lVar13 != 0) {
    *(undefined8 *)(*(longlong *)(lVar13 + 0x90) + 0x300) = 0;
  }
  if (*(char *)(param_3 + 0xc) != '\0') {
    FUN_140045650(param_1,param_2,param_3);
    goto LAB_14003f79d;
  }
  if (lVar13 != 0) {
    iVar11 = *(int *)(param_3 + 7);
    if (iVar11 == 1) {
      FUN_14003f800(param_1);
      lStack_160 = 0;
      local_168 = lVar13;
      FUN_140071bd0(param_1,&local_168,2);
    }
    else if (iVar11 == 2) {
      FUN_14003f800(param_1);
      if (*(int *)(lVar13 + 0x290) == 0) {
        lVar1 = *(longlong *)(lVar13 + 0x298);
      }
      else {
        lVar1 = *(longlong *)(lVar13 + 0x2e8);
      }
      if (lVar1 != 0) {
        local_168 = 0;
        lStack_160 = 0;
        FUN_140050a90(param_1,lVar13,&local_168);
      }
    }
    else if (iVar11 == 3) {
      FUN_14003f800(param_1);
      lStack_160 = 0;
      local_168 = lVar13;
      FUN_14006f710(param_1,&local_168,2);
    }
    else if (iVar11 == 4) {
      FUN_14003f800(param_1);
      lStack_160 = 0;
      local_168 = lVar13;
      FUN_1400739b0(param_1,&local_168,2);
    }
    else if (iVar11 == 5) {
      FUN_14003f800(param_1);
      lStack_160 = 0;
      local_168 = lVar13;
      FUN_140073b80(param_1,&local_168,2);
    }
    else if (iVar11 == 6) {
      FUN_14003f800(param_1);
      lStack_160 = 0;
      local_168 = lVar13;
      FUN_1400741d0(param_1,&local_168,2);
    }
    else if (iVar11 == 7) {
      FUN_14003f800(param_1);
      lStack_160 = 0;
      local_168 = lVar13;
      FUN_1400743f0(param_1,&local_168,2);
    }
    else if (iVar11 == 8) {
      FUN_14003f800(param_1);
      lStack_160 = 0;
      local_168 = lVar13;
      FUN_140074970(param_1,&local_168,2);
    }
    else if (iVar11 == 9) {
      FUN_14003f800(param_1);
      lStack_160 = 0;
      local_168 = lVar13;
      FUN_140074620(param_1,&local_168,2);
    }
    else {
      lVar1 = param_3[0xe];
      if ((*(char *)(lVar1 + 99) == '\0') && (*(char *)(lVar1 + 100) == '\0')) {
        if (((*(int *)(lVar1 + 0x38) == 10) && (iVar10 = FUN_140047cd0(lVar13), iVar10 == 0)) &&
           (iVar11 != 0xb)) {
          local_88 = 0;
          local_158 = 0;
          lStack_150 = 0;
          local_148 = 0;
          FUN_14004ba90(lVar13,3,1,&local_158);
        }
        local_168 = *(longlong *)(param_2 + 0x938);
        lStack_160 = *(longlong *)(param_2 + 0x940);
        FUN_1400c2bf0(*(undefined8 *)(param_2 + 0x908),*param_3,&local_168,
                      *(undefined8 *)(param_2 + 0x6a0));
        FUN_14003f800(param_1);
      }
      else {
        local_168 = param_3[0x1a];
        lStack_160 = param_3[0x1b];
        lVar13 = FUN_140126040(param_1,&local_168);
        FUN_14003f800(param_1);
        FUN_140135bf0(param_1,*(undefined8 *)(lVar13 + 0x30));
      }
    }
    goto LAB_14003f79d;
  }
  iVar11 = *(int *)(param_3 + 7);
  plVar14 = (longlong *)(param_2 + 0x938);
  uVar17 = *(undefined4 *)plVar14;
  uVar18 = *(undefined4 *)(param_2 + 0x93c);
  lVar13 = *plVar14;
  plVar4 = (longlong *)(param_2 + 0x940);
  uVar19 = *(undefined4 *)plVar4;
  uVar20 = *(undefined4 *)(param_2 + 0x944);
  lVar1 = *plVar4;
  if (iVar11 != 1) {
    if (iVar11 == 2) {
      local_158 = *(longlong *)(param_2 + 0x968);
      lStack_150 = *(longlong *)(param_2 + 0x970);
      local_148 = *(longlong *)(param_2 + 0x978);
      lStack_140 = *(longlong *)(param_2 + 0x980);
      local_138 = *(longlong *)(param_2 + 0x988);
      lVar13 = FUN_140128850(param_1,&local_158);
      if ((lVar13 != 0) && ((*(byte *)(lVar13 + 8) & 0xf) != 0)) {
        FUN_140002c80(param_1,lVar13);
      }
    }
    else if (iVar11 != 3) {
      if (iVar11 == 4) {
        local_168 = *plVar14;
        lStack_160 = *plVar4;
        plVar14 = (longlong *)FUN_1400ce050(&local_f8,uVar2,&local_168,3);
        local_158 = *plVar14;
        lStack_150 = plVar14[1];
        local_148 = plVar14[2];
        lStack_140 = plVar14[3];
        local_138 = plVar14[4];
        lStack_130 = plVar14[5];
        local_128 = plVar14[6];
        lStack_120 = plVar14[7];
        FUN_1400ccf10(&local_158);
      }
      else if (iVar11 == 5) {
        local_168 = *plVar14;
        lStack_160 = *plVar4;
        puVar12 = (undefined8 *)FUN_1400ce050(&local_f8,uVar2,&local_168,3);
        lStack_150 = puVar12[1];
        local_148 = puVar12[2];
        lStack_140 = puVar12[3];
        local_138 = puVar12[4];
        lStack_130 = puVar12[5];
        local_128 = puVar12[6];
        lStack_120 = puVar12[7];
        local_158 = CONCAT44((int)((ulonglong)*puVar12 >> 0x20),1);
        FUN_1400ccf10(&local_158);
      }
      else if (iVar11 == 6) {
        plVar14 = (longlong *)FUN_1400c36b0(&local_f8,uVar2,2);
        local_148 = plVar14[2];
        lStack_140 = plVar14[3];
        local_158 = *plVar14;
        lStack_150 = plVar14[1];
        local_128 = plVar14[6];
        lStack_120 = plVar14[7];
        local_138 = plVar14[4];
        lStack_130 = plVar14[5];
        if (lStack_140 != 0) {
          local_f8 = (uint)((int)local_158 != 0);
LAB_14003f548:
          local_c8 = 0;
          lStack_e0 = lStack_140;
          lStack_d8 = local_138;
          lStack_c0 = 0;
          local_b8 = 0;
          local_158 = CONCAT44(uStack_f4,local_f8);
          local_128 = 0;
          lStack_120 = 0;
          uStack_b0 = 0;
          local_a8 = 0;
          local_108 = (ulonglong)uStack_a4 << 0x20;
          local_118 = 0;
          uStack_114 = 0;
          uStack_110 = 0;
          uStack_10c = 0;
          lStack_150 = lVar13;
          local_148 = lVar1;
          uStack_f0 = uVar17;
          uStack_ec = uVar18;
          uStack_e8 = uVar19;
          uStack_e4 = uVar20;
          FUN_1400c9cd0(&local_158);
        }
      }
      else if (iVar11 == 7) {
        plVar14 = (longlong *)FUN_1400c36b0(&local_f8,uVar2,2);
        local_148 = plVar14[2];
        lStack_140 = plVar14[3];
        local_158 = *plVar14;
        lStack_150 = plVar14[1];
        local_128 = plVar14[6];
        lStack_120 = plVar14[7];
        local_138 = plVar14[4];
        lStack_130 = plVar14[5];
        if ((lStack_140 != 0) && ((int)local_158 == 0)) {
          local_f8 = 7;
          goto LAB_14003f548;
        }
      }
      else if (iVar11 == 8) {
        plVar14 = (longlong *)FUN_1400c36b0(&local_f8,uVar2,2);
        local_148 = plVar14[2];
        lStack_140 = plVar14[3];
        local_158 = *plVar14;
        lStack_150 = plVar14[1];
        local_128 = plVar14[6];
        lStack_120 = plVar14[7];
        local_138 = plVar14[4];
        lStack_130 = plVar14[5];
        if (lStack_140 != 0) {
          local_f8 = 2;
          goto LAB_14003f548;
        }
      }
      else if (((iVar11 != 9) && (*(char *)(param_3[0xe] + 99) == '\0')) &&
              (*(char *)(param_3[0xe] + 100) == '\0')) {
        if (iVar11 - 0xcU < 3) {
          lVar13 = *(longlong *)(param_2 + 0x958);
          local_f8 = 0;
          local_c8 = 0;
          lStack_c0 = 0;
          local_b8 = 0;
          uStack_b0 = 0;
          local_a8 = 0;
          lVar1 = *(longlong *)(param_2 + 0x948);
          lVar5 = *(longlong *)(param_2 + 0x950);
          uStack_f0 = uVar17;
          uStack_ec = uVar18;
          uStack_e8 = uVar19;
          uStack_e4 = uVar20;
          lStack_e0 = lVar1;
          lStack_d8 = lVar5;
          local_d0 = lVar13;
          if (iVar11 != 0xc) {
            if (iVar11 == 0xd) {
              local_168 = 0;
              lStack_160 = -1;
              local_78 = _DAT_1401e8c68;
              uStack_74 = uRam00000001401e8c6c;
              uStack_70 = (undefined4)DAT_1401e8c70;
              uStack_6c = DAT_1401e8c70._4_4_;
              local_98 = uVar17;
              uStack_94 = uVar18;
              uStack_90 = uVar19;
              uStack_8c = uVar20;
              iVar11 = FUN_1401b87a0(&local_78,&local_98,&local_168);
              local_f8 = (iVar11 != 0) + 1;
            }
            else {
              local_f8 = 0;
              if (iVar11 == 0xe) {
                local_f8 = 7;
              }
            }
          }
          local_158 = CONCAT44(uStack_f4,local_f8);
          lStack_150 = CONCAT44(uStack_ec,uStack_f0);
          local_148 = CONCAT44(uStack_e4,uStack_e8);
          lStack_140 = lStack_e0;
          local_138 = lStack_d8;
          lStack_130 = local_d0;
          local_128 = local_c8;
          lStack_120 = lStack_c0;
          local_108 = CONCAT44(uStack_a4,local_a8);
          local_118 = (undefined4)local_b8;
          uStack_114 = local_b8._4_4_;
          uStack_110 = (undefined4)uStack_b0;
          uStack_10c = uStack_b0._4_4_;
          FUN_1400c9cd0(&local_158);
          lVar16 = *(longlong *)(param_1 + 0x980);
          if (lVar16 != 0) {
            do {
              if (*(longlong *)(lVar16 + 0x78) == *(longlong *)(param_2 + 0x960)) {
                if ((lVar16 != 0) &&
                   ((*(uint *)(lVar16 + 0x70) & 0x100a) == *(uint *)(lVar16 + 0x70))) {
                  local_158 = lVar1;
                  lStack_150 = lVar5;
                  local_148 = lVar13;
                  FUN_14004ba90(lVar16,1,lVar1,&local_158);
                }
                break;
              }
              lVar16 = *(longlong *)(lVar16 + 0x88);
            } while (lVar16 != 0);
          }
        }
        else if (iVar11 != 0xf) {
          local_168 = *plVar14;
          lStack_160 = *plVar4;
          puVar15 = (undefined4 *)FUN_140067a00(&local_158,&local_168);
          local_168 = 0;
          lStack_160 = -1;
          local_78 = *puVar15;
          uStack_74 = puVar15[1];
          uStack_70 = puVar15[2];
          uStack_6c = puVar15[3];
          local_98 = uVar17;
          uStack_94 = uVar18;
          uStack_90 = uVar19;
          uStack_8c = uVar20;
          iVar11 = FUN_1401b87a0(&local_98,&local_78,&local_168);
          if (iVar11 == 0) {
            uVar17 = *puVar15;
            uVar18 = puVar15[1];
            uVar19 = puVar15[2];
            uVar20 = puVar15[3];
          }
          local_98 = uVar17;
          uStack_94 = uVar18;
          uStack_90 = uVar19;
          uStack_8c = uVar20;
          FUN_1400c2bf0(*(undefined8 *)(param_2 + 0x908),*param_3,&local_98,
                        *(undefined8 *)(param_2 + 0x6a0));
        }
      }
    }
  }
  FUN_14003f800(param_1);
LAB_14003f79d:
  local_158 = uVar2;
  lStack_150 = uVar3;
  local_148 = uVar6;
  lStack_140 = uVar7;
  local_138 = uVar8;
  lStack_130 = uVar9;
  FUN_140001a90(&local_158);
  return;
}

