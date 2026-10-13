
int FUN_1401950b0(longlong *param_1,int param_2,undefined8 *param_3,undefined8 *param_4)

{
  undefined8 uVar1;
  UINT UVar2;
  int iVar3;
  HGLOBAL hMem;
  int *piVar4;
  undefined8 *puVar5;
  ulonglong uVar6;
  bool bVar7;
  undefined8 local_128;
  ulonglong uStack_120;
  undefined2 local_118 [4];
  undefined8 local_110;
  undefined4 local_108;
  undefined4 local_104;
  undefined4 local_100;
  undefined8 local_f8;
  undefined8 uStack_f0;
  undefined8 local_e8;
  undefined4 local_e0 [2];
  HGLOBAL local_d8;
  undefined8 local_d0;
  undefined8 local_c8;
  undefined8 uStack_c0;
  undefined8 local_b8;
  undefined4 uStack_b0;
  undefined4 uStack_ac;
  undefined8 local_a8;
  undefined8 uStack_a0;
  undefined8 local_98;
  ulonglong uStack_90;
  undefined8 local_88;
  undefined8 uStack_80;
  undefined8 local_78;
  undefined8 uStack_70;
  undefined8 local_68;
  undefined8 uStack_60;
  undefined1 local_58 [16];
  undefined1 local_48 [48];
  
  hMem = GlobalAlloc(0x42,0x414);
  if (hMem == (HGLOBAL)0x0) {
    iVar3 = -0x7ff8fff2;
  }
  else {
    piVar4 = GlobalLock(hMem);
    if (piVar4 == (int *)0x0) {
      GlobalFree(hMem);
      iVar3 = -0x7fffbffb;
    }
    else {
      puVar5 = (undefined8 *)FUN_1400c6b20(local_48,0,0);
      local_78 = puVar5[2];
      uStack_70 = puVar5[3];
      uVar1 = *puVar5;
      uStack_80 = puVar5[1];
      local_68 = puVar5[4];
      uStack_60 = puVar5[5];
      if (((param_2 != 1) && (param_2 != 2)) && (bVar7 = param_2 == 3, param_2 = 0, bVar7)) {
        param_2 = 4;
      }
      *piVar4 = param_2;
      local_f8 = 0;
      uStack_f0 = 0;
      local_128 = *param_3;
      uStack_120 = param_3[1];
      local_e8 = 0;
      local_88 = uVar1;
      puVar5 = (undefined8 *)FUN_140187f00(local_58,uVar1,&local_128,&local_f8);
      local_98 = *puVar5;
      uStack_90 = puVar5[1];
      local_b8 = 0;
      local_c8 = 0;
      uStack_c0 = 0;
      local_128 = *param_4;
      uStack_120 = param_4[1];
      puVar5 = (undefined8 *)FUN_140187f00(&local_f8,uVar1,&local_128,&local_c8);
      local_128 = *puVar5;
      uStack_120 = puVar5[1];
      FUN_1401bace0(piVar4 + 1,0,0x208);
      uVar6 = 0x103;
      if (uStack_90 < 0x103) {
        uVar6 = uStack_90;
      }
      FUN_1401bacb0(piVar4 + 1,local_98,uVar6 * 2);
      FUN_1401bace0(piVar4 + 0x83,0,0x208);
      uVar6 = 0x103;
      if (uStack_120 < 0x103) {
        uVar6 = uStack_120;
      }
      FUN_1401bacb0(piVar4 + 0x83,local_128,uVar6 * 2);
      GlobalUnlock(hMem);
      UVar2 = RegisterClipboardFormatW(L"DropDescription");
      local_110 = 0;
      local_c8 = local_88;
      uStack_c0 = uStack_80;
      local_108 = 1;
      local_104 = 0xffffffff;
      local_a8 = local_68;
      uStack_a0 = uStack_60;
      local_100 = 1;
      local_e0[0] = 1;
      local_d0 = 0;
      local_b8 = local_78;
      uStack_b0 = (undefined4)uStack_70;
      uStack_ac = uStack_70._4_4_;
      local_118[0] = (short)UVar2;
      local_d8 = hMem;
      FUN_140001a90(&local_c8);
      iVar3 = (**(code **)(*param_1 + 0x38))(param_1,local_118,local_e0,1);
      if (iVar3 < 0) {
        FUN_1401b4010(iVar3,"Win_DragDropSetDescription");
        GlobalFree(hMem);
      }
      else {
        iVar3 = 0;
      }
    }
  }
  return iVar3;
}

