
undefined8 * FUN_1400cbd50(undefined8 *param_1,undefined4 *param_2)

{
  undefined8 uVar1;
  undefined8 uVar2;
  undefined8 uVar3;
  undefined8 uVar4;
  undefined8 *puVar5;
  undefined4 local_68;
  undefined4 uStack_64;
  undefined4 uStack_60;
  undefined4 uStack_5c;
  undefined1 local_58 [80];
  
  local_68 = *param_2;
  uStack_64 = param_2[1];
  uStack_60 = param_2[2];
  uStack_5c = param_2[3];
  puVar5 = (undefined8 *)FUN_1401ae920(local_58,&local_68,1);
  uVar1 = puVar5[1];
  uVar2 = puVar5[2];
  uVar3 = puVar5[3];
  *param_1 = *puVar5;
  param_1[1] = uVar1;
  uVar1 = puVar5[4];
  uVar4 = puVar5[5];
  param_1[2] = uVar2;
  param_1[3] = uVar3;
  uVar2 = puVar5[6];
  uVar3 = puVar5[7];
  param_1[4] = uVar1;
  param_1[5] = uVar4;
  uVar1 = puVar5[8];
  param_1[6] = uVar2;
  param_1[7] = uVar3;
  param_1[8] = uVar1;
  return param_1;
}

