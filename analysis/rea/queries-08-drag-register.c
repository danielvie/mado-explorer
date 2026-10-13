
void FUN_140195030(longlong param_1)

{
  HRESULT HVar1;
  
  HVar1 = RegisterDragDrop(*(HWND *)(param_1 + 0x50),(LPDROPTARGET)(param_1 + 0x38));
  if (-1 < HVar1) {
    HVar1 = CoCreateInstance((IID *)&DAT_1401d6b68,(LPUNKNOWN)0x0,1,(IID *)&DAT_1401d6d48,
                             (LPVOID *)(param_1 + 0x48));
    if (HVar1 < 0) {
      RevokeDragDrop(*(HWND *)(param_1 + 0x50));
    }
    else {
      LOCK();
      *(int *)(param_1 + 0x40) = *(int *)(param_1 + 0x40) + 1;
      UNLOCK();
    }
  }
  FUN_1401b4010(HVar1,"Win_DragDropRegisterTarget");
  return;
}

