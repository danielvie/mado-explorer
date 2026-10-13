
void FUN_1401b12c0(undefined8 param_1,longlong *param_2,PTP_WORK param_3)

{
  longlong *plVar1;
  longlong lVar2;
  longlong lVar3;
  longlong lVar4;
  undefined4 *lpTlsValue;
  
  OleInitialize((LPVOID)0x0);
  lpTlsValue = LocalAlloc(0x40,200);
  TlsSetValue(DAT_1401e88a4,lpTlsValue);
  *lpTlsValue = 0xffffffff;
  *(undefined8 *)(lpTlsValue + 0x14) = 0x100000;
  *(undefined8 *)(lpTlsValue + 0x24) = 0x100000;
  (*(code *)param_2[1])
            (param_2[2],
             (ulonglong)CONCAT22(*(undefined2 *)((longlong)param_2 + 0x1c),(short)param_2[3]) <<
             0x20);
  FUN_140001a10(lpTlsValue + 2);
  FUN_140001a10(lpTlsValue + 0x12);
  FUN_140001a10(lpTlsValue + 0x22);
  LocalFree(lpTlsValue);
  TlsSetValue(DAT_1401e88a4,(LPVOID)0x0);
  CloseThreadpoolWork(param_3);
  lVar3 = *param_2;
  LOCK();
  plVar1 = (longlong *)(lVar3 + 0x88);
  lVar2 = *plVar1;
  *plVar1 = *plVar1 + 1;
  UNLOCK();
  lVar4 = *(longlong *)(lVar3 + 0x90);
  while (lVar2 != lVar4) {
    lVar4 = *(longlong *)(lVar3 + 0x90);
  }
  lVar4 = *(longlong *)(lVar3 + 0x98);
  param_2[4] = lVar4;
  *(longlong **)(lVar3 + 0x98) = param_2;
  LOCK();
  plVar1 = (longlong *)(lVar3 + 0x90);
  lVar3 = *plVar1;
  *plVar1 = *plVar1 + 1;
  UNLOCK();
                    /* WARNING: Could not recover jumptable at 0x0001401b13d8. Too many branches */
                    /* WARNING: Treating indirect jump as call */
  CoUninitialize(lVar4,lVar2,lVar3);
  return;
}

