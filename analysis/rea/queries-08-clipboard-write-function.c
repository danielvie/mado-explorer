
int FUN_1400ccf10(int *param_1)

{
  BOOL BVar1;
  UINT uFormat;
  undefined8 *puVar2;
  HANDLE pvVar3;
  HGLOBAL hMem;
  undefined4 *puVar4;
  int iVar5;
  undefined8 local_b8;
  undefined8 uStack_b0;
  undefined8 local_a8;
  undefined4 uStack_a0;
  undefined4 uStack_9c;
  undefined8 local_98;
  undefined8 uStack_90;
  undefined8 local_88;
  undefined8 uStack_80;
  undefined8 local_78;
  undefined8 uStack_70;
  undefined8 local_68;
  undefined8 uStack_60;
  undefined8 local_58;
  undefined8 uStack_50;
  undefined1 local_48 [16];
  undefined1 local_38 [48];
  
  BVar1 = OpenClipboard((HWND)0x0);
  if (BVar1 != 0) {
    BVar1 = EmptyClipboard();
    if (BVar1 != 0) {
      iVar5 = 1;
      if (*(longlong *)(param_1 + 4) != 0) {
        puVar2 = (undefined8 *)FUN_1400c6b20(local_38,0,0);
        local_68 = puVar2[2];
        uStack_60 = puVar2[3];
        local_78 = *puVar2;
        uStack_70 = puVar2[1];
        local_58 = puVar2[4];
        uStack_50 = puVar2[5];
        local_a8 = 0;
        local_b8 = 0;
        uStack_b0 = 0;
        local_88 = *(undefined8 *)(param_1 + 2);
        uStack_80 = *(undefined8 *)(param_1 + 4);
        puVar2 = (undefined8 *)FUN_140187f00(local_48,local_78,&local_88,&local_b8);
        pvVar3 = (HANDLE)FUN_14018c6c0(*puVar2,puVar2[1] * 2,2);
        if (pvVar3 == (HANDLE)0x0) {
LAB_1400ccfe9:
          iVar5 = 0;
        }
        else {
          pvVar3 = SetClipboardData(0xd,pvVar3);
          if (pvVar3 == (HANDLE)0x0) goto LAB_1400ccfe9;
        }
        local_b8 = local_78;
        uStack_b0 = uStack_70;
        local_a8 = local_68;
        uStack_a0 = (undefined4)uStack_60;
        uStack_9c = uStack_60._4_4_;
        local_98 = local_58;
        uStack_90 = uStack_50;
        FUN_140001a90(&local_b8);
      }
      if (*(longlong *)(param_1 + 6) != 0) {
        local_b8 = *(undefined8 *)(param_1 + 6);
        uStack_b0 = *(undefined8 *)(param_1 + 8);
        local_a8 = *(undefined8 *)(param_1 + 10);
        pvVar3 = (HANDLE)FUN_14018c3e0(&local_b8);
        if (pvVar3 != (HANDLE)0x0) {
          pvVar3 = SetClipboardData(0xf,pvVar3);
          if ((pvVar3 != (HANDLE)0x0) && (iVar5 != 0)) {
            iVar5 = 1;
            goto LAB_1400cd060;
          }
        }
        iVar5 = 0;
      }
LAB_1400cd060:
      if (*param_1 == 0) goto LAB_1400cd0c8;
      hMem = GlobalAlloc(2,4);
      if (hMem != (HGLOBAL)0x0) {
        puVar4 = GlobalLock(hMem);
        if (puVar4 == (undefined4 *)0x0) {
          GlobalFree(hMem);
        }
        else {
          *puVar4 = 2;
          GlobalUnlock(hMem);
          uFormat = RegisterClipboardFormatW(L"Preferred DropEffect");
          pvVar3 = SetClipboardData(uFormat,hMem);
          if ((pvVar3 != (HANDLE)0x0) && (iVar5 != 0)) {
            iVar5 = 1;
            goto LAB_1400cd0c8;
          }
        }
      }
    }
  }
  iVar5 = 0;
LAB_1400cd0c8:
  CloseClipboard();
  return iVar5;
}

