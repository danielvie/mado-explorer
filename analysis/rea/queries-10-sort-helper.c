
void FUN_140165a20(longlong param_1,longlong param_2,longlong param_3,code *param_4)

{
  undefined1 uVar1;
  int iVar2;
  longlong lVar3;
  longlong lVar4;
  undefined1 *puVar5;
  undefined1 *puVar6;
  longlong lVar7;
  longlong lVar8;
  longlong lVar9;
  undefined1 *puVar10;
  undefined1 *puVar11;
  longlong *local_res10;
  longlong local_438 [2];
  longlong local_428 [126];
  
  if (1 < param_2) {
    param_2 = param_2 + -1;
    local_res10 = local_428;
    lVar4 = 0;
    do {
      if (lVar4 + 10 < param_2) {
        lVar8 = (lVar4 + param_2) / 2;
        puVar11 = (undefined1 *)(lVar8 * param_3 + param_1);
        puVar10 = (undefined1 *)(lVar4 * param_3 + param_1);
        iVar2 = (*param_4)(puVar11,puVar10);
        puVar6 = puVar11;
        puVar5 = puVar10;
        lVar9 = param_3;
        if (iVar2 < 0) {
          do {
            uVar1 = *puVar6;
            lVar9 = lVar9 + -1;
            *puVar6 = *puVar5;
            *puVar5 = uVar1;
            puVar6 = puVar6 + 1;
            puVar5 = puVar5 + 1;
          } while (0 < lVar9);
        }
        puVar6 = (undefined1 *)(param_2 * param_3 + param_1);
        iVar2 = (*param_4)(puVar6,puVar11);
        puVar5 = puVar11;
        lVar9 = param_3;
        if (iVar2 < 0) {
          do {
            uVar1 = *puVar6;
            lVar9 = lVar9 + -1;
            *puVar6 = *puVar5;
            puVar6 = puVar6 + 1;
            *puVar5 = uVar1;
            puVar5 = puVar5 + 1;
          } while (0 < lVar9);
          iVar2 = (*param_4)(puVar11,puVar10);
          puVar6 = puVar10;
          lVar9 = param_3;
          if (iVar2 < 0) {
            do {
              uVar1 = *puVar11;
              lVar9 = lVar9 + -1;
              *puVar11 = *puVar6;
              puVar11 = puVar11 + 1;
              *puVar6 = uVar1;
              puVar6 = puVar6 + 1;
            } while (0 < lVar9);
          }
        }
        lVar9 = param_2 + 1;
        lVar7 = lVar4;
        while( true ) {
          lVar7 = lVar7 + 1;
          lVar3 = lVar7 * param_3 + param_1;
          iVar2 = (*param_4)(lVar3,puVar10);
          for (; (iVar2 < 0 && (lVar7 != param_2)); lVar7 = lVar7 + 1) {
            lVar3 = lVar3 + param_3;
            iVar2 = (*param_4)(lVar3,puVar10);
          }
          lVar9 = lVar9 + -1;
          lVar3 = lVar9 * param_3 + param_1;
          iVar2 = (*param_4)(puVar10,lVar3);
          for (; (iVar2 < 0 && (lVar9 != lVar4)); lVar9 = lVar9 + -1) {
            lVar3 = lVar3 - param_3;
            iVar2 = (*param_4)(puVar10,lVar3);
          }
          puVar11 = (undefined1 *)(lVar9 * param_3 + param_1);
          lVar3 = param_3;
          if (lVar9 <= lVar7) break;
          puVar6 = (undefined1 *)(lVar7 * param_3 + param_1);
          do {
            uVar1 = *puVar6;
            lVar3 = lVar3 + -1;
            *puVar6 = *puVar11;
            puVar6 = puVar6 + 1;
            *puVar11 = uVar1;
            puVar11 = puVar11 + 1;
          } while (0 < lVar3);
        }
        do {
          uVar1 = *puVar10;
          lVar3 = lVar3 + -1;
          *puVar10 = *puVar11;
          puVar10 = puVar10 + 1;
          *puVar11 = uVar1;
          puVar11 = puVar11 + 1;
        } while (0 < lVar3);
        if (lVar8 < lVar9) {
          *local_res10 = lVar4;
          local_res10[1] = lVar9 + -1;
          local_res10[2] = lVar9 + 1;
          local_res10[3] = param_2;
          local_res10 = local_res10 + 4;
        }
        else {
          *local_res10 = lVar9 + 1;
          local_res10[1] = param_2;
          local_res10[2] = lVar4;
          local_res10[3] = lVar9 + -1;
          local_res10 = local_res10 + 4;
        }
      }
      else {
        lVar4 = lVar4 + 1;
        if (lVar4 <= param_2) {
          puVar11 = (undefined1 *)(lVar4 * param_3 + param_1);
          puVar10 = puVar11;
          lVar9 = lVar4;
joined_r0x000140165a96:
          do {
            if (0 < lVar4) {
              puVar5 = puVar11 + -param_3;
              iVar2 = (*param_4)(puVar11,puVar5);
              puVar6 = puVar5;
              lVar8 = param_3;
              if (iVar2 < 0) {
                do {
                  uVar1 = *puVar11;
                  lVar8 = lVar8 + -1;
                  *puVar11 = *puVar6;
                  puVar11 = puVar11 + 1;
                  *puVar6 = uVar1;
                  puVar6 = puVar6 + 1;
                } while (0 < lVar8);
                lVar4 = lVar4 + -1;
                puVar11 = puVar5;
                goto joined_r0x000140165a96;
              }
            }
            lVar4 = lVar9 + 1;
            puVar11 = puVar10 + param_3;
            puVar10 = puVar11;
            lVar9 = lVar4;
          } while (lVar4 <= param_2);
        }
      }
      lVar4 = local_res10[-2];
      param_2 = local_res10[-1];
      local_res10 = local_res10 + -2;
    } while (local_438 < local_res10);
  }
  return;
}

