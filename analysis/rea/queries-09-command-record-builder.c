
undefined8 *
FUN_140025540(longlong *param_1,longlong param_2,longlong *param_3,undefined8 *param_4,
             longlong param_5,undefined4 param_6)

{
  byte *pbVar1;
  longlong *plVar2;
  longlong lVar3;
  undefined8 uVar4;
  undefined8 uVar5;
  longlong lVar6;
  undefined8 *puVar7;
  undefined8 uVar8;
  longlong lVar9;
  ulonglong uVar10;
  longlong lVar11;
  undefined8 *puVar12;
  ulonglong uVar13;
  LPVOID lpAddress;
  longlong lVar14;
  undefined8 in_stack_ffffffffffffffa8;
  undefined4 uVar15;
  longlong local_38;
  longlong lStack_30;
  
  uVar15 = (undefined4)((ulonglong)in_stack_ffffffffffffffa8 >> 0x20);
  lVar11 = *(longlong *)(param_2 + 0x38);
  if (lVar11 != 0) {
    local_38 = *param_3;
    lStack_30 = param_3[1];
    uVar8 = CONCAT44(uVar15,1);
    lVar6 = FUN_1401b65d0(param_2,lVar11,0x18,&local_38,uVar8);
    uVar15 = (undefined4)((ulonglong)uVar8 >> 0x20);
    lVar9 = *(longlong *)(param_2 + 8);
    if (lVar6 != 0) {
      lVar9 = *(longlong *)(lVar6 + 8);
      *(longlong *)(param_2 + 0x10) = lVar9;
    }
    for (puVar12 = *(undefined8 **)(lVar11 + 0x10 + lVar9 * 0x18); puVar12 != (undefined8 *)0x0;
        puVar12 = (undefined8 *)puVar12[8]) {
      if ((*(char *)((longlong)puVar12 + 2) == '\0') && (puVar12[6] == param_5)) goto LAB_1400257a3;
    }
  }
  lVar6 = 0;
  lVar11 = param_1[4];
  lpAddress = (LPVOID)param_1[5];
  uVar10 = param_1[3];
  uVar13 = (ulonglong)((int)lVar11 + (int)lpAddress & 7);
  lVar9 = 8 - uVar13;
  if (uVar13 == 0) {
    lVar9 = lVar6;
  }
  lVar14 = lVar9 + 0x48;
  if (uVar10 < (ulonglong)(lVar11 + lVar14)) {
    if ((ulonglong)param_1[2] < (ulonglong)(lVar11 + lVar14)) {
      FUN_1401b5c50(param_1,lVar14,8);
      lpAddress = (LPVOID)param_1[5];
      lVar11 = param_1[4];
      uVar10 = param_1[3];
      uVar13 = (ulonglong)((int)lVar11 + (int)lpAddress & 7);
      lVar9 = 8 - uVar13;
      if (uVar13 == 0) {
        lVar9 = lVar6;
      }
      lVar14 = lVar9 + 0x48;
    }
    lVar3 = *param_1;
    if ((lVar3 != 0) && (uVar13 = lVar3 - 1U & lVar11 + lVar14, uVar13 != 0)) {
      lVar6 = lVar3 - uVar13;
    }
    uVar13 = lVar6 + lVar11 + lVar14;
    if (uVar10 < uVar13) {
      param_1[3] = uVar13;
      VirtualAlloc(lpAddress,uVar13,0x1000,4);
    }
  }
  lVar11 = *param_3;
  lVar6 = param_3[1];
  puVar12 = (undefined8 *)(param_1[5] + param_1[4] + lVar9);
  *puVar12 = 0;
  puVar12[1] = 0;
  puVar12[2] = 0;
  puVar12[3] = 0;
  puVar12[4] = 0;
  puVar12[5] = 0;
  puVar12[6] = 0;
  puVar12[7] = 0;
  puVar12[8] = 0;
  param_1[4] = param_1[4] + lVar14;
  param_1[6] = (longlong)puVar12;
  *(longlong *)(param_2 + 0x40) = *(longlong *)(param_2 + 0x40) + 1;
  puVar12[1] = *(undefined8 *)(param_2 + 0x40);
  puVar7 = (undefined8 *)FUN_1401b8530(&local_38,param_1,lVar11,lVar6);
  uVar8 = *param_4;
  uVar4 = param_4[1];
  uVar5 = puVar7[1];
  puVar12[2] = *puVar7;
  puVar12[3] = uVar5;
  puVar7 = (undefined8 *)FUN_1401b8530(&local_38,param_1,uVar8,uVar4);
  uVar8 = *puVar7;
  uVar4 = puVar7[1];
  puVar12[6] = param_5;
  puVar12[4] = uVar8;
  puVar12[5] = uVar4;
  *(undefined4 *)(puVar12 + 7) = param_6;
  lVar11 = *(longlong *)(param_2 + 0x38);
  if (lVar11 != 0) {
    local_38 = *param_3;
    lStack_30 = param_3[1];
    lVar6 = FUN_1401b65d0(param_2,lVar11,0x18,&local_38,CONCAT44(uVar15,1));
    lVar9 = *(longlong *)(param_2 + 8);
    if (lVar6 != 0) {
      lVar9 = *(longlong *)(lVar6 + 8);
      *(longlong *)(param_2 + 0x10) = lVar9;
    }
    lVar9 = *(longlong *)(lVar11 + 0x10 + lVar9 * 0x18);
    if (lVar9 != 0) {
      for (lVar11 = *(longlong *)(lVar9 + 0x40); lVar11 != 0; lVar11 = *(longlong *)(lVar11 + 0x40))
      {
        lVar9 = lVar11;
      }
      *(undefined8 **)(lVar9 + 0x40) = puVar12;
      goto LAB_1400257a3;
    }
  }
  local_38 = *param_3;
  lStack_30 = param_3[1];
  uVar8 = FUN_1401b6980(param_1,param_2,lVar11,0x18,0x10);
  uVar13 = 0;
  uVar10 = 0x31415926;
  if (param_3[1] != 0) {
    do {
      pbVar1 = (byte *)(local_38 + uVar13);
      uVar13 = uVar13 + 1;
      uVar10 = (uVar10 << 9 | uVar10 >> 0x37) + (ulonglong)*pbVar1;
    } while (uVar13 < (ulonglong)param_3[1]);
  }
  local_38 = *param_3;
  lStack_30 = param_3[1];
  uVar10 = (uVar10 ^ 0x31415926) * 0x40000 + ~(uVar10 ^ 0x31415926);
  uVar10 = (uVar10 >> 0x1f | uVar10 << 0x21) * 0x15;
  uVar10 = (uVar10 >> 0xb | uVar10 << 0x35) * 0x41;
  uVar10 = (uVar10 >> 0x16 | uVar10 << 0x2a) ^ uVar10;
  lVar11 = 0x31415928;
  if (1 < uVar10 + 0x31415926) {
    lVar11 = 0x31415926;
  }
  lVar9 = FUN_1401b67b0(param_1,param_2,uVar8,0x18,&local_38,1,lVar11 + uVar10);
  lVar11 = param_3[1];
  plVar2 = (longlong *)(lVar9 + *(longlong *)(param_2 + 0x10) * 0x18);
  *plVar2 = *param_3;
  plVar2[1] = lVar11;
  *(longlong *)(param_2 + 0x38) = lVar9;
  *(undefined8 **)(lVar9 + 0x10 + *(longlong *)(param_2 + 0x10) * 0x18) = puVar12;
LAB_1400257a3:
  *(undefined2 *)((longlong)puVar12 + 0x3c) = 0;
  return puVar12;
}

