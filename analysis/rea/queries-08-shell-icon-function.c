
int FUN_14019f200(undefined8 param_1,undefined8 param_2,longlong *param_3,undefined4 param_4,
                 undefined8 param_5,undefined4 param_6,undefined4 param_7)

{
  HBITMAP ho;
  HBITMAP ho_00;
  int iVar1;
  BOOL BVar2;
  short sStackX_14;
  short sStackX_16;
  HICON local_res18 [2];
  ICONINFO local_58;
  undefined8 local_38;
  undefined8 uStack_30;
  undefined8 local_28;
  undefined8 uStack_20;
  
  iVar1 = (**(code **)(*param_3 + 0x50))(param_3,param_4,param_6,local_res18);
  if (iVar1 < 0) {
    FUN_1401b4010(iVar1,"Win_GetIconFromImageList");
    return iVar1;
  }
  local_58.fIcon = 0;
  local_58.xHotspot = 0;
  local_58.yHotspot = 0;
  local_58._12_4_ = 0;
  local_58.hbmMask = (HBITMAP)0x0;
  local_58.hbmColor = (HBITMAP)0x0;
  BVar2 = GetIconInfo(local_res18[0],&local_58);
  DestroyIcon(local_res18[0]);
  ho_00 = local_58.hbmColor;
  ho = local_58.hbmMask;
  if (BVar2 < 0) {
    FUN_1401b4010(BVar2,"Win_GetIconFromImageList");
  }
  else {
    local_38 = 0;
    uStack_30 = 0;
    local_28 = 0;
    uStack_20 = 0;
    GetObjectW(local_58.hbmColor,0x20,&local_38);
    sStackX_16 = (short)((ulonglong)param_2 >> 0x30);
    sStackX_14 = (short)((ulonglong)param_2 >> 0x20);
    FUN_1401b0ce0(param_1,ho_00,param_5,local_38._4_4_,(undefined4)uStack_30,param_7,(int)sStackX_14
                  ,(int)sStackX_16);
    DeleteObject(ho_00);
    DeleteObject(ho);
  }
  return BVar2;
}

