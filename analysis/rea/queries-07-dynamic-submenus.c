
void FUN_1400c7550(longlong param_1,longlong param_2,int param_3)

{
  longlong *plVar1;
  longlong lVar2;
  HMENU pHVar3;
  ulonglong uVar4;
  undefined8 local_res8;
  
  if (*(char *)(param_2 + 0x65) == '\0') {
    if (((*(char *)(param_2 + 0x60) != '\0') && (*(char *)(param_2 + 99) == '\0')) &&
       (*(char *)(param_2 + 100) == '\0')) {
      pHVar3 = GetSubMenu(*(HMENU *)(param_2 + 0x98),*(int *)(param_2 + 0x90));
      plVar1 = *(longlong **)(param_1 + 0xd8);
      uVar4 = (ulonglong)*(ushort *)(param_2 + 0x90) | 0x10000;
      if (plVar1 == (longlong *)0x0) {
        plVar1 = *(longlong **)(param_1 + 0xd0);
        if (plVar1 != (longlong *)0x0) {
          (**(code **)(*plVar1 + 0x30))(plVar1,0x117,pHVar3,uVar4);
        }
      }
      else {
        local_res8 = 0;
        (**(code **)(*plVar1 + 0x38))(plVar1,0x117,pHVar3,uVar4,&local_res8);
      }
      FUN_14018e300(param_1,pHVar3,param_2,*(ushort *)(param_2 + 0x68) + 1,param_3);
      return;
    }
  }
  else if (param_3 != 0) {
    for (lVar2 = *(longlong *)(param_2 + 0x78); lVar2 != 0; lVar2 = *(longlong *)(lVar2 + 0x88)) {
      FUN_1400c7550(param_1,lVar2,param_3);
    }
  }
  return;
}

