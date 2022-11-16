﻿namespace AdventOfCode2021.Days
{
    internal class Day5
    {
        public static void Execute()
        {
            var input = Input;
            var lines = input.Split(Environment.NewLine)
                .Select(x =>
                {
                    var line = x.Split(" -> ");
                    var from = line[0].Split(",").Select(int.Parse).ToArray();
                    var to = line[1].Split(",").Select(int.Parse).ToArray();
                    var fromCoords = new Coordinates(from[0], from[1]);
                    var toCoords = new Coordinates(to[0], to[1]);
                    return new CoordinatesLine(fromCoords, toCoords);
                });

            // Consider only horizontal and vertical lines; How many points overlap?
            Console.WriteLine("Part 1 - " + Solve(lines));

            // Consider diagonal lines too
            Console.WriteLine("Part 2 - " + Solve(lines, false));
        }

        private static int Solve(IEnumerable<CoordinatesLine> lines, bool skipDiagonalLines = true)
        {
            var (dimX, dimY) = GetDimensions(lines);
            var board = new int[dimX + 1, dimY + 1];

            foreach (var line in lines)
            {
                if (skipDiagonalLines && (!line.IsVertical && !line.IsHorizontal))
                    continue;

                foreach (var coord in line.GetCoordinatesOnLine())
                {
                    board[coord.X, coord.Y]++;
                }
            }

            var count = 0;
            for (var row = 0; row < dimX + 1; ++row)
            {
                for (var col = 0; col < dimY + 1; ++col)
                {
                    if (board[row, col] >= 2)
                        count++;
                }
            }
            return count;
        }

        private static (int, int) GetDimensions(IEnumerable<CoordinatesLine> lines)
        {
            var dimX = 0;
            var dimY = 0;
            foreach (var line in lines)
            {
                if (line.From.X > dimX) dimX = line.From.X;
                if (line.To.X > dimX) dimX = line.To.X;
                if (line.From.Y > dimY) dimY = line.From.Y;
                if (line.To.Y > dimY) dimY = line.To.Y;
            }
            return (dimX, dimY);
        }

        private sealed record Coordinates(int X, int Y)
        {
            public override string ToString() => $"({X}, {Y})";
        }

        private sealed class CoordinatesLine
        {
            public Coordinates From { get; set; }
            public Coordinates To { get; set; }
            public CoordinatesLine(Coordinates from, Coordinates to)
            {
                From = from;
                To = to;
            }

            public IEnumerable<Coordinates> GetCoordinatesOnLine()
            {
                if (IsVertical)
                {
                    var min = Math.Min(From.Y, To.Y);
                    var max = Math.Max(From.Y, To.Y);
                    for (var y = min; y <= max; ++y)
                        yield return new Coordinates(From.X, y);
                }
                else if (IsHorizontal)
                {
                    var min = Math.Min(From.X, To.X);
                    var max = Math.Max(From.X, To.X);
                    for (var x = min; x <= max; ++x)
                        yield return new Coordinates(x, From.Y);
                }
                else
                {
                    var crtCoord = From;
                    while (crtCoord != To)
                    {
                        yield return crtCoord;

                        if (crtCoord.X < To.X) crtCoord = crtCoord with { X = crtCoord.X + 1 };
                        else if (crtCoord.X > To.X) crtCoord = crtCoord with { X = crtCoord.X - 1 };

                        if (crtCoord.Y < To.Y) crtCoord = crtCoord with { Y = crtCoord.Y + 1 };
                        else if (crtCoord.Y > To.Y) crtCoord = crtCoord with { Y = crtCoord.Y - 1 };
                    }
                    yield return To; // don't forget the last point on the line
                }
            }

            public bool IsVertical => From.X == To.X;
            public bool IsHorizontal => From.Y == To.Y;

            public override string ToString() => $"{From} -> {To}";
        }

        #region Input

        private const string TestInput = @"0,9 -> 5,9
8,0 -> 0,8
9,4 -> 3,4
2,2 -> 2,1
7,0 -> 7,4
6,4 -> 2,0
0,9 -> 2,9
3,4 -> 1,4
0,0 -> 8,8
5,5 -> 8,2";

        private const string Input = @"694,732 -> 290,328
872,938 -> 167,233
770,318 -> 770,437
974,980 -> 28,34
25,739 -> 431,333
132,311 -> 132,89
926,479 -> 926,37
239,395 -> 239,722
286,538 -> 713,538
216,945 -> 570,945
975,858 -> 854,858
846,437 -> 313,437
90,318 -> 90,151
748,429 -> 976,429
750,500 -> 750,951
935,922 -> 38,25
543,914 -> 359,914
339,820 -> 339,78
978,358 -> 978,245
975,976 -> 22,23
767,159 -> 180,159
583,956 -> 583,48
208,267 -> 208,765
848,906 -> 848,895
325,374 -> 587,374
257,936 -> 257,777
276,179 -> 579,179
179,134 -> 390,134
381,286 -> 381,190
931,220 -> 931,540
957,774 -> 305,774
610,682 -> 965,682
640,672 -> 344,376
241,795 -> 241,908
638,641 -> 646,633
919,860 -> 114,55
833,163 -> 18,978
73,51 -> 987,965
702,909 -> 750,909
579,473 -> 579,582
455,475 -> 455,498
77,100 -> 956,979
376,424 -> 376,361
923,802 -> 923,89
343,451 -> 648,756
554,57 -> 614,57
977,949 -> 977,115
249,109 -> 249,143
930,66 -> 933,69
654,948 -> 810,948
544,424 -> 296,424
716,52 -> 10,758
960,557 -> 899,557
379,293 -> 417,331
620,287 -> 508,399
518,961 -> 518,948
961,433 -> 702,433
735,166 -> 772,166
587,31 -> 715,31
705,55 -> 680,55
122,656 -> 578,656
576,25 -> 576,721
434,791 -> 434,177
588,979 -> 588,133
54,545 -> 54,236
949,14 -> 95,14
17,533 -> 17,316
218,443 -> 973,443
281,443 -> 281,797
237,847 -> 237,400
543,776 -> 309,542
600,267 -> 600,452
856,93 -> 856,95
724,43 -> 724,216
616,793 -> 616,488
549,794 -> 549,486
830,858 -> 39,67
623,545 -> 477,399
695,42 -> 71,666
709,951 -> 709,310
755,354 -> 755,341
162,86 -> 162,868
563,809 -> 987,809
573,838 -> 573,141
204,85 -> 204,453
96,304 -> 574,782
702,693 -> 681,672
917,628 -> 917,654
503,228 -> 118,228
911,202 -> 983,202
195,373 -> 131,373
978,905 -> 426,905
527,585 -> 527,561
241,520 -> 241,812
301,501 -> 576,226
982,169 -> 220,169
839,105 -> 768,105
898,387 -> 399,886
241,805 -> 296,805
555,526 -> 314,526
723,235 -> 56,235
565,900 -> 119,454
100,255 -> 755,255
985,982 -> 66,63
968,50 -> 255,50
676,39 -> 676,513
181,306 -> 438,49
958,333 -> 280,333
732,309 -> 845,309
604,457 -> 644,417
736,61 -> 515,61
553,60 -> 445,168
767,475 -> 771,471
813,110 -> 39,884
82,177 -> 457,177
266,64 -> 807,64
135,37 -> 924,826
449,595 -> 167,877
13,32 -> 964,983
156,932 -> 765,932
738,588 -> 738,522
130,26 -> 130,464
681,485 -> 986,180
463,93 -> 349,93
364,324 -> 364,376
858,228 -> 858,749
271,536 -> 224,583
80,953 -> 940,93
977,293 -> 977,985
417,256 -> 690,256
973,874 -> 150,51
562,34 -> 116,34
10,837 -> 10,43
500,500 -> 983,17
480,444 -> 807,444
150,842 -> 419,842
28,866 -> 684,210
791,750 -> 772,769
673,13 -> 673,777
842,693 -> 332,693
934,985 -> 15,66
102,165 -> 796,165
386,768 -> 386,863
807,174 -> 459,174
388,397 -> 388,120
313,360 -> 590,83
546,364 -> 28,364
36,50 -> 747,761
587,597 -> 526,597
133,16 -> 836,16
867,375 -> 725,233
875,296 -> 875,520
77,60 -> 614,60
702,654 -> 395,961
50,963 -> 982,31
30,110 -> 841,921
277,401 -> 397,281
125,962 -> 977,110
592,895 -> 837,895
904,159 -> 578,485
146,158 -> 146,253
901,201 -> 901,792
707,847 -> 707,936
354,709 -> 928,709
415,338 -> 610,143
686,669 -> 686,92
915,143 -> 609,449
938,862 -> 976,862
52,763 -> 342,763
141,840 -> 141,826
360,223 -> 834,697
821,989 -> 359,989
925,854 -> 925,249
287,48 -> 287,453
543,986 -> 369,986
576,886 -> 522,886
819,441 -> 819,629
432,661 -> 432,871
366,841 -> 356,841
786,104 -> 786,549
576,15 -> 42,549
757,135 -> 231,661
569,210 -> 197,582
378,32 -> 378,988
829,822 -> 195,188
461,457 -> 339,335
608,742 -> 608,288
582,650 -> 182,650
713,554 -> 713,581
702,919 -> 702,829
956,960 -> 406,410
22,461 -> 22,24
333,572 -> 333,225
374,582 -> 374,463
294,736 -> 265,736
29,462 -> 29,271
858,80 -> 192,746
419,186 -> 419,625
518,320 -> 244,46
523,971 -> 745,971
83,78 -> 657,78
583,868 -> 583,58
287,830 -> 709,830
162,691 -> 495,358
704,28 -> 704,846
350,278 -> 912,278
496,15 -> 454,15
139,967 -> 715,391
792,440 -> 947,440
77,773 -> 31,727
734,413 -> 835,514
124,342 -> 124,785
747,397 -> 988,638
542,158 -> 675,158
169,969 -> 910,228
856,865 -> 856,193
960,63 -> 58,965
14,13 -> 969,968
26,754 -> 750,30
402,297 -> 131,26
182,567 -> 446,567
944,891 -> 428,375
587,900 -> 474,900
274,533 -> 317,490
529,43 -> 606,120
644,584 -> 644,558
706,824 -> 348,466
700,416 -> 401,416
986,829 -> 217,60
241,383 -> 863,383
877,833 -> 866,833
376,423 -> 856,903
234,926 -> 185,975
747,441 -> 747,988
99,739 -> 99,49
763,554 -> 763,693
168,78 -> 168,594
81,922 -> 946,57
685,344 -> 42,344
39,656 -> 283,412
502,709 -> 158,365
143,423 -> 924,423
876,846 -> 107,77
356,120 -> 356,785
485,91 -> 404,91
976,403 -> 976,575
371,390 -> 371,851
380,206 -> 380,228
232,268 -> 232,338
150,48 -> 246,144
977,970 -> 25,18
50,205 -> 691,205
41,898 -> 41,97
181,512 -> 46,512
675,15 -> 837,177
687,317 -> 687,362
191,922 -> 745,922
209,136 -> 561,136
708,919 -> 133,344
54,148 -> 769,148
68,199 -> 210,199
959,282 -> 959,334
842,362 -> 641,362
392,712 -> 572,712
798,874 -> 74,150
107,482 -> 470,119
71,24 -> 815,768
216,35 -> 922,741
185,497 -> 160,522
585,246 -> 676,155
762,657 -> 762,871
85,441 -> 85,156
248,276 -> 346,178
291,917 -> 747,917
54,958 -> 989,23
864,591 -> 443,170
544,667 -> 268,667
819,184 -> 200,803
418,191 -> 493,116
553,741 -> 462,741
266,155 -> 707,596
571,308 -> 895,308
77,611 -> 641,47
210,595 -> 210,356
718,711 -> 79,72
816,310 -> 142,984
936,974 -> 141,179
287,888 -> 927,888
859,70 -> 859,128
34,604 -> 34,961
916,446 -> 183,446
471,615 -> 231,855
88,514 -> 88,616
183,301 -> 313,301
982,41 -> 185,838
62,607 -> 13,607
853,688 -> 399,234
487,152 -> 487,682
282,50 -> 312,50
448,285 -> 789,626
362,875 -> 362,293
795,940 -> 795,451
743,871 -> 743,19
493,297 -> 493,607
988,615 -> 364,615
657,214 -> 657,500
784,627 -> 805,627
580,269 -> 870,269
966,942 -> 655,942
42,871 -> 880,33
599,385 -> 952,32
31,662 -> 735,662
703,733 -> 703,791
771,866 -> 771,46
902,360 -> 902,258
632,806 -> 887,806
835,307 -> 819,307
166,875 -> 166,12
564,984 -> 564,54
90,411 -> 90,403
86,247 -> 86,329
104,781 -> 464,781
77,663 -> 264,850
223,634 -> 755,634
529,908 -> 529,793
443,22 -> 443,144
159,704 -> 40,704
102,811 -> 102,186
803,266 -> 803,937
569,586 -> 569,954
987,988 -> 12,13
264,288 -> 283,288
927,139 -> 817,139
523,206 -> 523,166
76,333 -> 651,908
40,910 -> 687,263
760,977 -> 117,334
596,344 -> 596,546
852,859 -> 753,760
581,266 -> 802,266
985,603 -> 985,246
631,381 -> 814,381
341,884 -> 341,308
555,255 -> 891,255
82,50 -> 82,561
50,110 -> 344,404
651,518 -> 651,879
86,904 -> 86,157
223,258 -> 908,943
593,507 -> 454,507
941,65 -> 74,932
758,33 -> 758,811
671,816 -> 202,347
857,905 -> 212,260
83,11 -> 83,167
346,531 -> 724,153
245,113 -> 772,640
694,149 -> 694,308
458,482 -> 177,201
305,248 -> 408,351
910,875 -> 56,21
410,303 -> 798,303
352,959 -> 352,224
413,210 -> 875,672
942,47 -> 942,588
839,872 -> 58,91
970,908 -> 140,78
748,256 -> 441,256
478,683 -> 609,683
807,930 -> 950,930
730,804 -> 730,817
960,826 -> 807,979
130,602 -> 142,590
968,860 -> 968,572
240,465 -> 633,72
718,428 -> 718,606
350,904 -> 350,743
198,30 -> 970,802
173,345 -> 173,693
873,254 -> 873,714
244,726 -> 244,830
620,399 -> 147,399
318,929 -> 318,83
837,800 -> 843,800
418,486 -> 865,486
916,53 -> 916,207
127,582 -> 127,952
888,162 -> 941,162
660,965 -> 110,415
981,187 -> 383,785
940,809 -> 940,889
342,479 -> 271,550
646,51 -> 646,340
260,947 -> 906,947
961,928 -> 638,928
988,15 -> 18,985
393,437 -> 460,437
888,91 -> 888,83
875,707 -> 860,722
387,187 -> 895,695
857,561 -> 928,561
15,52 -> 398,52
363,821 -> 324,782
761,535 -> 290,64
878,539 -> 878,317
577,890 -> 51,890
379,471 -> 379,627
65,91 -> 535,91
79,77 -> 897,895
863,324 -> 885,324
970,536 -> 970,879
924,743 -> 765,743
317,524 -> 89,524
585,568 -> 585,229
29,338 -> 179,338
937,925 -> 937,239
478,905 -> 542,905
191,147 -> 191,956
759,101 -> 59,801
131,315 -> 131,121
132,121 -> 805,794
97,430 -> 541,430
887,426 -> 516,426
311,968 -> 261,968
503,628 -> 665,628
277,670 -> 705,670
747,671 -> 836,671
892,55 -> 14,933
973,711 -> 835,573
18,932 -> 300,932
562,923 -> 562,850
416,982 -> 416,742
13,918 -> 870,61
252,313 -> 252,702
340,563 -> 340,636
858,88 -> 858,646
806,963 -> 54,963
674,703 -> 674,760
826,782 -> 826,362
655,558 -> 655,213
716,929 -> 23,929
701,904 -> 701,234
496,17 -> 896,17
577,139 -> 577,117
613,336 -> 925,24
804,726 -> 804,551
301,134 -> 616,134
840,695 -> 301,156
148,280 -> 209,280
531,353 -> 531,666
899,726 -> 899,162
817,23 -> 47,793
673,921 -> 673,15
543,863 -> 543,795
947,464 -> 947,164
54,42 -> 840,828
24,54 -> 24,381
958,293 -> 958,748
674,800 -> 674,909
895,24 -> 343,24
659,513 -> 182,36
754,268 -> 260,762
619,544 -> 619,212
961,607 -> 961,558
390,450 -> 390,796
486,560 -> 486,801
653,44 -> 225,44
662,604 -> 951,315
929,537 -> 429,37
44,958 -> 715,287
205,970 -> 352,970
190,867 -> 914,143
276,789 -> 602,463
239,534 -> 239,457
715,841 -> 695,841
411,864 -> 418,864
612,624 -> 652,624
382,305 -> 382,890
55,59 -> 748,752
984,16 -> 46,954
530,625 -> 530,776
944,67 -> 395,616
561,744 -> 561,79
638,628 -> 502,628
474,556 -> 474,953
786,227 -> 733,227
13,15 -> 977,979
900,137 -> 98,939
604,240 -> 228,616
152,32 -> 474,354
500,982 -> 442,982
989,345 -> 445,345
819,279 -> 819,68
603,266 -> 795,266
486,311 -> 783,311
434,58 -> 568,58
195,75 -> 195,806
39,78 -> 233,78
648,503 -> 648,847
306,867 -> 773,867";

        #endregion
    }
}
