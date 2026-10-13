
void FUN_14012e700(undefined8 param_1,longlong *param_2,undefined4 *param_3)

{
  char cVar1;
  ulonglong uVar2;
  longlong lVar3;
  undefined8 *puVar4;
  ulonglong uVar5;
  char *pcVar6;
  char *pcVar7;
  ulonglong uVar8;
  uint uVar9;
  double dVar10;
  undefined8 in_stack_ffffffffffffff08;
  undefined4 uVar12;
  undefined8 uVar11;
  undefined1 local_e8 [16];
  char *local_d8;
  longlong lStack_d0;
  undefined8 local_c8;
  undefined8 uStack_c0;
  undefined1 local_70 [72];
  
  uVar12 = (undefined4)((ulonglong)in_stack_ffffffffffffff08 >> 0x20);
  uVar5 = 0;
  pcVar7 = "{";
  uVar8 = uVar5;
  do {
    cVar1 = *pcVar7;
    pcVar7 = pcVar7 + 1;
    if (cVar1 == '\0') break;
    uVar8 = uVar8 + 1;
  } while (pcVar7 != (char *)0x0);
  if ((longlong)uVar8 < 0) {
    uVar2 = (uVar8 ^ (longlong)uVar8 >> 0x3f) - ((longlong)uVar8 >> 0x3f);
    if (uVar2 < uVar8) {
      uVar8 = uVar8 - uVar2;
      goto LAB_14012e75f;
    }
  }
  else {
LAB_14012e75f:
    if (uVar8 == 0) {
      uVar8 = uVar5;
    }
    if (uVar8 != 0) {
      FUN_1401b8310(param_1,param_2,uVar8);
      FUN_1401bacb0(param_2[1] + *param_2,&DAT_1401cf7d0,uVar8);
      param_2[1] = param_2[1] + uVar8;
    }
  }
  uVar11 = CONCAT44(uVar12,*param_3);
  FUN_1401b7fd0(&local_d8,param_1,param_2," \"ID\": %d",uVar11);
  uVar12 = (undefined4)((ulonglong)uVar11 >> 0x20);
  if (param_3[1] != 0) {
    uVar11 = CONCAT44(uVar12,param_3[1]);
    FUN_1401b7fd0(&local_d8,param_1,param_2,", \"Prev\": %d",uVar11);
    uVar12 = (undefined4)((ulonglong)uVar11 >> 0x20);
  }
  if (param_3[2] != 0) {
    uVar11 = CONCAT44(uVar12,param_3[2]);
    FUN_1401b7fd0(&local_d8,param_1,param_2,", \"Next\": %d",uVar11);
    uVar12 = (undefined4)((ulonglong)uVar11 >> 0x20);
  }
  if (param_3[3] != 0) {
    uVar11 = CONCAT44(uVar12,param_3[3]);
    FUN_1401b7fd0(&local_d8,param_1,param_2,", \"Parent\": %d",uVar11);
    uVar12 = (undefined4)((ulonglong)uVar11 >> 0x20);
  }
  if (param_3[4] != 0) {
    uVar11 = CONCAT44(uVar12,param_3[4]);
    FUN_1401b7fd0(&local_d8,param_1,param_2,", \"Child\": %d",uVar11);
    uVar12 = (undefined4)((ulonglong)uVar11 >> 0x20);
  }
  if (param_3[5] != 0) {
    FUN_1401b7fd0(&local_d8,param_1,param_2,", \"NextTab\": %d",CONCAT44(uVar12,param_3[5]));
  }
  FUN_1401b7fd0(&local_d8,param_1,param_2,", \"SplitRatio\": %f",(double)(float)param_3[7]);
  if (*(longlong *)(param_3 + 10) == 0) {
    pcVar7 = "false";
    if (param_3[6] != 0) {
      pcVar7 = "true";
    }
    FUN_1401b7fd0(local_e8,param_1,param_2,", \"Vertical\": %s",pcVar7);
    goto LAB_14012f1c6;
  }
  local_c8 = *(undefined8 *)(param_3 + 8);
  uStack_c0 = *(undefined8 *)(param_3 + 10);
  lVar3 = FUN_1401ae920(local_70,&local_c8,0);
  local_c8 = *(undefined8 *)(lVar3 + 0x20);
  uStack_c0 = *(undefined8 *)(lVar3 + 0x28);
  puVar4 = (undefined8 *)FUN_1401b7fd0(local_e8,param_1,param_2,", \"Path\": \"%S\"",&local_c8);
  local_d8 = (char *)*puVar4;
  lStack_d0 = puVar4[1];
  pcVar7 = local_d8;
  for (lVar3 = lStack_d0; lVar3 != 0; lVar3 = lVar3 + -1) {
    if (*pcVar7 == '\\') {
      *pcVar7 = '/';
    }
    pcVar7 = pcVar7 + 1;
  }
  pcVar7 = "false";
  if (*(short *)(param_3 + 0xc) != 0) {
    pcVar7 = "true";
  }
  FUN_1401b7fd0(local_e8,param_1,param_2,", \"Expanded\": %s",pcVar7);
  pcVar7 = "false";
  if (*(short *)((longlong)param_3 + 0x32) != 0) {
    pcVar7 = "true";
  }
  FUN_1401b7fd0(local_e8,param_1,param_2,", \"Hidden\": %s",pcVar7);
  FUN_1401b7fd0(local_e8,param_1,param_2,", \"SortedBy\": %d",param_3[0xd]);
  local_d8 = (&PTR_s_FoldersFirst_1401e7220)[(longlong)(int)param_3[0xe] * 2];
  lStack_d0 = *(undefined8 *)((longlong)(int)param_3[0xe] * 0x10 + 0x1401e7228);
  FUN_1401b7fd0(local_e8,param_1,param_2,", \"FolderSort\": \"%S\"",&local_d8);
  if (*(longlong *)(param_3 + 0x12) != 0) {
    local_d8 = *(char **)(param_3 + 0x10);
    lStack_d0 = *(longlong *)(param_3 + 0x12);
    FUN_1401b7fd0(local_e8,param_1,param_2,", \"Filter\": \"%S\"",&local_d8);
  }
  pcVar7 = "false";
  if (*(char *)(param_3 + 0x14) != '\0') {
    pcVar7 = "true";
  }
  FUN_1401b7fd0(local_e8,param_1,param_2,", \"FilterSticky\": %s",pcVar7);
  pcVar7 = "false";
  if (*(char *)((longlong)param_3 + 0x51) != '\0') {
    pcVar7 = "true";
  }
  FUN_1401b7fd0(local_e8,param_1,param_2,", \"HideFiles\": %s",pcVar7);
  pcVar7 = "false";
  if (*(char *)((longlong)param_3 + 0x52) != '\0') {
    pcVar7 = "true";
  }
  FUN_1401b7fd0(local_e8,param_1,param_2,", \"HideDirs\": %s",pcVar7);
  dVar10 = (double)(float)param_3[0x15];
  FUN_1401b7fd0(local_e8,param_1,param_2,", \"ZoomLevel\": %f",dVar10);
  if (param_3[0x16] != 0) {
    pcVar7 = ", \"Columns\": [";
    uVar8 = uVar5;
    do {
      cVar1 = *pcVar7;
      pcVar7 = pcVar7 + 1;
      if (cVar1 == '\0') break;
      uVar8 = uVar8 + 1;
    } while (pcVar7 != (char *)0x0);
    if ((longlong)uVar8 < 0) {
      uVar2 = (uVar8 ^ (longlong)uVar8 >> 0x3f) - ((longlong)uVar8 >> 0x3f);
      if (uVar2 < uVar8) {
        uVar8 = uVar8 - uVar2;
        goto LAB_14012eabd;
      }
    }
    else {
LAB_14012eabd:
      if (uVar8 == 0) {
        uVar8 = uVar5;
      }
      if (uVar8 != 0) {
        FUN_1401b8310(param_1,param_2,uVar8);
        FUN_1401bacb0(param_2[1] + *param_2,", \"Columns\": [",uVar8);
        param_2[1] = param_2[1] + uVar8;
      }
    }
    if (param_3[0x16] != 0) {
      do {
        uVar12 = (undefined4)((ulonglong)dVar10 >> 0x20);
        if ((int)uVar5 != 0) {
          pcVar7 = ", ";
          uVar8 = 0;
          do {
            cVar1 = *pcVar7;
            pcVar7 = pcVar7 + 1;
            if (cVar1 == '\0') break;
            uVar8 = uVar8 + 1;
          } while (pcVar7 != (char *)0x0);
          if ((longlong)uVar8 < 0) {
            uVar2 = (uVar8 ^ (longlong)uVar8 >> 0x3f) - ((longlong)uVar8 >> 0x3f);
            if (uVar8 <= uVar2) goto LAB_14012eb6e;
            uVar8 = uVar8 - uVar2;
          }
          if (uVar8 == 0) {
            uVar8 = 0;
          }
          if (uVar8 != 0) {
            FUN_1401b8310(param_1,param_2,uVar8);
            FUN_1401bacb0(param_2[1] + *param_2,&DAT_1401cfa70,uVar8);
            param_2[1] = param_2[1] + uVar8;
          }
        }
LAB_14012eb6e:
        dVar10 = (double)CONCAT44(uVar12,param_3[uVar5 * 2 + 0x17]);
        FUN_1401b7fd0(local_e8,param_1,param_2,"{\"Kind\": %d, \"SplitRatio\": %f}",dVar10,
                      (double)(float)param_3[uVar5 * 2 + 0x18]);
        uVar9 = (int)uVar5 + 1;
        uVar5 = (ulonglong)uVar9;
      } while (uVar9 < (uint)param_3[0x16]);
    }
    pcVar7 = "]";
    uVar8 = 0;
    do {
      cVar1 = *pcVar7;
      pcVar7 = pcVar7 + 1;
      if (cVar1 == '\0') break;
      uVar8 = uVar8 + 1;
    } while (pcVar7 != (char *)0x0);
    if ((longlong)uVar8 < 0) {
      uVar5 = (uVar8 ^ (longlong)uVar8 >> 0x3f) - ((longlong)uVar8 >> 0x3f);
      if (uVar8 <= uVar5) goto LAB_14012ec1e;
      uVar8 = uVar8 - uVar5;
    }
    if (uVar8 == 0) {
      uVar8 = 0;
    }
    if (uVar8 != 0) {
      FUN_1401b8310(param_1,param_2,uVar8);
      FUN_1401bacb0(param_2[1] + *param_2,&DAT_1401cf520,uVar8);
      param_2[1] = param_2[1] + uVar8;
    }
  }
LAB_14012ec1e:
  uVar8 = 0;
  if (*(longlong *)(param_3 + 0x34) != 0) {
    pcVar7 = ", \"FilterTypes\": [";
    uVar5 = uVar8;
    do {
      cVar1 = *pcVar7;
      pcVar7 = pcVar7 + 1;
      if (cVar1 == '\0') break;
      uVar5 = uVar5 + 1;
    } while (pcVar7 != (char *)0x0);
    if ((longlong)uVar5 < 0) {
      uVar2 = (uVar5 ^ (longlong)uVar5 >> 0x3f) - ((longlong)uVar5 >> 0x3f);
      if (uVar2 < uVar5) {
        uVar5 = uVar5 - uVar2;
        goto LAB_14012ec63;
      }
    }
    else {
LAB_14012ec63:
      if (uVar5 == 0) {
        uVar5 = uVar8;
      }
      if (uVar5 != 0) {
        FUN_1401b8310(param_1,param_2,uVar5);
        FUN_1401bacb0(param_2[1] + *param_2,", \"FilterTypes\": [",uVar5);
        param_2[1] = param_2[1] + uVar5;
      }
    }
    if (*(longlong *)(param_3 + 0x34) != 0) {
      do {
        if ((int)uVar8 != 0) {
          pcVar7 = ", ";
          uVar5 = 0;
          do {
            cVar1 = *pcVar7;
            pcVar7 = pcVar7 + 1;
            if (cVar1 == '\0') break;
            uVar5 = uVar5 + 1;
          } while (pcVar7 != (char *)0x0);
          if ((longlong)uVar5 < 0) {
            uVar2 = (uVar5 ^ (longlong)uVar5 >> 0x3f) - ((longlong)uVar5 >> 0x3f);
            if (uVar5 <= uVar2) goto LAB_14012ed1e;
            uVar5 = uVar5 - uVar2;
          }
          if (uVar5 == 0) {
            uVar5 = 0;
          }
          if (uVar5 != 0) {
            FUN_1401b8310(param_1,param_2,uVar5);
            FUN_1401bacb0(param_2[1] + *param_2,&DAT_1401cfa70,uVar5);
            param_2[1] = param_2[1] + uVar5;
          }
        }
LAB_14012ed1e:
        puVar4 = (undefined8 *)(*(longlong *)(param_3 + 0x42) + uVar8 * 0x18);
        local_d8 = (char *)*puVar4;
        lStack_d0 = puVar4[1];
        FUN_1401b7fd0(local_e8,param_1,param_2,&DAT_1401cff84,&local_d8);
        uVar8 = (ulonglong)((int)uVar8 + 1);
      } while (uVar8 < *(ulonglong *)(param_3 + 0x34));
    }
    pcVar7 = "]";
    uVar8 = 0;
    do {
      cVar1 = *pcVar7;
      pcVar7 = pcVar7 + 1;
      if (cVar1 == '\0') break;
      uVar8 = uVar8 + 1;
    } while (pcVar7 != (char *)0x0);
    if ((longlong)uVar8 < 0) {
      uVar5 = (uVar8 ^ (longlong)uVar8 >> 0x3f) - ((longlong)uVar8 >> 0x3f);
      if (uVar8 <= uVar5) goto LAB_14012edde;
      uVar8 = uVar8 - uVar5;
    }
    if (uVar8 == 0) {
      uVar8 = 0;
    }
    if (uVar8 != 0) {
      FUN_1401b8310(param_1,param_2,uVar8);
      FUN_1401bacb0(param_2[1] + *param_2,&DAT_1401cf520,uVar8);
      param_2[1] = param_2[1] + uVar8;
    }
  }
LAB_14012edde:
  uVar8 = 0;
  pcVar7 = "false";
  if (param_3[0x45] == 4) goto LAB_14012f1c6;
  FUN_1401b7fd0(local_e8,param_1,param_2,", \"InspectorSplitRatio\": %f",
                (double)(float)param_3[0x44]);
  FUN_1401b7fd0(local_e8,param_1,param_2,", \"InspectorLayout\": %d",param_3[0x45]);
  pcVar6 = "false";
  if (*(char *)(param_3 + 0x46) != '\0') {
    pcVar6 = "true";
  }
  FUN_1401b7fd0(local_e8,param_1,param_2,", \"InspectorDetails\": %s",pcVar6);
  pcVar6 = "false";
  if (*(char *)((longlong)param_3 + 0x119) != '\0') {
    pcVar6 = "true";
  }
  FUN_1401b7fd0(local_e8,param_1,param_2,", \"InspectorDetailsExtra\": %s",pcVar6);
  pcVar6 = "false";
  if (*(char *)((longlong)param_3 + 0x11a) != '\0') {
    pcVar6 = "true";
  }
  FUN_1401b7fd0(local_e8,param_1,param_2,", \"InspectorWordWrap\": %s",pcVar6);
  pcVar6 = "false";
  if (*(char *)((longlong)param_3 + 0x11b) != '\0') {
    pcVar6 = "true";
  }
  FUN_1401b7fd0(local_e8,param_1,param_2,", \"InspectorHideLineNumbers\": %s",pcVar6);
  pcVar6 = "false";
  if (*(char *)(param_3 + 0x47) != '\0') {
    pcVar6 = "true";
  }
  FUN_1401b7fd0(local_e8,param_1,param_2,", \"InspectorShowWhitespace\": %s",pcVar6);
  pcVar6 = "false";
  if (*(char *)((longlong)param_3 + 0x11d) != '\0') {
    pcVar6 = "true";
  }
  FUN_1401b7fd0(local_e8,param_1,param_2,", \"InspectorHideGrid\": %s",pcVar6);
  if (*(char *)((longlong)param_3 + 0x11e) != '\0') {
    pcVar7 = "true";
  }
  FUN_1401b7fd0(local_e8,param_1,param_2,", \"InspectorHideMinimap\": %s",pcVar7);
  if ((param_3[0x48] == 0) || (param_3[0x4b] == 0)) goto LAB_14012f1c6;
  FUN_1401b7fd0(local_e8,param_1,param_2,", \"InspectorSortedBy\": %d",param_3[0x48]);
  local_d8 = (&PTR_s_FoldersFirst_1401e7220)[(longlong)(int)param_3[0x49] * 2];
  lStack_d0 = *(undefined8 *)((longlong)(int)param_3[0x49] * 0x10 + 0x1401e7228);
  FUN_1401b7fd0(local_e8,param_1,param_2,", \"InspectorFolderSort\": \"%S\"",&local_d8);
  dVar10 = (double)(float)param_3[0x4a];
  FUN_1401b7fd0(local_e8,param_1,param_2,", \"InspectorZoomLevel\": %f",dVar10);
  if (param_3[0x4b] == 0) goto LAB_14012f1c6;
  pcVar7 = ", \"InspectorColumns\": [";
  uVar5 = uVar8;
  do {
    cVar1 = *pcVar7;
    pcVar7 = pcVar7 + 1;
    if (cVar1 == '\0') break;
    uVar5 = uVar5 + 1;
  } while (pcVar7 != (char *)0x0);
  if ((longlong)uVar5 < 0) {
    uVar2 = (uVar5 ^ (longlong)uVar5 >> 0x3f) - ((longlong)uVar5 >> 0x3f);
    if (uVar2 < uVar5) {
      uVar5 = uVar5 - uVar2;
      goto LAB_14012f032;
    }
  }
  else {
LAB_14012f032:
    if (uVar5 == 0) {
      uVar5 = uVar8;
    }
    if (uVar5 != 0) {
      FUN_1401b8310(param_1,param_2,uVar5);
      FUN_1401bacb0(param_2[1] + *param_2,", \"InspectorColumns\": [",uVar5);
      param_2[1] = param_2[1] + uVar5;
    }
  }
  if (param_3[0x4b] != 0) {
    do {
      uVar12 = (undefined4)((ulonglong)dVar10 >> 0x20);
      if ((int)uVar8 != 0) {
        pcVar7 = ", ";
        uVar5 = 0;
        do {
          cVar1 = *pcVar7;
          pcVar7 = pcVar7 + 1;
          if (cVar1 == '\0') break;
          uVar5 = uVar5 + 1;
        } while (pcVar7 != (char *)0x0);
        if ((longlong)uVar5 < 0) {
          uVar2 = (uVar5 ^ (longlong)uVar5 >> 0x3f) - ((longlong)uVar5 >> 0x3f);
          if (uVar5 <= uVar2) goto LAB_14012f0e0;
          uVar5 = uVar5 - uVar2;
        }
        if (uVar5 == 0) {
          uVar5 = 0;
        }
        if (uVar5 != 0) {
          FUN_1401b8310(param_1,param_2,uVar5);
          FUN_1401bacb0(param_2[1] + *param_2,&DAT_1401cfa70,uVar5);
          param_2[1] = param_2[1] + uVar5;
        }
      }
LAB_14012f0e0:
      dVar10 = (double)CONCAT44(uVar12,param_3[uVar8 * 2 + 0x4c]);
      FUN_1401b7fd0(local_e8,param_1,param_2,"{\"Kind\": %d, \"SplitRatio\": %f}",dVar10,
                    (double)(float)param_3[uVar8 * 2 + 0x4d]);
      uVar9 = (int)uVar8 + 1;
      uVar8 = (ulonglong)uVar9;
    } while (uVar9 < (uint)param_3[0x4b]);
  }
  pcVar7 = "]";
  uVar8 = 0;
  do {
    cVar1 = *pcVar7;
    pcVar7 = pcVar7 + 1;
    if (cVar1 == '\0') break;
    uVar8 = uVar8 + 1;
  } while (pcVar7 != (char *)0x0);
  if ((longlong)uVar8 < 0) {
    uVar5 = (uVar8 ^ (longlong)uVar8 >> 0x3f) - ((longlong)uVar8 >> 0x3f);
    if (uVar8 <= uVar5) goto LAB_14012f1c6;
    uVar8 = uVar8 - uVar5;
  }
  if (uVar8 == 0) {
    uVar8 = 0;
  }
  if (uVar8 != 0) {
    FUN_1401b8310(param_1,param_2,uVar8);
    FUN_1401bacb0(param_2[1] + *param_2,&DAT_1401cf520,uVar8);
    param_2[1] = param_2[1] + uVar8;
  }
LAB_14012f1c6:
  pcVar7 = " }";
  uVar8 = 0;
  do {
    cVar1 = *pcVar7;
    pcVar7 = pcVar7 + 1;
    if (cVar1 == '\0') break;
    uVar8 = uVar8 + 1;
  } while (pcVar7 != (char *)0x0);
  if ((longlong)uVar8 < 0) {
    uVar5 = (uVar8 ^ (longlong)uVar8 >> 0x3f) - ((longlong)uVar8 >> 0x3f);
    if (uVar8 <= uVar5) {
      return;
    }
    uVar8 = uVar8 - uVar5;
  }
  if (uVar8 == 0) {
    uVar8 = 0;
  }
  if (uVar8 != 0) {
    FUN_1401b8310(param_1,param_2,uVar8);
    FUN_1401bacb0(param_2[1] + *param_2,&DAT_1401cfa98,uVar8);
    param_2[1] = param_2[1] + uVar8;
  }
  return;
}

