
int FUN_1401a16e0(longlong *param_1,undefined8 param_2,longlong *param_3,ulonglong *param_4,
                 undefined8 param_5,undefined8 *param_6)

{
  ulonglong uVar1;
  longlong lVar2;
  int iVar3;
  DWORD dwMessageId;
  ulonglong uVar4;
  ulonglong uVar5;
  undefined8 *puVar6;
  longlong lVar7;
  ulonglong uVar8;
  ulonglong uVar9;
  uint uVar10;
  longlong lVar11;
  ulonglong uVar12;
  longlong *plVar13;
  ulonglong uVar14;
  LPVOID lpAddress;
  ulonglong uVar15;
  longlong *local_178;
  longlong local_170;
  longlong local_168;
  ulonglong uStack_160;
  longlong local_158;
  longlong lStack_150;
  undefined8 local_148;
  longlong local_138;
  ulonglong uStack_130;
  undefined8 local_128;
  undefined8 uStack_120;
  undefined8 local_108;
  longlong lStack_100;
  ulonglong local_f8;
  undefined8 uStack_f0;
  undefined1 local_b8 [16];
  undefined1 local_a8 [112];
  
  uVar1 = *param_4;
  uVar9 = 0;
  local_178 = (longlong *)0x0;
  uVar4 = 1;
  uVar15 = 0;
  *param_6 = 0;
  if (1 < uVar1) {
    do {
      uVar4 = uVar4 * 2;
    } while (uVar4 < uVar1);
  }
  lVar7 = param_1[4];
  local_170 = uVar4 * 8;
  lpAddress = (LPVOID)param_1[5];
  uVar4 = param_1[3];
  uVar5 = (ulonglong)((int)lVar7 + (int)lpAddress & 7);
  uVar12 = 8 - uVar5;
  if (uVar5 == 0) {
    uVar12 = uVar9;
  }
  lVar11 = uVar12 + local_170;
  if (uVar4 < (ulonglong)(lVar7 + lVar11)) {
    if ((ulonglong)param_1[2] < (ulonglong)(lVar7 + lVar11)) {
      FUN_1401b5c50(param_1,lVar11,8);
      lpAddress = (LPVOID)param_1[5];
      lVar7 = param_1[4];
      uVar4 = param_1[3];
      uVar5 = (ulonglong)((int)lVar7 + (int)lpAddress & 7);
      uVar12 = 8 - uVar5;
      if (uVar5 == 0) {
        uVar12 = uVar9;
      }
      lVar11 = local_170 + uVar12;
    }
    lVar2 = *param_1;
    uVar5 = uVar9;
    if ((lVar2 != 0) && (uVar14 = lVar2 - 1U & lVar7 + lVar11, uVar14 != 0)) {
      uVar5 = lVar2 - uVar14;
    }
    uVar5 = uVar5 + lVar7 + lVar11;
    if (uVar4 < uVar5) {
      param_1[3] = uVar5;
      VirtualAlloc(lpAddress,uVar5,0x1000,4);
    }
  }
  local_170 = param_1[5] + param_1[4] + uVar12;
  param_1[6] = local_170;
  param_1[4] = param_1[4] + lVar11;
  plVar13 = (longlong *)param_4[2];
  local_168 = *plVar13;
  uStack_160 = plVar13[1];
  puVar6 = (undefined8 *)FUN_1400c8f40(local_a8,0,&local_168);
  local_128 = *puVar6;
  uStack_120 = puVar6[1];
  local_f8 = puVar6[6];
  uStack_f0 = puVar6[7];
  local_108 = puVar6[4];
  lStack_100 = puVar6[5];
  uVar4 = uVar9;
  if ((param_3[1] == 0) && ((int)local_128 - 2U < 2)) {
    local_168 = lStack_100;
    uStack_160 = local_f8;
    iVar3 = FUN_14019ef30(param_1,param_2,&local_168,&local_178);
    if (iVar3 < 0) goto LAB_1401a1b19;
    uVar12 = uVar9;
    uVar5 = uVar9;
    uVar15 = uVar9;
    if (uVar1 != 0) {
      do {
        if (0 < (int)uVar12) {
          local_138 = *plVar13;
          uStack_130 = plVar13[1];
          lVar7 = FUN_1400c8f40(local_a8,0,&local_138);
          local_f8 = *(ulonglong *)(lVar7 + 0x30);
          uStack_f0 = *(undefined8 *)(lVar7 + 0x38);
        }
        local_138 = *plVar13;
        uStack_130 = plVar13[1];
        uVar14 = local_f8 + 1;
        uVar15 = uStack_130;
        if (((longlong)uStack_130 < 0) &&
           (uVar8 = (uStack_130 ^ (longlong)uStack_130 >> 0x3f) - ((longlong)uStack_130 >> 0x3f),
           uVar15 = uVar9, uVar8 < uStack_130)) {
          uVar15 = uStack_130 - uVar8;
        }
        local_168 = local_138 + uVar14;
        uStack_160 = uVar9;
        if (uVar14 < uVar15) {
          uStack_160 = uVar15 - uVar14;
        }
        local_148 = 0;
        local_158 = 0;
        lStack_150 = 0;
        FUN_140187f00(local_b8,param_1,&local_168,&local_158);
        FUN_1401b9a40(param_1,&local_158,1);
        *(undefined2 *)(local_158 + lStack_150 * 2) = 0;
        iVar3 = (**(code **)(*local_178 + 0x18))
                          (local_178,param_2,0,local_158,0,local_170 + uVar5 * 8,0);
        if (iVar3 < 0) goto LAB_1401a1b1e;
        uVar10 = (int)uVar12 + 1;
        uVar15 = (ulonglong)((int)uVar4 + 1);
        plVar13 = plVar13 + 2;
        uVar12 = (ulonglong)uVar10;
        uVar5 = (longlong)(int)uVar10;
        uVar4 = uVar15;
      } while ((ulonglong)(longlong)(int)uVar10 < uVar1);
    }
  }
  else {
    local_168 = *param_3;
    uStack_160 = param_3[1];
    iVar3 = FUN_14019ef30(param_1,param_2,&local_168,&local_178);
    uVar4 = uVar15;
    if (iVar3 < 0) goto LAB_1401a1b19;
    uVar12 = uVar9;
    uVar5 = uVar9;
    uVar4 = uVar9;
    if (uVar1 != 0) {
      do {
        local_168 = *plVar13;
        uStack_160 = plVar13[1];
        local_148 = 0;
        local_158 = 0;
        lStack_150 = 0;
        lVar7 = FUN_1400c8f40(local_a8,0,&local_168);
        local_168 = *(longlong *)(lVar7 + 0x18);
        uStack_160 = *(ulonglong *)(lVar7 + 0x20);
        FUN_140187f00(local_b8,param_1,&local_168,&local_158);
        FUN_1401b9a40(param_1,&local_158,1);
        *(undefined2 *)(local_158 + lStack_150 * 2) = 0;
        iVar3 = (**(code **)(*local_178 + 0x18))
                          (local_178,param_2,0,local_158,0,local_170 + uVar5 * 8,0);
        if (iVar3 < 0) goto LAB_1401a1b1e;
        uVar10 = (int)uVar12 + 1;
        uVar15 = (ulonglong)((int)uVar4 + 1);
        plVar13 = plVar13 + 2;
        uVar12 = (ulonglong)uVar10;
        uVar5 = (longlong)(int)uVar10;
        uVar4 = uVar15;
      } while ((ulonglong)(longlong)(int)uVar10 < uVar1);
    }
  }
  iVar3 = (**(code **)(*local_178 + 0x50))
                    (local_178,param_2,uVar1 & 0xffffffff,local_170,param_5,0,param_6);
  uVar4 = uVar15;
LAB_1401a1b19:
  if (iVar3 != 0) {
LAB_1401a1b1e:
    dwMessageId = GetLastError();
    FormatMessageA(0x1300,(LPCVOID)0x0,dwMessageId,0,(LPSTR)0x0,0,(va_list *)0x0);
    FUN_1400c3130("C:\\Users\\Vjekoslav\\Projects\\Voidstar\\File Pilot\\Source\\Win\\WinMain.c",
                  "Win__CheckLogHREUSLT",0x1024,"source: %s: HR: 0x%x, msg: %s\n",
                  "Win_GetUIObjectOfPaths",iVar3,0);
    LocalFree((HLOCAL)0x0);
  }
  if (local_178 != (longlong *)0x0) {
    (**(code **)(*local_178 + 0x10))();
  }
  lVar7 = local_170;
  local_178 = (longlong *)0x0;
  if ((int)uVar4 != 0) {
    do {
      CoTaskMemFree(*(LPVOID *)(lVar7 + uVar9 * 8));
      uVar9 = uVar9 + 1;
    } while (uVar9 < uVar4);
  }
  return iVar3;
}

