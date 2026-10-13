
void FUN_1401916a0(longlong *param_1)

{
  short sVar1;
  longlong lVar2;
  LPCWSTR lpFileName;
  UINT UVar3;
  uint uVar4;
  DWORD DVar5;
  undefined4 uVar6;
  BOOL BVar7;
  int iVar8;
  char *pcVar9;
  HANDLE pvVar10;
  ulonglong uVar11;
  HDEVNOTIFY pvVar12;
  ulonglong uVar13;
  short *psVar14;
  longlong lVar15;
  ulonglong uVar16;
  LPVOID pvVar17;
  ulonglong uVar18;
  char *pcVar19;
  char *pcVar20;
  int iVar21;
  ulonglong uVar22;
  longlong lVar23;
  longlong extraout_XMM0_Qa;
  double dVar24;
  LPDWORD lpBytesReturned;
  DWORD local_280;
  DWORD local_27c;
  LPCWSTR local_278;
  longlong lStack_270;
  undefined8 local_268;
  LPCWSTR local_260;
  longlong lStack_258;
  undefined8 local_250;
  longlong local_240;
  longlong lStack_238;
  HANDLE local_230;
  longlong lStack_228;
  longlong local_220;
  longlong lStack_218;
  longlong local_210;
  longlong lStack_208;
  longlong local_200;
  longlong lStack_1f8;
  longlong local_1f0;
  longlong lStack_1e8;
  _OVERLAPPED local_1e0;
  longlong local_1c0;
  longlong lStack_1b8;
  longlong local_1b0;
  longlong lStack_1a8;
  longlong local_1a0;
  longlong lStack_198;
  undefined8 local_190 [2];
  longlong local_180;
  longlong lStack_178;
  longlong local_170;
  longlong lStack_168;
  undefined8 local_160;
  undefined8 uStack_158;
  undefined8 local_150;
  undefined4 local_148;
  undefined4 local_144;
  undefined8 local_140;
  HANDLE local_138;
  undefined8 local_130;
  undefined8 uStack_128;
  undefined8 local_120;
  undefined8 uStack_118;
  longlong local_100;
  longlong lStack_f8;
  longlong local_f0;
  longlong lStack_e8;
  longlong local_e0;
  longlong lStack_d8;
  longlong local_d0;
  longlong lStack_c8;
  undefined8 local_c0;
  undefined8 uStack_b8;
  undefined8 local_b0;
  undefined8 uStack_a8;
  undefined8 local_a0;
  undefined8 uStack_98;
  undefined8 local_90;
  undefined8 uStack_88;
  undefined8 local_80;
  undefined8 uStack_78;
  undefined8 uStack_70;
  undefined8 uStack_68;
  undefined8 uStack_60;
  undefined8 local_58;
  undefined8 uStack_50;
  undefined8 local_48;
  
  local_278 = (LPCWSTR)0x0;
  lStack_270 = 0;
  local_268 = 0;
  local_230 = (HANDLE)param_1[10];
  lVar15 = param_1[0xb];
  lStack_228 = lVar15;
  if (lVar15 != 0) {
    FUN_1401b9a40(lVar15,&local_278,lVar15);
    lVar23 = lStack_270;
    FUN_1401bacb0(local_278 + lStack_270,local_230,lVar15 * 2);
    lStack_270 = lVar23 + lVar15;
  }
  FUN_1401b9a40(param_1,&local_278,1);
  local_278[lStack_270] = L'\\';
  lStack_270 = lStack_270 + 1;
  FUN_1401b9a40(param_1,&local_278,1);
  lpFileName = local_278;
  uVar22 = 0;
  local_278[lStack_270] = L'\0';
  UVar3 = GetDriveTypeW(local_278);
  local_1c0 = param_1[8];
  lStack_1b8 = param_1[9];
  pcVar19 = "SYNC";
  pcVar20 = "ASYNC";
  uVar4 = (uint)(UVar3 != 4);
  *(uint *)(param_1 + 0x49) = uVar4;
  DVar5 = 0x2000000;
  if (UVar3 != 4) {
    DVar5 = 0x42000000;
  }
  pcVar9 = "SYNC";
  if (uVar4 != 0) {
    pcVar9 = "ASYNC";
  }
  FUN_1400c3130("C:\\Users\\Vjekoslav\\Projects\\Voidstar\\File Pilot\\Source\\Win\\WinMain.c",
                "Win_DirWatcherProc",0x2862,"Starting dir watcher for \'%S\' in %s mode\n",
                &local_1c0,pcVar9);
  local_1f0 = param_1[8];
  lStack_1e8 = param_1[9];
  if ((int)param_1[0x49] != 0) {
    pcVar19 = pcVar20;
  }
  FUN_140191480(param_1,"\'%S\' - Start in %s mode",&local_1f0,pcVar19);
  pvVar10 = CreateFileW(lpFileName,1,7,(LPSECURITY_ATTRIBUTES)0x0,3,DVar5,(HANDLE)0x0);
  if (pvVar10 == (HANDLE)0xffffffffffffffff) {
    local_220 = param_1[8];
    lStack_218 = param_1[9];
    local_240 = FUN_1400c3130("C:\\Users\\Vjekoslav\\Projects\\Voidstar\\File Pilot\\Source\\Win\\WinMain.c"
                              ,"Win_DirWatcherProc",0x292a,"Dir_WatchWorkStart FAILED \'%S\'\n",
                              &local_220);
    FUN_140191480(param_1,"\'%S\' - Dir_WatchWorkStart FAILED",&local_240);
    LOCK();
    param_1[0x50] = param_1[0x50] + 1;
    UNLOCK();
    WaitForSingleObject((HANDLE)param_1[0x4f],0xffffffff);
    goto LAB_14019235d;
  }
  param_1[0x4b] = (longlong)pvVar10;
  psVar14 = &DAT_1401d4780;
  local_150 = 0;
  local_160 = 0;
  uStack_158 = 0;
  local_260 = (LPCWSTR)0x0;
  lStack_258 = 0;
  local_250 = 0;
  uVar16 = uVar22;
  do {
    sVar1 = *psVar14;
    psVar14 = psVar14 + 1;
    if (sVar1 == 0) break;
    uVar16 = uVar16 + 1;
  } while (psVar14 != (short *)0x0);
  if ((longlong)uVar16 < 0) {
    uVar11 = (uVar16 ^ (longlong)uVar16 >> 0x3f) - ((longlong)uVar16 >> 0x3f);
    if (uVar11 < uVar16) {
      uVar16 = uVar16 - uVar11;
      goto LAB_1401918af;
    }
  }
  else {
LAB_1401918af:
    if (uVar16 == 0) {
      uVar16 = uVar22;
    }
    if (uVar16 != 0) {
      FUN_1401b9a40(param_1,&local_260,uVar16);
      lVar15 = lStack_258;
      FUN_1401bacb0(local_260 + lStack_258,&DAT_1401d4780,uVar16 * 2);
      lStack_258 = lVar15 + uVar16;
    }
  }
  local_230 = (HANDLE)param_1[10];
  lVar15 = param_1[0xb];
  lStack_228 = lVar15;
  if (lVar15 != 0) {
    FUN_1401b9a40(param_1,&local_260,lVar15);
    lVar23 = lStack_258;
    FUN_1401bacb0(local_260 + lStack_258,local_230,lVar15 * 2);
    lStack_258 = lVar23 + lVar15;
  }
  FUN_1401b9a40(param_1,&local_260,1);
  lpBytesReturned = (LPDWORD)0x0;
  local_260[lStack_258] = L'\0';
  pvVar10 = CreateFileW(local_260,0x80000000,3,(LPSECURITY_ATTRIBUTES)0x0,3,0,(HANDLE)0x0);
  if (pvVar10 == (HANDLE)0xffffffffffffffff) {
    GetLastError();
  }
  else {
    param_1[0x4c] = (longlong)pvVar10;
    local_140 = 0;
    local_148 = 0x38;
    local_130 = 0;
    uStack_128 = 0;
    local_144 = 6;
    local_120 = 0;
    uStack_118 = 0;
    local_138 = pvVar10;
    pvVar12 = RegisterDeviceNotificationW((HANDLE)param_1[0x4a],&local_148,0);
    param_1[0x4d] = (longlong)pvVar12;
  }
  pvVar10 = CreateEventW((LPSECURITY_ATTRIBUTES)0x0,1,0,(LPCWSTR)0x0);
  local_100 = 0x1000000;
  param_1[0x4e] = (longlong)pvVar10;
  lVar15 = param_1[4];
  pvVar17 = (LPVOID)param_1[5];
  uVar16 = param_1[3];
  uVar13 = (ulonglong)((int)lVar15 + (int)pvVar17 & 7);
  uVar11 = 8 - uVar13;
  if (uVar13 == 0) {
    uVar11 = uVar22;
  }
  lVar23 = uVar11 + 0x1000000;
  if (uVar16 < (ulonglong)(lVar15 + lVar23)) {
    if ((ulonglong)param_1[2] < (ulonglong)(lVar15 + lVar23)) {
      FUN_1401b5c50(param_1,lVar23,8);
      pvVar17 = (LPVOID)param_1[5];
      lVar15 = param_1[4];
      uVar16 = param_1[3];
      uVar13 = (ulonglong)((int)lVar15 + (int)pvVar17 & 7);
      uVar11 = 8 - uVar13;
      if (uVar13 == 0) {
        uVar11 = uVar22;
      }
      lVar23 = uVar11 + 0x1000000;
    }
    lVar2 = *param_1;
    uVar13 = uVar22;
    if ((lVar2 != 0) && (uVar18 = lVar2 - 1U & lVar15 + lVar23, uVar18 != 0)) {
      uVar13 = lVar2 - uVar18;
    }
    uVar13 = uVar13 + lVar15 + lVar23;
    if (uVar16 < uVar13) {
      param_1[3] = uVar13;
      VirtualAlloc(pvVar17,uVar13,0x1000,4);
    }
  }
  lVar15 = param_1[4] + uVar11 + param_1[5];
  FUN_1401bace0(lVar15,0,0x1000000);
  param_1[4] = param_1[4] + lVar23;
  param_1[6] = lVar15;
  local_c0 = 0;
  uStack_b8 = 0;
  local_b0 = 0;
  uStack_a8 = 0;
  local_48 = 0;
  local_a0 = 0;
  uStack_98 = 0;
  local_90 = 0;
  uStack_88 = 0;
  uStack_78 = 0;
  uStack_70 = 0;
  uStack_68 = 0;
  uStack_60 = 0;
  local_80 = 0;
  local_58 = 0;
  uStack_50 = 0;
  param_1[0x18] = local_100;
  param_1[0x19] = lVar15;
  param_1[0x1a] = local_f0;
  param_1[0x1b] = lStack_e8;
  param_1[0x1c] = local_e0;
  param_1[0x1d] = lStack_d8;
  param_1[0x1e] = local_d0;
  param_1[0x1f] = lStack_c8;
  param_1[0x20] = 0;
  param_1[0x21] = 0;
  param_1[0x22] = 0;
  param_1[0x23] = 0;
  param_1[0x24] = 0;
  param_1[0x25] = 0;
  param_1[0x26] = 0;
  param_1[0x27] = 0;
  param_1[0x28] = 0;
  param_1[0x29] = 0;
  param_1[0x2a] = 0;
  param_1[0x2b] = 0;
  *(undefined4 *)(param_1 + 0x2c) = 0;
  *(undefined4 *)((longlong)param_1 + 0x164) = 0;
  *(undefined4 *)(param_1 + 0x2d) = 0;
  *(undefined4 *)((longlong)param_1 + 0x16c) = 0;
  param_1[0x2e] = 0;
  param_1[0x2f] = 0;
  lStack_f8 = lVar15;
  lVar15 = FUN_1401b6980(param_1,param_1 + 0x32,param_1[0x39],0x20,param_1[0x32] + 0x400);
  param_1[0x39] = lVar15;
  lVar15 = param_1[4];
  pvVar17 = (LPVOID)param_1[5];
  uVar16 = param_1[3];
  uVar13 = (ulonglong)((int)lVar15 + (int)pvVar17 & 7);
  uVar11 = 8 - uVar13;
  if (uVar13 == 0) {
    uVar11 = uVar22;
  }
  lVar23 = uVar11 + 0x8000;
  if (uVar16 < (ulonglong)(lVar23 + lVar15)) {
    if ((ulonglong)param_1[2] < (ulonglong)(lVar23 + lVar15)) {
      FUN_1401b5c50(param_1,lVar23,8);
      lVar15 = param_1[4];
      pvVar17 = (LPVOID)param_1[5];
      uVar16 = param_1[3];
      uVar13 = (ulonglong)((int)lVar15 + (int)pvVar17 & 7);
      uVar11 = 8 - uVar13;
      if (uVar13 == 0) {
        uVar11 = uVar22;
      }
      lVar23 = uVar11 + 0x8000;
    }
    lVar2 = *param_1;
    uVar13 = uVar22;
    if ((lVar2 != 0) && (uVar18 = lVar2 - 1U & lVar23 + lVar15, uVar18 != 0)) {
      uVar13 = lVar2 - uVar18;
    }
    uVar13 = uVar13 + lVar23 + lVar15;
    if (uVar16 < uVar13) {
      param_1[3] = uVar13;
      VirtualAlloc(pvVar17,uVar13,0x1000,4);
    }
  }
  pvVar17 = (LPVOID)(param_1[4] + uVar11 + param_1[5]);
  FUN_1401bace0(pvVar17,0,0x8000);
  param_1[4] = param_1[4] + lVar23;
  param_1[6] = (longlong)pvVar17;
  local_1e0.Internal = 0;
  local_1e0.InternalHigh = 0;
  local_1e0.u.Pointer = (PVOID)0x0;
  local_1e0.hEvent = (HANDLE)0x0;
  iVar21 = 0;
  if ((int)param_1[0x49] == 0) {
    do {
      DVar5 = WaitForSingleObject((HANDLE)param_1[0x4e],0);
      if (DVar5 != 0x102) break;
      lpBytesReturned = &local_27c;
      local_27c = 0;
      BVar7 = ReadDirectoryChangesW
                        ((HANDLE)param_1[0x4b],pvVar17,0x8000,1,0x1f,lpBytesReturned,
                         (LPOVERLAPPED)0x0,(LPOVERLAPPED_COMPLETION_ROUTINE)0x0);
      iVar8 = iVar21;
      if (BVar7 == 0) {
        DVar5 = GetLastError();
        if (DVar5 == 0x3e3) {
          local_1a0 = param_1[8];
          lStack_198 = param_1[9];
          local_190[0] = FUN_1400c3130("C:\\Users\\Vjekoslav\\Projects\\Voidstar\\File Pilot\\Source\\Win\\WinMain.c"
                                       ,"Win_DirWatcherProc",0x28f4,"Watcher stopped for \'%S\'\n",
                                       &local_1a0);
          FUN_140191480(param_1,"\'%S\' - Stopped",local_190);
        }
        else {
          lpBytesReturned = (LPDWORD)(ulonglong)DVar5;
          local_180 = param_1[8];
          lStack_178 = param_1[9];
          uVar6 = FUN_1400c3130("C:\\Users\\Vjekoslav\\Projects\\Voidstar\\File Pilot\\Source\\Win\\WinMain.c"
                                ,"Win_DirWatcherProc",0x28f9,
                                "ReadDirectoryChangesW (synced) failed for \'%S\', error %lu",
                                &local_180,lpBytesReturned);
          local_170 = param_1[8];
          lStack_168 = param_1[9];
          FUN_140191480(param_1,"\'%S\' - ReadDirectoryChangesW (synced) failed, error %lu",
                        &local_170,uVar6);
        }
      }
      else if (local_27c != 0) {
        iVar8 = FUN_1401923a0(param_1,pvVar17);
        FUN_1401bace0(pvVar17,0,0x8000);
      }
      LOCK();
      uVar16 = param_1[0x28];
      if (uVar16 == 0) {
        param_1[0x28] = 0;
        uVar16 = 0;
      }
      UNLOCK();
      LOCK();
      uVar11 = param_1[0x20];
      if (uVar11 == 0) {
        param_1[0x20] = 0;
        uVar11 = 0;
      }
      UNLOCK();
      uVar13 = uVar22;
      if (uVar11 < uVar16) {
        uVar13 = uVar16 - uVar11;
      }
      dVar24 = (double)uVar13 * 0.0009765625;
      lVar15 = 0;
      if ((9.223372036854776e+18 <= dVar24) &&
         (dVar24 = dVar24 - 9.223372036854776e+18, dVar24 < 9.223372036854776e+18)) {
        lVar15 = -0x8000000000000000;
      }
      uVar16 = (longlong)dVar24 + lVar15;
      if ((iVar8 == 0) && (uVar16 < 0x81)) {
        local_200 = param_1[8];
        lStack_1f8 = param_1[9];
        FUN_140191480(param_1,"\'%S\' - Wake up main thread %lu, read %llu",&local_200,0,uVar16);
      }
      else {
        local_200 = param_1[8];
        lStack_1f8 = param_1[9];
        FUN_140191480(param_1,"\'%S\' - Wake up main thread %lu, read %llu",&local_200,1,uVar16);
        PostMessageW((HWND)param_1[0x4a],0x8001,0,0);
      }
    } while (BVar7 != 0);
  }
  else {
    local_1e0.hEvent = CreateEventW((LPSECURITY_ATTRIBUTES)0x0,0,0,(LPCWSTR)0x0);
    lpBytesReturned = (LPDWORD)0x0;
    uVar4 = ReadDirectoryChangesW
                      ((HANDLE)param_1[0x4b],pvVar17,0x8000,1,0x1f,(LPDWORD)0x0,&local_1e0,
                       (LPOVERLAPPED_COMPLETION_ROUTINE)0x0);
    uVar16 = (ulonglong)uVar4;
    if (uVar4 == 0) {
      DVar5 = GetLastError();
      local_240 = param_1[8];
      lStack_238 = param_1[9];
      lpBytesReturned = (LPDWORD)(ulonglong)DVar5;
      uVar6 = FUN_1400c3130("C:\\Users\\Vjekoslav\\Projects\\Voidstar\\File Pilot\\Source\\Win\\WinMain.c"
                            ,"Win_DirWatcherProc",0x28a9,
                            "Initial ReadDirectoryChangesW failed for \'%S\', error %lu\n",
                            &local_240,lpBytesReturned);
      local_220 = extraout_XMM0_Qa;
      FUN_140191480(param_1,"\'%S\' - Initial ReadDirectoryChangesW failed, error %lu",&local_220,
                    uVar6);
    }
    else {
      do {
        local_230 = local_1e0.hEvent;
        lStack_228 = param_1[0x4e];
        DVar5 = WaitForMultipleObjectsEx(2,&local_230,0,0xffffffff,1);
        iVar8 = iVar21;
        if (DVar5 == 0) {
          local_280 = 0;
          BVar7 = GetOverlappedResult((HANDLE)param_1[0x4b],&local_1e0,&local_280,0);
          if (BVar7 == 0) {
            DVar5 = GetLastError();
            if (DVar5 == 0x3fe) {
              local_1b0 = param_1[8];
              lStack_1a8 = param_1[9];
              FUN_140191480(param_1,"\'%S\' - Error notify enum dir",&local_1b0);
              iVar8 = 1;
            }
            else {
              uVar16 = uVar22;
              if (DVar5 == 0) goto LAB_140191e3d;
            }
          }
          else {
LAB_140191e3d:
            uVar16 = uVar22;
            if (local_280 != 0) {
              iVar8 = FUN_1401923a0(param_1,pvVar17);
              FUN_1401bace0(pvVar17,0,0x8000);
              lpBytesReturned = (LPDWORD)0x0;
              uVar4 = ReadDirectoryChangesW
                                ((HANDLE)param_1[0x4b],pvVar17,0x8000,1,0x1f,(LPDWORD)0x0,&local_1e0
                                 ,(LPOVERLAPPED_COMPLETION_ROUTINE)0x0);
              uVar16 = (ulonglong)uVar4;
            }
          }
        }
        else if (DVar5 == 1) {
          uVar16 = 0;
        }
        LOCK();
        uVar11 = param_1[0x28];
        if (uVar11 == 0) {
          param_1[0x28] = 0;
          uVar11 = 0;
        }
        UNLOCK();
        LOCK();
        uVar13 = param_1[0x20];
        if (uVar13 == 0) {
          param_1[0x20] = 0;
          uVar13 = 0;
        }
        UNLOCK();
        uVar18 = uVar22;
        if (uVar13 < uVar11) {
          uVar18 = uVar11 - uVar13;
        }
        dVar24 = (double)uVar18 * 0.0009765625;
        lVar15 = 0;
        if ((9.223372036854776e+18 <= dVar24) &&
           (dVar24 = dVar24 - 9.223372036854776e+18, dVar24 < 9.223372036854776e+18)) {
          lVar15 = -0x8000000000000000;
        }
        uVar11 = (longlong)dVar24 + lVar15;
        if ((iVar8 == 0) && (uVar11 < 0x81)) {
          local_210 = param_1[8];
          lStack_208 = param_1[9];
          FUN_140191480(param_1,"\'%S\' - Wake up main thread %lu, read %llu",&local_210,0,uVar11);
        }
        else {
          local_210 = param_1[8];
          lStack_208 = param_1[9];
          FUN_140191480(param_1,"\'%S\' - Wake up main thread %lu, read %llu",&local_210,1,uVar11);
          PostMessageW((HWND)param_1[0x4a],0x8001,0,0);
        }
      } while ((int)uVar16 != 0);
    }
  }
  local_220 = param_1[8];
  lStack_218 = param_1[9];
  local_240 = FUN_1400c3130("C:\\Users\\Vjekoslav\\Projects\\Voidstar\\File Pilot\\Source\\Win\\WinMain.c"
                            ,"Win_DirWatcherProc",0x2911,
                            "\'%S\' - Win_DirWatcherProc exiting, wait on exitEvent\n",&local_220,
                            lpBytesReturned);
  FUN_140191480(param_1,"\'%S\' - Win_DirWatcherProc exiting, wait on exitEvent\n",&local_240);
  LOCK();
  param_1[0x50] = param_1[0x50] + 1;
  UNLOCK();
  WaitForSingleObject((HANDLE)param_1[0x4f],0xffffffff);
  local_1f0 = param_1[8];
  lStack_1e8 = param_1[9];
  local_240 = FUN_1400c3130("C:\\Users\\Vjekoslav\\Projects\\Voidstar\\File Pilot\\Source\\Win\\WinMain.c"
                            ,"Win_DirWatcherProc",0x2918,"\'%S\' - Win_DirWatcherProc exited\n",
                            &local_1f0);
  FUN_140191480(param_1,"\'%S\' - Win_DirWatcherProc exited\n",&local_240);
  if (((int)param_1[0x49] != 0) && (local_1e0.hEvent != (HANDLE)0x0)) {
    CloseHandle(local_1e0.hEvent);
  }
  CloseHandle((HANDLE)param_1[0x4b]);
  if (param_1[0x4c] != 0) {
    UnregisterDeviceNotification((HDEVNOTIFY)param_1[0x4d]);
    CloseHandle((HANDLE)param_1[0x4c]);
  }
  CloseHandle((HANDLE)param_1[0x4e]);
  CloseHandle((HANDLE)param_1[0x4f]);
LAB_14019235d:
  FUN_140001a10(param_1 + 0x3d);
  FUN_140001a10(param_1);
  return;
}

