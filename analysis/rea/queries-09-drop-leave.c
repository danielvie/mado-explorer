
undefined8 FUN_140195bd0(longlong param_1)

{
  longlong lVar1;
  longlong lVar2;
  undefined1 local_1c8 [448];
  
  lVar2 = DAT_1401e8b90;
  lVar1 = DAT_1401e8b90 + 0x8a0;
  if (*(longlong **)(DAT_1401e8b90 + 0x920) != (longlong *)0x0) {
    (**(code **)(**(longlong **)(DAT_1401e8b90 + 0x920) + 0x28))();
    (**(code **)(**(longlong **)(lVar2 + 0x920) + 0x10))();
    *(undefined8 *)(lVar2 + 0x920) = 0;
  }
  if (*(longlong **)(param_1 + 0x10) != (longlong *)0x0) {
    (**(code **)(**(longlong **)(param_1 + 0x10) + 0x20))();
  }
  if (*(HHOOK *)(lVar2 + 0x960) != (HHOOK)0x0) {
    UnhookWindowsHookEx(*(HHOOK *)(lVar2 + 0x960));
    *(undefined8 *)(lVar2 + 0x960) = 0;
  }
  FUN_140194940(lVar1);
  FUN_1401bace0(local_1c8,0,0x1b8);
  FUN_1401a9d60(lVar2,4,local_1c8);
  return 0;
}

