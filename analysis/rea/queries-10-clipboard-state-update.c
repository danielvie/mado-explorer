
void FUN_1400666b0(undefined8 param_1)

{
  undefined8 *puVar1;
  longlong *plVar2;
  longlong lVar3;
  undefined4 uVar4;
  undefined4 uVar5;
  undefined8 uVar6;
  uint uVar7;
  undefined4 local_res8 [2];
  undefined4 local_res10 [2];
  char *local_res18;
  char *local_res20;
  undefined8 in_stack_fffffffffffff9b8;
  undefined4 uVar8;
  undefined4 local_638;
  int local_634;
  undefined1 local_630 [512];
  int local_430;
  undefined1 local_42c [512];
  int local_22c;
  undefined1 local_228 [520];
  
  uVar8 = (undefined4)((ulonglong)in_stack_fffffffffffff9b8 >> 0x20);
  glEnable(0xbe2);
  glEnable(0xc11);
  glBlendFunc(1,0x303);
  lVar3 = DAT_1401e8b58;
  *(undefined8 *)(DAT_1401e8b58 + 0x54) = 0;
  *(undefined8 *)(lVar3 + 0x5c) = 0;
  *(undefined8 *)(lVar3 + 100) = 0;
  *(undefined8 *)(lVar3 + 0x6c) = 0;
  *(undefined8 *)(lVar3 + 0x74) = 0;
  *(undefined8 *)(lVar3 + 0x7c) = 0;
  *(undefined8 *)(lVar3 + 0x84) = 0;
  *(undefined8 *)(lVar3 + 0x8c) = 0;
  FUN_1401bace0(lVar3 + 0x98,0,0x200);
  (*DAT_1401e89c0)(1,local_res8);
  (*DAT_1401e89e8)(local_res8[0]);
  (*DAT_1401e88a8)(1,local_res10);
  (*DAT_1401e89c8)(0x8892,local_res10[0]);
  local_res20 = 
  "#version 330 core\n\nuniform sampler2DArray textureSampler;\nuniform bool textureIsMono;\n\nin vec2 fragShapePos; // Pixel space location of the shape\nin vec2 fragShapeDim; // Pixel space scale of the shape\nsmooth in vec2 fragUV;\nsmooth in vec4 fragColor;\nsmooth in vec4 fragGradient;\nsmooth in vec4 fragCornerRadii; // Radius of the shape corners\nsmooth in vec4 fragShapeThicknessMask; // Exclude borders\nin float fragDepth; // Texture array index\nin float fragShapeThickness; // Shape outline\nin float fragShapeSoftness; // Shape edge softness\nin float fragDashLength;\nin float fragGapLength;\nin float fragDashCornerRad;\n\nout vec4 outColor;\n\nfloat RoundedBoxSDF(vec2 centerPos, vec2 dim, vec4 radii)\n{\n\tradii.xy = (centerPos.x > 0.0) ? radii.xy : radii.zw;\n\tradii.x  = (centerPos.y > 0.0) ? radii.x  : radii.y;\n\tvec2 q = abs(centerPos) - dim + radii.x;\n\treturn min(max(q.x, q.y), 0.0) + length(max(q, 0.0)) - radii.x;\n}\n\nbool IsDashedOutline(vec2 fragCoord, vec2 shapePos, vec2 shapeDim, float thickness, vec4 radii)\n{\n\tfloat totalLength = fragDashLength + fragGapLength;\n\tfloat dist = 0.0;\n\n\tif (fragCoord.x < shapePos.x + thickness || fragCoord.x > shapePos.x + shapeDim.x - thickness)\n\t{\n\t\tdist = mod(fragCoord.y - shapePos.y, totalLength);\n\t}\n\telse if (fragCoord.y < shapePos.y + thickness || fragCoord.y > shapePos.y + shapeDim.y - thickness)\n\t{\n\t\tdist = mod(fragCoord.x - shapePos.x, totalLength);\n\t}\n\n\tvec2 centerPos = fragCoord - shapePos - shapeDim / 2.0;\n\tfloat distance = RoundedBoxSDF(centerPos, shapeDim / 2.0, radii);\n\n\treturn dist < fragDashLength || distance > 0.0;\n}\n\nvoid main()\n{\n\tvec4 sampled = texture(textureSampler, vec3(fragUV, fragDepth));\n\tvec4 tintColor = fragColor;\n\n\tvec4 mono = vec4(sampled.r);\n\tvec4 pixel = tintColor * mix(sampled, mono, vec4(textureIsMono));\n\n\tif (fragDashLength > 0)\n\t{\n\t\tvec4 dashCornerRadii = vec4(fragDashCornerRad);\n\t\tbool isDashed = IsDashedOutline(gl_FragCoord.xy, fragShapePos, fragShapeDim, fragShapeThickness, dashCornerRadii);\n\t\tif (!isDashed) discard;\n\t}\n\n\t// We don\'t support rotation wit..." /* TRUNCATED STRING LITERAL */
  ;
  local_res18 = 
  "#version 330 core\n\nuniform mat4 proj;\n\nlayout(location = 0) in vec2 p0;\nlayout(location = 1) in vec2 p1;\nlayout(location = 2) in vec2 p2;\nlayout(location = 3) in vec2 p3;\nlayout(location = 4) in vec2 vertMinUV;\nlayout(location = 5) in vec2 vertMaxUV;\nlayout(location = 6) in vec4 vertColor;\nlayout(location = 7) in vec4 vertGradient;\nlayout(location = 8) in vec4 vertCornerRadii;\nlayout(location = 9) in vec4 vertShapeThicknessMask;\nlayout(location = 10) in float vertDepth;\nlayout(location = 11) in float vertShapeThickness;\nlayout(location = 12) in float vertShapeSoftness;\nlayout(location = 13) in float vertDashLength;\nlayout(location = 14) in float vertGapLength;\nlayout(location = 15) in float vertDashCornerRad;\n\nout vec2 fragShapePos;\nout vec2 fragShapeDim;\nsmooth out vec2 fragUV;\nsmooth out vec4 fragColor;\nsmooth out vec4 fragGradient;\nsmooth out vec4 fragCornerRadii;\nsmooth out vec4 fragShapeThicknessMask;\nout float fragDepth;\nout float fragShapeThickness;\nout float fragShapeSoftness;\nout float fragDashLength;\nout float fragGapLength;\nout float fragDashCornerRad;\n\nvoid main()\n{\n\tvec2 positions[4] = vec2[](p0, p1, p2, p3);\n\tvec2 uvs[4] = vec2[](vec2(vertMinUV.x, vertMaxUV.y), vertMaxUV, vertMinUV, vec2(vertMaxUV.x, vertMinUV.y));\n\n\tvec2 vertPos = positions[gl_VertexID];\n\tvec2 vertUV = uvs[gl_VertexID];\n\n\tgl_Position = proj * vec4(vertPos, 0.0, 1.0);\n\n\tfragShapePos = p0;\n\tfragShapeDim = vec2(abs(p3.x - p0.x), abs(p3.y - p0.y));\n\tfragUV = vertUV;\n\tfragColor = vertColor;\n\tfragGradient = vertGradient;\n\tfragCornerRadii = vertCornerRadii;\n\tfragShapeThicknessMask = vertShapeThicknessMask;\n\tfragDepth = vertDepth;\n\tfragShapeThickness = vertShapeThickness;\n\tfragShapeSoftness = vertShapeSoftness;\n\tfragDashLength = vertDashLength;\n\tfragGapLength = vertGapLength;\n\tfragDashCornerRad = vertDashCornerRad;\n}\n"
  ;
  FUN_1401bace0(&local_638,0,0x610);
  uVar4 = (*DAT_1401e8a30)(0x8b31);
  (*DAT_1401e8a38)(uVar4,1,&local_res18);
  (*DAT_1401e8a40)(uVar4);
  (*DAT_1401e8a10)(uVar4,0x8b81,&local_634);
  if (local_634 == 0) {
    (*DAT_1401e8a18)(uVar4,0x200,0,local_630);
  }
  uVar5 = (*DAT_1401e8a30)(0x8b30);
  (*DAT_1401e8a38)(uVar5,1,&local_res20);
  (*DAT_1401e8a40)(uVar5);
  (*DAT_1401e8a10)(uVar5,0x8b81,&local_430);
  if (local_430 == 0) {
    (*DAT_1401e8a18)(uVar5,0x200,0,local_42c);
  }
  local_638 = (*DAT_1401e8a48)();
  (*DAT_1401e8a50)(local_638,uVar4);
  (*DAT_1401e8a50)(local_638,uVar5);
  (*DAT_1401e8a58)(local_638);
  (*DAT_1401e8a20)(local_638,0x8b82,&local_22c);
  if (local_22c == 0) {
    (*DAT_1401e8a28)(local_638,0x200,0,local_228);
  }
  (*DAT_1401e8a60)(uVar4);
  (*DAT_1401e8a60)(uVar5);
  uVar4 = local_638;
  uVar5 = (*DAT_1401e8a98)(local_638,&DAT_1401d37e0);
  *(undefined4 *)(DAT_1401e8b58 + 0x44) = uVar5;
  uVar5 = (*DAT_1401e8a98)(uVar4,"textureIsMono");
  uVar7 = 0;
  *(undefined4 *)(DAT_1401e8b58 + 0x48) = uVar5;
  (*DAT_1401e8a00)(0,2,0x1404,0,CONCAT44(uVar8,0x48),0);
  (*DAT_1401e8a00)(1,2,0x1404,0,0x48,8);
  (*DAT_1401e8a00)(2,2,0x1404,0,0x48,0x10);
  (*DAT_1401e8a00)(3,2,0x1404,0,0x48,0x18);
  (*DAT_1401e8a00)(4,2,0x1406,0,0x48,0x20);
  (*DAT_1401e8a00)(5,2,0x1406,0,0x48,0x28);
  (*DAT_1401e8a00)(6,4,0x1401,1,0x48,0x30);
  (*DAT_1401e8a00)(7,4,0x1401,1,0x48,0x38);
  (*DAT_1401e8a00)(8,4,0x1401,0,0x48,0x34);
  (*DAT_1401e8a00)(9,4,0x1401,0,0x48,0x3c);
  (*DAT_1401e8a00)(10,1,0x1402,0,0x48,0x40);
  (*DAT_1401e8a00)(0xb,1,0x1401,0,0x48,0x42);
  (*DAT_1401e8a00)(0xc,1,0x1401,0,0x48,0x43);
  (*DAT_1401e8a00)(0xd,1,0x1401,0,0x48,0x44);
  (*DAT_1401e8a00)(0xe,1,0x1401,0,0x48,0x45);
  (*DAT_1401e8a00)(0xf,1,0x1401,0,0x48,0x46);
  do {
    (*DAT_1401e89f0)(uVar7);
    (*DAT_1401e8a08)(uVar7,1);
    uVar7 = uVar7 + 1;
  } while (uVar7 < 0x10);
  (*DAT_1401e8a68)(uVar4);
  uVar4 = (*DAT_1401e8a98)(uVar4,"textureSampler");
  (*DAT_1401e8a80)(uVar4,0);
  lVar3 = DAT_1401e8b58;
  puVar1 = (undefined8 *)(DAT_1401e8b58 + 0x2d8);
  plVar2 = (longlong *)(DAT_1401e8b58 + 0x2a0);
  *(undefined8 *)(DAT_1401e8b58 + 8) = 0x80000000;
  uVar6 = FUN_1401b6980(lVar3,plVar2,*puVar1,0x10,*plVar2 + 0x400);
  lVar3 = DAT_1401e8b58;
  *(undefined8 *)(DAT_1401e8b58 + 0x2d8) = uVar6;
  *(undefined8 *)(lVar3 + 0x2e0) = param_1;
  return;
}

