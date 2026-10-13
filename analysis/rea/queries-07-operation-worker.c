
void FUN_1400cb980(undefined8 param_1,undefined8 param_2)

{
  longlong *plVar1;
  longlong lVar2;
  longlong *plVar3;
  PTP_WORK pwk;
  longlong lVar4;
  ulonglong uVar5;
  undefined8 *pv;
  PTP_CALLBACK_ENVIRON pcbe;
  longlong lVar6;
  longlong lVar7;
  LPVOID lpAddress;
  longlong lVar8;
  
  plVar3 = DAT_1401e8b90;
  pcbe = (PTP_CALLBACK_ENVIRON)(DAT_1401e8b90 + 0x350);
  LOCK();
  plVar1 = DAT_1401e8b90 + 0x361;
  lVar4 = *plVar1;
  *plVar1 = *plVar1 + 1;
  plVar1 = DAT_1401e8b90;
  UNLOCK();
  lVar6 = plVar3[0x362];
  while (lVar4 != lVar6) {
    lVar6 = plVar3[0x362];
  }
  pv = (undefined8 *)plVar3[0x363];
  if (pv == (undefined8 *)0x0) {
    lVar7 = 0;
    lVar4 = DAT_1401e8b90[4];
    lpAddress = (LPVOID)DAT_1401e8b90[5];
    uVar5 = (ulonglong)((int)lVar4 + (int)lpAddress & 7);
    lVar6 = 8 - uVar5;
    if (uVar5 == 0) {
      lVar6 = lVar7;
    }
    lVar8 = lVar6 + 0x28;
    if ((ulonglong)DAT_1401e8b90[3] < (ulonglong)(lVar4 + lVar8)) {
      if ((ulonglong)DAT_1401e8b90[2] < (ulonglong)(lVar4 + lVar8)) {
        FUN_1401b5c50(DAT_1401e8b90,lVar8,8);
        lpAddress = (LPVOID)plVar1[5];
        lVar4 = plVar1[4];
        uVar5 = (ulonglong)((int)lVar4 + (int)lpAddress & 7);
        lVar6 = 8 - uVar5;
        if (uVar5 == 0) {
          lVar6 = lVar7;
        }
        lVar8 = lVar6 + 0x28;
      }
      lVar2 = *plVar1;
      if ((lVar2 != 0) && (uVar5 = lVar2 - 1U & lVar4 + lVar8, uVar5 != 0)) {
        lVar7 = lVar2 - uVar5;
      }
      uVar5 = lVar7 + lVar4 + lVar8;
      if ((ulonglong)plVar1[3] < uVar5) {
        plVar1[3] = uVar5;
        VirtualAlloc(lpAddress,uVar5,0x1000,4);
      }
    }
    pv = (undefined8 *)(plVar1[5] + plVar1[4] + lVar6);
    *pv = 0;
    pv[1] = 0;
    pv[2] = 0;
    pv[3] = 0;
    pv[4] = 0;
    plVar1[4] = plVar1[4] + lVar8;
    plVar1[6] = (longlong)pv;
  }
  else {
    plVar3[0x363] = pv[4];
  }
  LOCK();
  plVar3[0x362] = plVar3[0x362] + 1;
  plVar1 = DAT_1401e8b90;
  UNLOCK();
  *pv = pcbe;
  pv[1] = param_1;
  pv[2] = param_2;
  *(undefined4 *)(pv + 3) = *(undefined4 *)((longlong)plVar1 + 0x94);
  *(int *)((longlong)pv + 0x1c) = (int)plVar1[0x13];
  pwk = CreateThreadpoolWork(FUN_1401b12c0,pv,pcbe);
                    /* WARNING: Could not recover jumptable at 0x0001400cbb2a. Too many branches */
                    /* WARNING: Treating indirect jump as call */
  SubmitThreadpoolWork(pwk);
  return;
}

