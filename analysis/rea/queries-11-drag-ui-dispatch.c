
void FUN_1401a9d60(longlong param_1,int param_2,undefined8 param_3)

{
  char cVar1;
  undefined8 *puVar2;
  bool bVar3;
  undefined8 *puVar4;
  DWORD DVar5;
  HWND pHVar6;
  bool bVar7;
  
  bVar3 = 2 < param_2 - 2U;
  pHVar6 = GetFocus();
  bVar7 = pHVar6 == *(HWND *)(param_1 + 0x9c8);
  cVar1 = *(char *)(param_1 + 0x100);
  *(undefined8 *)(param_1 + 0x100) = 0;
  if (((bVar7) && (bVar3)) || (cVar1 == '\0')) {
    *(char *)(param_1 + 0x100) = cVar1;
  }
  else {
    *(undefined1 *)(param_1 + 0x102) = 1;
  }
  cVar1 = *(char *)(param_1 + 0x108);
  *(undefined8 *)(param_1 + 0x108) = 0;
  if (((bVar7) && (bVar3)) || (cVar1 == '\0')) {
    *(char *)(param_1 + 0x108) = cVar1;
  }
  else {
    *(undefined1 *)(param_1 + 0x10a) = 1;
  }
  cVar1 = *(char *)(param_1 + 0x110);
  *(undefined8 *)(param_1 + 0x110) = 0;
  if (((bVar7) && (bVar3)) || (cVar1 == '\0')) {
    *(char *)(param_1 + 0x110) = cVar1;
  }
  else {
    *(undefined1 *)(param_1 + 0x112) = 1;
  }
  *(undefined4 *)(param_1 + 0x118) = 0;
  *(undefined1 *)(param_1 + 0x128) = 0;
  puVar4 = *(undefined8 **)(param_1 + 0x6d0);
  while (puVar4 != (undefined8 *)0x0) {
    puVar2 = (undefined8 *)puVar4[6];
    *puVar4 = *(undefined8 *)(param_1 + 0x6f8);
    *(undefined8 **)(param_1 + 0x6f8) = puVar4;
    puVar4 = puVar2;
  }
  puVar4 = *(undefined8 **)(param_1 + 0x6e8);
  while (puVar4 != (undefined8 *)0x0) {
    puVar2 = (undefined8 *)puVar4[6];
    *puVar4 = *(undefined8 *)(param_1 + 0x6f8);
    *(undefined8 **)(param_1 + 0x6f8) = puVar4;
    puVar4 = puVar2;
  }
  *(undefined8 *)(param_1 + 0x708) = 0;
  *(undefined8 *)(param_1 + 0x6c8) = 0;
  *(undefined8 *)(param_1 + 0x6d0) = 0;
  *(undefined8 *)(param_1 + 0x6d8) = 0;
  *(undefined8 *)(param_1 + 0x6e0) = 0;
  *(undefined8 *)(param_1 + 0x6e8) = 0;
  *(undefined8 *)(param_1 + 0x6f0) = 0;
  if (param_2 == 0) {
    while (*(int *)(param_1 + 0x48) == 0) {
      LOCK();
      if (DAT_1401e8bb8 == 0) {
        DAT_1401e8bb8 = 0;
      }
      UNLOCK();
      if ((DAT_1401e8bb8 == *(int *)(param_1 + 0x50)) || (*(int *)(param_1 + 0x54) != 0)) break;
      DVar5 = MsgWaitForMultipleObjectsEx(0,(HANDLE *)0x0,0xffffffff,0x1cff,4);
      if (DVar5 == 0xffffffff) {
        DVar5 = GetLastError();
        FormatMessageA(0x1300,(LPCVOID)0x0,DVar5,0,(LPSTR)0x0,0,(va_list *)0x0);
        FUN_1400c3130("C:\\Users\\Vjekoslav\\Projects\\Voidstar\\File Pilot\\Source\\Win\\WinMain.c"
                      ,"Win_WaitForMessages",0x7a4,"WAIT_FAILED: %s\n",0);
        LocalFree((HLOCAL)0x0);
      }
      else {
        *(undefined4 *)(param_1 + 0x58) = 1;
        if (DVar5 == 0) {
          FUN_1401ad6e0();
        }
      }
    }
    FUN_1401ad6e0(param_1);
    if (*(int *)(param_1 + 0x48) != 0) {
      return;
    }
    FUN_1401af0a0(param_1);
    param_2 = 0;
  }
  else {
    if (param_2 == 1) {
      FUN_1401afbc0(param_1,1,param_3);
      return;
    }
    FUN_1401ad6e0(param_1);
    if (*(int *)(param_1 + 0x48) != 0) {
      return;
    }
    FUN_1401af0a0(param_1);
  }
  FUN_1401afbc0(param_1,param_2,param_3);
  FUN_1401ad170(param_1,param_3);
  return;
}

