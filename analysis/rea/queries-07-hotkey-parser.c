
void FUN_140035100(longlong param_1,longlong *param_2,int param_3)

{
  ulonglong *puVar1;
  bool bVar2;
  int iVar3;
  int iVar4;
  int iVar5;
  undefined4 *puVar6;
  undefined1 *puVar7;
  ulonglong uVar8;
  int *piVar9;
  longlong lVar10;
  int iVar11;
  int iVar12;
  ulonglong uVar13;
  ulonglong uVar14;
  int iVar15;
  ulonglong *puVar16;
  longlong lVar17;
  int iVar18;
  ulonglong local_1a8;
  ulonglong uStack_1a0;
  undefined4 local_198;
  int local_188;
  undefined4 uStack_184;
  ulonglong uStack_180;
  ulonglong *local_170;
  ulonglong local_168;
  ulonglong uStack_160;
  undefined4 local_158;
  undefined8 local_148;
  undefined8 uStack_140;
  undefined8 local_138;
  undefined8 uStack_130;
  undefined8 local_128;
  undefined8 uStack_120;
  undefined4 local_118;
  undefined4 uStack_114;
  undefined4 uStack_110;
  undefined4 uStack_10c;
  ulonglong local_108;
  longlong local_f8;
  int local_f0;
  int iStack_ec;
  int iStack_e8;
  int iStack_e4;
  char *local_d8;
  undefined8 local_d0;
  undefined *local_c8;
  undefined8 local_c0;
  longlong local_b8;
  longlong local_b0;
  longlong local_a8;
  longlong local_a0;
  undefined4 local_98;
  undefined4 uStack_94;
  undefined4 uStack_90;
  undefined4 uStack_8c;
  undefined4 local_88;
  undefined4 uStack_84;
  undefined4 uStack_80;
  undefined4 uStack_7c;
  undefined4 local_78;
  undefined4 uStack_74;
  undefined4 uStack_70;
  undefined4 uStack_6c;
  ulonglong local_68;
  ulonglong uStack_60;
  
  puVar6 = (undefined4 *)FUN_1400c6b20(&local_148,0,0);
  uVar8 = 0;
  local_170 = (ulonglong *)(param_1 + 0x68);
  local_98 = *puVar6;
  uStack_94 = puVar6[1];
  uStack_90 = puVar6[2];
  uStack_8c = puVar6[3];
  local_88 = puVar6[4];
  uStack_84 = puVar6[5];
  uStack_80 = puVar6[6];
  uStack_7c = puVar6[7];
  local_78 = puVar6[8];
  uStack_74 = puVar6[9];
  uStack_70 = puVar6[10];
  uStack_6c = puVar6[0xb];
  if (*local_170 != 0) {
    do {
      for (puVar6 = *(undefined4 **)(*(longlong *)(param_1 + 0xa0) + 0x10 + uVar8 * 0x18);
          puVar6 != (undefined4 *)0x0; puVar6 = *(undefined4 **)(puVar6 + 0x10)) {
        *puVar6 = 0;
      }
      uVar8 = (ulonglong)((int)uVar8 + 1);
    } while (uVar8 < *local_170);
  }
  uVar8 = param_2[2];
  iVar18 = param_3 + 1;
  uVar13 = (ulonglong)iVar18;
  if (uVar13 < uVar8 - 1) {
    do {
      lVar10 = param_2[3];
      local_f8 = uVar13 * 0x14;
      if (*(int *)(local_f8 + lVar10) == 1) {
        local_d8 = "Command";
        local_d0 = 7;
        iVar3 = FUN_1400355d0(param_2,iVar18,&local_d8);
        uVar14 = (ulonglong)iVar3;
        bVar2 = false;
        local_1a8 = 0;
        if (((iVar3 < 0) || (uVar8 - 1 <= uVar14)) || (*(int *)(lVar10 + uVar14 * 0x14 + 0xc) != 1))
        {
LAB_14003529d:
          iVar3 = 0;
        }
        else {
          puVar16 = (ulonglong *)(lVar10 + 0x14 + uVar14 * 0x14);
          local_68 = *puVar16;
          uStack_60 = puVar16[1];
          if ((*(int *)(lVar10 + 0x24 + uVar14 * 0x14) != iVar3) || ((int)local_68 != 4))
          goto LAB_14003529d;
          local_198 = *(undefined4 *)(lVar10 + 0x24 + uVar14 * 0x14);
          uStack_1a0._0_4_ = (int)uStack_60;
          bVar2 = true;
          iVar3 = (int)uStack_1a0;
          local_1a8 = local_68;
          uStack_1a0 = uStack_60;
        }
        local_c8 = &DAT_1401d1a90;
        local_c0 = 4;
        iVar4 = FUN_1400355d0(param_2,iVar18,&local_c8);
        puVar16 = local_170;
        lVar17 = (longlong)iVar4;
        uStack_180 = 0;
        iVar12 = 0;
        if ((-1 < iVar4) &&
           (local_158 = *(undefined4 *)(lVar10 + 0x10 + lVar17 * 0x14),
           *(int *)(lVar10 + lVar17 * 0x14 + 0xc) == 1)) {
          puVar1 = (ulonglong *)(lVar10 + 0x14 + lVar17 * 0x14);
          local_168 = *puVar1;
          uStack_160 = puVar1[1];
          if (*(int *)(lVar10 + 0x24 + lVar17 * 0x14) == iVar4) {
            iVar12 = (int)*puVar1;
            local_188 = (int)*puVar1;
            uStack_184 = *(undefined4 *)((longlong)puVar1 + 4);
            uStack_180 = puVar1[1];
          }
        }
        if ((bVar2) && (iVar12 != 0)) {
          iVar15 = (int)(local_1a8 >> 0x20);
          lVar10 = *(longlong *)(param_1 + 0xa0);
          if (lVar10 != 0) {
            local_b8 = (longlong)iVar15 + *param_2;
            local_b0 = (longlong)(iVar3 - iVar15);
            lVar17 = FUN_1401b65d0(local_170,lVar10,0x18,&local_b8,1);
            uVar8 = puVar16[1];
            if (lVar17 != 0) {
              uVar8 = *(ulonglong *)(lVar17 + 8);
              puVar16[2] = uVar8;
              lVar10 = *(longlong *)(param_1 + 0xa0);
            }
            puVar1 = *(ulonglong **)(lVar10 + 0x10 + uVar8 * 0x18);
            if (puVar1 != (ulonglong *)0x0) {
              local_1a8 = puVar1[2];
              uStack_1a0 = puVar1[3];
              local_108 = puVar1[8];
              uStack_140 = puVar1[1];
              local_148 = *puVar1 & 0xffffffff00000000;
              local_168 = puVar1[4];
              uStack_160 = puVar1[5];
              local_118 = (undefined4)puVar1[6];
              uStack_114 = *(undefined4 *)((longlong)puVar1 + 0x34);
              uStack_110 = (undefined4)puVar1[7];
              uStack_10c = *(undefined4 *)((longlong)puVar1 + 0x3c);
              local_138 = local_1a8;
              uStack_130 = uStack_1a0;
              local_128 = local_168;
              uStack_120 = uStack_160;
              if (iVar12 == 2) {
                iVar12 = (int)(uStack_180 >> 0x20);
                iVar3 = 2;
                if (iVar12 < 2) {
                  iVar3 = iVar12;
                }
                iVar15 = 0;
                iVar12 = 0;
                iVar4 = iVar4 + 1;
                if (0 < iVar3) {
                  lVar10 = (longlong)iVar4 * 0x14;
                  iVar11 = iVar4;
LAB_140035431:
                  do {
                    iVar11 = iVar11 + 1;
                    lVar10 = lVar10 + 0x14;
                    if ((ulonglong)(longlong)iVar11 < (ulonglong)param_2[2]) {
                      if (*(int *)(param_2[3] + 0x10 + lVar10) != iVar4) goto LAB_140035431;
                      piVar9 = (int *)(lVar10 + param_2[3]);
                      local_f0 = *piVar9;
                      iStack_ec = piVar9[1];
                      iStack_e8 = piVar9[2];
                      iStack_e4 = piVar9[3];
                      if (local_f0 == 4) {
                        local_a8 = (longlong)iStack_ec + *param_2;
                        local_a0 = (longlong)(iStack_e8 - iStack_ec);
                        iVar5 = FUN_140034f60(&local_a8,&local_148,iVar12);
                        if (iVar5 != 0) {
                          iVar15 = iVar15 + 1;
                        }
                      }
                    }
                    iVar12 = iVar12 + 1;
                  } while (iVar12 < iVar3);
                }
                puVar16 = local_170;
                local_1a8 = local_138;
                uStack_1a0 = uStack_130;
                local_168 = local_128;
                uStack_160 = uStack_120;
                if (iVar15 == iVar3) {
LAB_1400354c5:
                  local_1a8 = local_138;
                  uStack_1a0 = uStack_130;
                  local_168 = local_128;
                  uStack_160 = uStack_120;
                  iVar3 = FUN_140035fd0(puVar16,&local_148);
                  if (iVar3 == 0) {
                    puVar7 = (undefined1 *)
                             FUN_140025540(param_1,puVar16,&local_1a8,&local_168,
                                           CONCAT44(uStack_114,local_118),uStack_110);
                    *puVar7 = (undefined1)local_148;
                    puVar7[1] = local_148._1_1_;
                    puVar7[2] = local_148._2_1_;
                    puVar7[3] = local_148._3_1_;
                  }
                }
              }
              else if (iVar12 == 8) goto LAB_1400354c5;
            }
          }
        }
      }
      uVar8 = param_2[2];
      piVar9 = (int *)(local_f8 + 0x10 + param_2[3]);
      do {
        iVar18 = iVar18 + 1;
        piVar9 = piVar9 + 5;
        uVar13 = uVar13 + 1;
        if (uVar8 <= (ulonglong)(longlong)iVar18) break;
      } while (*piVar9 != param_3);
    } while ((ulonglong)(longlong)iVar18 < uVar8 - 1);
  }
  local_148 = CONCAT44(uStack_94,local_98);
  uStack_140 = CONCAT44(uStack_8c,uStack_90);
  local_138 = CONCAT44(uStack_84,local_88);
  uStack_130 = CONCAT44(uStack_7c,uStack_80);
  local_128 = CONCAT44(uStack_74,local_78);
  uStack_120 = CONCAT44(uStack_6c,uStack_70);
  FUN_140001a90(&local_148);
  return;
}

