
void FUN_14019b830(longlong param_1)

{
  int *piVar1;
  longlong lVar2;
  
  lVar2 = *(longlong *)(param_1 + 0x40);
  FUN_1400ca3b0(param_1,param_1 + 0x48);
  FUN_140001a10(param_1);
  LOCK();
  piVar1 = (int *)(lVar2 + 0xa0);
  *piVar1 = *piVar1 + -1;
  UNLOCK();
  return;
}

