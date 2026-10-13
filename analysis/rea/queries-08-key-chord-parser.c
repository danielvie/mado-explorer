
uint FUN_140034f60(longlong *param_1,undefined4 *param_2,int param_3)

{
  ulonglong uVar1;
  char cVar2;
  byte bVar3;
  char *pcVar4;
  uint uVar5;
  ulonglong *puVar6;
  ulonglong uVar7;
  uint uVar8;
  ulonglong uVar9;
  ulonglong uVar10;
  ulonglong uVar11;
  char *pcVar12;
  uint uVar13;
  char cVar14;
  byte bVar15;
  ulonglong local_88;
  ulonglong uStack_80;
  ulonglong local_78 [8];
  
  pcVar4 = (char *)*param_1;
  uVar1 = param_1[1];
  uVar9 = 0;
  bVar15 = 0;
  cVar14 = '\0';
  uVar7 = uVar9;
  if (uVar1 != 0) {
    puVar6 = local_78;
    uVar10 = uVar9;
    uVar11 = uVar9;
    pcVar12 = pcVar4;
    do {
      if (*pcVar4 == '+') {
        if (uVar11 != 0) {
          if (uVar7 < 4) {
            *puVar6 = (ulonglong)pcVar12;
            puVar6[1] = uVar11;
          }
          uVar7 = uVar7 + 1;
          puVar6 = puVar6 + 2;
        }
        pcVar12 = pcVar4 + 1;
        uVar11 = uVar9;
      }
      else {
        uVar11 = uVar11 + 1;
      }
      pcVar4 = pcVar4 + 1;
      uVar10 = uVar10 + 1;
    } while (uVar10 < uVar1);
    if (uVar11 != 0) {
      if (uVar7 < 4) {
        local_78[uVar7 * 2] = (ulonglong)pcVar12;
        local_78[uVar7 * 2 + 1] = uVar11;
      }
      uVar7 = uVar7 + 1;
    }
  }
  if (uVar7 != 0) {
    puVar6 = local_78;
    do {
      local_88 = *puVar6;
      uStack_80 = puVar6[1];
      if (uVar9 == uVar7 - 1) {
        cVar2 = FUN_14003c940(&local_88,2);
        if (cVar2 != '\0') {
          cVar14 = cVar2;
        }
      }
      else {
        bVar3 = FUN_14003c940(&local_88,1);
        if (bVar3 != 0) {
          bVar15 = bVar15 | bVar3;
        }
      }
      uVar9 = uVar9 + 1;
      puVar6 = puVar6 + 2;
    } while (uVar9 < uVar7);
  }
  uVar13 = 0;
  if (param_1[1] == 0) {
    *param_2 = 0;
LAB_140035097:
    uVar5 = 1;
    if (2 < (byte)(cVar14 - 0x3aU)) goto LAB_1400350a1;
  }
  else {
    uVar5 = uVar13;
    if (cVar14 != '\0') {
      *(byte *)((longlong)param_3 + (longlong)param_2) = bVar15;
      *(char *)((longlong)param_3 + 2 + (longlong)param_2) = cVar14;
      goto LAB_140035097;
    }
LAB_1400350a1:
    if (3 < (byte)(cVar14 - 0x3dU)) goto LAB_1400350ac;
  }
  uVar5 = uVar13;
LAB_1400350ac:
  if (((((cVar14 != '\a') && (cVar14 != ' ')) || (param_3 != 0)) ||
      ((param_2[0xe] != 0x10 || (uVar8 = uVar13, bVar15 != 0)))) &&
     ((uVar8 = 1, cVar14 == '\n' && ((param_3 == 0 && (uVar8 = 1, bVar15 == 0)))))) {
    uVar8 = uVar13;
  }
  return uVar8 & uVar5;
}

