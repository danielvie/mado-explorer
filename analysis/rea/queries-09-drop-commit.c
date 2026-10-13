
int FUN_140196070(longlong param_1,undefined8 param_2,undefined8 param_3,undefined4 *param_4,
                 undefined4 *param_5)

{
  longlong *plVar1;
  longlong *plVar2;
  longlong lVar3;
  int iVar4;
  undefined4 uVar5;
  undefined4 local_res8 [2];
  undefined1 local_1d8 [448];
  
  lVar3 = DAT_1401e8b90;
  iVar4 = 0;
  uVar5 = 0;
  if (*(HHOOK *)(DAT_1401e8b90 + 0x960) != (HHOOK)0x0) {
    UnhookWindowsHookEx(*(HHOOK *)(DAT_1401e8b90 + 0x960));
    *(undefined8 *)(lVar3 + 0x960) = 0;
  }
  *(undefined1 *)(lVar3 + 0x9ab) = 1;
  plVar1 = *(longlong **)(param_1 + 0x10);
  if (plVar1 != (longlong *)0x0) {
    (**(code **)(*plVar1 + 0x20))(plVar1);
  }
  plVar2 = *(longlong **)(lVar3 + 0x920);
  if (plVar2 != (longlong *)0x0) {
    local_res8[0] = *(undefined4 *)(lVar3 + 0x8fc);
    iVar4 = (**(code **)(*plVar2 + 0x30))
                      (plVar2,param_2,*(undefined4 *)(lVar3 + 0x928),param_4,local_res8);
    uVar5 = 0;
    if (-1 < iVar4) {
      uVar5 = local_res8[0];
    }
    (**(code **)(**(longlong **)(lVar3 + 0x920) + 0x10))();
    *(undefined8 *)(lVar3 + 0x920) = 0;
  }
  *param_5 = uVar5;
  if (plVar1 != (longlong *)0x0) {
    param_5 = param_4;
    (**(code **)(*plVar1 + 0x30))(plVar1,param_2,&param_5,uVar5);
  }
  (**(code **)(**(longlong **)(lVar3 + 0x958) + 0x10))();
  *(undefined8 *)(lVar3 + 0x958) = 0;
  FUN_1401bace0(local_1d8,0,0x1b8);
  FUN_1401a9d60(lVar3,5,local_1d8);
  return iVar4;
}

