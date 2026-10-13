
void FUN_1401a0640(longlong *param_1,undefined8 *param_2,longlong *param_3)

{
  longlong lVar1;
  HRESULT HVar2;
  longlong lVar3;
  longlong lVar4;
  longlong *plVar5;
  ulonglong uVar6;
  LPVOID lpAddress;
  ulonglong uVar7;
  longlong lVar8;
  longlong lVar9;
  uint uVar10;
  int local_res20 [2];
  int local_68 [2];
  longlong *local_60;
  longlong *local_58;
  longlong *local_50;
  longlong *local_48;
  longlong local_40;
  longlong lStack_38;
  ulonglong uVar11;
  
  local_58 = (longlong *)0x0;
  local_50 = (longlong *)0x0;
  local_60 = (longlong *)0x0;
  local_48 = (longlong *)0x0;
  HVar2 = CoCreateInstance((IID *)&DAT_1401d6c68,(LPUNKNOWN)0x0,1,(IID *)&DAT_1401d6c58,&local_58);
  if (((-1 < HVar2) &&
      (HVar2 = (**(code **)(*local_58 + 0x18))(local_58,*param_2,0,0x80000000,1,&local_50),
      -1 < HVar2)) && (HVar2 = (**(code **)(*local_50 + 0x68))(local_50,0,&local_60), -1 < HVar2)) {
    local_40 = 0;
    lStack_38 = 0;
    HVar2 = (**(code **)(*local_60 + 0x20))(local_60,&local_40);
    if ((-1 < HVar2) &&
       (((local_40 == 0x4d37a5273cc4a650 && (lStack_38 == -0x45121438bdcee957)) ||
        ((HVar2 = (**(code **)(*local_58 + 0x50))(local_58,&local_48), -1 < HVar2 &&
         (HVar2 = (**(code **)(*local_48 + 0x40))(local_48,local_60,&DAT_1401d6c78,0,0,0,0),
         -1 < HVar2)))))) {
      local_res20[0] = 0;
      local_68[0] = 0;
      (**(code **)(*local_60 + 0x18))(local_60,local_res20,local_68);
      lVar4 = param_1[4];
      lpAddress = (LPVOID)param_1[5];
      uVar7 = param_1[3];
      uVar10 = local_68[0] * local_res20[0] * 4;
      uVar11 = (ulonglong)uVar10;
      uVar6 = (ulonglong)((int)lVar4 + (int)lpAddress & 7);
      lVar9 = 8 - uVar6;
      if (uVar6 == 0) {
        lVar9 = 0;
      }
      lVar8 = lVar9 + uVar11;
      if (uVar7 < (ulonglong)(lVar4 + lVar8)) {
        if ((ulonglong)param_1[2] < (ulonglong)(lVar4 + lVar8)) {
          FUN_1401b5c50(param_1,lVar8,8);
          lpAddress = (LPVOID)param_1[5];
          lVar4 = param_1[4];
          uVar7 = param_1[3];
          uVar6 = (ulonglong)((int)lVar4 + (int)lpAddress & 7);
          lVar9 = 8 - uVar6;
          if (uVar6 == 0) {
            lVar9 = 0;
          }
          lVar8 = lVar9 + uVar11;
        }
        lVar3 = 0;
        lVar1 = *param_1;
        if ((lVar1 != 0) && (uVar6 = lVar1 - 1U & lVar4 + lVar8, uVar6 != 0)) {
          lVar3 = lVar1 - uVar6;
        }
        uVar6 = lVar3 + lVar4 + lVar8;
        if (uVar7 < uVar6) {
          param_1[3] = uVar6;
          VirtualAlloc(lpAddress,uVar6,0x1000,4);
        }
      }
      lVar9 = param_1[5] + param_1[4] + lVar9;
      FUN_1401bace0(lVar9,0,uVar11);
      param_1[4] = param_1[4] + lVar8;
      param_1[6] = lVar9;
      plVar5 = local_48;
      if (local_48 == (longlong *)0x0) {
        plVar5 = local_60;
      }
      HVar2 = (**(code **)(*plVar5 + 0x38))(plVar5,0,local_res20[0] * 4,uVar10,lVar9);
      if (-1 < HVar2) {
        *(int *)(param_3 + 1) = local_res20[0];
        *(int *)((longlong)param_3 + 0xc) = local_68[0];
        *param_3 = lVar9;
        *(undefined4 *)(param_3 + 2) = 0x40001;
      }
    }
  }
  FUN_1401b4010(HVar2,"Win_GetPreview");
  if (local_58 != (longlong *)0x0) {
    (**(code **)(*local_58 + 0x10))();
  }
  local_58 = (longlong *)0x0;
  if (local_50 != (longlong *)0x0) {
    (**(code **)(*local_50 + 0x10))();
  }
  local_50 = (longlong *)0x0;
  if (local_60 != (longlong *)0x0) {
    (**(code **)(*local_60 + 0x10))();
  }
  local_60 = (longlong *)0x0;
  if (local_48 != (longlong *)0x0) {
    (**(code **)(*local_48 + 0x10))();
  }
  return;
}

