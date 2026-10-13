
/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

void entry(void)

{
  undefined1 auVar1 [16];
  char *pcVar2;
  char *pcVar3;
  undefined8 uVar4;
  wchar_t *pwVar5;
  ATOM AVar6;
  BOOL BVar7;
  undefined4 uVar8;
  int iVar9;
  int iVar10;
  undefined4 uVar11;
  UINT UVar12;
  int iVar13;
  uint uVar14;
  DWORD dwMilliseconds;
  LPVOID lpParam;
  undefined8 *puVar15;
  undefined8 uVar16;
  HMODULE pHVar17;
  DWORDLONG dwlConditionMask;
  HANDLE pvVar18;
  LPWSTR pWVar19;
  LPWSTR *hMem;
  WCHAR *pWVar20;
  undefined8 *puVar21;
  longlong *plVar22;
  undefined8 *puVar23;
  longlong *plVar24;
  undefined1 (*pauVar25) [16];
  longlong lVar26;
  undefined8 uVar27;
  longlong lVar28;
  HMONITOR pHVar29;
  HCURSOR pHVar30;
  HWND pHVar31;
  INT_PTR IVar32;
  HDC hdc;
  ulonglong uVar33;
  int Y;
  int X;
  bool bVar34;
  undefined4 uVar35;
  undefined4 uVar36;
  undefined4 uVar37;
  undefined4 uVar38;
  undefined4 uVar39;
  undefined4 uVar40;
  undefined4 uVar41;
  int local_res8 [2];
  int local_res10 [2];
  HMODULE local_res18;
  DWORD local_res20 [2];
  char *pcVar42;
  undefined8 in_stack_fffffffffffff1e8;
  undefined8 in_stack_fffffffffffff1f8;
  undefined8 in_stack_fffffffffffff200;
  char *local_de8;
  longlong lStack_de0;
  _SYSTEMTIME local_dd8;
  wchar_t *local_dc8;
  undefined8 uStack_dc0;
  wchar_t *local_db8;
  undefined8 uStack_db0;
  undefined8 local_da8;
  char *local_d98;
  longlong lStack_d90;
  undefined8 local_d88;
  undefined8 uStack_d80;
  undefined8 local_d78;
  char *local_d68;
  longlong lStack_d60;
  undefined8 local_d58;
  undefined8 uStack_d50;
  undefined8 local_d48;
  LARGE_INTEGER local_d40;
  LARGE_INTEGER local_d38 [2];
  undefined1 *local_d28;
  WCHAR aWStack_d20 [4];
  char *local_d18;
  WCHAR aWStack_d10 [4];
  undefined4 local_d08;
  undefined4 uStack_d04;
  undefined8 uStack_d00;
  undefined8 uStack_cf8;
  WCHAR aWStack_cf0 [20];
  undefined1 local_cc8 [32];
  char *local_ca8;
  WCHAR aWStack_ca0 [4];
  ulonglong local_c98;
  WCHAR aWStack_c90 [24];
  undefined8 local_c60;
  WCHAR local_c58 [16];
  WCHAR local_c38 [32];
  undefined1 local_bf8;
  WORD local_bb4;
  WORD local_bb2;
  undefined4 local_bb0;
  WNDCLASSW local_b08;
  undefined1 local_ab8 [16];
  undefined1 local_aa8 [16];
  undefined1 local_a98 [16];
  undefined8 local_a88;
  undefined8 uStack_a80;
  undefined8 local_a68;
  undefined8 uStack_a60;
  ulonglong local_a58;
  undefined1 local_a48 [48];
  undefined8 local_a18;
  undefined8 local_9f8 [48];
  undefined1 local_878 [16];
  undefined1 local_868 [16];
  WCHAR local_858 [16];
  _SYSTEM_INFO local_838;
  undefined1 local_808 [240];
  int local_718;
  int local_708;
  _SYSTEMTIME local_704 [11];
  undefined8 local_648 [130];
  undefined1 local_238 [504];
  
  uVar11 = (undefined4)((ulonglong)in_stack_fffffffffffff1f8 >> 0x20);
  uVar39 = (undefined4)((ulonglong)in_stack_fffffffffffff200 >> 0x20);
  uVar35 = (undefined4)((ulonglong)in_stack_fffffffffffff1e8 >> 0x20);
  local_res18 = GetModuleHandleW((LPCWSTR)0x0);
  OleInitialize((LPVOID)0x0);
  SetUnhandledExceptionFilter(FUN_140196560);
  DAT_1401e88a4 = TlsAlloc();
  FUN_14018c8a0(0);
  uVar36 = 1;
  uVar27 = CONCAT44(uVar35,0x2a);
  pcVar42 = "C:\\Users\\Vjekoslav\\Projects\\Voidstar\\File Pilot\\Source\\Win\\WinMain.c";
  lpParam = (LPVOID)FUN_1401b5920(0x1b80,0,0x80,0x20000,0x100000,
                                  "C:\\Users\\Vjekoslav\\Projects\\Voidstar\\File Pilot\\Source\\Win\\WinMain.c"
                                  ,uVar27,"WinMainCRTStartup");
  iVar10 = 0;
  aWStack_d20[0] = L'\v';
  aWStack_d20[1] = L'\0';
  aWStack_d20[2] = L'\0';
  aWStack_d20[3] = L'\0';
  aWStack_d10[0] = L'\v';
  aWStack_d10[1] = L'\0';
  aWStack_d10[2] = L'\0';
  aWStack_d10[3] = L'\0';
  local_d28 = &DAT_1401cea20;
  local_d08 = 1;
  local_d18 = "consola.ttf";
  uStack_d00._0_2_ = L'\0';
  uStack_d00._2_2_ = L'\0';
  uStack_d00._4_2_ = L'\0';
  uStack_d00._6_2_ = L'\0';
  uStack_cf8._0_2_ = L'\0';
  uStack_cf8._2_2_ = L'\0';
  uStack_cf8._4_2_ = L'\0';
  uStack_cf8._6_2_ = L'\0';
  aWStack_cf0[0] = L'\0';
  aWStack_cf0[1] = L'\0';
  aWStack_cf0[2] = L'\0';
  aWStack_cf0[3] = L'\0';
  aWStack_cf0[4] = L'\0';
  aWStack_cf0[5] = L'\0';
  aWStack_cf0[6] = L'\0';
  aWStack_cf0[7] = L'\0';
  aWStack_cf0[8] = L'\0';
  aWStack_cf0[9] = L'\0';
  aWStack_cf0[10] = L'\0';
  aWStack_cf0[0xb] = L'\0';
  aWStack_cf0[0xc] = L'\0';
  aWStack_cf0[0xd] = L'\0';
  aWStack_cf0[0xe] = L'\0';
  aWStack_cf0[0xf] = L'\0';
  DAT_1401e8be0 = 0x5c;
  DAT_1401e8b90 = lpParam;
  puVar15 = (undefined8 *)FUN_1401b84a0(&local_de8,lpParam,&DAT_1401d38c8);
  _DAT_1401e8be8 = *puVar15;
  DAT_1401e8bf0 = puVar15[1];
  local_res20[0] = 0x20;
  BVar7 = GetComputerNameW(local_858,local_res20);
  if (BVar7 != 0) {
    puVar15 = (undefined8 *)FUN_1401b9be0(&local_de8,lpParam,local_858);
    uVar16 = puVar15[1];
    *(undefined8 *)((longlong)lpParam + 0xf70) = *puVar15;
    *(undefined8 *)((longlong)lpParam + 0xf78) = uVar16;
    local_d68 = (char *)*puVar15;
    lStack_d60 = puVar15[1];
    puVar15 = (undefined8 *)FUN_140188d00(&local_de8,lpParam,&local_d68);
    uVar16 = puVar15[1];
    *(undefined8 *)((longlong)lpParam + 0xf80) = *puVar15;
    *(undefined8 *)((longlong)lpParam + 0xf88) = uVar16;
  }
  uVar16 = FUN_14016e1b0(lpParam,(longlong)lpParam + 0xf90,0);
  *(undefined8 *)((longlong)lpParam + 0x10c0) = uVar16;
  uVar16 = FUN_14016e1b0(lpParam,(longlong)lpParam + 0xf90,0);
  *(undefined8 *)((longlong)lpParam + 0x10c8) = uVar16;
  uVar16 = FUN_14016e1b0(lpParam,(longlong)lpParam + 0xf90,0);
  uVar33 = (ulonglong)pcVar42 & 0xffffffff00000000;
  *(undefined8 *)((longlong)lpParam + 0x10d0) = uVar16;
  uVar16 = FUN_1401b5dc0(lpParam,(longlong)lpParam + 0x10d8,
                         *(undefined8 *)((longlong)lpParam + 0x10e8),8,0x80,uVar33);
  uVar33 = uVar33 & 0xffffffff00000000;
  *(undefined8 *)((longlong)lpParam + 0x10e8) = uVar16;
  uVar16 = FUN_1401b5dc0(lpParam,(longlong)lpParam + 0x10f0,
                         *(undefined8 *)((longlong)lpParam + 0x1100),4,0x80,uVar33);
  uVar33 = uVar33 & 0xffffffff00000000;
  *(undefined8 *)((longlong)lpParam + 0x1100) = uVar16;
  uVar35 = 0;
  uVar16 = FUN_1401b5dc0(lpParam,(longlong)lpParam + 0x1108,
                         *(undefined8 *)((longlong)lpParam + 0x1118),4,0x80,uVar33);
  uVar41 = (undefined4)(uVar33 >> 0x20);
  *(undefined8 *)((longlong)lpParam + 0x1118) = uVar16;
  pHVar17 = GetModuleHandleW(L"user32.dll");
  if (pHVar17 != (HMODULE)0x0) {
    DAT_1401e8b78 = GetProcAddress(pHVar17,"SetProcessDpiAwarenessContext");
    DAT_1401e8b80 = GetProcAddress(pHVar17,"GetDpiForWindow");
    DAT_1401e8b88 = GetProcAddress(pHVar17,"GetSystemMetricsForDpi");
  }
  pHVar17 = LoadLibraryW(L"shcore.dll");
  if (pHVar17 != (HMODULE)0x0) {
    DAT_1401e8b68 = GetProcAddress(pHVar17,"SetProcessDpiAwareness");
    DAT_1401e8b70 = GetProcAddress(pHVar17,"GetDpiForMonitor");
    FreeLibrary(pHVar17);
  }
  if (DAT_1401e8b78 == (FARPROC)0x0) {
    if (DAT_1401e8b68 != (FARPROC)0x0) {
      (*DAT_1401e8b68)(2);
    }
  }
  else {
    (*DAT_1401e8b78)(0xfffffffffffffffc);
  }
  local_cc8._0_4_ = 0x11c;
  local_cc8._12_4_ = 0;
  local_cc8._16_4_ = 0;
  FUN_1401bace0(local_cc8 + 0x14,0,0x100);
  local_bb2 = 0;
  local_bb0._0_2_ = 0;
  local_bb0._2_1_ = '\0';
  local_bb0._3_1_ = '\0';
  uVar16 = VerSetConditionMask(0,2,3);
  uVar16 = VerSetConditionMask(uVar16,1,3);
  dwlConditionMask = VerSetConditionMask(uVar16,0x20,3);
  local_cc8._4_4_ = 6;
  local_cc8._8_4_ = 2;
  local_bb4 = 0;
  BVar7 = VerifyVersionInfoW((LPOSVERSIONINFOEXW)local_cc8,0x23,dwlConditionMask);
  *(uint *)((longlong)lpParam + 0x88) = (uint)(BVar7 == 0);
  uVar8 = FUN_1401a4550();
  *(undefined4 *)((longlong)lpParam + 0x8c) = uVar8;
  uVar8 = FUN_1401a41c0();
  *(undefined4 *)((longlong)lpParam + 0x94) = uVar8;
  FUN_1401a3ea0();
  iVar9 = FUN_1401a3e70();
  *(uint *)((longlong)lpParam + 0x98) = (uint)(iVar9 != 0);
  if ((*(int *)((longlong)lpParam + 0x88) == 0) &&
     (pHVar17 = GetModuleHandleW(L"ntdll.dll"), pHVar17 != (HMODULE)0x0)) {
    DAT_1401e8b60 = GetProcAddress(pHVar17,"NtQueryDirectoryFile");
  }
  *(undefined4 *)((longlong)lpParam + 100) = 0x60;
  GetSystemInfo(&local_838);
  DAT_1401e8df8 = FUN_1400679e0(2);
  *(undefined4 *)((longlong)lpParam + 0x1a50) = DAT_1401e8df8;
  QueryPerformanceFrequency(&local_d40);
  DAT_1401e8b98 = local_d40;
  BVar7 = QueryPerformanceCounter(local_d38);
  if (BVar7 != 0) {
    DAT_1401e8ba0 = local_d38[0];
  }
  pvVar18 = CreateMutexW((LPSECURITY_ATTRIBUTES)0x0,0,L"FilePilot_Config_Mutex");
  *(HANDLE *)((longlong)lpParam + 0xa20) = pvVar18;
  pvVar18 = CreateMutexW((LPSECURITY_ATTRIBUTES)0x0,0,L"FilePilot_UserData_Mutex");
  *(HANDLE *)((longlong)lpParam + 0xa28) = pvVar18;
  pvVar18 = CreateMutexW((LPSECURITY_ATTRIBUTES)0x0,0,L"FilePilot_Session_Mutex");
  lStack_d60 = 0x12;
  *(HANDLE *)((longlong)lpParam + 0xa30) = pvVar18;
  local_d68 = "Voidstar\\FilePilot";
  iVar9 = SHGetKnownFolderPath(&DAT_1401d6c38,0,0,local_648);
  uVar8 = (undefined4)((ulonglong)uVar27 >> 0x20);
  if (-1 < iVar9) {
    local_dd8.wYear = 0;
    local_dd8.wMonth = 0;
    local_dd8.wDayOfWeek = 0;
    local_dd8.wDay = 0;
    local_dd8.wHour = 0;
    local_dd8.wMinute = 0;
    local_dd8.wSecond = 0;
    local_dd8.wMilliseconds = 0;
    GetLocalTime(&local_dd8);
    local_db8 = (wchar_t *)0x0;
    uStack_db0 = 0;
    local_da8 = 0;
    FUN_1401b7ec0(&local_de8,lpParam,&local_db8,local_648[0]);
    FUN_1401b7f60(&local_de8,lpParam,&local_db8,0x5c);
    local_dc8 = L"Voidstar\\FilePilot";
    uStack_dc0 = 0x12;
    FUN_1401b8280(&local_de8,lpParam,&local_db8,&local_dc8);
    FUN_1401b7d60(&local_de8,lpParam,&local_db8,"\\Logs");
    local_dc8 = local_db8;
    uStack_dc0 = uStack_db0;
    puVar15 = (undefined8 *)FUN_140187eb0(&local_de8,lpParam,&local_dc8);
    uVar16 = puVar15[1];
    uVar27 = CONCAT44(uVar8,(uint)local_dd8.wDay);
    *(undefined8 *)((longlong)lpParam + 200) = *puVar15;
    *(undefined8 *)((longlong)lpParam + 0xd0) = uVar16;
    FUN_1401b7fd0(&local_de8,lpParam,&local_db8,"\\FPilot-%04d-%02d-%02d-%02d-%02d-%02d.log",
                  CONCAT44(uVar35,(uint)local_dd8.wYear),CONCAT44(uVar41,(uint)local_dd8.wMonth),
                  uVar27,CONCAT44(uVar36,(uint)local_dd8.wHour),
                  CONCAT44(uVar11,(uint)local_dd8.wMinute),CONCAT44(uVar39,(uint)local_dd8.wSecond))
    ;
    puVar15 = (undefined8 *)FUN_14016dbd0(&local_de8,lpParam,&local_db8);
    local_dc8 = (wchar_t *)*puVar15;
    uStack_dc0 = puVar15[1];
    puVar15 = (undefined8 *)FUN_140187eb0(&local_de8,lpParam,&local_dc8);
    uVar16 = puVar15[1];
    *(undefined8 *)((longlong)lpParam + 0xd8) = *puVar15;
    *(undefined8 *)((longlong)lpParam + 0xe0) = uVar16;
  }
  FUN_14018ff70(lpParam);
  local_res8[0] = 0;
  pWVar19 = GetCommandLineW();
  hMem = CommandLineToArgvW(pWVar19,local_res8);
  FUN_140199ab0(lpParam);
  if (local_res8[0] == 2) {
    pWVar19 = hMem[1];
    local_dd8.wYear = 0;
    local_dd8.wMonth = 0;
    local_dd8.wDayOfWeek = 0;
    local_dd8.wDay = 0;
    local_dd8.wHour = 0xffff;
    local_dd8.wMinute = 0xffff;
    local_dd8.wSecond = 0xffff;
    local_dd8.wMilliseconds = 0xffff;
    local_dc8 = L"--root";
    uStack_dc0 = 6;
    iVar9 = FUN_1401b9fa0(&local_dc8,pWVar19,&local_dd8);
    if (iVar9 == 0) {
      local_dd8.wYear = 0;
      local_dd8.wMonth = 0;
      local_dd8.wDayOfWeek = 0;
      local_dd8.wDay = 0;
      local_dc8 = L"--uninstall";
      local_dd8.wHour = 0xffff;
      local_dd8.wMinute = 0xffff;
      local_dd8.wSecond = 0xffff;
      local_dd8.wMilliseconds = 0xffff;
      uStack_dc0 = 0xb;
      iVar9 = FUN_1401b9fa0(&local_dc8,pWVar19,&local_dd8);
      if (iVar9 != 0) {
        lVar28 = FUN_1400c4370(local_a98,lpParam);
        local_a18 = *(undefined8 *)(lVar28 + 0x30);
        if ((int)local_a18 == 3) {
          iVar10 = FUN_1400c6770();
          if (iVar10 < 2) {
            FUN_1400cc780();
          }
          else {
            MessageBoxW((HWND)0x0,L"Please close all other open instances of File Pilot.",
                        L"File Pilot Uninstall",0x30);
          }
        }
        goto LAB_14018a7d4;
      }
      local_dd8.wYear = 0;
      local_dd8.wMonth = 0;
      local_dd8.wDayOfWeek = 0;
      local_dd8.wDay = 0;
      local_dc8 = L"--uninstallex";
      local_dd8.wHour = 0xffff;
      local_dd8.wMinute = 0xffff;
      local_dd8.wSecond = 0xffff;
      local_dd8.wMilliseconds = 0xffff;
      uStack_dc0 = 0xd;
      iVar9 = FUN_1401b9fa0(&local_dc8,pWVar19,&local_dd8);
      if (iVar9 != 0) {
        FUN_1401b1760();
        goto LAB_14018a7d4;
      }
      puVar15 = (undefined8 *)FUN_1401b6030(&local_de8,pWVar19);
      local_dc8 = (wchar_t *)*puVar15;
      uStack_dc0 = puVar15[1];
      puVar15 = (undefined8 *)FUN_1401ba140(&local_d98,&local_dc8,0);
      local_dc8 = (wchar_t *)*puVar15;
      uStack_dc0 = puVar15[1];
      puVar15 = (undefined8 *)FUN_140188d00(&local_de8,lpParam,&local_dc8);
      local_dc8 = (wchar_t *)*puVar15;
      uStack_dc0 = puVar15[1];
      puVar21 = (undefined8 *)FUN_1400cbd50(local_a48,&local_dc8);
      local_a88 = puVar21[2];
      uStack_a80 = puVar21[3];
      local_a58 = puVar21[8];
      puVar15 = puVar21 + 4;
      uVar35 = *(undefined4 *)puVar15;
      uVar36 = *(undefined4 *)((longlong)puVar21 + 0x24);
      pcVar42 = (char *)*puVar15;
      pwVar5 = (wchar_t *)*puVar15;
      puVar23 = puVar21 + 5;
      uVar11 = *(undefined4 *)puVar23;
      uVar39 = *(undefined4 *)((longlong)puVar21 + 0x2c);
      uVar4 = *puVar23;
      uVar16 = *puVar23;
      local_a68 = puVar21[6];
      uStack_a60 = puVar21[7];
      if ((char)((ulonglong)*puVar21 >> 8) != '\0') {
        if ((local_a58 & 2) == 0) {
          local_dc8 = (wchar_t *)*puVar15;
          uStack_dc0 = *puVar23;
          plVar24 = (longlong *)FUN_140067a00(local_ab8,&local_dc8);
          local_dd8.wYear = 0;
          local_dd8.wMonth = 0;
          local_dd8.wDayOfWeek = 0;
          local_dd8.wDay = 0;
          local_dd8.wHour = 0xffff;
          local_dd8.wMinute = 0xffff;
          local_dd8.wSecond = 0xffff;
          local_dd8.wMilliseconds = 0xffff;
          local_de8 = (char *)*plVar24;
          lStack_de0 = plVar24[1];
          local_dc8 = pwVar5;
          uStack_dc0 = uVar16;
          local_d98 = local_de8;
          lStack_d90 = lStack_de0;
          iVar9 = FUN_1401b87a0(&local_d98,&local_dc8,&local_dd8);
          if (iVar9 == 0) {
            local_d98 = pcVar42;
            lStack_d90 = uVar4;
            plVar22 = (longlong *)FUN_1401b9a30(&local_dc8,&local_d98);
            local_d98 = (char *)*plVar22;
            lStack_d90 = plVar22[1];
            pWVar20 = (WCHAR *)FUN_1401b9050(&local_de8,&local_d98,lStack_de0 + 1);
            aWStack_cf0._0_8_ = *(undefined8 *)pWVar20;
            aWStack_cf0._8_8_ = *(undefined8 *)(pWVar20 + 4);
          }
          uVar35 = (undefined4)*plVar24;
          uVar36 = *(undefined4 *)((longlong)plVar24 + 4);
          uVar11 = (undefined4)plVar24[1];
          uVar39 = *(undefined4 *)((longlong)plVar24 + 0xc);
        }
        uStack_d00._4_2_ = (short)uVar36;
        uStack_d00._6_2_ = (short)((uint)uVar36 >> 0x10);
        uStack_d00._0_2_ = (short)uVar35;
        uStack_d00._2_2_ = (short)((uint)uVar35 >> 0x10);
        uStack_cf8._4_2_ = (short)uVar39;
        uStack_cf8._6_2_ = (short)((uint)uVar39 >> 0x10);
        uStack_cf8._0_2_ = (short)uVar11;
        uStack_cf8._2_2_ = (short)((uint)uVar11 >> 0x10);
      }
    }
    else {
      pWVar20 = (WCHAR *)FUN_1401b84a0(&local_de8,lpParam,&DAT_1401c5f10);
      uStack_d00 = *(undefined8 *)pWVar20;
      uStack_cf8 = *(undefined8 *)(pWVar20 + 4);
    }
  }
  LocalFree(hMem);
  FUN_1401b0920(lpParam);
  lVar28 = lStack_d60;
  pcVar42 = local_d68;
  puVar15 = (undefined8 *)&DAT_1401e8bf8;
  local_9f8[0xe] = 0;
  local_9f8[0xf] = 0;
  local_9f8[0x2a] = 0;
  local_9f8[0x2b] = 0;
  local_9f8[0x2c] = 0;
  local_9f8[0x2d] = 0;
  local_9f8[0x2e] = 0;
  local_9f8[0x2f] = 0;
  local_9f8[0] = 0x47600e225e6c858f;
  local_9f8[1] = 0x7371b61733eafe9a;
  local_9f8[2] = 0x424cdb2cb4bfcc3a;
  local_9f8[3] = 0x41c6879ae97f29b0;
  local_9f8[4] = 0x4565123f374de290;
  local_9f8[5] = 0x7b465e92c4396491;
  local_9f8[6] = 0x46af238ffdd39ad0;
  local_9f8[7] = 0xc7690348856cb4ad;
  local_9f8[8] = 0x48d36d194bd8d571;
  local_9f8[9] = 0x430e0820224297be;
  local_9f8[10] = 0x46764e1e33e28130;
  local_9f8[0xb] = 0xbbc33b5c39985a83;
  local_9f8[0xc] = 0x455b99b518989b1d;
  local_9f8[0xd] = 0xfcdde4747cab1c84;
  local_9f8[0x10] = 0x4d8a68ad1777f761;
  local_9f8[0x11] = 0xdd33fa59b730bd87;
  local_9f8[0x12] = 0x4cf665f93eb685db;
  local_9f8[0x13] = 0x3d9f7265efe33aa0;
  local_9f8[0x14] = 0x4fcf6fbaf1b32785;
  local_9f8[0x15] = 0x9170157f8e7b559d;
  local_9f8[0x16] = 0x44c32e2ba77f5d77;
  local_9f8[0x17] = 0x514a0501a6aba2a6;
  local_9f8[0x18] = 0x4ec1ab48625b53c3;
  local_9f8[0x19] = 0x19fc4641efa11fba;
  local_9f8[0x1a] = 0x4c97f46ab97d20bb;
  local_9f8[0x1b] = 0x54084308365e10ba;
  local_9f8[0x1c] = 0x4bfca0fb7c5a40ef;
  local_9f8[0x1d] = 0x8efab9e0f2c04a87;
  local_9f8[0x1e] = 0x494ec1bf905e63b6;
  local_9f8[0x1f] = 0x1ad2d332b7659cb2;
  local_9f8[0x20] = 0x4bb0c50a0762d272;
  local_9f8[0x21] = 0x809b72cd7d6982a3;
  local_9f8[0x22] = 0x4e5d02e71ac14e77;
  local_9f8[0x23] = 0xb79851aeb12e44b7;
  local_9f8[0x24] = 0x4ae3ae11fd228cb7;
  local_9f8[0x25] = 0xfeb80a91f3164c86;
  local_9f8[0x26] = 0x4dc3fdc162ab5d82;
  local_9f8[0x27] = 0x975d491d0d07dda9;
  local_9f8[0x28] = 0x42f21d43f38bf404;
  local_9f8[0x29] = 0x23fc280bde670593;
LAB_1401899e0:
  do {
    uVar35 = (undefined4)((ulonglong)uVar27 >> 0x20);
    if (iVar10 != 7) {
      if (iVar10 == 0x15) {
        iVar10 = SHGetKnownFolderPath(&DAT_1401d6bf8,0,0);
        if (-1 < iVar10) {
          local_d58 = 0;
          uStack_d50 = 0;
          local_d48 = 0;
          FUN_1401b7ec0(local_ab8,lpParam,&local_d58,local_648[0]);
          FUN_1401b7f60(&local_d98,lpParam,&local_d58,0x5c);
          local_de8 = pcVar42;
          lStack_de0 = lVar28;
          FUN_1401b8280(&local_d68,lpParam,&local_d58,&local_de8);
          FUN_1401b7d60(&local_dc8);
          *puVar15 = local_d58;
          puVar15[1] = uStack_d50;
        }
        iVar10 = 0x16;
        puVar15 = puVar15 + 2;
        goto LAB_1401899e0;
      }
      if (iVar10 == 0x16) {
        iVar10 = SHGetKnownFolderPath(&DAT_1401d6bf8,0,0);
        if (-1 < iVar10) {
          local_d88 = 0;
          uStack_d80 = 0;
          local_d78 = 0;
          FUN_1401b7ec0(&local_dd8,lpParam,&local_d88,local_648[0]);
          FUN_1401b7f60(local_878,lpParam,&local_d88,0x5c);
          local_de8 = pcVar42;
          lStack_de0 = lVar28;
          FUN_1401b8280(local_868,lpParam,&local_d88,&local_de8);
          FUN_1401b7d60(local_aa8);
          *puVar15 = local_d88;
          puVar15[1] = uStack_d80;
        }
        iVar10 = 0x17;
        puVar15 = puVar15 + 2;
        goto LAB_1401899e0;
      }
      if (iVar10 == 0x17) {
        iVar10 = SHGetKnownFolderPath(&DAT_1401d6c38,0,0,local_648);
        if (-1 < iVar10) {
          local_d88 = 0;
          uStack_d80 = 0;
          local_d78 = 0;
          FUN_1401b7ec0(&local_db8,lpParam,&local_d88,local_648[0]);
          FUN_1401b7f60(&local_db8,lpParam,&local_d88,0x5c);
          local_de8 = pcVar42;
          lStack_de0 = lVar28;
          FUN_1401b8280(&local_db8,lpParam,&local_d88,&local_de8);
          FUN_1401b7d60(&local_db8,lpParam,&local_d88,"\\FPilot-Session.json");
          *puVar15 = local_d88;
          puVar15[1] = uStack_d80;
        }
        break;
      }
      iVar9 = SHGetKnownFolderPath(local_9f8 + (longlong)iVar10 * 2,0,0,local_648);
      FUN_1401b4010(iVar9);
      if (-1 < iVar9) {
        puVar23 = (undefined8 *)FUN_1401b8680(&local_db8);
        uVar16 = puVar23[1];
        *puVar15 = *puVar23;
        puVar15[1] = uVar16;
      }
    }
    uVar35 = (undefined4)((ulonglong)uVar27 >> 0x20);
    iVar10 = iVar10 + 1;
    puVar15 = puVar15 + 2;
  } while (iVar10 < 0x18);
  lVar28 = lRam00000001401e8d40;
  pcVar42 = _DAT_1401e8d38;
  local_de8 = _DAT_1401e8d38;
  lStack_de0 = lRam00000001401e8d40;
  plVar24 = (longlong *)FUN_14016dc40(&local_db8,&local_de8,DAT_1401e8be0,0);
  local_d98 = (char *)*plVar24;
  lStack_d90 = plVar24[1];
  if ((int)lStack_d90 != 0) {
    local_de8 = pcVar42;
    lStack_de0 = lVar28;
    puVar15 = (undefined8 *)FUN_1401b9a30(&local_db8,&local_de8);
    local_de8 = (char *)*puVar15;
    lStack_de0 = puVar15[1];
    puVar15 = (undefined8 *)FUN_1401b90b0(local_aa8,&local_de8,0,local_d98 + 1);
    pcVar42 = (char *)*puVar15;
    uVar27 = puVar15[1];
    local_de8 = pcVar42;
    lStack_de0 = uVar27;
    plVar24 = (longlong *)FUN_1401b8b00(&local_db8,lpParam,"%S$Recycle.Bin",&local_de8);
    uVar8 = (undefined4)*plVar24;
    uVar37 = *(undefined4 *)((longlong)plVar24 + 4);
    local_de8 = (char *)*plVar24;
    uVar38 = (undefined4)plVar24[1];
    uVar40 = *(undefined4 *)((longlong)plVar24 + 0xc);
    lStack_de0 = plVar24[1];
    pauVar25 = (undefined1 (*) [16])FUN_140187eb0(&local_db8,lpParam,&local_de8);
    auVar1 = *pauVar25;
    uVar36 = auVar1._0_4_;
    uVar11 = auVar1._4_4_;
    uVar39 = auVar1._8_4_;
    uVar41 = auVar1._12_4_;
    BVar7 = PathFileExistsW(auVar1._0_8_);
    if (BVar7 == 0) {
      local_de8 = pcVar42;
      lStack_de0 = uVar27;
      plVar24 = (longlong *)FUN_1401b8b00(&local_db8,lpParam,"%SRecycler",&local_de8);
      uVar8 = (undefined4)*plVar24;
      uVar37 = *(undefined4 *)((longlong)plVar24 + 4);
      local_de8 = (char *)*plVar24;
      uVar38 = (undefined4)plVar24[1];
      uVar40 = *(undefined4 *)((longlong)plVar24 + 0xc);
      lStack_de0 = plVar24[1];
      pauVar25 = (undefined1 (*) [16])FUN_140187eb0(&local_db8,lpParam,&local_de8);
      auVar1 = *pauVar25;
      uVar36 = auVar1._0_4_;
      uVar11 = auVar1._4_4_;
      uVar39 = auVar1._8_4_;
      uVar41 = auVar1._12_4_;
      BVar7 = PathFileExistsW(auVar1._0_8_);
      if (BVar7 == 0) goto LAB_140189d3c;
    }
    _DAT_1401e8c68 = uVar8;
    uRam00000001401e8c6c = uVar37;
    DAT_1401e8c70._0_4_ = uVar38;
    DAT_1401e8c70._4_4_ = uVar40;
    *(undefined4 *)((longlong)lpParam + 0x1300) = uVar8;
    *(undefined4 *)((longlong)lpParam + 0x1304) = uVar37;
    *(undefined4 *)((longlong)lpParam + 0x1308) = uVar38;
    *(undefined4 *)((longlong)lpParam + 0x130c) = uVar40;
    *(undefined4 *)((longlong)lpParam + 0x12f0) = uVar36;
    *(undefined4 *)((longlong)lpParam + 0x12f4) = uVar11;
    *(undefined4 *)((longlong)lpParam + 0x12f8) = uVar39;
    *(undefined4 *)((longlong)lpParam + 0x12fc) = uVar41;
  }
LAB_140189d3c:
  FUN_1401b0a20((longlong)lpParam + 0xc18,0x400);
  FUN_1401b0a20((longlong)lpParam + 0xdc8,0x80);
  puVar15 = (undefined8 *)FUN_14019d900(&local_db8,lpParam);
  if (puVar15[1] != 0) {
    local_de8 = (char *)*puVar15;
    lStack_de0 = puVar15[1];
    puVar15 = (undefined8 *)FUN_140188d00(&local_db8,lpParam,&local_de8);
    local_de8 = (char *)*puVar15;
    lStack_de0 = puVar15[1];
    puVar15 = (undefined8 *)FUN_140067a00(&local_db8,&local_de8);
    pcVar42 = (char *)*puVar15;
    uVar27 = puVar15[1];
    local_de8 = pcVar42;
    lStack_de0 = uVar27;
    puVar15 = (undefined8 *)FUN_1401b8b00(&local_db8,lpParam,"%S\\FPilot-Config.json",&local_de8);
    pcVar2 = (char *)*puVar15;
    uVar16 = puVar15[1];
    local_de8 = pcVar42;
    lStack_de0 = uVar27;
    puVar15 = (undefined8 *)FUN_1401b8b00(&local_db8,lpParam,"%S\\FPilot-UserData.json",&local_de8);
    pcVar3 = (char *)*puVar15;
    uVar4 = puVar15[1];
    local_de8 = pcVar42;
    lStack_de0 = uVar27;
    plVar24 = (longlong *)FUN_1401b8b00(&local_db8,lpParam,"%S\\FPilot-Session.json",&local_de8);
    pcVar42 = (char *)*plVar24;
    lVar28 = plVar24[1];
    local_de8 = pcVar2;
    lStack_de0 = uVar16;
    lVar26 = FUN_1400cbd50(local_a98,&local_de8);
    if (*(char *)(lVar26 + 1) != '\0') {
      _DAT_1401e8d48 = pcVar2;
      uRam00000001401e8d50 = uVar16;
    }
    local_de8 = pcVar3;
    lStack_de0 = uVar4;
    lVar26 = FUN_1400cbd50(local_a98,&local_de8);
    if (*(char *)(lVar26 + 1) != '\0') {
      _DAT_1401e8d58 = pcVar3;
      uRam00000001401e8d60 = uVar4;
    }
    local_de8 = pcVar42;
    lStack_de0 = lVar28;
    lVar26 = FUN_1400cbd50(local_a98,&local_de8);
    if (*(char *)(lVar26 + 1) != '\0') {
      _DAT_1401e8d68 = pcVar42;
      lRam00000001401e8d70 = lVar28;
    }
  }
  _DAT_1401e8db3 = 0x504;
  _DAT_1401e8dc9 = 0x404040404040404;
  _DAT_1401e8dd1 = 0x404;
  _DAT_1401e8de9 = 0x404040404040404;
  _DAT_1401e8da8 = 0x404040404040404;
  _DAT_1401e8db0 = 0x404;
  _DAT_1401e8db6 = 0x705;
  DAT_1401e8db2 = 7;
  _DAT_1401e8d99 = 0x4040504;
  _DAT_1401e8da6 = 0x704;
  _DAT_1401e8dd3 = 0x4040704;
  _DAT_1401e8df3 = 0x4040504;
  _DAT_1401e8d9f = 0x5040404;
  _DAT_1401e8db9 = 0x404040404040404;
  uRam00000001401e8dc1 = 0x404040404040404;
  DAT_1401e8d98 = 4;
  _DAT_1401e8dd9 = 0x404040404040404;
  uRam00000001401e8de1 = 0x404040404040404;
  _DAT_1401e8d9d = 0x404;
  _DAT_1401e8da3 = 0x404;
  DAT_1401e8da5 = 4;
  DAT_1401e8db5 = 4;
  DAT_1401e8db8 = 4;
  _DAT_1401e8dd7 = 0x404;
  _DAT_1401e8df1 = _DAT_1401e8dd1;
  FUN_14018cdb0(*(undefined4 *)((longlong)lpParam + 100));
  uVar27 = CONCAT44(uVar35,0x20d);
  DAT_1401e8b58 =
       FUN_1401b5920(0x2f0,0,8,0x10000,0x100000,
                     "C:\\Users\\Vjekoslav\\Projects\\Voidstar\\File Pilot\\Source\\Win\\WinMain.c",
                     uVar27,"WinMainCRTStartup");
  uVar36 = (undefined4)((ulonglong)uVar27 >> 0x20);
  uVar27 = FUN_1401a7b90(lpParam);
  FUN_1401a3910(lpParam);
  uVar16 = FUN_1401b6980(DAT_1401e8b90,(longlong *)((longlong)DAT_1401e8b90 + 0x688),
                         *(undefined8 *)((longlong)DAT_1401e8b90 + 0x6c0),0x18,
                         *(longlong *)((longlong)DAT_1401e8b90 + 0x688) + 0x100);
  *(undefined8 *)((longlong)DAT_1401e8b90 + 0x6c0) = uVar16;
  FUN_140166750((longlong)lpParam + 0x718);
  FUN_1401a2ca0(lpParam);
  uVar35 = 1;
  lVar28 = FUN_1401b5920(0xe8,0,8,0x10000,0x100000,
                         "C:\\Users\\Vjekoslav\\Projects\\Voidstar\\File Pilot\\Source\\Win\\WinMain.c"
                         ,CONCAT44(uVar36,0x218),"WinMainCRTStartup");
  *(undefined4 *)(lVar28 + 0x68) = 1;
  FUN_1400cbb40(lVar28,FUN_1401b1fc0,0);
  *(longlong *)((longlong)lpParam + 0x10a8) = lVar28;
  pWVar20 = (WCHAR *)FUN_1401a0160(&local_db8,lpParam);
  if (*(longlong *)(pWVar20 + 4) != 0) {
    aWStack_cf0._16_8_ = *(undefined8 *)pWVar20;
    aWStack_cf0._24_8_ = *(undefined8 *)(pWVar20 + 4);
  }
  bVar34 = DAT_1401e8b78 != (FARPROC)0x0;
  if (DAT_1401e8b78 == (FARPROC)0x0) {
LAB_14018a0b5:
    uVar36 = 0;
  }
  else {
    iVar10 = FUN_14018c900();
    uVar36 = 1;
    if (iVar10 == 0) goto LAB_14018a0b5;
  }
  uVar11 = FUN_1401b0600((longlong)lpParam + 0xa0);
  local_cc8._16_4_ = SUB84(local_d28,0);
  local_cc8._20_4_ = (undefined4)((ulonglong)local_d28 >> 0x20);
  local_cc8._24_2_ = aWStack_d20[0];
  local_cc8._26_2_ = aWStack_d20[1];
  local_cc8._28_2_ = aWStack_d20[2];
  local_cc8._30_2_ = aWStack_d20[3];
  local_c98 = CONCAT44(uStack_d04,local_d08);
  local_ca8 = local_d18;
  aWStack_ca0[0] = aWStack_d10[0];
  aWStack_ca0[1] = aWStack_d10[1];
  aWStack_ca0[2] = aWStack_d10[2];
  aWStack_ca0[3] = aWStack_d10[3];
  aWStack_c90[0] = (WCHAR)uStack_d00;
  aWStack_c90[1] = uStack_d00._2_2_;
  aWStack_c90[2] = uStack_d00._4_2_;
  aWStack_c90[3] = uStack_d00._6_2_;
  aWStack_c90[4] = (WCHAR)uStack_cf8;
  aWStack_c90[5] = uStack_cf8._2_2_;
  aWStack_c90[6] = uStack_cf8._4_2_;
  aWStack_c90[7] = uStack_cf8._6_2_;
  aWStack_c90[8] = aWStack_cf0[0];
  aWStack_c90[9] = aWStack_cf0[1];
  aWStack_c90[10] = aWStack_cf0[2];
  aWStack_c90[0xb] = aWStack_cf0[3];
  aWStack_c90[0xc] = aWStack_cf0[4];
  aWStack_c90[0xd] = aWStack_cf0[5];
  aWStack_c90[0xe] = aWStack_cf0[6];
  aWStack_c90[0xf] = aWStack_cf0[7];
  aWStack_c90[0x10] = aWStack_cf0[8];
  aWStack_c90[0x11] = aWStack_cf0[9];
  aWStack_c90[0x12] = aWStack_cf0[10];
  aWStack_c90[0x13] = aWStack_cf0[0xb];
  aWStack_c90[0x14] = aWStack_cf0[0xc];
  aWStack_c90[0x15] = aWStack_cf0[0xd];
  aWStack_c90[0x16] = aWStack_cf0[0xe];
  aWStack_c90[0x17] = aWStack_cf0[0xf];
  *(undefined4 *)((longlong)lpParam + 0x9c) = uVar11;
  local_cc8._0_8_ = (longlong)lpParam + 0xe8;
  local_c38[0] = L'\0';
  local_c38[1] = L'\0';
  local_c38[2] = L'\0';
  local_c38[3] = L'\0';
  local_c38[4] = L'\0';
  local_c38[5] = L'\0';
  local_c38[6] = L'\0';
  local_c38[7] = L'\0';
  local_c38[8] = L'\0';
  local_c38[9] = L'\0';
  local_c38[10] = L'\0';
  local_c38[0xb] = L'\0';
  local_c38[0xc] = L'\0';
  local_c38[0xd] = L'\0';
  local_c38[0xe] = L'\0';
  local_c38[0xf] = L'\0';
  local_c38[0x10] = L'\0';
  local_c38[0x11] = L'\0';
  local_c38[0x12] = L'\0';
  local_c38[0x13] = L'\0';
  local_c38[0x14] = L'\0';
  local_c38[0x15] = L'\0';
  local_c38[0x16] = L'\0';
  local_c38[0x17] = L'\0';
  local_cc8._8_4_ = (undefined4)uVar27;
  local_cc8._12_4_ = (undefined4)((ulonglong)uVar27 >> 0x20);
  local_c60._1_3_ = 0;
  local_c60._0_1_ = bVar34;
  local_c60._4_2_ = (short)uVar36;
  local_c60._6_2_ = (short)((uint)uVar36 >> 0x10);
  local_c58[0] = L'\0';
  local_c58[1] = L'\0';
  local_c58[2] = L'\0';
  local_c58[3] = L'\0';
  local_c58[4] = L'\0';
  local_c58[5] = L'\0';
  local_c58[6] = L'\0';
  local_c58[7] = L'\0';
  local_c58[8] = L'\0';
  local_c58[9] = L'\0';
  local_c58[10] = L'\0';
  local_c58[0xb] = L'\0';
  local_c58[0xc] = L'\0';
  local_c58[0xd] = L'\0';
  local_c58[0xe] = L'\0';
  local_c38[0x18] = L'\0';
  local_c38[0x19] = L'\0';
  local_c38[0x1a] = L'\0';
  local_c38[0x1b] = L'\0';
  local_c38[0x1c] = L'\0';
  local_c38[0x1d] = L'\0';
  local_c38[0x1e] = L'\0';
  local_c38[0x1f] = L'\0';
  local_bf8 = 0;
  FUN_1401bace0(local_808,0,0x1b8);
  FUN_140129430(local_cc8,local_808);
  bVar34 = local_718 == 2;
  Y = -0x80000000;
  iVar10 = 0x300;
  iVar9 = 0x400;
  X = -0x80000000;
  if (local_708 != 0) {
    local_dd8._8_8_ = FUN_1401881c0(local_704[0]._0_8_);
    local_res10[0]._0_2_ = local_704[0].wSecond;
    local_res10[0]._2_2_ = local_704[0].wMilliseconds;
    Y = SUB84(local_704[0]._0_8_,4);
    X = (int)local_704[0]._0_8_;
    local_dd8.wYear = local_704[0].wYear;
    local_dd8.wMonth = local_704[0].wMonth;
    local_dd8.wDayOfWeek = local_704[0].wDayOfWeek;
    local_dd8.wDay = local_704[0].wDay;
    pHVar29 = MonitorFromRect((LPCRECT)&local_dd8,0);
    iVar10 = local_res10[0];
    iVar9 = local_704[0]._8_4_;
    if (pHVar29 == (HMONITOR)0x0) {
      Y = -0x80000000;
      X = -0x80000000;
    }
  }
  local_b08.style = 0x2b;
  local_b08._4_4_ = 0;
  local_b08.lpfnWndProc = FUN_1401b2e80;
  local_b08.hInstance = local_res18;
  local_b08.cbClsExtra = 0;
  local_b08.cbWndExtra = 0;
  local_b08.lpszClassName = L"File Pilot";
  local_b08.hIcon = (HICON)0x0;
  local_b08.hCursor = (HCURSOR)0x0;
  local_b08.lpszMenuName = (LPCWSTR)0x0;
  local_b08.hCursor = LoadCursorW((HINSTANCE)0x0,(LPCWSTR)&DAT_00007f00);
  *(HCURSOR *)((longlong)lpParam + 0x9d8) = local_b08.hCursor;
  pHVar30 = LoadCursorW((HINSTANCE)0x0,(LPCWSTR)&DAT_00007f89);
  *(HCURSOR *)((longlong)lpParam + 0x9e0) = pHVar30;
  pHVar30 = LoadCursorW((HINSTANCE)0x0,(LPCWSTR)&DAT_00007f84);
  *(HCURSOR *)((longlong)lpParam + 0x9e8) = pHVar30;
  pHVar30 = LoadCursorW((HINSTANCE)0x0,(LPCWSTR)&DAT_00007f85);
  *(HCURSOR *)((longlong)lpParam + 0x9f0) = pHVar30;
  pHVar30 = LoadCursorW((HINSTANCE)0x0,(LPCWSTR)&DAT_00007f01);
  *(HCURSOR *)((longlong)lpParam + 0x9f8) = pHVar30;
  pHVar30 = LoadCursorW((HINSTANCE)0x0,(LPCWSTR)&DAT_00007f86);
  *(HCURSOR *)((longlong)lpParam + 0xa00) = pHVar30;
  pHVar30 = LoadCursorW((HINSTANCE)0x0,(LPCWSTR)&DAT_00007f88);
  *(HCURSOR *)((longlong)lpParam + 0xa08) = pHVar30;
  UVar12 = RegisterWindowMessageW(L"MSWHEEL_ROLLMSG");
  *(UINT *)((longlong)lpParam + 0x9d0) = UVar12;
  local_b08.hbrBackground = CreateSolidBrush(0);
  iVar13 = 0;
  AVar6 = RegisterClassW(&local_b08);
  if (AVar6 != 0) {
    pHVar31 = CreateWindowExW(0,local_b08.lpszClassName,L"File Pilot",0xcf0000,X,Y,iVar9,iVar10,
                              (HWND)0x0,(HMENU)0x0,local_res18,lpParam);
    *(HWND *)((longlong)lpParam + 0x9c8) = pHVar31;
  }
  if (*(longlong *)((longlong)lpParam + 0x9c8) != 0) {
    FUN_140196fa0();
    if (DAT_1401e8b80 == (FARPROC)0x0) {
      if (DAT_1401e8b70 != (FARPROC)0x0) {
        local_res10[0] = 0x60;
        local_res18 = (HMODULE)CONCAT44(local_res18._4_4_,0x60);
        pHVar29 = MonitorFromWindow(*(HWND *)((longlong)lpParam + 0x9c8),2);
        if ((pHVar29 != (HMONITOR)0x0) &&
           (IVar32 = (*DAT_1401e8b70)(pHVar29,0,local_res10,&local_res18), -1 < (int)IVar32)) {
          *(int *)((longlong)lpParam + 100) = local_res10[0];
          goto LAB_14018a46d;
        }
      }
      hdc = GetDC((HWND)0x0);
      iVar10 = GetDeviceCaps(hdc,0x58);
      *(int *)((longlong)lpParam + 100) = iVar10;
      ReleaseDC((HWND)0x0,hdc);
    }
    else {
      IVar32 = (*DAT_1401e8b80)(*(undefined8 *)((longlong)lpParam + 0x9c8));
      *(int *)((longlong)lpParam + 100) = (int)IVar32;
    }
LAB_14018a46d:
    *(undefined4 *)((longlong)lpParam + 0x60) = 1;
    FUN_140133a80(local_cc8,local_238);
    uVar27 = FUN_1401b59c0(lpParam,0x28,8,1,
                           "C:\\Users\\Vjekoslav\\Projects\\Voidstar\\File Pilot\\Source\\Win\\WinMain.c"
                           ,CONCAT44(uVar35,0x2c1),"WinMainCRTStartup");
    *(undefined8 *)((longlong)lpParam + 0x1a48) = uVar27;
    iVar13 = FUN_14019c050(*(undefined8 *)((longlong)lpParam + 0x9c8));
  }
  FUN_14018cdb0();
  if (iVar13 != 0) {
    *(undefined8 *)((longlong)lpParam + 0x8a0) = FUN_1401961c0;
    *(undefined1 **)((longlong)lpParam + 0x8a8) = &LAB_140195a60;
    *(code **)((longlong)lpParam + 0x8b0) = FUN_140196220;
    *(code **)((longlong)lpParam + 0x8b8) = FUN_140195a70;
    *(code **)((longlong)lpParam + 0x8c0) = FUN_140195c90;
    *(code **)((longlong)lpParam + 0x8c8) = FUN_140195bd0;
    *(code **)((longlong)lpParam + 0x8d0) = FUN_140196070;
    *(undefined8 *)((longlong)lpParam + 0x8f0) = *(undefined8 *)((longlong)lpParam + 0x9c8);
    *(undefined8 **)((longlong)lpParam + 0x8d8) = (undefined8 *)((longlong)lpParam + 0x8a0);
    FUN_140195030();
    FUN_1400666b0(0);
    AddClipboardFormatListener(*(HWND *)((longlong)lpParam + 0x9c8));
    *(undefined4 *)((longlong)lpParam + 0x898) = 1;
    UVar12 = GetDoubleClickTime();
    *(ulonglong *)((longlong)lpParam + 0x130) = (ulonglong)UVar12;
    local_res10[0] = 0;
    BVar7 = SystemParametersInfoW(0x66,0,local_res10,0);
    if (BVar7 != 0) {
      *(int *)((longlong)lpParam + 0x11c) = local_res10[0];
    }
    *(undefined4 *)((longlong)lpParam + 0x120) = 0x78;
    local_res18 = (HMODULE)CONCAT44(local_res18._4_4_,3);
    BVar7 = SystemParametersInfoW(0x68,0,&local_res18,0);
    if ((BVar7 != 0) && ((int)local_res18 == -1)) {
      *(undefined2 *)((longlong)lpParam + 0x124) = 1;
    }
    *(short *)((longlong)lpParam + 0x126) = (short)local_res18;
    iVar10 = *(int *)((longlong)lpParam + 0x48);
    while (iVar10 == 0) {
      iVar10 = FUN_1400c4360();
      if (iVar10 == 2) {
        *(undefined4 *)((longlong)lpParam + 0x40) = 1;
        if (bVar34) {
          FUN_140194910(lpParam);
          ShowWindow(*(HWND *)((longlong)lpParam + 0x9c8),3);
        }
        else {
          ShowWindow(*(HWND *)((longlong)lpParam + 0x9c8),10);
          SetWindowPos(*(HWND *)((longlong)lpParam + 0x9c8),(HWND)0x0,0,0,0,0,0x27);
        }
      }
      if (*(int *)((longlong)lpParam + 0x40) == 0) {
        FUN_1401a45e0(lpParam);
      }
      FUN_1401bace0(local_cc8,0,0x1b8);
      FUN_1401a9d60(lpParam);
      if (((bVar34) && (uVar14 = FUN_1400c4360(), 2 < uVar14)) &&
         (uVar33 = FUN_1400c7090(), auVar1._8_8_ = 0, auVar1._0_8_ = uVar33,
         lVar28 = SUB168(ZEXT816(0x624dd2f1a9fbe77) * auVar1,8),
         500 < (uVar33 - lVar28 >> 1) + lVar28 >> 9)) {
        FUN_140196230(lpParam);
        bVar34 = false;
      }
      iVar10 = FUN_1400c4360();
      if (iVar10 == 0x3c) {
        FUN_1400cb980(FUN_14018d230);
      }
      iVar10 = *(int *)((longlong)lpParam + 0x48);
    }
    local_cc8._8_4_ = 0;
    local_cc8._12_4_ = 0;
    local_cc8._0_8_ = (longlong)lpParam + 0xe8;
    local_c38[0] = L'\0';
    local_c38[1] = L'\0';
    local_c38[2] = L'\0';
    local_c38[3] = L'\0';
    local_c38[4] = L'\0';
    local_c38[5] = L'\0';
    local_c38[6] = L'\0';
    local_c38[7] = L'\0';
    local_c38[8] = L'\0';
    local_c38[9] = L'\0';
    local_c38[10] = L'\0';
    local_c38[0xb] = L'\0';
    local_c38[0xc] = L'\0';
    local_c38[0xd] = L'\0';
    local_c38[0xe] = L'\0';
    local_c38[0xf] = L'\0';
    local_c38[0x10] = L'\0';
    local_c38[0x11] = L'\0';
    local_c38[0x12] = L'\0';
    local_c38[0x13] = L'\0';
    local_c38[0x14] = L'\0';
    local_c38[0x15] = L'\0';
    local_c38[0x16] = L'\0';
    local_c38[0x17] = L'\0';
    local_cc8._16_4_ = 0;
    local_cc8._20_2_ = L'\0';
    local_cc8._22_2_ = L'\0';
    local_cc8._24_2_ = L'\0';
    local_cc8._26_2_ = L'\0';
    local_cc8._28_2_ = L'\0';
    local_cc8._30_2_ = L'\0';
    local_ca8 = (char *)0x0;
    aWStack_ca0[0] = L'\0';
    aWStack_ca0[1] = L'\0';
    aWStack_ca0[2] = L'\0';
    aWStack_ca0[3] = L'\0';
    local_c98 = local_c98 & 0xffffffff00000000;
    aWStack_c90[0] = L'\0';
    aWStack_c90[1] = L'\0';
    aWStack_c90[2] = L'\0';
    aWStack_c90[3] = L'\0';
    aWStack_c90[4] = L'\0';
    aWStack_c90[5] = L'\0';
    aWStack_c90[6] = L'\0';
    aWStack_c90[7] = L'\0';
    aWStack_c90[8] = L'\0';
    aWStack_c90[9] = L'\0';
    aWStack_c90[10] = L'\0';
    aWStack_c90[0xb] = L'\0';
    aWStack_c90[0xc] = L'\0';
    aWStack_c90[0xd] = L'\0';
    aWStack_c90[0xe] = L'\0';
    aWStack_c90[0xf] = L'\0';
    aWStack_c90[0x10] = L'\0';
    aWStack_c90[0x11] = L'\0';
    aWStack_c90[0x12] = L'\0';
    aWStack_c90[0x13] = L'\0';
    aWStack_c90[0x14] = L'\0';
    aWStack_c90[0x15] = L'\0';
    aWStack_c90[0x16] = L'\0';
    aWStack_c90[0x17] = L'\0';
    local_c60._0_2_ = L'\0';
    local_c60._2_2_ = L'\0';
    local_c60._4_2_ = L'\0';
    local_c60._6_2_ = L'\0';
    local_c58[0] = L'\0';
    local_c58[1] = L'\0';
    local_c58[2] = L'\0';
    local_c58[3] = L'\0';
    local_c58[4] = L'\0';
    local_c58[5] = L'\0';
    local_c58[6] = L'\0';
    local_c58[7] = L'\0';
    local_c58[8] = L'\0';
    local_c58[9] = L'\0';
    local_c58[10] = L'\0';
    local_c58[0xb] = L'\0';
    local_c58[0xc] = L'\0';
    local_c58[0xd] = L'\0';
    local_c58[0xe] = L'\0';
    local_c38[0x18] = L'\0';
    local_c38[0x19] = L'\0';
    local_c38[0x1a] = L'\0';
    local_c38[0x1b] = L'\0';
    local_c38[0x1c] = L'\0';
    local_c38[0x1d] = L'\0';
    local_c38[0x1e] = L'\0';
    local_c38[0x1f] = L'\0';
    local_bf8 = 0;
    FUN_140122600();
  }
LAB_14018a7d4:
  dwMilliseconds = 0x14;
  iVar10 = *(int *)((longlong)lpParam + 0x1b20);
  while (iVar10 != 0) {
    Sleep(dwMilliseconds);
    dwMilliseconds = FUN_1400679f0(100,dwMilliseconds + 0x14);
    iVar10 = *(int *)((longlong)lpParam + 0x1b20);
  }
                    /* WARNING: Subroutine does not return */
  ExitProcess(0);
}

