
undefined8
FUN_140195a70(longlong param_1,longlong *param_2,uint param_3,undefined8 param_4,undefined4 *param_5
             )

{
  longlong *plVar1;
  longlong lVar2;
  undefined4 *puVar3;
  byte bVar4;
  HHOOK pHVar5;
  byte bVar6;
  undefined1 local_1c8 [448];
  
  puVar3 = param_5;
  lVar2 = DAT_1401e8b90;
  *(undefined1 *)(DAT_1401e8b90 + 0x9a8) = 1;
  *(undefined4 *)(lVar2 + 0x8fc) = 7;
  *(bool *)(lVar2 + 0x9a9) = ((byte)param_3 & 3) == 2;
  bVar4 = (byte)(param_3 >> 2) & 2;
  bVar6 = bVar4 | 1;
  if ((param_3 & 4) == 0) {
    bVar6 = bVar4;
  }
  bVar4 = bVar6 | 4;
  if ((param_3 & 0x20) == 0) {
    bVar4 = bVar6;
  }
  *(byte *)(lVar2 + 0x9aa) = bVar4;
  *param_5 = 7;
  *(longlong **)(lVar2 + 0x958) = param_2;
  (**(code **)(*param_2 + 8))(param_2);
  if (*(longlong *)(lVar2 + 0x960) == 0) {
    pHVar5 = SetWindowsHookExW(0xe,FUN_140194fa0,(HINSTANCE)0x0,0);
    *(HHOOK *)(lVar2 + 0x960) = pHVar5;
  }
  FUN_1401949d0(lVar2 + 0x968,param_2);
  FUN_1401bace0(local_1c8,0,0x1b8);
  FUN_1401a9d60(lVar2,2,local_1c8);
  plVar1 = *(longlong **)(param_1 + 0x10);
  if (plVar1 != (longlong *)0x0) {
    param_5 = (undefined4 *)param_4;
    (**(code **)(*plVar1 + 0x18))(plVar1,*(undefined8 *)(lVar2 + 0x8f0),param_2,&param_5,*puVar3);
  }
  return 0;
}

