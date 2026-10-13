
undefined4
FUN_140169c00(longlong param_1,longlong param_2,int param_3,int param_4,undefined4 *param_5)

{
  longlong lVar1;
  undefined4 local_b8;
  undefined4 uStack_b4;
  undefined4 uStack_b0;
  undefined4 uStack_ac;
  undefined1 local_a8 [80];
  undefined1 local_58 [88];
  
  lVar1 = *(longlong *)(param_1 + 0xcf8);
  if (param_4 == 0x16) {
    return 1;
  }
  if (param_4 == 0x19) {
    if (param_3 == 2) {
      FUN_1401692c0(param_1,lVar1);
      return 0;
    }
  }
  else {
    if ((param_4 != 0x1a) && (param_4 != 0x1b)) {
      if ((param_4 == 0x1d) && (param_3 == 1)) {
        local_b8 = *param_5;
        uStack_b4 = param_5[1];
        uStack_b0 = param_5[2];
        uStack_ac = param_5[3];
        lVar1 = FUN_14010e440(local_a8,6);
        (**(code **)(lVar1 + 0x40))(local_58,&local_b8);
        return 0;
      }
      local_b8 = *param_5;
      uStack_b4 = param_5[1];
      uStack_b0 = param_5[2];
      uStack_ac = param_5[3];
      FUN_14017c3d0(lVar1 + 0x98,*(undefined8 *)(lVar1 + 0x88),param_4,&local_b8,param_3 != 2);
      return 0;
    }
    if (param_3 == 2) {
      FUN_1401688a0(param_1,*(undefined8 *)(param_2 + 0x30),lVar1);
    }
  }
  return 0;
}

