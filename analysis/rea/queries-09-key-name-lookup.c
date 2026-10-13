
/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

uint FUN_14003c940(longlong *param_1,uint param_2)

{
  ulonglong uVar1;
  undefined8 *puVar2;
  char cVar3;
  ulonglong uVar4;
  int iVar5;
  uint uVar6;
  char cVar7;
  ulonglong uVar8;
  longlong lVar9;
  uint uVar10;
  ulonglong *puVar11;
  ulonglong *puVar12;
  ulonglong uVar13;
  ulonglong uVar14;
  char *pcVar15;
  longlong lVar16;
  undefined8 local_78;
  undefined8 local_70;
  longlong local_68;
  longlong lStack_60;
  longlong local_58;
  longlong lStack_50;
  undefined8 local_48;
  undefined8 uStack_40;
  
  lVar16 = param_1[1];
  if (lVar16 != 0) {
    pcVar15 = (char *)*param_1;
    do {
      cVar3 = *pcVar15;
      cVar7 = cVar3 + ' ';
      if (0x19 < (byte)(cVar3 + 0xbfU)) {
        cVar7 = cVar3;
      }
      *pcVar15 = cVar7;
      lVar16 = lVar16 + -1;
      pcVar15 = pcVar15 + 1;
    } while (lVar16 != 0);
  }
  lVar16 = DAT_1401e8940;
  if (DAT_1401e8940 != 0) {
    uVar8 = 0x31415926;
    puVar11 = (ulonglong *)0x0;
    if ((ulonglong *)param_1[1] != (ulonglong *)0x0) {
      do {
        puVar12 = (ulonglong *)((longlong)puVar11 + 1);
        uVar8 = (uVar8 << 9 | uVar8 >> 0x37) + (ulonglong)*(byte *)((longlong)puVar11 + *param_1);
        puVar11 = puVar12;
      } while (puVar12 < (ulonglong *)param_1[1]);
    }
    uVar1 = DAT_1401e8910 * 2 - 1;
    uVar8 = (uVar8 ^ 0x31415926) * 0x40000 + ~(uVar8 ^ 0x31415926);
    uVar8 = (uVar8 >> 0x1f | uVar8 << 0x21) * 0x15;
    uVar8 = (uVar8 >> 0xb | uVar8 << 0x35) * 0x41;
    uVar8 = (uVar8 >> 0x16 | uVar8 << 0x2a) ^ uVar8;
    lVar9 = 0x31415928;
    if (1 < uVar8 + 0x31415926) {
      lVar9 = 0x31415926;
    }
    uVar8 = lVar9 + uVar8;
    uVar13 = uVar8 & uVar1;
    puVar11 = (ulonglong *)0x0;
    uVar14 = uVar13;
    do {
      puVar12 = (ulonglong *)(uVar14 * 0x10 + _DAT_1401e8938);
      uVar4 = *puVar12;
      if (uVar4 == uVar8) {
        local_68 = *param_1;
        lStack_60 = param_1[1];
        local_78 = 0;
        puVar2 = (undefined8 *)(lVar16 + puVar12[1] * 0x18);
        local_48 = *puVar2;
        uStack_40 = puVar2[1];
        local_70 = 0xffffffffffffffff;
        local_58 = local_68;
        lStack_50 = lStack_60;
        iVar5 = FUN_1401b87a0(&local_48,&local_58,&local_78);
        if (iVar5 == 0) goto LAB_14003cae8;
      }
      else {
LAB_14003cae8:
        puVar12 = puVar11;
        if (uVar4 == 0) goto LAB_14003cb09;
      }
      uVar14 = uVar14 + 1 & uVar1;
      puVar11 = puVar12;
      if ((puVar12 != (ulonglong *)0x0) || (uVar14 == uVar13)) goto LAB_14003cb09;
    } while( true );
  }
  uVar10 = param_2 & 0xffffff00;
LAB_14003cb75:
  if (param_2 == 1) {
    if (3 < (byte)(uVar10 - 1)) {
      return uVar10 - 1 & 0xffffff00;
    }
  }
  else if (param_2 == 2) {
    uVar6 = uVar10 & 0xff;
    if ((byte)((char)uVar10 - 1U) < 4) {
      uVar6 = 0;
    }
    return uVar6;
  }
  return uVar10 & 0xff;
LAB_14003cb09:
  uVar8 = DAT_1401e8910;
  if (puVar11 != (ulonglong *)0x0) {
    _DAT_1401e8918 = puVar11[1];
    uVar8 = _DAT_1401e8918;
  }
  uVar10 = (uint)*(byte *)(lVar16 + 0x10 + uVar8 * 0x18);
  goto LAB_14003cb75;
}

