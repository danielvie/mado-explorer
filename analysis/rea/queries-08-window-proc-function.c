
/* WARNING: Type propagation algorithm not settling */

LRESULT FUN_1401b2e80(HWND param_1,uint param_2,ulonglong param_3,int *param_4)

{
  byte bVar1;
  longlong lVar2;
  undefined8 uVar3;
  longlong lVar4;
  char cVar5;
  ushort uVar6;
  undefined4 uVar7;
  int iVar8;
  int iVar9;
  BOOL BVar10;
  uint uVar11;
  HICON lParam;
  ulonglong uVar12;
  HANDLE hProcess;
  UINT_PTR UVar13;
  short *psVar14;
  undefined8 *puVar15;
  ushort *puVar16;
  longlong lVar17;
  ulonglong uVar18;
  tagPOINT tVar19;
  int *piVar20;
  int *piVar21;
  wchar_t *pwVar22;
  ushort uVar23;
  short sVar24;
  uint uVar25;
  int iVar26;
  LRESULT LVar27;
  ulonglong uVar28;
  ulonglong uVar29;
  int local_res10 [2];
  uint local_2a8;
  tagRECT local_298;
  tagPOINT local_288;
  tagPOINT local_280 [3];
  int local_268;
  int iStack_264;
  int iStack_260;
  int iStack_25c;
  undefined1 local_258 [8];
  HWND local_250;
  undefined8 uStack_248;
  RECT local_240;
  LPARAM local_230;
  undefined4 local_208;
  undefined4 uStack_204;
  undefined4 uStack_200;
  undefined4 uStack_1fc;
  undefined1 local_1f8 [448];
  
  lVar4 = DAT_1401e8b90;
  uVar28 = 0;
  LVar27 = 0;
  uVar6 = (ushort)((ulonglong)param_4 >> 0x10);
  if (0x100 < param_2) {
    if (param_2 < 0x201) {
      if (param_2 == 0x200) goto LAB_1401b3b2b;
      uVar23 = (ushort)param_3;
      switch(param_2) {
      case 0x101:
      case 0x104:
      case 0x105:
        goto switchD_1401b35a4_caseD_101;
      case 0x102:
        LOCK();
        if (DAT_1401e8bb8 == 0) {
          DAT_1401e8bb8 = 0;
        }
        UNLOCK();
        *(int *)(DAT_1401e8b90 + 0x50) = DAT_1401e8bb8;
        local_res10[0] = CONCAT22(local_res10[0]._2_2_,uVar23);
        if ((0x1f < uVar23) && (uVar23 != *(ushort *)(lVar4 + 0x138))) {
          if ((*(longlong *)(lVar4 + 0x6c8) == 0) ||
             ((psVar14 = *(short **)(lVar4 + 0x6d8), *psVar14 == 0 ||
              ((int *)(longlong)*(int *)(psVar14 + 0x10) != param_4)))) {
            psVar14 = (short *)FUN_1401ae840(lVar4);
            psVar14[4] = 1;
            psVar14[5] = 0;
            param_3 = (ulonglong)(ushort)local_res10[0];
          }
          if (((short)param_3 != 0x20) || ((*(byte *)((longlong)psVar14 + 5) & 6) == 0)) {
            local_298._0_8_ = local_res10;
            psVar14[6] = 1;
            psVar14[7] = 0;
            local_298.right = 1;
            local_298.bottom = 0;
            puVar15 = (undefined8 *)
                      FUN_140188d50(local_280,lVar4,(tagPOINT *)&local_298,lVar4 + 0x700);
            uVar3 = puVar15[1];
            *(undefined8 *)(psVar14 + 8) = *puVar15;
            *(undefined8 *)(psVar14 + 0xc) = uVar3;
          }
        }
        *(undefined2 *)(lVar4 + 0x138) = 0;
        return 0;
      case 0x103:
        *(ushort *)(DAT_1401e8b90 + 0x138) = uVar23;
        return 0;
      case 0x106:
        return 0;
      }
    }
    else {
      sVar24 = (short)(param_3 >> 0x10);
      if (param_2 < 0x31e) {
        if (param_2 == 0x31d) {
          *(undefined4 *)(DAT_1401e8b90 + 0x898) = 1;
          return 0;
        }
        switch(param_2) {
        case 0x201:
        case 0x203:
          LOCK();
          if (DAT_1401e8bb8 == 0) {
            DAT_1401e8bb8 = 0;
          }
          UNLOCK();
          *(int *)(DAT_1401e8b90 + 0x50) = DAT_1401e8bb8;
          FUN_1401ad540(lVar4,0,0);
          return 0;
        case 0x202:
          LOCK();
          if (DAT_1401e8bb8 == 0) {
            DAT_1401e8bb8 = 0;
          }
          UNLOCK();
          *(int *)(DAT_1401e8b90 + 0x50) = DAT_1401e8bb8;
          cVar5 = *(char *)(lVar4 + 0x100);
          *(undefined1 *)(lVar4 + 0x100) = 0;
          if (cVar5 == '\0') {
            cVar5 = '\x01';
          }
          *(char *)(lVar4 + 0x102) = cVar5;
          ReleaseCapture();
          return 0;
        case 0x204:
        case 0x206:
          LOCK();
          if (DAT_1401e8bb8 == 0) {
            DAT_1401e8bb8 = 0;
          }
          UNLOCK();
          *(int *)(DAT_1401e8b90 + 0x50) = DAT_1401e8bb8;
          FUN_1401ad540(lVar4,1);
          return 0;
        case 0x205:
          LOCK();
          if (DAT_1401e8bb8 == 0) {
            DAT_1401e8bb8 = 0;
          }
          UNLOCK();
          *(int *)(DAT_1401e8b90 + 0x50) = DAT_1401e8bb8;
          cVar5 = *(char *)(lVar4 + 0x108);
          *(undefined1 *)(lVar4 + 0x108) = 0;
          if (cVar5 == '\0') {
            cVar5 = '\x01';
          }
          *(char *)(lVar4 + 0x10a) = cVar5;
          ReleaseCapture();
          return 0;
        case 0x207:
        case 0x209:
          LOCK();
          if (DAT_1401e8bb8 == 0) {
            DAT_1401e8bb8 = 0;
          }
          UNLOCK();
          *(int *)(DAT_1401e8b90 + 0x50) = DAT_1401e8bb8;
          FUN_1401ad540(lVar4,2);
          return 0;
        case 0x208:
          LOCK();
          if (DAT_1401e8bb8 == 0) {
            DAT_1401e8bb8 = 0;
          }
          UNLOCK();
          *(int *)(DAT_1401e8b90 + 0x50) = DAT_1401e8bb8;
          cVar5 = *(char *)(lVar4 + 0x110);
          *(undefined1 *)(lVar4 + 0x110) = 0;
          if (cVar5 == '\0') {
            cVar5 = '\x01';
          }
          *(char *)(lVar4 + 0x112) = cVar5;
          ReleaseCapture();
          return 0;
        case 0x20a:
          LOCK();
          if (DAT_1401e8bb8 == 0) {
            DAT_1401e8bb8 = 0;
          }
          UNLOCK();
          *(int *)(DAT_1401e8b90 + 0x50) = DAT_1401e8bb8;
          *(short *)(lVar4 + 0x11a) = *(short *)(lVar4 + 0x11a) + sVar24;
          return 0;
        case 0x20b:
        case 0x20c:
        case 0x20d:
          LOCK();
          if (DAT_1401e8bb8 == 0) {
            DAT_1401e8bb8 = 0;
          }
          UNLOCK();
          *(int *)(DAT_1401e8b90 + 0x50) = DAT_1401e8bb8;
          puVar16 = (ushort *)FUN_1401ae840(lVar4);
          *puVar16 = (ushort)((param_2 - 0x20b & 0xfffffffd) == 0);
          puVar16[4] = 1;
          puVar16[5] = 0;
          puVar16[1] = (ushort)(param_2 == 0x20c);
          *(char *)(puVar16 + 2) = (sVar24 != 1) + '=';
          return 0;
        case 0x20e:
          LOCK();
          if (DAT_1401e8bb8 == 0) {
            DAT_1401e8bb8 = 0;
          }
          UNLOCK();
          *(int *)(DAT_1401e8b90 + 0x50) = DAT_1401e8bb8;
          *(short *)(lVar4 + 0x118) = *(short *)(lVar4 + 0x118) - sVar24;
          return 0;
        case 0x219:
          if ((param_3 != 0x8001) && (param_3 != 0x8004)) {
            return 0;
          }
          if (param_4[1] != 6) {
            return 0;
          }
          tVar19 = *(tagPOINT *)(param_4 + 4);
          iVar26 = 0;
          local_2a8 = 0;
          uVar18 = uVar28;
          uVar12 = uVar28;
          uVar29 = uVar28;
          local_280[0] = tVar19;
          if (*(longlong *)(DAT_1401e8b90 + 0x1310) != 0) {
            do {
              lVar2 = *(longlong *)(*(longlong *)(lVar4 + 0x1348) + 0x10 + uVar12);
              if (*(tagPOINT *)(lVar2 + 0x260) == tVar19) {
                local_268 = *(int *)(lVar2 + 0x40);
                iStack_264 = *(undefined4 *)(lVar2 + 0x44);
                iStack_260 = *(undefined4 *)(lVar2 + 0x48);
                iStack_25c = *(undefined4 *)(lVar2 + 0x4c);
                FUN_1400c3130("C:\\Users\\Vjekoslav\\Projects\\Voidstar\\File Pilot\\Source\\Win\\WinMain.c"
                              ,"Win_WindowProc",0x3c9,
                              "Device removed for watcher on volume \'%S\'\n",&local_268);
                FUN_1401945d0(lVar2,0);
                uVar18 = uVar28;
                do {
                  LOCK();
                  lVar17 = *(longlong *)(lVar2 + 0x280);
                  if (lVar17 == 0) {
                    *(longlong *)(lVar2 + 0x280) = 0;
                    lVar17 = 0;
                  }
                  UNLOCK();
                  local_208 = *(undefined4 *)(lVar2 + 0x40);
                  uStack_204 = *(undefined4 *)(lVar2 + 0x44);
                  uStack_200 = *(undefined4 *)(lVar2 + 0x48);
                  uStack_1fc = *(undefined4 *)(lVar2 + 0x4c);
                  uVar11 = (int)uVar18 + 1;
                  uVar25 = (uint)(lVar17 == 1);
                  uVar18 = (ulonglong)uVar11;
                  FUN_1400c3130("C:\\Users\\Vjekoslav\\Projects\\Voidstar\\File Pilot\\Source\\Win\\WinMain.c"
                                ,"Win_WindowProc",0x3d2,
                                "Force closing dir watcher for \'%S\', %d. try, success: %d\n",
                                &local_208,uVar11,uVar25);
                  if (uVar25 != 0) {
                    uVar18 = 1;
                    local_2a8 = 1;
                    goto LAB_1401b3896;
                  }
                  Sleep(100);
                } while ((int)uVar11 < 5);
                uVar18 = (ulonglong)local_2a8;
LAB_1401b3896:
                SetEvent(*(HANDLE *)(lVar2 + 0x278));
                tVar19 = local_280[0];
              }
              iVar26 = (int)uVar18;
              uVar29 = uVar29 + 1;
              uVar12 = uVar12 + 0x18;
            } while (uVar29 < *(ulonglong *)(lVar4 + 0x1310));
          }
          return (longlong)iVar26;
        case 0x232:
          *(undefined4 *)(DAT_1401e8b90 + 0x84) = 0;
          if (*(int *)(lVar4 + 0x6c) != 0) {
            *(undefined4 *)(lVar4 + 0x68) = 1;
          }
          *(undefined4 *)(lVar4 + 0x6c) = 0;
          return 0;
        case 0x2e0:
          *(uint *)(DAT_1401e8b90 + 100) = (uint)(param_3 >> 0x10) & 0xffff;
          piVar20 = param_4;
          FUN_14018cdb0();
          SetWindowPos(param_1,(HWND)0x0,*param_4,piVar20[1],param_4[2] - *param_4,
                       param_4[3] - piVar20[1],0x14);
          if (*(int *)(lVar4 + 0x84) != 0) {
            *(undefined4 *)(lVar4 + 0x6c) = 1;
            return 0;
          }
          *(undefined4 *)(lVar4 + 0x68) = 1;
          return 0;
        }
      }
      else if (param_2 < 0x8004) {
        if (param_2 == 0x8003) {
LAB_1401b3b2b:
          LOCK();
          if (DAT_1401e8bb8 == 0) {
            DAT_1401e8bb8 = 0;
          }
          UNLOCK();
          *(int *)(DAT_1401e8b90 + 0x50) = DAT_1401e8bb8;
          return 0;
        }
        if (param_2 == 800) goto LAB_1401b3006;
        if ((param_2 == 0x8001) || (param_2 == 0x8002)) goto LAB_1401b3b2b;
      }
      else {
        if (param_2 == 0x8005) {
          if (*(char *)(DAT_1401e8b90 + 0x9a8) == '\0') {
            return 0;
          }
          LOCK();
          if (DAT_1401e8bb8 == 0) {
            DAT_1401e8bb8 = 0;
          }
          UNLOCK();
LAB_1401b3b82:
          *(int *)(DAT_1401e8b90 + 0x50) = DAT_1401e8bb8;
          *(short *)(lVar4 + 0x11a) = sVar24;
          return 0;
        }
        if (param_2 == 0x8006) {
          if (*(char *)(DAT_1401e8b90 + 0x9a8) == '\0') {
            return 0;
          }
          LOCK();
          if (DAT_1401e8bb8 == 0) {
            DAT_1401e8bb8 = 0;
          }
          UNLOCK();
          sVar24 = -sVar24;
          goto LAB_1401b3b82;
        }
      }
    }
    goto switchD_1401b2ef5_caseD_4;
  }
  if (param_2 == 0x100) {
switchD_1401b35a4_caseD_101:
    if (((param_3 == 0x73) || (param_3 == 9)) && ((uVar6 >> 0xd & 1) != 0)) {
      LVar27 = DefWindowProcW(param_1,param_2,param_3,(LPARAM)param_4);
      return LVar27;
    }
    LOCK();
    if (DAT_1401e8bb8 == 0) {
      DAT_1401e8bb8 = 0;
    }
    UNLOCK();
    *(int *)(DAT_1401e8b90 + 0x50) = DAT_1401e8bb8;
    if ((uint)param_3 == 0xd) {
      if (((ulonglong)param_4 >> 0x18 & 1) != 0) {
        param_3 = 0x6c;
      }
    }
    else if (0xfe < (uint)param_3) {
      return 0;
    }
    bVar1 = *(byte *)((param_3 & 0xffffffff) + 0x18c + lVar4);
    if (bVar1 < 3) {
      return 0;
    }
    if (bVar1 == 4) {
      return 0;
    }
    uVar11 = (uint)param_4 & 0xffff;
    puVar16 = (ushort *)FUN_1401ae840(lVar4);
    *(byte *)(puVar16 + 2) = bVar1;
    *(uint *)(puVar16 + 0x10) = (uint)param_4;
    *puVar16 = (ushort)(((ulonglong)param_4 & 0xffffffff80000000) == 0);
    puVar16[1] = (ushort)(((ulonglong)param_4 & 0xffffffff80000000) != 0);
    if (((ulonglong)param_4 & 0xffff) == 0) {
      uVar11 = 1;
    }
    *(uint *)(puVar16 + 4) = uVar11;
    return 0;
  }
  switch(param_2) {
  case 1:
    lParam = LoadIconW(*(HINSTANCE *)(param_4 + 2),(LPCWSTR)0x65);
    SendMessageW(param_1,0x80,1,(LPARAM)lParam);
    SendMessageW(param_1,0x80,0,(LPARAM)lParam);
    break;
  case 2:
    if (*(HDEVNOTIFY *)(DAT_1401e8b90 + 0xa18) != (HDEVNOTIFY)0x0) {
      UnregisterDeviceNotification(*(HDEVNOTIFY *)(DAT_1401e8b90 + 0xa18));
    }
    PostQuitMessage(0);
    break;
  case 3:
    FUN_140196fa0(DAT_1401e8b90);
  default:
    goto switchD_1401b2ef5_caseD_4;
  case 5:
    if (param_3 == 0) {
      LOCK();
      if (DAT_1401e8bb8 == 0) {
        DAT_1401e8bb8 = 0;
      }
      UNLOCK();
      *(int *)(DAT_1401e8b90 + 0x50) = DAT_1401e8bb8;
    }
  case 0xf:
    BeginPaint(param_1,(LPPAINTSTRUCT)local_258);
    FUN_1401bace0(local_1f8,0,0x1b8);
    FUN_1401a9d60(lVar4,1,local_1f8);
    EndPaint(param_1,(PAINTSTRUCT *)local_258);
    break;
  case 8:
    LOCK();
    if (DAT_1401e8bb8 == 0) {
      DAT_1401e8bb8 = 0;
    }
    UNLOCK();
    *(int *)(DAT_1401e8b90 + 0x50) = DAT_1401e8bb8;
    if (*(int *)(lVar4 + 0x8f8) == 0) {
      hProcess = GetCurrentProcess();
      SetProcessWorkingSetSize(hProcess,0xffffffffffffffff,0xffffffffffffffff);
      ReleaseCapture();
    }
    break;
  case 0x1a:
    uVar18 = uVar28;
    piVar20 = param_4;
    if (param_4 == (int *)0x0) {
      return 0;
    }
    do {
      piVar21 = (int *)((longlong)piVar20 + 2);
      if ((short)*piVar20 == 0) break;
      uVar18 = uVar18 + 1;
      piVar20 = piVar21;
    } while (piVar21 != (int *)0x0);
    if ((longlong)uVar18 < 0) {
      uVar12 = (uVar18 ^ (longlong)uVar18 >> 0x3f) - ((longlong)uVar18 >> 0x3f);
      if (uVar12 < uVar18) {
        uVar18 = uVar18 - uVar12;
        goto LAB_1401b2f80;
      }
      pwVar22 = L"ImmersiveColorSet";
      uVar18 = uVar28;
LAB_1401b2fc2:
      if (*pwVar22 == L'\0') goto LAB_1401b3006;
    }
    else {
LAB_1401b2f80:
      if (uVar18 == 0) {
        uVar18 = uVar28;
      }
      pwVar22 = L"ImmersiveColorSet";
      uVar12 = uVar28;
      if (uVar18 != 0) {
        do {
          if (*pwVar22 == L'\0') break;
          if (*pwVar22 != *(wchar_t *)((longlong)param_4 + uVar12 * 2)) {
            pwVar22 = L"ColorPrevalence";
            goto LAB_1401b2fd4;
          }
          uVar12 = uVar12 + 1;
          pwVar22 = pwVar22 + 1;
        } while (uVar12 < uVar18);
      }
      if (uVar12 == uVar18) goto LAB_1401b2fc2;
    }
    pwVar22 = L"ColorPrevalence";
    if (uVar18 != 0) {
LAB_1401b2fd4:
      do {
        if (*pwVar22 == L'\0') break;
        if (*pwVar22 != *(wchar_t *)((longlong)param_4 + uVar28 * 2)) {
          return 0;
        }
        uVar28 = uVar28 + 1;
        pwVar22 = pwVar22 + 1;
      } while (uVar28 < uVar18);
    }
    if (uVar28 != uVar18) {
      return 0;
    }
    if (*pwVar22 != L'\0') {
      return 0;
    }
LAB_1401b3006:
    uVar7 = FUN_1401b0600(DAT_1401e8b90 + 0xa0);
    *(undefined4 *)(lVar4 + 0x9c) = uVar7;
    return 0;
  case 0x20:
    if (*(int *)(DAT_1401e8b90 + 0x1a40) != 0) {
      return 0;
    }
    goto switchD_1401b2ef5_caseD_4;
  case 0x24:
    param_4[6] = *(int *)(DAT_1401e8b90 + 0x888);
    param_4[7] = *(int *)(lVar4 + 0x88c);
    break;
  case 0x83:
    local_res10[0] = 0;
    if ((DAT_1401e8b88 == (code *)0x0) || (*(int *)(DAT_1401e8b90 + 0x60) == 0)) {
      iVar26 = GetSystemMetrics(0x5c);
      iVar9 = GetSystemMetrics(0x20);
      iVar26 = iVar26 + iVar9;
      iVar9 = GetSystemMetrics(0x5c);
      iVar8 = GetSystemMetrics(0x21);
    }
    else {
      iVar26 = (*DAT_1401e8b88)(0x5c,*(undefined4 *)(DAT_1401e8b90 + 100));
      iVar9 = (*DAT_1401e8b88)(0x20,*(undefined4 *)(lVar4 + 100));
      iVar26 = iVar26 + iVar9;
      iVar9 = (*DAT_1401e8b88)(0x5c,*(undefined4 *)(lVar4 + 100));
      iVar8 = (*DAT_1401e8b88)(0x21,*(undefined4 *)(lVar4 + 100));
    }
    iVar9 = iVar9 + iVar8;
    BVar10 = IsZoomed(param_1);
    if (BVar10 != 0) {
      local_258._0_4_ = 0x30;
      local_230 = 0;
      local_250 = (HWND)0x0;
      uStack_248._0_4_ = 0;
      uStack_248._4_4_ = 0;
      local_240.left = 0;
      local_240.top = 0;
      local_240.right = 0;
      local_240.bottom = 0;
      local_res10[0] = iVar9;
      local_250 = FindWindowW(L"Shell_TrayWnd",(LPCWSTR)0x0);
      if ((((local_250 != (HWND)0x0) &&
           (UVar13 = SHAppBarMessage(4,(PAPPBARDATA)local_258), (UVar13 & 1) != 0)) &&
          (UVar13 = SHAppBarMessage(5,(PAPPBARDATA)local_258), UVar13 != 0)) &&
         (uStack_248._4_4_ == 3)) {
        iVar9 = iVar9 + 1;
      }
    }
    *param_4 = *param_4 + iVar26;
    param_4[2] = param_4[2] - iVar26;
    param_4[3] = param_4[3] - iVar9;
    param_4[1] = param_4[1] + local_res10[0];
    break;
  case 0x84:
    LOCK();
    if (DAT_1401e8bb8 == 0) {
      DAT_1401e8bb8 = 0;
    }
    UNLOCK();
    *(int *)(DAT_1401e8b90 + 0x50) = DAT_1401e8bb8;
    iVar26 = (int)(short)param_4;
    local_280[0].y = (int)(short)uVar6;
    local_280[0].x = iVar26;
    local_288 = local_280[0];
    ScreenToClient(param_1,&local_288);
    local_298.left = 0;
    local_298.top = 0;
    local_298.right = 0;
    local_298.bottom = 0;
    GetWindowRect(param_1,&local_298);
    if ((DAT_1401e8b88 == (code *)0x0) || (*(int *)(lVar4 + 0x60) == 0)) {
      local_res10[0] = GetSystemMetrics(0x5c);
      iVar9 = GetSystemMetrics(0x20);
      local_res10[0] = local_res10[0] + iVar9;
      local_2a8 = GetSystemMetrics(0x5c);
      iVar9 = GetSystemMetrics(0x21);
    }
    else {
      local_res10[0] = (*DAT_1401e8b88)(0x5c,*(undefined4 *)(lVar4 + 100));
      iVar9 = (*DAT_1401e8b88)(0x20,*(undefined4 *)(lVar4 + 100));
      local_res10[0] = local_res10[0] + iVar9;
      local_2a8 = (*DAT_1401e8b88)(0x5c,*(undefined4 *)(lVar4 + 100));
      iVar9 = (*DAT_1401e8b88)(0x21,*(undefined4 *)(lVar4 + 100));
    }
    iVar8 = *(int *)(lVar4 + 0x884);
    if ((-1 < (longlong)local_288) && (local_288.y < *(int *)(lVar4 + 0x880))) {
      LVar27 = 2;
      if (*(ulonglong *)(lVar4 + 0x768) != 0) {
        do {
          piVar20 = (int *)(*(longlong *)(lVar4 + 0x778) + uVar28 * 0x10);
          local_268 = *piVar20;
          iStack_264 = piVar20[1];
          iStack_260 = piVar20[2];
          iStack_25c = piVar20[3];
          if ((((local_268 <= local_288.x) && (iStack_264 <= local_288.y)) &&
              (local_288.x < iStack_260)) && (local_288.y < iStack_25c)) {
            LVar27 = 1;
          }
          uVar28 = (ulonglong)((int)uVar28 + 1);
        } while (uVar28 < *(ulonglong *)(lVar4 + 0x768));
      }
      BVar10 = IsZoomed(param_1);
      if (BVar10 != 0) {
        return LVar27;
      }
      if ((local_298.left <= local_280[0].x) && (local_280[0].x < local_298.left + local_res10[0]))
      {
        if (iVar8 <= local_288.y) {
          return 10;
        }
        return 0xd;
      }
      if ((local_280[0].x < local_298.right) && (local_298.right - local_res10[0] < local_280[0].x))
      {
        if (iVar8 <= local_288.y) {
          return 0xb;
        }
        return 0xe;
      }
      if (iVar8 <= local_288.y) {
        return LVar27;
      }
      return 0xc;
    }
    if (*(int *)(lVar4 + 0x88) != 0) {
      if ((local_280[0].y < local_298.bottom) &&
         ((int)(local_298.bottom - (local_2a8 + iVar9)) <= local_280[0].y)) {
        if (iVar26 < local_298.left + local_res10[0]) {
          return 0x10;
        }
        if (local_280[0].x < local_298.right - local_res10[0]) {
          return 0xf;
        }
        return 0x11;
      }
      if (iVar26 < local_298.left + local_res10[0]) {
        return 10;
      }
      if (local_298.right - local_res10[0] <= iVar26) {
        return 0xb;
      }
    }
switchD_1401b2ef5_caseD_4:
    LVar27 = DefWindowProcW(param_1,param_2,param_3,(LPARAM)param_4);
    break;
  case 0xa1:
  case 0xa4:
  case 0xa7:
    if (param_3 == 2) {
      *(undefined4 *)(DAT_1401e8b90 + 0x84) = 1;
      local_298.left = 0;
      local_298.top = 0;
      local_298.right = 0;
      local_298.bottom = 0;
      GetWindowRect(param_1,&local_298);
      local_280[0].x = local_298.left;
      local_280[0].y = local_298.top;
      ((tagPOINT *)(lVar4 + 0x70))->x = local_298.left;
      ((tagPOINT *)(lVar4 + 0x70))->y = local_298.top;
      LOCK();
      if (DAT_1401e8bb8 == 0) {
        DAT_1401e8bb8 = 0;
      }
      UNLOCK();
      *(int *)(lVar4 + 0x50) = DAT_1401e8bb8;
      uVar28 = (ulonglong)(param_2 == 0xa4);
      if (param_2 == 0xa7) {
        uVar28 = 2;
      }
      *(undefined4 *)(lVar4 + 0x78 + uVar28 * 4) = 1;
    }
    LVar27 = DefWindowProcW(param_1,param_2,param_3,(LPARAM)param_4);
  }
  return LVar27;
}

