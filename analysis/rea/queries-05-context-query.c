
undefined8 FUN_1401a45f0(longlong *param_1)

{
  longlong lVar1;
  uint uVar2;
  int iVar3;
  HMENU pHVar4;
  longlong lVar5;
  longlong lVar6;
  longlong lVar7;
  longlong lVar8;
  uint uVar9;
  ulonglong uVar10;
  LPVOID lpAddress;
  ulonglong uVar11;
  
  lVar5 = 0;
  pHVar4 = CreatePopupMenu();
  param_1[0x1c] = (longlong)pHVar4;
  if (pHVar4 != (HMENU)0x0) {
    uVar9 = -(uint)((int)param_1[0xd] != 0) & 0x100;
    uVar2 = uVar9 | 0x10;
    if (*(int *)((longlong)param_1 + 0x6c) == 0) {
      uVar2 = uVar9;
    }
    uVar9 = uVar2 | 0x20;
    if (param_1[0xe] != 0) {
      uVar9 = uVar2;
    }
    uVar2 = uVar9 | 0x80;
    if (param_1[0xe] == 0) {
      uVar2 = uVar9;
    }
    iVar3 = (**(code **)(*(longlong *)param_1[0x19] + 0x18))
                      ((longlong *)param_1[0x19],pHVar4,0,0,0x7fff,uVar2);
    if (-1 < iVar3) {
      (*(code *)**(undefined8 **)param_1[0x19])
                ((undefined8 *)param_1[0x19],&DAT_1401d6a58,param_1 + 0x1a);
      (*(code *)**(undefined8 **)param_1[0x19])
                ((undefined8 *)param_1[0x19],&DAT_1401d6ba8,param_1 + 0x1b);
      lVar6 = param_1[4];
      lpAddress = (LPVOID)param_1[5];
      uVar11 = param_1[3];
      uVar10 = (ulonglong)((int)lVar6 + (int)lpAddress & 7);
      lVar7 = 8 - uVar10;
      if (uVar10 == 0) {
        lVar7 = lVar5;
      }
      lVar8 = lVar7 + 0xe0;
      if (uVar11 < (ulonglong)(lVar6 + lVar8)) {
        if ((ulonglong)param_1[2] < (ulonglong)(lVar6 + lVar8)) {
          FUN_1401b5c50(param_1,lVar8,8);
          lpAddress = (LPVOID)param_1[5];
          lVar6 = param_1[4];
          uVar11 = param_1[3];
          uVar10 = (ulonglong)((int)lVar6 + (int)lpAddress & 7);
          lVar7 = 8 - uVar10;
          if (uVar10 == 0) {
            lVar7 = lVar5;
          }
          lVar8 = lVar7 + 0xe0;
        }
        lVar1 = *param_1;
        if ((lVar1 != 0) && (uVar10 = lVar1 - 1U & lVar6 + lVar8, uVar10 != 0)) {
          lVar5 = lVar1 - uVar10;
        }
        uVar10 = lVar5 + lVar6 + lVar8;
        if (uVar11 < uVar10) {
          param_1[3] = uVar10;
          VirtualAlloc(lpAddress,uVar10,0x1000,4);
        }
      }
      lVar7 = param_1[5] + param_1[4] + lVar7;
      param_1[6] = lVar7;
      param_1[4] = param_1[4] + lVar8;
      param_1[0x14] = lVar7;
      *(undefined1 *)(lVar7 + 0x60) = 1;
      *(undefined1 *)(param_1[0x14] + 0x61) = 1;
      *(longlong *)(param_1[0x14] + 0x98) = param_1[0x1c];
      FUN_14018e300(param_1,param_1[0x1c],param_1[0x14],0,0);
      return 1;
    }
  }
  return 0;
}

