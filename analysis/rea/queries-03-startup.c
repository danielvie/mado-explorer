
/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

void FUN_140129430(undefined8 *param_1,longlong param_2)

{
  int *piVar1;
  char *pcVar2;
  undefined4 uVar3;
  undefined8 uVar4;
  undefined8 uVar5;
  undefined8 uVar6;
  undefined8 uVar7;
  undefined8 uVar8;
  undefined8 uVar9;
  bool bVar10;
  bool bVar11;
  code *pcVar12;
  uint uVar13;
  int iVar14;
  undefined8 *puVar15;
  undefined8 *puVar16;
  longlong lVar17;
  ulonglong *puVar18;
  undefined1 (*pauVar19) [16];
  longlong lVar20;
  longlong lVar21;
  undefined8 *puVar22;
  undefined8 *puVar23;
  undefined1 *puVar24;
  undefined8 uVar25;
  longlong *plVar26;
  ulonglong uVar27;
  undefined1 *puVar28;
  undefined1 *puVar29;
  longlong lVar30;
  longlong *plVar31;
  undefined1 *puVar32;
  ulonglong *puVar33;
  undefined1 *puVar34;
  ulonglong uVar35;
  LPVOID pvVar36;
  undefined8 *puVar37;
  bool bVar38;
  int local_res10;
  ulonglong *local_res18;
  longlong local_res20;
  undefined1 *local_598;
  undefined1 *puStack_590;
  char *local_588;
  ulonglong uStack_580;
  ulonglong local_578;
  ulonglong uStack_570;
  ulonglong local_568;
  ulonglong local_558;
  ulonglong uStack_550;
  longlong local_548;
  longlong lStack_540;
  longlong local_538;
  ulonglong uStack_530;
  code *local_528;
  LPVOID pvStack_520;
  longlong *local_518;
  undefined8 uStack_510;
  ulonglong local_4f8;
  ulonglong uStack_4f0;
  undefined8 local_4e8;
  undefined8 uStack_4e0;
  undefined8 local_4d8;
  undefined1 *local_4a8;
  longlong lStack_4a0;
  ulonglong local_498;
  ulonglong uStack_490;
  ulonglong local_488;
  undefined1 local_480;
  undefined4 local_47c;
  char *local_458;
  undefined8 uStack_450;
  undefined8 local_448;
  undefined8 uStack_440;
  undefined8 local_438;
  ulonglong uStack_430;
  ulonglong local_408 [5];
  ulonglong uStack_3e0;
  ulonglong local_3d8;
  ulonglong uStack_3d0;
  ulonglong local_3c8;
  ulonglong uStack_3c0;
  ulonglong local_3b8;
  ulonglong uStack_3b0;
  ulonglong local_3a8;
  ulonglong uStack_3a0;
  ulonglong local_398;
  ulonglong uStack_390;
  ulonglong local_388;
  ulonglong uStack_380;
  ulonglong local_378;
  ulonglong uStack_370;
  ulonglong local_368;
  ulonglong uStack_360;
  ulonglong local_358;
  ulonglong uStack_350;
  undefined1 local_348 [2] [16];
  undefined8 local_328;
  undefined8 uStack_320;
  undefined4 local_318;
  undefined4 uStack_314;
  undefined4 uStack_310;
  undefined4 uStack_30c;
  undefined8 local_308;
  undefined1 local_2f8 [16];
  ulonglong local_2e8;
  char *local_2d8 [7];
  undefined8 local_2a0;
  char *local_298;
  undefined8 local_290;
  char *local_288;
  undefined8 local_280;
  char *local_278;
  undefined8 local_270;
  undefined1 local_268 [272];
  ulonglong local_158;
  ulonglong uStack_150;
  ulonglong local_148;
  ulonglong uStack_140;
  ulonglong local_138;
  ulonglong uStack_130;
  ulonglong local_128;
  ulonglong uStack_120;
  ulonglong local_118;
  ulonglong uStack_110;
  ulonglong local_108;
  ulonglong uStack_100;
  ulonglong local_f8;
  ulonglong uStack_f0;
  ulonglong local_e8;
  ulonglong uStack_e0;
  ulonglong local_d8;
  ulonglong uStack_d0;
  ulonglong local_c8;
  ulonglong uStack_c0;
  ulonglong local_b8;
  ulonglong uStack_b0;
  ulonglong local_a8;
  ulonglong uStack_a0;
  
  puVar22 = (undefined8 *)0x0;
  local_548 = 0x40000;
  lStack_540 = 0x100000;
  local_538 = 0;
  uStack_530 = 0;
  local_528 = (code *)0x0;
  pvStack_520 = (LPVOID)0x0;
  local_518 = (longlong *)0x0;
  uStack_510 = 0;
  FUN_1401b5c50(&local_548,0x1280,0x40);
  pvVar36 = pvStack_520;
  pcVar12 = local_528;
  uVar35 = (ulonglong)((int)pvStack_520 + (int)local_528 & 0x3f);
  puVar16 = (undefined8 *)(0x40 - uVar35);
  if (uVar35 == 0) {
    puVar16 = puVar22;
  }
  uVar35 = (longlong)local_528 + 0x1280 + (longlong)puVar16;
  puVar15 = puVar22;
  if ((local_548 != 0) &&
     (uVar27 = local_548 - 1U & uVar35, puVar15 = (undefined8 *)0x0, uVar27 != 0)) {
    puVar15 = (undefined8 *)(local_548 - uVar27);
  }
  uVar27 = (longlong)puVar15 + uVar35;
  if (uStack_530 < uVar27) {
    uStack_530 = uVar27;
    VirtualAlloc(pvStack_520,uVar27,0x1000,4);
  }
  plVar31 = (longlong *)((longlong)puVar16 + (longlong)pvVar36 + (longlong)pcVar12);
  FUN_1401bace0(plVar31 + 8,0,0x1240);
  *plVar31 = local_548;
  plVar31[1] = lStack_540;
  plVar31[2] = local_538;
  plVar31[3] = uStack_530;
  plVar31[4] = uVar35;
  plVar31[5] = (longlong)pvStack_520;
  plVar31[6] = (longlong)plVar31;
  *(undefined4 *)(plVar31 + 7) = (undefined4)uStack_510;
  *(undefined4 *)((longlong)plVar31 + 0x3c) = uStack_510._4_4_;
  *(longlong **)*param_1 = plVar31;
  plVar26 = *(longlong **)*param_1;
  plVar26[0x121] = plVar26[0x121] + 1;
  *(undefined4 *)(plVar26 + 0x18) = 1;
  local_528 = (code *)uVar35;
  local_518 = plVar31;
  puVar16 = (undefined8 *)FUN_140110890(&local_548,plVar26);
  DAT_1401e88b0 = plVar26 + 8;
  DAT_1401e8908 = *puVar16;
  DAT_1401e8910 = puVar16[1];
  _DAT_1401e8918 = puVar16[2];
  uRam00000001401e8920 = puVar16[3];
  _DAT_1401e8928 = puVar16[4];
  uRam00000001401e8930 = puVar16[5];
  _DAT_1401e8938 = puVar16[6];
  DAT_1401e8940 = puVar16[7];
  DAT_1401e8948 = plVar26 + 0x241;
  _DAT_1401e8950 = plVar26 + 0x18;
  DAT_1401e8958 = param_2;
  *DAT_1401e88b0 = 0x100000;
  plVar26[0x10] = 0x100000;
  lVar17 = param_1[3];
  plVar26[0x19] = param_1[2];
  plVar26[0x1a] = lVar17;
  lVar17 = param_1[5];
  plVar26[0x1b] = param_1[4];
  plVar26[0x1c] = lVar17;
  lVar17 = param_1[7];
  plVar26[0x1d] = param_1[6];
  plVar26[0x1e] = lVar17;
  lVar17 = param_1[9];
  plVar26[0x1f] = param_1[8];
  plVar26[0x20] = lVar17;
  lVar17 = param_1[0xb];
  plVar26[0x21] = param_1[10];
  plVar26[0x22] = lVar17;
  plVar26[0x23] = param_1[0xc];
  plVar31 = (longlong *)FUN_1401b8530(&local_578,plVar26,plVar26[0x19],plVar26[0x1a]);
  lVar17 = plVar31[1];
  plVar26[0x19] = *plVar31;
  plVar26[0x1a] = lVar17;
  plVar26[0x24] = param_1[1];
  plVar31 = (longlong *)FUN_14009de50(&local_548,plVar26,0);
  lVar17 = plVar31[1];
  plVar26[0x239] = *plVar31;
  plVar26[0x23a] = lVar17;
  lVar17 = plVar31[3];
  plVar26[0x23b] = plVar31[2];
  plVar26[0x23c] = lVar17;
  lVar17 = plVar31[5];
  plVar26[0x23d] = plVar31[4];
  plVar26[0x23e] = lVar17;
  lVar17 = plVar31[7];
  plVar26[0x23f] = plVar31[6];
  plVar26[0x240] = lVar17;
  _DAT_1401e88c8 = plVar26[0x239];
  DAT_1401e88d0 = plVar26[0x23a];
  _DAT_1401e88d8 = plVar26[0x23b];
  lRam00000001401e88e0 = plVar26[0x23c];
  _DAT_1401e88e8 = plVar26[0x23d];
  lRam00000001401e88f0 = plVar26[0x23e];
  _DAT_1401e88f8 = (undefined4)plVar26[0x23f];
  uRam00000001401e88fc = *(undefined4 *)((longlong)plVar26 + 0x11fc);
  DAT_1401e8900._0_4_ = (undefined4)plVar26[0x240];
  DAT_1401e8900._4_4_ = *(undefined4 *)((longlong)plVar26 + 0x1204);
  _DAT_1401e8978 = FUN_14010ea50;
  _DAT_1401e8980 = FUN_14010ee70;
  _DAT_1401e8988 = FUN_14010eb60;
  _DAT_1401e8990 = FUN_14010ef80;
  _DAT_1401e8998 = FUN_14010e710;
  _DAT_1401e89a0 = FUN_14010e820;
  _DAT_1401e89a8 = FUN_14010e930;
  _DAT_1401e89b0 = FUN_14010ed80;
  _DAT_1401e89b8 = FUN_14010ec70;
  plVar26[0x6d] = 0x800;
  lVar17 = FUN_1401b59c0(plVar26 + 0x65,0x4000,8,1,
                         "C:\\Users\\Vjekoslav\\Projects\\Voidstar\\File Pilot\\Source\\Program\\Program.c"
                         ,0x2d62,"Prog_GotoTableInit");
  plVar26[0x6f] = lVar17;
  lVar17 = FUN_140114d00(0x100);
  plVar26[0x41] = lVar17;
  lVar17 = FUN_1401b6980(plVar26 + 0xa1,plVar26 + 0xa9,plVar26[0xb0],0x18,plVar26[0xa9] + 0x100);
  plVar26[0xb0] = lVar17;
  lVar17 = FUN_1401b6980(plVar26 + 0xa1,plVar26 + 0xb1,plVar26[0xb8],0x18,plVar26[0xb1] + 0x100);
  plVar26[0xb8] = lVar17;
  lVar17 = FUN_1401b6980(plVar26,plVar26 + 0xb9,plVar26[0xc0],0x18,plVar26[0xb9] + 0x100);
  plVar26[0xc0] = lVar17;
  lVar17 = FUN_1401b5dc0(plVar26,plVar26 + 0xcc,plVar26[0xce],0x10,0x80,0);
  plVar26[0xce] = lVar17;
  puVar16 = (undefined8 *)FUN_1400c6b20(&local_548,0,0);
  pcVar2 = (char *)*puVar16;
  uVar25 = puVar16[1];
  uVar4 = puVar16[2];
  uVar5 = puVar16[3];
  uVar6 = puVar16[4];
  uVar7 = puVar16[5];
  lVar17 = FUN_1401b5dc0(plVar26,plVar26 + 0xd7,plVar26[0xd9],0x20,0x19,0);
  plVar26[0xd9] = lVar17;
  lVar17 = FUN_1401b6980(plVar26,plVar26 + 0xcf,plVar26[0xd6],0x18,plVar26[0xcf] + 0x19);
  plVar26[0xd6] = lVar17;
  local_2d8[2] = (char *)0x0;
  local_2d8[3] = (char *)0x0;
  local_2d8[4] = (char *)0x0;
  local_2d8[5] = (char *)0x0;
  FUN_1401bace0(local_268,0,0x110);
  local_2d8[1] = (char *)0x12;
  local_2a0 = 0x17;
  local_2d8[0] = "PlacesKeywordsHome";
  local_290 = 0x13;
  local_2d8[6] = "PlacesKeywordsDocuments";
  local_280 = 0x16;
  local_298 = "PlacesKeywordsMusic";
  local_288 = "PlacesKeywordsPictures";
  local_278 = "PlacesKeywordsVideos";
  local_488 = 0;
  local_4d8 = 0;
  local_270 = 0x14;
  local_4f8 = 0;
  uStack_4f0 = 0;
  local_4e8 = 0;
  uStack_4e0 = 0;
  lVar17 = FUN_140128670(plVar26,&local_4f8);
  uStack_580 = 0x10;
  *(byte *)(lVar17 + 8) = *(byte *)(lVar17 + 8) | 8;
  *(undefined1 *)(lVar17 + 9) = 1;
  local_588 = "PlacesKeywordsPC";
  puVar16 = (undefined8 *)FUN_140126400(&local_578,&local_588,0);
  local_4a8 = (undefined1 *)0x0;
  local_498 = 0;
  uStack_490 = 0;
  lStack_4a0 = 0;
  local_598 = (undefined1 *)*puVar16;
  puStack_590 = (undefined1 *)puVar16[1];
  local_488 = 0;
  local_480 = 0x2c;
  local_47c = 0;
  local_588 = pcVar2;
  puVar18 = (ulonglong *)FUN_1401b9110(local_2f8,pcVar2,&local_598,&local_4a8);
  uVar35 = puVar18[2];
  puVar16 = (undefined8 *)*puVar18;
  puVar15 = puVar22;
  puVar23 = puVar22;
  local_568 = uVar35;
  if (puVar16 != (undefined8 *)0x0) {
    do {
      puVar15 = (undefined8 *)
                FUN_1401b59c0(plVar26,0x20,8,1,
                              "C:\\Users\\Vjekoslav\\Projects\\Voidstar\\File Pilot\\Source\\Program\\Program.c"
                              ,0x4e,"Prog_Init");
      puVar37 = (undefined8 *)(ulonglong)((int)puVar23 + 1);
      puVar23 = (undefined8 *)(uVar35 + (longlong)puVar23 * 0x10);
      uVar8 = *puVar23;
      uVar9 = puVar23[1];
      puVar15[3] = puVar22;
      *puVar15 = uVar8;
      puVar15[1] = uVar9;
      puVar22 = puVar15;
      puVar23 = puVar37;
    } while (puVar37 < puVar16);
  }
  local_598 = (undefined1 *)0x0;
  puStack_590 = (undefined1 *)0x0;
  lVar17 = FUN_1401b6ec0(plVar26,plVar26 + 0xcf,plVar26[0xd6],0x18,&local_598,1);
  plVar26[0xd6] = lVar17;
  local_res18 = (ulonglong *)0x0;
  local_res20 = 0;
  *(undefined8 **)(lVar17 + 0x10 + plVar26[0xd1] * 0x18) = puVar15;
  local_res10 = 1;
  do {
    plVar31 = plVar26 + 0xd7;
    puVar16 = (undefined8 *)0x0;
    puVar34 = *(undefined1 **)(&DAT_1401e8bf8 + local_res20);
    lVar17 = *(longlong *)(local_res20 + 0x1401e8c00);
    if (lVar17 != 0) {
      local_598 = puVar34;
      puStack_590 = (undefined1 *)lVar17;
      pauVar19 = (undefined1 (*) [16])FUN_1401ae920(&local_458,&local_598,1);
      local_348[0] = *pauVar19;
      local_328 = *(undefined8 *)pauVar19[2];
      uStack_320 = *(undefined8 *)(pauVar19[2] + 8);
      local_318 = *(undefined4 *)pauVar19[3];
      uStack_314 = *(undefined4 *)(pauVar19[3] + 4);
      uStack_310 = *(undefined4 *)(pauVar19[3] + 8);
      uStack_30c = *(undefined4 *)(pauVar19[3] + 0xc);
      local_308 = *(undefined8 *)pauVar19[4];
      if (local_348[0][1] != '\0') {
        local_4e8 = CONCAT44(local_348[0]._8_4_,(int)local_348[0][5]);
        local_498 = local_4e8;
        uStack_4e0 = *(undefined8 *)pauVar19[1];
        local_4d8 = *(undefined8 *)(pauVar19[1] + 8);
        local_4a8 = puVar34;
        lStack_4a0 = lVar17;
        uStack_490 = *(undefined8 *)pauVar19[1];
        local_488 = *(undefined8 *)(pauVar19[1] + 8);
        lVar20 = FUN_140128670(plVar26,&local_4a8);
        *(byte *)(lVar20 + 8) = *(byte *)(lVar20 + 8) | 8;
        *(int *)(lVar20 + 0x1c) = local_res10;
        *(bool *)(lVar20 + 9) = 0xb < (longlong)local_res18 - 9U;
        if ((longlong)local_res18 < 0xe) {
          local_4e8 = 0;
          uStack_4e0 = 0;
          local_598 = puVar34;
          puStack_590 = (undefined1 *)lVar17;
          lVar20 = FUN_1400c8f40(local_408,0,&local_598);
          uVar8 = *(undefined8 *)(lVar20 + 0x18);
          uVar9 = *(undefined8 *)(lVar20 + 0x20);
          lVar21 = FUN_1401b5dc0(plVar26,plVar31,plVar26[0xd9],0x20,1,0);
          lVar20 = *plVar31;
          plVar26[0xd9] = lVar21;
          puVar22 = (undefined8 *)(lVar20 * 0x20 + lVar21);
          *puVar22 = puVar34;
          puVar22[1] = lVar17;
          puVar22 = (undefined8 *)(lVar20 * 0x20 + 0x10 + lVar21);
          *puVar22 = uVar8;
          puVar22[1] = uVar9;
          *plVar31 = *plVar31 + 1;
        }
        local_598 = *(undefined1 **)((longlong)local_2d8 + local_res20);
        puStack_590 = *(undefined1 **)((longlong)local_2d8 + local_res20 + 8);
        puVar22 = (undefined8 *)FUN_140126400(&local_578,&local_598,0);
        local_458 = (char *)0x0;
        local_448 = 0;
        uStack_440 = 0;
        local_598 = (undefined1 *)*puVar22;
        puStack_590 = (undefined1 *)puVar22[1];
        uStack_450 = 0;
        local_438 = 0;
        uStack_430 = CONCAT71(uStack_430._1_7_,0x2c);
        uStack_430 = uStack_430 & 0xffffffff;
        puVar18 = (ulonglong *)FUN_1401b9110(&local_4f8,local_588,&local_598,&local_458);
        puVar22 = (undefined8 *)*puVar18;
        uVar35 = puVar18[2];
        puVar15 = puVar16;
        puVar23 = puVar16;
        local_2e8 = uVar35;
        if (puVar22 != (undefined8 *)0x0) {
          do {
            puVar23 = (undefined8 *)
                      FUN_1401b59c0(plVar26,0x20,8,1,
                                    "C:\\Users\\Vjekoslav\\Projects\\Voidstar\\File Pilot\\Source\\Program\\Program.c"
                                    ,0x71,"Prog_Init");
            puVar37 = (undefined8 *)(ulonglong)((int)puVar16 + 1);
            puVar16 = (undefined8 *)(uVar35 + (longlong)puVar16 * 0x10);
            uVar8 = *puVar16;
            uVar9 = puVar16[1];
            puVar23[3] = puVar15;
            *puVar23 = uVar8;
            puVar23[1] = uVar9;
            puVar16 = puVar37;
            puVar15 = puVar23;
          } while (puVar37 < puVar22);
        }
        local_598 = puVar34;
        puStack_590 = (undefined1 *)lVar17;
        lVar20 = FUN_1401b6ec0(plVar26,plVar26 + 0xcf,plVar26[0xd6],0x18,&local_598,1);
        plVar26[0xd6] = lVar20;
        *(undefined8 **)(lVar20 + 0x10 + plVar26[0xd1] * 0x18) = puVar23;
        local_598 = puVar34;
        puStack_590 = (undefined1 *)lVar17;
        puVar16 = (undefined8 *)FUN_140067a00(&local_558,&local_598);
        local_598 = (undefined1 *)*puVar16;
        puStack_590 = (undefined1 *)puVar16[1];
        lVar17 = FUN_1401b6ec0(plVar26,plVar26 + 0xb9,plVar26[0xc0],0x18,&local_598,1);
        plVar26[0xc0] = lVar17;
        *(undefined4 *)(lVar17 + 0x10 + plVar26[0xbb] * 0x18) = 1;
      }
    }
    puVar34 = (undefined1 *)0x0;
    local_res10 = local_res10 + 1;
    local_res18 = (ulonglong *)((longlong)local_res18 + 1);
    local_res20 = local_res20 + 0x10;
  } while (local_res20 < 0x180);
  local_458 = pcVar2;
  uStack_450 = uVar25;
  local_448 = uVar4;
  uStack_440 = uVar5;
  local_438 = uVar6;
  uStack_430 = uVar7;
  FUN_140001a90(&local_458);
  FUN_140165a20(plVar26[0xd9],plVar26[0xd7],0x20,&LAB_140141070);
  plVar31 = plVar26 + 0x17e;
  plVar26[0x133] = 0;
  plVar26[0x137] = 0;
  plVar26[0x134] = 0x100000000;
  plVar26[0x138] = 0x100000000;
  plVar26[0x135] = 0x200000000;
  plVar26[0x139] = 0x200000000;
  plVar26[0x136] = 0x300000000;
  plVar26[0x13a] = 0x300000000;
  *(undefined4 *)(plVar26 + 0x13b) = 1;
  puVar28 = (undefined1 *)plVar26[0x180];
  bVar11 = false;
  if (puVar28 == (undefined1 *)0x0) {
    lVar17 = plVar26[4];
    pvVar36 = (LPVOID)plVar26[5];
    puVar32 = (undefined1 *)plVar26[3];
    uVar35 = (ulonglong)((int)lVar17 + (int)pvVar36 & 7);
    puVar28 = (undefined1 *)(8 - uVar35);
    if (uVar35 == 0) {
      puVar28 = puVar34;
    }
    puVar29 = puVar28 + 0x10;
    if (puVar32 < puVar29 + lVar17) {
      if ((undefined1 *)plVar26[2] < puVar29 + lVar17) {
        FUN_1401b5c50(plVar26,puVar29,8);
        pvVar36 = (LPVOID)plVar26[5];
        lVar17 = plVar26[4];
        puVar32 = (undefined1 *)plVar26[3];
        uVar35 = (ulonglong)((int)lVar17 + (int)pvVar36 & 7);
        puVar28 = (undefined1 *)(8 - uVar35);
        if (uVar35 == 0) {
          puVar28 = puVar34;
        }
        puVar29 = puVar28 + 0x10;
      }
      lVar20 = *plVar26;
      puVar24 = puVar34;
      if ((lVar20 != 0) &&
         (uVar35 = lVar20 - 1U & (ulonglong)(puVar29 + lVar17), puVar24 = (undefined1 *)0x0,
         uVar35 != 0)) {
        puVar24 = (undefined1 *)(lVar20 - uVar35);
      }
      puVar24 = puVar24 + (longlong)(puVar29 + lVar17);
      if (puVar32 < puVar24) {
        plVar26[3] = (longlong)puVar24;
        VirtualAlloc(pvVar36,(SIZE_T)puVar24,0x1000,4);
      }
    }
    puVar28 = puVar28 + plVar26[5] + plVar26[4];
    plVar26[6] = (longlong)puVar28;
    plVar26[4] = (longlong)(puVar29 + plVar26[4]);
    *plVar31 = 0;
    plVar26[0x17f] = 1;
  }
  else {
    uVar35 = plVar26[0x17f];
    uVar27 = *plVar31 + 1;
    if (uVar35 < uVar27) {
      if (uVar27 < uVar35 * 2) {
        uVar27 = uVar35 * 2;
      }
      puVar28 = (undefined1 *)
                FUN_1401b5ae0(plVar26,puVar28,uVar35 << 4,uVar27 << 4,8,0,
                              "C:\\Users\\Vjekoslav\\Projects\\Voidstar\\File Pilot\\Source\\Program\\Voidstar.h"
                              ,0xa4b,"_ArrayGrow");
      plVar26[0x17f] = uVar27;
    }
  }
  puVar18 = (ulonglong *)(plVar26 + 0x17f);
  lVar17 = *plVar31;
  plVar26[0x180] = (longlong)puVar28;
  *(char **)(puVar28 + lVar17 * 0x10) = "HK_ToggleExpandFolder";
  *(undefined8 *)(puVar28 + lVar17 * 0x10 + 8) = 0x15;
  lVar17 = *plVar31;
  puVar28 = (undefined1 *)plVar26[0x180];
  *plVar31 = lVar17 + 1;
  if (puVar28 == (undefined1 *)0x0) {
    lVar17 = plVar26[4];
    pvVar36 = (LPVOID)plVar26[5];
    puVar32 = (undefined1 *)plVar26[3];
    uVar35 = (ulonglong)((int)lVar17 + (int)pvVar36 & 7);
    puVar28 = (undefined1 *)(8 - uVar35);
    if (uVar35 == 0) {
      puVar28 = puVar34;
    }
    puVar29 = puVar28 + 0x10;
    if (puVar32 < puVar29 + lVar17) {
      if ((undefined1 *)plVar26[2] < puVar29 + lVar17) {
        FUN_1401b5c50(plVar26,puVar29,8);
        pvVar36 = (LPVOID)plVar26[5];
        lVar17 = plVar26[4];
        puVar32 = (undefined1 *)plVar26[3];
        uVar35 = (ulonglong)((int)lVar17 + (int)pvVar36 & 7);
        puVar28 = (undefined1 *)(8 - uVar35);
        if (uVar35 == 0) {
          puVar28 = puVar34;
        }
        puVar29 = puVar28 + 0x10;
      }
      lVar20 = *plVar26;
      puVar24 = puVar34;
      if ((lVar20 != 0) &&
         (uVar35 = lVar20 - 1U & (ulonglong)(puVar29 + lVar17), puVar24 = (undefined1 *)0x0,
         uVar35 != 0)) {
        puVar24 = (undefined1 *)(lVar20 - uVar35);
      }
      puVar24 = puVar24 + (longlong)(puVar29 + lVar17);
      if (puVar32 < puVar24) {
        plVar26[3] = (longlong)puVar24;
        VirtualAlloc(pvVar36,(SIZE_T)puVar24,0x1000,4);
      }
    }
    puVar28 = puVar28 + plVar26[5] + plVar26[4];
    plVar26[6] = (longlong)puVar28;
    plVar26[4] = (longlong)(puVar29 + plVar26[4]);
    *plVar31 = 0;
    plVar26[0x17f] = 1;
  }
  else {
    uVar35 = *puVar18;
    uVar27 = lVar17 + 2;
    if (uVar35 < uVar27) {
      if (uVar27 < uVar35 * 2) {
        uVar27 = uVar35 * 2;
      }
      puVar28 = (undefined1 *)
                FUN_1401b5ae0(plVar26,puVar28,uVar35 << 4,uVar27 << 4,8,0,
                              "C:\\Users\\Vjekoslav\\Projects\\Voidstar\\File Pilot\\Source\\Program\\Voidstar.h"
                              ,0xa4b,"_ArrayGrow");
      *puVar18 = uVar27;
    }
  }
  lVar17 = *plVar31;
  plVar26[0x180] = (longlong)puVar28;
  *(char **)(puVar28 + lVar17 * 0x10) = "HK_InspectorToggle";
  *(undefined8 *)(puVar28 + lVar17 * 0x10 + 8) = 0x12;
  lVar17 = *plVar31;
  puVar28 = (undefined1 *)plVar26[0x180];
  *plVar31 = lVar17 + 1;
  if (puVar28 == (undefined1 *)0x0) {
    lVar17 = plVar26[4];
    pvVar36 = (LPVOID)plVar26[5];
    puVar32 = (undefined1 *)plVar26[3];
    uVar35 = (ulonglong)((int)lVar17 + (int)pvVar36 & 7);
    puVar28 = (undefined1 *)(8 - uVar35);
    if (uVar35 == 0) {
      puVar28 = puVar34;
    }
    puVar29 = puVar28 + 0x10;
    if (puVar32 < puVar29 + lVar17) {
      if ((undefined1 *)plVar26[2] < puVar29 + lVar17) {
        FUN_1401b5c50(plVar26,puVar29,8);
        pvVar36 = (LPVOID)plVar26[5];
        lVar17 = plVar26[4];
        puVar32 = (undefined1 *)plVar26[3];
        uVar35 = (ulonglong)((int)lVar17 + (int)pvVar36 & 7);
        puVar28 = (undefined1 *)(8 - uVar35);
        if (uVar35 == 0) {
          puVar28 = puVar34;
        }
        puVar29 = puVar28 + 0x10;
      }
      lVar20 = *plVar26;
      puVar24 = puVar34;
      if ((lVar20 != 0) &&
         (uVar35 = lVar20 - 1U & (ulonglong)(puVar29 + lVar17), puVar24 = (undefined1 *)0x0,
         uVar35 != 0)) {
        puVar24 = (undefined1 *)(lVar20 - uVar35);
      }
      puVar24 = puVar24 + (longlong)(puVar29 + lVar17);
      if (puVar32 < puVar24) {
        plVar26[3] = (longlong)puVar24;
        VirtualAlloc(pvVar36,(SIZE_T)puVar24,0x1000,4);
      }
    }
    puVar28 = puVar28 + plVar26[5] + plVar26[4];
    plVar26[6] = (longlong)puVar28;
    plVar26[4] = (longlong)(puVar29 + plVar26[4]);
    *plVar31 = 0;
    *puVar18 = 1;
  }
  else {
    uVar35 = *puVar18;
    uVar27 = lVar17 + 2;
    if (uVar35 < uVar27) {
      if (uVar27 < uVar35 * 2) {
        uVar27 = uVar35 * 2;
      }
      puVar28 = (undefined1 *)
                FUN_1401b5ae0(plVar26,puVar28,uVar35 << 4,uVar27 << 4,8,0,
                              "C:\\Users\\Vjekoslav\\Projects\\Voidstar\\File Pilot\\Source\\Program\\Voidstar.h"
                              ,0xa4b,"_ArrayGrow");
      *puVar18 = uVar27;
    }
  }
  lVar17 = *plVar31;
  plVar26[0x180] = (longlong)puVar28;
  *(char **)(puVar28 + lVar17 * 0x10) = "HK_PanelSplitRight";
  *(undefined8 *)(puVar28 + lVar17 * 0x10 + 8) = 0x12;
  lVar17 = *plVar31;
  puVar28 = (undefined1 *)plVar26[0x180];
  *plVar31 = lVar17 + 1;
  if (puVar28 == (undefined1 *)0x0) {
    lVar17 = plVar26[4];
    pvVar36 = (LPVOID)plVar26[5];
    puVar32 = (undefined1 *)plVar26[3];
    uVar35 = (ulonglong)((int)lVar17 + (int)pvVar36 & 7);
    puVar28 = (undefined1 *)(8 - uVar35);
    if (uVar35 == 0) {
      puVar28 = puVar34;
    }
    puVar29 = puVar28 + 0x10;
    if (puVar32 < puVar29 + lVar17) {
      if ((undefined1 *)plVar26[2] < puVar29 + lVar17) {
        FUN_1401b5c50(plVar26,puVar29,8);
        pvVar36 = (LPVOID)plVar26[5];
        lVar17 = plVar26[4];
        puVar32 = (undefined1 *)plVar26[3];
        uVar35 = (ulonglong)((int)lVar17 + (int)pvVar36 & 7);
        puVar28 = (undefined1 *)(8 - uVar35);
        if (uVar35 == 0) {
          puVar28 = puVar34;
        }
        puVar29 = puVar28 + 0x10;
      }
      lVar20 = *plVar26;
      puVar24 = puVar34;
      if ((lVar20 != 0) &&
         (uVar35 = lVar20 - 1U & (ulonglong)(puVar29 + lVar17), puVar24 = (undefined1 *)0x0,
         uVar35 != 0)) {
        puVar24 = (undefined1 *)(lVar20 - uVar35);
      }
      puVar24 = puVar24 + (longlong)(puVar29 + lVar17);
      if (puVar32 < puVar24) {
        plVar26[3] = (longlong)puVar24;
        VirtualAlloc(pvVar36,(SIZE_T)puVar24,0x1000,4);
      }
    }
    puVar28 = puVar28 + plVar26[5] + plVar26[4];
    plVar26[6] = (longlong)puVar28;
    plVar26[4] = (longlong)(puVar29 + plVar26[4]);
    *plVar31 = 0;
    *puVar18 = 1;
  }
  else {
    uVar35 = *puVar18;
    uVar27 = lVar17 + 2;
    if (uVar35 < uVar27) {
      if (uVar27 < uVar35 * 2) {
        uVar27 = uVar35 * 2;
      }
      puVar28 = (undefined1 *)
                FUN_1401b5ae0(plVar26,puVar28,uVar35 << 4,uVar27 << 4,8,0,
                              "C:\\Users\\Vjekoslav\\Projects\\Voidstar\\File Pilot\\Source\\Program\\Voidstar.h"
                              ,0xa4b,"_ArrayGrow");
      *puVar18 = uVar27;
    }
  }
  lVar17 = *plVar31;
  plVar26[0x180] = (longlong)puVar28;
  *(char **)(puVar28 + lVar17 * 0x10) = "HK_PanelSplitBottom";
  *(undefined8 *)(puVar28 + lVar17 * 0x10 + 8) = 0x13;
  *plVar31 = *plVar31 + 1;
  lVar20 = FUN_1401b5dc0(plVar26,plVar31,plVar26[0x180],0x10,1,0);
  lVar17 = *plVar31;
  plVar26[0x180] = lVar20;
  *(char **)(lVar20 + lVar17 * 0x10) = "HK_TabNew";
  *(undefined8 *)(lVar20 + 8 + lVar17 * 0x10) = 9;
  *plVar31 = *plVar31 + 1;
  lVar20 = FUN_1401b5dc0(plVar26,plVar31,plVar26[0x180],0x10,1,0);
  lVar17 = *plVar31;
  plVar26[0x180] = lVar20;
  *(char **)(lVar20 + lVar17 * 0x10) = "HK_TabClose";
  *(undefined8 *)(lVar20 + 8 + lVar17 * 0x10) = 0xb;
  *plVar31 = *plVar31 + 1;
  lVar20 = FUN_1401b5dc0(plVar26,plVar31,plVar26[0x180],0x10,1,0);
  lVar17 = *plVar31;
  plVar26[0x180] = lVar20;
  *(char **)(lVar20 + lVar17 * 0x10) = "HK_OptionsOpen";
  *(undefined8 *)(lVar20 + 8 + lVar17 * 0x10) = 0xe;
  *plVar31 = *plVar31 + 1;
  lVar20 = FUN_1401b5dc0(plVar26,plVar31,plVar26[0x180],0x10,1,0);
  lVar17 = *plVar31;
  plVar26[0x180] = lVar20;
  *(char **)(lVar20 + lVar17 * 0x10) = "HK_ExplorerOpen";
  *(undefined8 *)(lVar20 + 8 + lVar17 * 0x10) = 0xf;
  lVar17 = *plVar31;
  puVar28 = (undefined1 *)plVar26[0x180];
  *plVar31 = lVar17 + 1;
  if (puVar28 == (undefined1 *)0x0) {
    lVar17 = plVar26[4];
    pvVar36 = (LPVOID)plVar26[5];
    puVar32 = (undefined1 *)plVar26[3];
    uVar35 = (ulonglong)((int)lVar17 + (int)pvVar36 & 7);
    puVar28 = (undefined1 *)(8 - uVar35);
    if (uVar35 == 0) {
      puVar28 = puVar34;
    }
    puVar29 = puVar28 + 0x10;
    if (puVar32 < puVar29 + lVar17) {
      if ((undefined1 *)plVar26[2] < puVar29 + lVar17) {
        FUN_1401b5c50(plVar26,puVar29,8);
        pvVar36 = (LPVOID)plVar26[5];
        lVar17 = plVar26[4];
        puVar32 = (undefined1 *)plVar26[3];
        uVar35 = (ulonglong)((int)lVar17 + (int)pvVar36 & 7);
        puVar28 = (undefined1 *)(8 - uVar35);
        if (uVar35 == 0) {
          puVar28 = puVar34;
        }
        puVar29 = puVar28 + 0x10;
      }
      lVar20 = *plVar26;
      puVar24 = puVar34;
      if ((lVar20 != 0) &&
         (uVar35 = lVar20 - 1U & (ulonglong)(puVar29 + lVar17), puVar24 = (undefined1 *)0x0,
         uVar35 != 0)) {
        puVar24 = (undefined1 *)(lVar20 - uVar35);
      }
      puVar24 = puVar24 + (longlong)(puVar29 + lVar17);
      if (puVar32 < puVar24) {
        plVar26[3] = (longlong)puVar24;
        VirtualAlloc(pvVar36,(SIZE_T)puVar24,0x1000,4);
      }
    }
    puVar28 = puVar28 + plVar26[5] + plVar26[4];
    plVar26[6] = (longlong)puVar28;
    plVar26[4] = (longlong)(puVar29 + plVar26[4]);
    *plVar31 = 0;
    *puVar18 = 1;
  }
  else {
    uVar35 = *puVar18;
    uVar27 = lVar17 + 2;
    if (uVar35 < uVar27) {
      if (uVar27 < uVar35 * 2) {
        uVar27 = uVar35 * 2;
      }
      puVar28 = (undefined1 *)
                FUN_1401b5ae0(plVar26,puVar28,uVar35 << 4,uVar27 << 4,8,0,
                              "C:\\Users\\Vjekoslav\\Projects\\Voidstar\\File Pilot\\Source\\Program\\Voidstar.h"
                              ,0xa4b,"_ArrayGrow");
      *puVar18 = uVar27;
    }
  }
  lVar17 = *plVar31;
  plVar26[0x180] = (longlong)puVar28;
  *(char **)(puVar28 + lVar17 * 0x10) = "HK_ConsoleOpen";
  *(undefined8 *)(puVar28 + lVar17 * 0x10 + 8) = 0xe;
  lVar17 = *plVar31;
  puVar28 = (undefined1 *)plVar26[0x180];
  *plVar31 = lVar17 + 1;
  if (puVar28 == (undefined1 *)0x0) {
    lVar17 = plVar26[4];
    pvVar36 = (LPVOID)plVar26[5];
    puVar32 = (undefined1 *)plVar26[3];
    uVar35 = (ulonglong)((int)lVar17 + (int)pvVar36 & 7);
    puVar28 = (undefined1 *)(8 - uVar35);
    if (uVar35 == 0) {
      puVar28 = puVar34;
    }
    puVar29 = puVar28 + 0x10;
    if (puVar32 < puVar29 + lVar17) {
      if ((undefined1 *)plVar26[2] < puVar29 + lVar17) {
        FUN_1401b5c50(plVar26,puVar29,8);
        pvVar36 = (LPVOID)plVar26[5];
        lVar17 = plVar26[4];
        puVar32 = (undefined1 *)plVar26[3];
        uVar35 = (ulonglong)((int)lVar17 + (int)pvVar36 & 7);
        puVar28 = (undefined1 *)(8 - uVar35);
        if (uVar35 == 0) {
          puVar28 = puVar34;
        }
        puVar29 = puVar28 + 0x10;
      }
      lVar20 = *plVar26;
      puVar24 = puVar34;
      if ((lVar20 != 0) &&
         (uVar35 = lVar20 - 1U & (ulonglong)(puVar29 + lVar17), puVar24 = (undefined1 *)0x0,
         uVar35 != 0)) {
        puVar24 = (undefined1 *)(lVar20 - uVar35);
      }
      puVar24 = puVar24 + (longlong)(puVar29 + lVar17);
      if (puVar32 < puVar24) {
        plVar26[3] = (longlong)puVar24;
        VirtualAlloc(pvVar36,(SIZE_T)puVar24,0x1000,4);
      }
    }
    puVar28 = puVar28 + plVar26[5] + plVar26[4];
    plVar26[6] = (longlong)puVar28;
    plVar26[4] = (longlong)(puVar29 + plVar26[4]);
    *plVar31 = 0;
    *puVar18 = 1;
  }
  else {
    uVar35 = *puVar18;
    uVar27 = lVar17 + 2;
    if (uVar35 < uVar27) {
      if (uVar27 < uVar35 * 2) {
        uVar27 = uVar35 * 2;
      }
      puVar28 = (undefined1 *)
                FUN_1401b5ae0(plVar26,puVar28,uVar35 << 4,uVar27 << 4,8,0,
                              "C:\\Users\\Vjekoslav\\Projects\\Voidstar\\File Pilot\\Source\\Program\\Voidstar.h"
                              ,0xa4b,"_ArrayGrow");
      *puVar18 = uVar27;
    }
  }
  lVar17 = *plVar31;
  plVar26[0x180] = (longlong)puVar28;
  *(char **)(puVar28 + lVar17 * 0x10) = "HK_ToggleFileExtension";
  *(undefined8 *)(puVar28 + lVar17 * 0x10 + 8) = 0x16;
  lVar17 = *plVar31;
  puVar28 = (undefined1 *)plVar26[0x180];
  *plVar31 = lVar17 + 1;
  if (puVar28 == (undefined1 *)0x0) {
    lVar17 = plVar26[4];
    pvVar36 = (LPVOID)plVar26[5];
    puVar32 = (undefined1 *)plVar26[3];
    uVar35 = (ulonglong)((int)lVar17 + (int)pvVar36 & 7);
    puVar28 = (undefined1 *)(8 - uVar35);
    if (uVar35 == 0) {
      puVar28 = puVar34;
    }
    puVar29 = puVar28 + 0x10;
    if (puVar32 < puVar29 + lVar17) {
      if ((undefined1 *)plVar26[2] < puVar29 + lVar17) {
        FUN_1401b5c50(plVar26,puVar29,8);
        pvVar36 = (LPVOID)plVar26[5];
        lVar17 = plVar26[4];
        puVar32 = (undefined1 *)plVar26[3];
        uVar35 = (ulonglong)((int)lVar17 + (int)pvVar36 & 7);
        puVar28 = (undefined1 *)(8 - uVar35);
        if (uVar35 == 0) {
          puVar28 = puVar34;
        }
        puVar29 = puVar28 + 0x10;
      }
      lVar20 = *plVar26;
      puVar24 = puVar34;
      if ((lVar20 != 0) &&
         (uVar35 = lVar20 - 1U & (ulonglong)(puVar29 + lVar17), puVar24 = (undefined1 *)0x0,
         uVar35 != 0)) {
        puVar24 = (undefined1 *)(lVar20 - uVar35);
      }
      puVar24 = puVar24 + (longlong)(puVar29 + lVar17);
      if (puVar32 < puVar24) {
        plVar26[3] = (longlong)puVar24;
        VirtualAlloc(pvVar36,(SIZE_T)puVar24,0x1000,4);
      }
    }
    puVar28 = puVar28 + plVar26[5] + plVar26[4];
    plVar26[6] = (longlong)puVar28;
    plVar26[4] = (longlong)(puVar29 + plVar26[4]);
    *plVar31 = 0;
    *puVar18 = 1;
  }
  else {
    uVar35 = *puVar18;
    uVar27 = lVar17 + 2;
    if (uVar35 < uVar27) {
      if (uVar27 < uVar35 * 2) {
        uVar27 = uVar35 * 2;
      }
      puVar28 = (undefined1 *)
                FUN_1401b5ae0(plVar26,puVar28,uVar35 << 4,uVar27 << 4,8,0,
                              "C:\\Users\\Vjekoslav\\Projects\\Voidstar\\File Pilot\\Source\\Program\\Voidstar.h"
                              ,0xa4b,"_ArrayGrow");
      *puVar18 = uVar27;
    }
  }
  lVar17 = *plVar31;
  plVar26[0x180] = (longlong)puVar28;
  *(char **)(puVar28 + lVar17 * 0x10) = "HK_ToggleHiddenFiles";
  *(undefined8 *)(puVar28 + lVar17 * 0x10 + 8) = 0x14;
  lVar17 = *plVar31;
  puVar28 = (undefined1 *)plVar26[0x180];
  *plVar31 = lVar17 + 1;
  if (puVar28 == (undefined1 *)0x0) {
    lVar17 = plVar26[4];
    pvVar36 = (LPVOID)plVar26[5];
    puVar32 = (undefined1 *)plVar26[3];
    uVar35 = (ulonglong)((int)lVar17 + (int)pvVar36 & 7);
    puVar28 = (undefined1 *)(8 - uVar35);
    if (uVar35 == 0) {
      puVar28 = puVar34;
    }
    puVar29 = puVar28 + 0x10;
    if (puVar32 < puVar29 + lVar17) {
      if ((undefined1 *)plVar26[2] < puVar29 + lVar17) {
        FUN_1401b5c50(plVar26,puVar29,8);
        pvVar36 = (LPVOID)plVar26[5];
        lVar17 = plVar26[4];
        puVar32 = (undefined1 *)plVar26[3];
        uVar35 = (ulonglong)((int)lVar17 + (int)pvVar36 & 7);
        puVar28 = (undefined1 *)(8 - uVar35);
        if (uVar35 == 0) {
          puVar28 = puVar34;
        }
        puVar29 = puVar28 + 0x10;
      }
      lVar20 = *plVar26;
      puVar24 = puVar34;
      if ((lVar20 != 0) &&
         (uVar35 = lVar20 - 1U & (ulonglong)(puVar29 + lVar17), puVar24 = (undefined1 *)0x0,
         uVar35 != 0)) {
        puVar24 = (undefined1 *)(lVar20 - uVar35);
      }
      puVar24 = puVar24 + (longlong)(puVar29 + lVar17);
      if (puVar32 < puVar24) {
        plVar26[3] = (longlong)puVar24;
        VirtualAlloc(pvVar36,(SIZE_T)puVar24,0x1000,4);
      }
    }
    puVar28 = puVar28 + plVar26[4] + plVar26[5];
    plVar26[6] = (longlong)puVar28;
    plVar26[4] = (longlong)(puVar29 + plVar26[4]);
    *plVar31 = 0;
    *puVar18 = 1;
  }
  else {
    uVar35 = *puVar18;
    uVar27 = lVar17 + 2;
    if (uVar35 < uVar27) {
      if (uVar27 < uVar35 * 2) {
        uVar27 = uVar35 * 2;
      }
      puVar28 = (undefined1 *)
                FUN_1401b5ae0(plVar26,puVar28,uVar35 << 4,uVar27 << 4,8,0,
                              "C:\\Users\\Vjekoslav\\Projects\\Voidstar\\File Pilot\\Source\\Program\\Voidstar.h"
                              ,0xa4b,"_ArrayGrow");
      *puVar18 = uVar27;
    }
  }
  lVar17 = *plVar31;
  plVar26[0x180] = (longlong)puVar28;
  *(char **)(puVar28 + lVar17 * 0x10) = "HK_ToggleSystemFiles";
  *(undefined8 *)(puVar28 + lVar17 * 0x10 + 8) = 0x14;
  lVar17 = *plVar31;
  puVar28 = (undefined1 *)plVar26[0x180];
  *plVar31 = lVar17 + 1;
  if (puVar28 == (undefined1 *)0x0) {
    pvVar36 = (LPVOID)plVar26[5];
    lVar17 = plVar26[4];
    uVar13 = (int)lVar17 + (int)pvVar36 & 7;
    puVar28 = puVar34;
    if (uVar13 != 0) {
      puVar28 = (undefined1 *)(8 - (ulonglong)uVar13);
    }
    puVar29 = (undefined1 *)plVar26[3];
    puVar32 = puVar28 + 0x10;
    if (puVar29 < puVar32 + lVar17) {
      if ((undefined1 *)plVar26[2] < puVar32 + lVar17) {
        FUN_1401b5c50(plVar26,puVar32,8);
        pvVar36 = (LPVOID)plVar26[5];
        lVar17 = plVar26[4];
        puVar29 = (undefined1 *)plVar26[3];
        uVar35 = (ulonglong)((int)lVar17 + (int)pvVar36 & 7);
        puVar28 = (undefined1 *)(8 - uVar35);
        if (uVar35 == 0) {
          puVar28 = puVar34;
        }
        puVar32 = puVar28 + 0x10;
      }
      lVar20 = *plVar26;
      puVar24 = puVar34;
      if ((lVar20 != 0) &&
         (uVar35 = lVar20 - 1U & (ulonglong)(puVar32 + lVar17), puVar24 = (undefined1 *)0x0,
         uVar35 != 0)) {
        puVar24 = (undefined1 *)(lVar20 - uVar35);
      }
      puVar24 = puVar24 + (longlong)(puVar32 + lVar17);
      if (puVar29 < puVar24) {
        plVar26[3] = (longlong)puVar24;
        VirtualAlloc(pvVar36,(SIZE_T)puVar24,0x1000,4);
      }
    }
    puVar28 = puVar28 + plVar26[5] + plVar26[4];
    plVar26[6] = (longlong)puVar28;
    plVar26[4] = (longlong)(puVar32 + plVar26[4]);
    *plVar31 = 0;
    *puVar18 = 1;
  }
  else {
    uVar35 = *puVar18;
    uVar27 = lVar17 + 2;
    if (uVar35 < uVar27) {
      if (uVar27 < uVar35 * 2) {
        uVar27 = uVar35 * 2;
      }
      puVar28 = (undefined1 *)
                FUN_1401b5ae0(plVar26,puVar28,uVar35 << 4,uVar27 << 4,8,0,
                              "C:\\Users\\Vjekoslav\\Projects\\Voidstar\\File Pilot\\Source\\Program\\Voidstar.h"
                              ,0xa4b,"_ArrayGrow");
      *puVar18 = uVar27;
    }
  }
  lVar17 = *plVar31;
  plVar26[0x180] = (longlong)puVar28;
  *(char **)(puVar28 + lVar17 * 0x10) = "HK_ToggleRecents";
  *(undefined8 *)(puVar28 + lVar17 * 0x10 + 8) = 0x10;
  lVar17 = *plVar31;
  puVar28 = (undefined1 *)plVar26[0x180];
  *plVar31 = lVar17 + 1;
  if (puVar28 == (undefined1 *)0x0) {
    lVar17 = plVar26[4];
    pvVar36 = (LPVOID)plVar26[5];
    puVar32 = (undefined1 *)plVar26[3];
    uVar35 = (ulonglong)((int)lVar17 + (int)pvVar36 & 7);
    puVar28 = (undefined1 *)(8 - uVar35);
    if (uVar35 == 0) {
      puVar28 = puVar34;
    }
    puVar29 = puVar28 + 0x10;
    if (puVar32 < puVar29 + lVar17) {
      if ((undefined1 *)plVar26[2] < puVar29 + lVar17) {
        FUN_1401b5c50(plVar26,puVar29,8);
        pvVar36 = (LPVOID)plVar26[5];
        lVar17 = plVar26[4];
        puVar32 = (undefined1 *)plVar26[3];
        uVar35 = (ulonglong)((int)lVar17 + (int)pvVar36 & 7);
        puVar28 = (undefined1 *)(8 - uVar35);
        if (uVar35 == 0) {
          puVar28 = puVar34;
        }
        puVar29 = puVar28 + 0x10;
      }
      lVar20 = *plVar26;
      puVar24 = puVar34;
      if ((lVar20 != 0) &&
         (uVar35 = lVar20 - 1U & (ulonglong)(puVar29 + lVar17), puVar24 = (undefined1 *)0x0,
         uVar35 != 0)) {
        puVar24 = (undefined1 *)(lVar20 - uVar35);
      }
      puVar24 = puVar24 + (longlong)(puVar29 + lVar17);
      if (puVar32 < puVar24) {
        plVar26[3] = (longlong)puVar24;
        VirtualAlloc(pvVar36,(SIZE_T)puVar24,0x1000,4);
      }
    }
    puVar28 = puVar28 + plVar26[5] + plVar26[4];
    plVar26[6] = (longlong)puVar28;
    plVar26[4] = (longlong)(puVar29 + plVar26[4]);
    *plVar31 = 0;
    *puVar18 = 1;
  }
  else {
    uVar35 = *puVar18;
    uVar27 = lVar17 + 2;
    if (uVar35 < uVar27) {
      if (uVar27 < uVar35 * 2) {
        uVar27 = uVar35 * 2;
      }
      puVar28 = (undefined1 *)
                FUN_1401b5ae0(plVar26,puVar28,uVar35 << 4,uVar27 << 4,8,0,
                              "C:\\Users\\Vjekoslav\\Projects\\Voidstar\\File Pilot\\Source\\Program\\Voidstar.h"
                              ,0xa4b,"_ArrayGrow");
      *puVar18 = uVar27;
    }
  }
  lVar17 = *plVar31;
  plVar26[0x180] = (longlong)puVar28;
  *(char **)(puVar28 + lVar17 * 0x10) = "HK_ToggleSortRecents";
  *(undefined8 *)(puVar28 + lVar17 * 0x10 + 8) = 0x14;
  *plVar31 = *plVar31 + 1;
  puVar16 = (undefined8 *)FUN_1400c6b20(&local_548,0,0);
  local_408[0] = plVar26[0x19];
  local_408[1] = plVar26[0x1a];
  local_408[2] = plVar26[0x1b];
  local_408[3] = plVar26[0x1c];
  pcVar2 = (char *)*puVar16;
  uVar25 = puVar16[1];
  uVar4 = puVar16[2];
  uVar5 = puVar16[3];
  uVar6 = puVar16[4];
  uVar7 = puVar16[5];
  local_408[4] = plVar26[0x1d];
  uStack_3e0 = plVar26[0x1e];
  local_3d8 = plVar26[0x1f];
  uStack_3d0 = plVar26[0x20];
  local_3c8 = plVar26[0x21];
  uStack_3c0 = plVar26[0x22];
  local_3b8 = plVar26[0x23];
  lVar17 = FUN_140036170(local_408);
  plVar26[0x25] = lVar17;
  FUN_1400226c0(lVar17);
  local_408[0] = plVar26[0x19];
  local_408[1] = plVar26[0x1a];
  local_408[2] = plVar26[0x1b];
  local_408[3] = plVar26[0x1c];
  local_408[4] = plVar26[0x1d];
  uStack_3e0 = plVar26[0x1e];
  local_3d8 = plVar26[0x1f];
  uStack_3d0 = plVar26[0x20];
  local_3c8 = plVar26[0x21];
  uStack_3c0 = plVar26[0x22];
  local_3b8 = plVar26[0x23];
  puVar18 = (ulonglong *)FUN_140039820(local_2d8,pcVar2,0,local_408);
  local_158 = *puVar18;
  uStack_150 = puVar18[1];
  local_148 = puVar18[2];
  uStack_140 = puVar18[3];
  local_138 = puVar18[4];
  uStack_130 = puVar18[5];
  local_128 = puVar18[6];
  uStack_120 = puVar18[7];
  local_118 = puVar18[8];
  uStack_110 = puVar18[9];
  local_108 = puVar18[10];
  uStack_100 = puVar18[0xb];
  local_f8 = puVar18[0xc];
  uStack_f0 = puVar18[0xd];
  local_e8 = puVar18[0xe];
  uStack_e0 = puVar18[0xf];
  local_d8 = puVar18[0x10];
  uStack_d0 = puVar18[0x11];
  local_c8 = puVar18[0x12];
  uStack_c0 = puVar18[0x13];
  local_b8 = puVar18[0x14];
  uStack_b0 = puVar18[0x15];
  local_a8 = puVar18[0x16];
  uStack_a0 = puVar18[0x17];
  iVar14 = (int)local_128;
  DAT_1401e88b8 = local_118;
  plVar26[0x26] = local_118;
  if ((int)local_128 != 5) {
    local_408[0] = plVar26[0x19];
    local_408[1] = plVar26[0x1a];
    local_408[2] = plVar26[0x1b];
    local_408[3] = plVar26[0x1c];
    local_408[4] = plVar26[0x1d];
    uStack_3e0 = plVar26[0x1e];
    local_3d8 = plVar26[0x1f];
    uStack_3d0 = plVar26[0x20];
    local_3c8 = plVar26[0x21];
    uStack_3c0 = plVar26[0x22];
    local_3b8 = plVar26[0x23];
    DAT_1401e88b8 = FUN_140036170(local_408);
    plVar26[0x26] = DAT_1401e88b8;
    if (iVar14 != 0) {
      FUN_14010fca0(0x15);
      FUN_1400356f0(plVar26);
    }
  }
  *(undefined4 *)(*(longlong *)(plVar26[0x26] + 0x48) + 0x408) =
       *(undefined4 *)((longlong)param_1 + 0x6c);
  *(undefined4 *)((longlong)plVar26 + 0x874) = *(undefined4 *)(param_1 + 0xd);
  if (iVar14 == 2) {
    FUN_14010e440(local_348,8);
  }
  else if (iVar14 == 4) {
    lVar17 = FUN_14010e440(local_348,0xb);
    (**(code **)(lVar17 + 8))(&local_548,local_158 & 0xffffffff);
  }
  FUN_1400226c0(plVar26[0x26]);
  FUN_14002f300(plVar26[0x26]);
  FUN_1401543c0(plVar26);
  local_408[0] = local_158;
  local_408[1] = uStack_150;
  local_408[2] = local_148;
  local_408[3] = uStack_140;
  local_408[4] = local_138;
  uStack_3e0 = uStack_130;
  local_3d8 = local_128;
  uStack_3d0 = uStack_120;
  local_3c8 = local_118;
  uStack_3c0 = uStack_110;
  local_3b8 = local_108;
  uStack_3b0 = uStack_100;
  local_3a8 = local_f8;
  uStack_3a0 = uStack_f0;
  local_398 = local_e8;
  uStack_390 = uStack_e0;
  local_388 = local_d8;
  uStack_380 = uStack_d0;
  local_378 = local_c8;
  uStack_370 = uStack_c0;
  local_368 = local_b8;
  uStack_360 = uStack_b0;
  local_358 = local_a8;
  uStack_350 = uStack_a0;
  FUN_140022400(plVar26,plVar26[0x26],local_408);
  local_458 = pcVar2;
  uStack_450 = uVar25;
  local_448 = uVar4;
  uStack_440 = uVar5;
  local_438 = uVar6;
  uStack_430 = uVar7;
  FUN_140001a90(&local_458);
  plVar26[0x127] = 0x200;
  lVar17 = plVar26[4];
  pvVar36 = (LPVOID)plVar26[5];
  puVar28 = (undefined1 *)plVar26[3];
  uVar35 = (ulonglong)((int)lVar17 + (int)pvVar36 & 7);
  puVar32 = (undefined1 *)(8 - uVar35);
  if (uVar35 == 0) {
    puVar32 = puVar34;
  }
  puVar29 = puVar32 + 0x2000;
  if (puVar28 < puVar29 + lVar17) {
    if ((undefined1 *)plVar26[2] < puVar29 + lVar17) {
      FUN_1401b5c50(plVar26,puVar29,8);
      pvVar36 = (LPVOID)plVar26[5];
      lVar17 = plVar26[4];
      puVar28 = (undefined1 *)plVar26[3];
      uVar35 = (ulonglong)((int)lVar17 + (int)pvVar36 & 7);
      puVar32 = (undefined1 *)(8 - uVar35);
      if (uVar35 == 0) {
        puVar32 = puVar34;
      }
      puVar29 = puVar32 + 0x2000;
    }
    lVar20 = *plVar26;
    puVar24 = puVar34;
    if ((lVar20 != 0) &&
       (uVar35 = lVar20 - 1U & (ulonglong)(puVar29 + lVar17), puVar24 = (undefined1 *)0x0,
       uVar35 != 0)) {
      puVar24 = (undefined1 *)(lVar20 - uVar35);
    }
    puVar24 = puVar24 + (longlong)(puVar29 + lVar17);
    if (puVar28 < puVar24) {
      plVar26[3] = (longlong)puVar24;
      VirtualAlloc(pvVar36,(SIZE_T)puVar24,0x1000,4);
    }
  }
  puVar32 = puVar32 + plVar26[4] + plVar26[5];
  FUN_1401bace0(puVar32,0,0x2000);
  plVar26[4] = (longlong)(puVar29 + plVar26[4]);
  plVar26[6] = (longlong)puVar32;
  local_598 = (undefined1 *)0x0;
  puStack_590 = (undefined1 *)0x0;
  plVar26[0x129] = (longlong)puVar32;
  lVar17 = FUN_140055be0(plVar26,&local_598,0,0);
  plVar26[0x12f] = lVar17;
  puVar18 = local_408;
  lVar17 = 0xc;
  local_408[0] = _DAT_1401e8bf8;
  local_408[1] = uRam00000001401e8c00;
  local_408[2] = _DAT_1401e8c08;
  local_408[3] = uRam00000001401e8c10;
  local_408[4] = _DAT_1401e8c18;
  uStack_3e0 = uRam00000001401e8c20;
  local_3d8 = _DAT_1401e8c28;
  uStack_3d0 = uRam00000001401e8c30;
  local_3c8 = _DAT_1401e8c38;
  uStack_3c0 = uRam00000001401e8c40;
  local_3b8 = _DAT_1401e8c48;
  uStack_3b0 = uRam00000001401e8c50;
  local_3a8 = _DAT_1401e8c58;
  uStack_3a0 = uRam00000001401e8c60;
  local_398 = _DAT_1401e8c88;
  uStack_390 = uRam00000001401e8c90;
  local_388 = _DAT_1401e8c98;
  uStack_380 = uRam00000001401e8ca0;
  local_378 = _DAT_1401e8cd8;
  uStack_370 = uRam00000001401e8ce0;
  local_368 = _DAT_1401e8ce8;
  uStack_360 = uRam00000001401e8cf0;
  local_358 = _DAT_1401e8cf8;
  uStack_350 = uRam00000001401e8d00;
  do {
    local_598 = (undefined1 *)*puVar18;
    puStack_590 = (undefined1 *)puVar18[1];
    FUN_140055be0(plVar26,&local_598,0,0);
    local_598 = (undefined1 *)*puVar18;
    puVar28 = (undefined1 *)puVar18[1];
    puStack_590 = puVar28;
    uVar25 = FUN_1401b6980(plVar26,plVar26 + 0xb9,plVar26[0xc0],0x18,0x10);
    uVar35 = 0x31415926;
    puVar32 = puVar34;
    if (puVar28 != (undefined1 *)0x0) {
      do {
        puVar29 = puVar32 + 1;
        uVar35 = (uVar35 << 9 | uVar35 >> 0x37) + (ulonglong)(byte)local_598[(longlong)puVar32];
        puVar32 = puVar29;
      } while (puVar29 < puVar28);
    }
    local_598 = (undefined1 *)*puVar18;
    puStack_590 = (undefined1 *)puVar18[1];
    uVar35 = (uVar35 ^ 0x31415926) * 0x40000 + ~(uVar35 ^ 0x31415926);
    uVar35 = (uVar35 >> 0x1f | uVar35 << 0x21) * 0x15;
    uVar35 = (uVar35 >> 0xb | uVar35 << 0x35) * 0x41;
    uVar35 = (uVar35 >> 0x16 | uVar35 << 0x2a) ^ uVar35;
    lVar20 = 0x31415928;
    if (1 < uVar35 + 0x31415926) {
      lVar20 = 0x31415926;
    }
    lVar20 = FUN_1401b67b0(plVar26,plVar26 + 0xb9,uVar25,0x18,&local_598,1,lVar20 + uVar35);
    uVar35 = *puVar18;
    uVar27 = puVar18[1];
    puVar18 = puVar18 + 2;
    puVar33 = (ulonglong *)(lVar20 + plVar26[0xbb] * 0x18);
    *puVar33 = uVar35;
    puVar33[1] = uVar27;
    plVar26[0xc0] = lVar20;
    *(undefined4 *)(lVar20 + 0x10 + plVar26[0xbb] * 0x18) = 1;
    lVar17 = lVar17 + -1;
  } while (lVar17 != 0);
  plVar31 = (longlong *)FUN_1401b7220(plVar26,plVar26 + 0x122,0x70,1,0x10);
  plVar26[0x121] = plVar26[0x121] + 1;
  *plVar31 = plVar26[0x121];
  plVar31[6] = 0;
  plVar31[7] = 0x3f8000003f800000;
  plVar26[0x123] = (longlong)plVar31;
  lVar20 = FUN_140110150(plVar26);
  *(longlong **)(lVar20 + 0x538) = plVar31;
  lVar17 = plVar31[1];
  if (lVar17 == 0) {
    plVar31[1] = lVar20;
  }
  else {
    for (lVar21 = *(longlong *)(lVar17 + 0x540); lVar21 != 0; lVar21 = *(longlong *)(lVar21 + 0x540)
        ) {
      lVar17 = lVar21;
    }
    *(longlong *)(lVar17 + 0x540) = lVar20;
  }
  if ((plVar26[0x124] == 0) ||
     (*(longlong *)(plVar26[0x124] + 0x78) != *(longlong *)(lVar20 + 0x78))) {
    plVar26[0x124] = lVar20;
    *(undefined2 *)(lVar20 + 0x678) = 1;
    plVar26[0x14a] = lVar20;
    for (lVar17 = *(longlong *)(*(longlong *)(lVar20 + 0x538) + 8); lVar17 != 0;
        lVar17 = *(longlong *)(lVar17 + 0x540)) {
      *(ushort *)(lVar17 + 0x548) = (ushort)(lVar17 != lVar20);
    }
  }
  piVar1 = (int *)param_1[1];
  uStack_580 = *(ulonglong *)(piVar1 + 2);
  local_588 = (char *)(*(longlong *)(piVar1 + 4) + (longlong)piVar1);
  plVar31 = (longlong *)FUN_140165d80(&local_578,plVar26,&local_588,piVar1[1] * *piVar1);
  plVar26[0x235] = *plVar31;
  *(int *)(plVar26 + 0x236) = *piVar1;
  *(int *)((longlong)plVar26 + 0x11b4) = piVar1[1];
  *(undefined4 *)((longlong)plVar26 + 0x11ba) = 0x10001;
  FUN_140178370(plVar26);
  *(undefined4 *)(DAT_1401e8bc8 + 0x17cc) = 0xe00a;
  *(undefined4 *)(DAT_1401e8bc8 + 0x17d0) = 0xe00b;
  *(undefined4 *)(DAT_1401e8bc8 + 0x17d4) = 0xe00c;
  *(undefined4 *)(DAT_1401e8bc8 + 0x17d8) = 0xe00d;
  *(code **)(DAT_1401e8bc8 + 0x80) = FUN_14012c540;
  *(longlong **)(DAT_1401e8bc8 + 0x88) = plVar26;
  lVar17 = DAT_1401e8bc8;
  puVar28 = *(undefined1 **)(DAT_1401e8bc8 + 0x1738);
  uVar35 = 1;
  if (puVar28 == (undefined1 *)0x0) {
    do {
      uVar27 = uVar35;
      uVar35 = uVar27 * 2;
    } while (uVar35 < 0x56);
    lVar21 = plVar26[4];
    pvVar36 = (LPVOID)plVar26[5];
    puVar32 = (undefined1 *)plVar26[3];
    lVar30 = uVar27 * 0x60;
    uVar27 = (ulonglong)((int)lVar21 + (int)pvVar36 & 7);
    puVar28 = (undefined1 *)(8 - uVar27);
    if (uVar27 == 0) {
      puVar28 = puVar34;
    }
    puVar29 = puVar28 + lVar30;
    if (puVar32 < puVar29 + lVar21) {
      if ((undefined1 *)plVar26[2] < puVar29 + lVar21) {
        FUN_1401b5c50(plVar26,puVar29,8);
        pvVar36 = (LPVOID)plVar26[5];
        lVar21 = plVar26[4];
        puVar32 = (undefined1 *)plVar26[3];
        uVar27 = (ulonglong)((int)lVar21 + (int)pvVar36 & 7);
        puVar28 = (undefined1 *)(8 - uVar27);
        if (uVar27 == 0) {
          puVar28 = puVar34;
        }
        puVar29 = puVar28 + lVar30;
      }
      lVar30 = *plVar26;
      puVar24 = puVar34;
      if ((lVar30 != 0) &&
         (uVar27 = lVar30 - 1U & (ulonglong)(puVar29 + lVar21), puVar24 = (undefined1 *)0x0,
         uVar27 != 0)) {
        puVar24 = (undefined1 *)(lVar30 - uVar27);
      }
      puVar24 = puVar24 + (longlong)(puVar29 + lVar21);
      if (puVar32 < puVar24) {
        plVar26[3] = (longlong)puVar24;
        VirtualAlloc(pvVar36,(SIZE_T)puVar24,0x1000,4);
      }
    }
    puVar28 = puVar28 + plVar26[5] + plVar26[4];
    plVar26[6] = (longlong)puVar28;
    plVar26[4] = (longlong)(puVar29 + plVar26[4]);
    *(undefined8 *)(lVar17 + 0x1728) = 0;
  }
  else {
    uVar27 = *(ulonglong *)(DAT_1401e8bc8 + 0x1730);
    uVar35 = *(longlong *)(DAT_1401e8bc8 + 0x1728) + 0x56;
    if (uVar35 <= uVar27) goto LAB_14012b85d;
    if (uVar35 < uVar27 * 2) {
      uVar35 = uVar27 * 2;
    }
    puVar28 = (undefined1 *)
              FUN_1401b5ae0(plVar26,puVar28,uVar27 * 0x30,uVar35 * 0x30,8,0,
                            "C:\\Users\\Vjekoslav\\Projects\\Voidstar\\File Pilot\\Source\\Program\\Voidstar.h"
                            ,0xa4b,"_ArrayGrow");
  }
  *(ulonglong *)(lVar17 + 0x1730) = uVar35;
LAB_14012b85d:
  *(undefined1 **)(DAT_1401e8bc8 + 0x1738) = puVar28;
  *(longlong *)(DAT_1401e8bc8 + 0x1728) = *(longlong *)(DAT_1401e8bc8 + 0x1728) + 0x56;
  FUN_14010c520(plVar26);
  local_548 = 3;
  local_528 = FUN_140100d60;
  uVar35 = (ulonglong)*(uint *)(plVar26 + 0x143);
  lStack_540 = 0;
  local_538 = 0;
  uStack_530 = 0;
  plVar26[uVar35 * 7 + 0x144] = 3;
  (plVar26 + uVar35 * 7 + 0x144)[1] = 0;
  pvStack_520 = (LPVOID)0x0;
  plVar26[uVar35 * 7 + 0x146] = 0;
  (plVar26 + uVar35 * 7 + 0x146)[1] = 0;
  plVar26[uVar35 * 7 + 0x148] = (longlong)FUN_140100d60;
  (plVar26 + uVar35 * 7 + 0x148)[1] = 0;
  local_res18 = (ulonglong *)0x0;
  plVar26[uVar35 * 7 + 0x14a] = lVar20;
  *(int *)(plVar26 + 0x143) = (int)plVar26[0x143] + 1;
  local_598 = (undefined1 *)0x0;
  puStack_590 = (undefined1 *)0x0;
  lVar17 = *(longlong *)(plVar26[0x26] + 0x48);
  local_558 = *(ulonglong *)(lVar17 + 0xac8);
  uStack_550 = *(ulonglong *)(lVar17 + 0xad0);
  puVar18 = *(ulonglong **)(lVar17 + 0xae0);
  local_4e8 = *(undefined8 *)(lVar17 + 0xaf0);
  local_518 = (longlong *)lVar20;
  if (puVar18 != (ulonglong *)0x0) {
    local_558 = *(ulonglong *)(lVar17 + 0xac8);
    local_588 = (char *)0x1;
    puVar28 = puVar34;
    uStack_580 = uStack_550;
    uStack_550 = *(ulonglong *)(lVar17 + 0xad0);
    do {
      local_578 = *puVar18;
      uStack_570 = puVar18[1];
      iVar14 = FUN_1401b87a0(&local_578,&local_558,&local_588);
      if (iVar14 != 0) {
        uStack_580 = CONCAT44(uStack_580._4_4_,1);
        puStack_590 = (undefined1 *)uStack_580;
        local_598 = puVar28;
        break;
      }
      puVar18 = (ulonglong *)puVar18[3];
      puVar28 = puVar28 + 1;
    } while (puVar18 != (ulonglong *)0x0);
  }
  bVar38 = local_598 == (undefined1 *)0x0;
  puVar18 = local_res18;
  if ((*(undefined1 **)(*(longlong *)(plVar26[0x25] + 0x48) + 0xaf0) <= local_598) &&
     (puVar33 = (ulonglong *)plVar26[0x3e], puVar33 != (ulonglong *)0x0)) {
    local_578 = *(ulonglong *)(lVar17 + 0xac8);
    uStack_570 = *(ulonglong *)(lVar17 + 0xad0);
    local_588 = (char *)0x0;
    uStack_580 = 0xffffffffffffffff;
    do {
      local_558 = *puVar33;
      uStack_550 = puVar33[1];
      iVar14 = FUN_1401b87a0(&local_558,&local_578,&local_588);
      puVar18 = puVar33;
      if (iVar14 != 0) break;
      puVar33 = (ulonglong *)puVar33[0x18];
      puVar18 = local_res18;
    } while (puVar33 != (ulonglong *)0x0);
  }
  local_res18 = puVar18;
  plVar31 = DAT_1401e88b0;
  lVar17 = param_1[8];
  local_588 = (char *)param_1[0xc];
  if ((lVar17 != 0) || (iVar14 = 0, local_588 != (char *)0x0)) {
    iVar14 = 1;
  }
  if (local_res18 == (ulonglong *)0x0) {
    bVar10 = true;
    if (bVar38) {
      puVar18 = (ulonglong *)FUN_140140030(local_2d8,plVar26,iVar14);
      local_158 = *puVar18;
      uStack_150 = puVar18[1];
      local_148 = puVar18[2];
      uStack_140 = puVar18[3];
      local_138 = puVar18[4];
      uStack_130 = puVar18[5];
      local_128 = puVar18[6];
      uStack_120 = puVar18[7];
      local_118 = puVar18[8];
      uStack_110 = puVar18[9];
      local_108 = puVar18[10];
      uStack_100 = puVar18[0xb];
      local_f8 = puVar18[0xc];
      uStack_f0 = puVar18[0xd];
      local_e8 = puVar18[0xe];
      uStack_e0 = puVar18[0xf];
      local_d8 = puVar18[0x10];
      uStack_d0 = puVar18[0x11];
      local_c8 = puVar18[0x12];
      uStack_c0 = puVar18[0x13];
      local_b8 = puVar18[0x14];
      uStack_b0 = puVar18[0x15];
      local_a8 = puVar18[0x16];
      uStack_a0 = puVar18[0x17];
      bVar10 = true;
      if ((int)uStack_a0 != 0) {
        bVar10 = bVar11;
      }
    }
  }
  else {
    lVar21 = DAT_1401e88b0[4];
    pvVar36 = (LPVOID)DAT_1401e88b0[5];
    uVar35 = (ulonglong)((int)lVar21 + (int)pvVar36 & 7);
    puVar28 = (undefined1 *)(8 - uVar35);
    if (uVar35 == 0) {
      puVar28 = puVar34;
    }
    puVar32 = puVar28 + 200;
    if ((undefined1 *)DAT_1401e88b0[3] < puVar32 + lVar21) {
      if ((undefined1 *)DAT_1401e88b0[2] < puVar32 + lVar21) {
        FUN_1401b5c50(DAT_1401e88b0,puVar32,8);
        pvVar36 = (LPVOID)plVar31[5];
        lVar21 = plVar31[4];
        uVar35 = (ulonglong)((int)lVar21 + (int)pvVar36 & 7);
        puVar28 = (undefined1 *)(8 - uVar35);
        if (uVar35 == 0) {
          puVar28 = puVar34;
        }
        puVar32 = puVar28 + 200;
      }
      lVar30 = *plVar31;
      puVar29 = puVar34;
      if ((lVar30 != 0) &&
         (uVar35 = lVar30 - 1U & (ulonglong)(puVar32 + lVar21), puVar29 = (undefined1 *)0x0,
         uVar35 != 0)) {
        puVar29 = (undefined1 *)(lVar30 - uVar35);
      }
      puVar29 = puVar29 + (longlong)(puVar32 + lVar21);
      if ((undefined1 *)plVar31[3] < puVar29) {
        plVar31[3] = (longlong)puVar29;
        VirtualAlloc(pvVar36,(SIZE_T)puVar29,0x1000,4);
      }
    }
    puVar28 = puVar28 + plVar31[4] + plVar31[5];
    FUN_1401bace0(puVar28,0,200);
    plVar31[4] = (longlong)(puVar32 + plVar31[4]);
    plVar31[6] = (longlong)puVar28;
    FUN_14012dbb0(DAT_1401e88b0,local_res18,puVar28);
    if (iVar14 != 0) {
      *(undefined8 *)(puVar28 + 0x98) = 0;
    }
    if ((int)*(char *)((longlong)plVar26 + 0x813) != *(int *)(puVar28 + 0x20)) {
      *(undefined4 *)(DAT_1401e8958 + 0xf0) = 2;
    }
    *(undefined4 *)(DAT_1401e8958 + 0xfc) = 1;
    *(undefined4 *)(DAT_1401e8958 + 0x100) = 1;
    *(undefined8 *)(DAT_1401e8958 + 0x104) = *(undefined8 *)(puVar28 + 0x10);
    *(undefined8 *)(DAT_1401e8958 + 0x10c) = *(undefined8 *)(puVar28 + 0x18);
    lVar21 = FUN_14010e440(local_348,0x5f);
    plVar31 = (longlong *)(**(code **)(lVar21 + 0x48))(&local_548,puVar28);
    *(undefined4 *)(*plVar31 + 0x1a0) = 1;
    bVar10 = bVar11;
  }
  plVar31 = DAT_1401e88b0;
  if (local_588 == (char *)0x0) {
    if (lVar17 != 0) {
      local_558 = param_1[9];
      uStack_550 = param_1[10];
      local_588 = (char *)0x1;
      uVar35 = param_1[7];
      uVar25 = param_1[8];
      uStack_580 = 0xffffffffffffffff;
      local_598 = &DAT_1401c5f10;
      puStack_590 = (undefined1 *)0x4;
      local_578 = uVar35;
      uStack_570 = uVar25;
      iVar14 = FUN_1401b87a0(&local_578,&local_598,&local_588);
      if (iVar14 != 0) {
        uVar35 = 0;
        uVar25 = 0;
      }
      if (uStack_550 != 0) {
        local_578 = param_1[9];
        uStack_570 = param_1[10];
        lVar17 = FUN_14010e440(local_348,0x34);
        plVar31 = (longlong *)(**(code **)(lVar17 + 0x40))(&local_548,&local_578);
        *(undefined4 *)(*plVar31 + 0x1a0) = 1;
      }
      local_578 = uVar35;
      uStack_570 = uVar25;
      plVar26 = (longlong *)FUN_140115170(plVar26[0x41],&local_578);
      if (plVar26 == (longlong *)0x0) {
        local_578 = uVar35;
        uStack_570 = uVar25;
        puVar18 = (ulonglong *)FUN_14005b060(&local_4f8,*(undefined8 *)(lVar20 + 0x90),&local_578);
      }
      else {
        local_4f8 = *(ulonglong *)*plVar26;
        uStack_4f0 = ((ulonglong *)*plVar26)[1];
        local_4e8 = CONCAT44(*(undefined4 *)((longlong)plVar26 + 0xc),
                             (int)*(short *)((longlong)plVar26 + 10));
        puVar18 = &local_4f8;
        uStack_4e0 = *(undefined8 *)plVar26[2];
        local_4d8 = ((undefined8 *)plVar26[2])[1];
      }
      uVar35 = puVar18[2];
      uVar3 = *(undefined4 *)((longlong)puVar18 + 0x14);
      local_498 = puVar18[2];
      local_558 = puVar18[3];
      uStack_490 = puVar18[3];
      local_578 = *puVar18;
      uStack_570 = puVar18[1];
      uStack_550 = puVar18[4];
      local_488 = uStack_550;
      lVar17 = FUN_14010e440(local_348,0x35);
      lVar17 = (**(code **)(lVar17 + 0x20))(&local_548,*(undefined8 *)(lVar20 + 0x78));
      lVar17 = (**(code **)(lVar17 + 0x40))(&local_458,&local_578);
      lVar17 = (**(code **)(lVar17 + 0x28))(&local_4a8,(int)uVar35);
      lVar17 = (**(code **)(lVar17 + 0x10))(&local_4f8,uVar3);
      plVar26 = (longlong *)(**(code **)(lVar17 + 0x40))(local_408,&local_558);
      *(undefined4 *)(*plVar26 + 0x1a0) = 1;
      return;
    }
  }
  else {
    lVar17 = DAT_1401e88b0[4];
    pvVar36 = (LPVOID)DAT_1401e88b0[5];
    uVar35 = (ulonglong)((int)lVar17 + (int)pvVar36 & 7);
    puVar28 = (undefined1 *)(8 - uVar35);
    if (uVar35 == 0) {
      puVar28 = puVar34;
    }
    puVar32 = puVar28 + 0x1a0;
    if ((undefined1 *)DAT_1401e88b0[3] < puVar32 + lVar17) {
      if ((undefined1 *)DAT_1401e88b0[2] < puVar32 + lVar17) {
        FUN_1401b5c50(DAT_1401e88b0,puVar32,8);
        pvVar36 = (LPVOID)plVar31[5];
        lVar17 = plVar31[4];
        uVar35 = (ulonglong)((int)lVar17 + (int)pvVar36 & 7);
        puVar28 = (undefined1 *)(8 - uVar35);
        if (uVar35 == 0) {
          puVar28 = puVar34;
        }
        puVar32 = puVar28 + 0x1a0;
      }
      lVar21 = *plVar31;
      puVar29 = puVar34;
      if ((lVar21 != 0) &&
         (uVar35 = lVar21 - 1U & (ulonglong)(puVar32 + lVar17), puVar29 = (undefined1 *)0x0,
         uVar35 != 0)) {
        puVar29 = (undefined1 *)(lVar21 - uVar35);
      }
      puVar29 = puVar29 + (longlong)(puVar32 + lVar17);
      if ((undefined1 *)plVar31[3] < puVar29) {
        plVar31[3] = (longlong)puVar29;
        VirtualAlloc(pvVar36,(SIZE_T)puVar29,0x1000,4);
      }
    }
    puVar32 = puVar28 + plVar31[4] + plVar31[5];
    FUN_1401bace0(puVar32,0,0x1a0);
    plVar31[6] = (longlong)puVar32;
    plVar31[4] = (longlong)(puVar28 + plVar31[4] + 0x1a0);
    local_578 = param_1[0xb];
    uStack_570 = param_1[0xc];
    iVar14 = FUN_1401349d0(plVar26,puVar32,&local_578);
    if (iVar14 != 0) {
      local_578 = *(ulonglong *)(puVar32 + 0x20);
      uStack_570 = *(ulonglong *)(puVar32 + 0x28);
      local_588 = (char *)0x1;
      uStack_580 = 0xffffffffffffffff;
      local_598 = &DAT_1401c5f10;
      puStack_590 = (undefined1 *)0x4;
      iVar14 = FUN_1401b87a0(&local_578,&local_598,&local_588);
      if (iVar14 != 0) {
        *(undefined8 *)(puVar32 + 0x20) = 0;
        *(undefined8 *)(puVar32 + 0x28) = 0;
      }
      lVar17 = FUN_14010e440(local_348,0x21);
      lVar17 = (**(code **)(lVar17 + 0x20))(&local_548,*(undefined8 *)(lVar20 + 0x78));
      plVar31 = (longlong *)(**(code **)(lVar17 + 0x48))(&local_458,puVar32);
      *(undefined4 *)(*plVar31 + 0x1a0) = 1;
      bVar10 = bVar11;
    }
  }
  if (bVar10) {
    local_598 = (undefined1 *)0x0;
    puStack_590 = (undefined1 *)0x0;
    lVar17 = *(longlong *)(DAT_1401e88b8 + 0x48);
    puVar18 = *(ulonglong **)(lVar17 + 0xae0);
    local_4e8 = *(undefined8 *)(lVar17 + 0xaf0);
    if (puVar18 != (ulonglong *)0x0) {
      local_578 = *(ulonglong *)(lVar17 + 0xac8);
      uStack_570 = *(undefined8 *)(lVar17 + 0xad0);
      local_588 = (char *)0x1;
      uStack_580 = *(ulonglong *)(lVar17 + 0xad0);
      do {
        local_558 = *puVar18;
        uStack_550 = puVar18[1];
        iVar14 = FUN_1401b87a0(&local_558,&local_578,&local_588);
        if (iVar14 != 0) {
          puStack_590 = (undefined1 *)CONCAT44(puStack_590._4_4_,1);
          local_598 = puVar34;
          break;
        }
        puVar18 = (ulonglong *)puVar18[3];
        puVar34 = puVar34 + 1;
      } while (puVar18 != (ulonglong *)0x0);
    }
    if (((int)local_598 == 0) || (uVar35 = 0, uVar25 = 0, (int)local_598 != 1)) {
      uVar35 = _DAT_1401e8bf8;
      uVar25 = uRam00000001401e8c00;
    }
    local_578 = uVar35;
    uStack_570 = uVar25;
    plVar26 = (longlong *)FUN_140115170(plVar26[0x41],&local_578);
    if (plVar26 == (longlong *)0x0) {
      local_578 = uVar35;
      uStack_570 = uVar25;
      puVar18 = (ulonglong *)FUN_14005b060(&local_4f8,*(undefined8 *)(lVar20 + 0x90),&local_578);
    }
    else {
      local_4f8 = *(ulonglong *)*plVar26;
      uStack_4f0 = ((ulonglong *)*plVar26)[1];
      local_4e8 = CONCAT44(*(undefined4 *)((longlong)plVar26 + 0xc),
                           (int)*(short *)((longlong)plVar26 + 10));
      puVar18 = &local_4f8;
      uStack_4e0 = *(undefined8 *)plVar26[2];
      local_4d8 = ((undefined8 *)plVar26[2])[1];
    }
    uVar35 = puVar18[2];
    uVar3 = *(undefined4 *)((longlong)puVar18 + 0x14);
    local_498 = puVar18[2];
    local_558 = puVar18[3];
    uStack_490 = puVar18[3];
    local_578 = *puVar18;
    uStack_570 = puVar18[1];
    uStack_550 = puVar18[4];
    local_488 = uStack_550;
    lVar17 = FUN_14010e440(local_408,0x35);
    lVar17 = (**(code **)(lVar17 + 0x20))(local_348,*(undefined8 *)(lVar20 + 0x78));
    lVar17 = (**(code **)(lVar17 + 0x40))(&local_548,&local_578);
    lVar17 = (**(code **)(lVar17 + 0x28))(&local_458,(int)uVar35);
    lVar17 = (**(code **)(lVar17 + 0x10))(&local_4a8,uVar3);
    plVar26 = (longlong *)(**(code **)(lVar17 + 0x40))(&local_4f8,&local_558);
    *(undefined4 *)(*plVar26 + 0x1a0) = 1;
  }
  return;
}

