pub const BENCHMARK: &str = "SR18main";

pub const SCB: [(usize, &str); 400] = [
    (1, "10-3-13.cnf"),
    (2, "10-4-10.cnf"),
    (3, "1mod8.rules.7-7.cnf"),
    (4, "20180321_110704973_p_cnf_320_1120.cnf"),
    (5, "20180321_110706599_p_cnf_320_1120.cnf"),
    (6, "20180321_110707239_p_cnf_320_1120.cnf"),
    (7, "20180321_140823961_p_cnf_320_1120.cnf"),
    (8, "20180321_140824282_p_cnf_320_1120.cnf"),
    (9, "20180321_140826713_p_cnf_320_1120.cnf"),
    (10, "20180321_140827428_p_cnf_320_1120.cnf"),
    (11, "20180321_140833987_p_cnf_320_1120.cnf"),
    (12, "20180322_164223076_p_cnf_320_1120.cnf"),
    (13, "20180322_164224543_p_cnf_320_1120.cnf"),
    (14, "20180322_164226378_p_cnf_320_1120.cnf"),
    (15, "20180322_164226661_p_cnf_320_1120.cnf"),
    (16, "20180322_164238439_p_cnf_320_1120.cnf"),
    (17, "20180322_164241329_p_cnf_320_1120.cnf"),
    (18, "20180322_164241842_p_cnf_320_1120.cnf"),
    (19, "20180322_164245263_p_cnf_320_1120.cnf"),
    (20, "20180326_095801070_p_cnf_320_1120.cnf"),
    (21, "20180326_095804286_p_cnf_320_1120.cnf"),
    (22, "20180326_095804936_p_cnf_320_1120.cnf"),
    (23, "20180326_095805836_p_cnf_320_1120.cnf"),
    (24, "5or7mod8.rules.7-7.cnf"),
    (25, "6-3-8.cnf"),
    (26, "6-4-6.cnf"),
    (27, "6-4-7.cnf"),
    (28, "6-5-6.cnf"),
    (29, "7-4-7.cnf"),
    (30, "7-5-6.cnf"),
    (31, "8-4-10.cnf"),
    (32, "8-4-8.cnf"),
    (33, "8-4-9.cnf"),
    (34, "8-5-6.cnf"),
    (35, "9-4-10.cnf"),
    (36, "9-4-9.cnf"),
    (37, "C3-2-31.cnf"),
    (38, "CNP-5-0.cnf"),
    (39, "CNP-5-100.cnf"),
    (40, "CNP-5-1000.cnf"),
    (41, "CNP-5-1100.cnf"),
    (42, "CNP-5-1200.cnf"),
    (43, "CNP-5-1300.cnf"),
    (44, "CNP-5-1400.cnf"),
    (45, "CNP-5-1500.cnf"),
    (46, "CNP-5-1600.cnf"),
    (47, "CNP-5-1700.cnf"),
    (48, "CNP-5-1800.cnf"),
    (49, "CNP-5-1900.cnf"),
    (50, "CNP-5-200.cnf"),
    (51, "CNP-5-300.cnf"),
    (52, "CNP-5-400.cnf"),
    (53, "CNP-5-500.cnf"),
    (54, "CNP-5-600.cnf"),
    (55, "CNP-5-700.cnf"),
    (56, "CNP-5-800.cnf"),
    (57, "CNP-5-900.cnf"),
    (58, "Cake_8_16.cnf"),
    (59, "Cake_9_19.cnf"),
    (60, "Cake_9_20.cnf"),
    (61, "Karatsuba4477457x5308417.cnf"),
    (62, "Karatsuba6972593x13466917.cnf"),
    (63, "Karatsuba7654321x1234567.cnf"),
    (64, "Nb11T118.cnf"),
    (65, "Nb13T165.cnf"),
    (66, "Nb13T166.cnf"),
    (67, "Nb14T194.cnf"),
    (68, "Nb27T6.cnf"),
    (69, "Nb29T6.cnf"),
    (70, "Nb37T6.cnf"),
    (71, "Nb39T6.cnf"),
    (72, "Nb42T6.cnf"),
    (73, "Nb44T6.cnf"),
    (74, "Nb45T6.cnf"),
    (75, "Nb49T6.cnf"),
    (76, "Nb51T6.cnf"),
    (77, "Nb52T6.cnf"),
    (78, "Nb54T6.cnf"),
    (79, "Nb8T60.cnf"),
    (80, "Nb8T61.cnf"),
    (81, "Nb8T62.cnf"),
    (82, "Nb8T63.cnf"),
    (83, "Problem11_label29_false-unreach-call.c.cnf"),
    (84, "Problem11_label51_false-unreach-call.c.cnf"),
    (85, "Problem14_label14_false-unreach-call.c.cnf"),
    (86, "Problem14_label19_true-unreach-call.c.cnf"),
    (87, "Problem14_label20_true-unreach-call.c.cnf"),
    (88, "Problem14_label48_true-unreach-call.c.cnf"),
    (89, "Problem14_label55_true-unreach-call.c.cnf"),
    (90, "Problem14_label57_false-unreach-call.c.cnf"),
    (91, "T103.2.0.cnf"),
    (92, "T105.2.0.cnf"),
    (93, "T107.2.0.cnf"),
    (94, "T116.2.0.cnf"),
    (95, "T117.2.0.cnf"),
    (96, "T122.2.0.cnf"),
    (97, "T125.2.0.cnf"),
    (98, "T129.2.0.cnf"),
    (99, "T56.2.0.cnf"),
    (100, "T62.2.0.cnf"),
    (101, "T77.2.0.cnf"),
    (102, "T82.2.0.cnf"),
    (103, "T84.2.0.cnf"),
    (104, "T86.2.0.cnf"),
    (105, "T87.2.0.cnf"),
    (106, "T90.2.0.cnf"),
    (107, "T92.2.0.cnf"),
    (108, "T93.2.0.cnf"),
    (109, "T96.2.0.cnf"),
    (110, "T98.2.0.cnf"),
    (111, "TimetableCNFEncoding_10_UNKNOWN.cnf"),
    (112, "TimetableCNFEncoding_16_UNKNOWN.cnf"),
    (113, "TimetableCNFEncoding_17_UNKNOWN.cnf"),
    (114, "TimetableCNFEncoding_1_UNKNOWN.cnf"),
    (115, "TimetableCNFEncoding_20_UNKNOWN.cnf"),
    (116, "TimetableCNFEncoding_2_UNKNOWN.cnf"),
    (117, "TimetableCNFEncoding_3_UNKNOWN.cnf"),
    (118, "TimetableCNFEncoding_4_UNKNOWN.cnf"),
    (119, "TimetableCNFEncoding_5_UNKNOWN.cnf"),
    (120, "TimetableCNFEncoding_6_UNKNOWN.cnf"),
    (121, "TimetableCNFEncoding_8_UNKNOWN.cnf"),
    (122, "TimetableCNFEncoding_9_UNKNOWN.cnf"),
    (123, "a_rphp035_05.cnf"),
    (124, "a_rphp045_05.cnf"),
    (125, "a_rphp055_04.cnf"),
    (126, "a_rphp056_05.cnf"),
    (127, "a_rphp065_04.cnf"),
    (128, "a_rphp085_04.cnf"),
    (129, "a_rphp098_04.cnf"),
    (130, "ad-A-3-7-17.cnf"),
    (131, "ae_rphp035_05.cnf"),
    (132, "ae_rphp045_05.cnf"),
    (133, "ae_rphp055_04.cnf"),
    (134, "ae_rphp056_05.cnf"),
    (135, "ae_rphp062_05.cnf"),
    (136, "ae_rphp075_04.cnf"),
    (137, "ae_rphp095_04.cnf"),
    (138, "apn-sbox5-cut3-symmbreak.cnf"),
    (139, "apn-sbox5-cut4-symmbreak.cnf"),
    (140, "apn-sbox5-cut5-symmbreak.cnf"),
    (141, "apn-sbox6-cut3-helpbox23.cnf"),
    (142, "apn-sbox6-cut3-helpbox26.cnf"),
    (143, "apn-sbox6-cut3-helpbox28.cnf"),
    (144, "apn-sbox6-cut3-helpbox29.cnf"),
    (145, "apn-sbox6-cut3-helpbox30.cnf"),
    (146, "apn-sbox6-cut3-helpbox31.cnf"),
    (147, "apn-sbox6-cut4-helpbox23.cnf"),
    (148, "apn-sbox6-cut4-helpbox26.cnf"),
    (149, "apn-sbox6-cut4-helpbox28.cnf"),
    (150, "apn-sbox6-cut4-helpbox29.cnf"),
    (151, "apn-sbox6-cut4-helpbox30.cnf"),
    (152, "apn-sbox6-cut4-helpbox31.cnf"),
    (153, "apn-sbox6-cut5-helpbox23.cnf"),
    (154, "apn-sbox6-cut5-helpbox26.cnf"),
    (155, "apn-sbox6-cut5-helpbox28.cnf"),
    (156, "apn-sbox6-cut5-helpbox29.cnf"),
    (157, "apn-sbox6-cut5-helpbox30.cnf"),
    (158, "assoc2.c.cnf"),
    (159, "assoc_mult_err_3.c.cnf"),
    (160, "avg_true-unreach-call.i.cnf"),
    (161, "ax-d-4-7-5.cnf"),
    (162, "bibd-16-80-15-3-2_glb.cnf"),
    (163, "bibd-19-57-9-3-1_glb.cnf"),
    (164, "bibd-8-70-35-4-15_glb.cnf"),
    (165, "bx-d-4-7-6.cnf"),
    (166, "bx-d-4-7-8.cnf"),
    (167, "by-X-2-7-100.cnf"),
    (168, "by-alt-5-7.cnf"),
    (169, "bz-X-4-7-6.cnf"),
    (170, "cannonball-opt-y.rules.4-10.cnf"),
    (171, "cms-scheel-md4-families-r24-c4-p4-9-13-18.cnf"),
    (172, "cms-scheel-md4-families-r24-c4-p6-13-17-19.cnf"),
    (173, "cms-scheel-md4-families-r24-c5-p0-5-11-13-18.cnf"),
    (174, "cms-scheel-md4-families-r24-c5-p0-7-10-15-19.cnf"),
    (175, "cms-scheel-md4-families-r24-c5-p1-5-10-12-18.cnf"),
    (176, "cms-scheel-md4-families-r24-c5-p1-8-13-18-19.cnf"),
    (177, "cms-scheel-md4-families-r24-c5-p1-8-9-16-19.cnf"),
    (178, "cms-scheel-md4-families-r24-c5-p5-13-16-17-19.cnf"),
    (179, "cms-scheel-md4-families-r24-c6-p0-5-10-11-17-18.cnf"),
    (180, "cms-scheel-md4-families-r24-c6-p1-9-13-16-18-19.cnf"),
    (181, "cms-scheel-md4-families-r24-c6-p2-4-9-16-17-19.cnf"),
    (
        182,
        "cms-scheel-md4-families-r24-c8-p2-3-4-5-7-12-16-19.cnf",
    ),
    (
        183,
        "cms-scheel-md5-families-r24-c11-p1-4-6-9-10-11-13-15-17-18-19.cnf",
    ),
    (184, "cms-scheel-md5-families-r24-c4-p1-3-4-16.cnf"),
    (185, "cms-scheel-md5-families-r24-c5-p2-7-8-15-19.cnf"),
    (186, "cms-scheel-md5-families-r24-c5-p6-11-15-16-19.cnf"),
    (187, "cms-scheel-md5-families-r24-c6-p1-4-6-13-14-18.cnf"),
    (188, "cms-scheel-md5-families-r24-c7-p1-3-6-10-11-14-15.cnf"),
    (189, "cms-scheel-md5-families-r24-c7-p1-5-6-12-16-18-19.cnf"),
    (
        190,
        "cms-scheel-md5-families-r24-c8-p0-3-5-6-14-16-18-19.cnf",
    ),
    (191, "commutativity.c.cnf"),
    (192, "commutativity3.c.cnf"),
    (193, "course0.12_2018_3_1.cnf"),
    (194, "course0.12_2018_3_3.cnf"),
    (195, "course0.12_2018_3_5.cnf"),
    (196, "course0.16_2018_3_5.cnf"),
    (197, "course0.2_2018_3.cnf"),
    (198, "course0.2_2018_3_5.cnf"),
    (199, "course_flat_0.12_2018_3.cnf"),
    (200, "course_flat_0.12_2018_3_1.cnf"),
    (201, "course_flat_0.12_2018_3_3.cnf"),
    (202, "course_flat_0.12_2018_3_5.cnf"),
    (203, "course_flat_0.16_2018_3.cnf"),
    (204, "course_flat_0.2_2018_3.cnf"),
    (205, "courses0.12_2017_9.cnf"),
    (206, "courses0.2_2017_9.cnf"),
    (207, "courses_flat_0.12_2017_9.cnf"),
    (208, "courses_flat_0.16_2017_9.cnf"),
    (209, "cz-3-7-7.cnf"),
    (210, "cz-alt-3-7.cnf"),
    (211, "cz-d-4-7-5.cnf"),
    (212, "dist10.c.cnf"),
    (213, "dist4.c.cnf"),
    (214, "dist5.c.cnf"),
    (215, "dist6.c.cnf"),
    (216, "dist7.c.cnf"),
    (217, "dist8.c.cnf"),
    (218, "dist9.c.cnf"),
    (219, "e_rphp035_05.cnf"),
    (220, "e_rphp055_04.cnf"),
    (221, "e_rphp056_05.cnf"),
    (222, "e_rphp065_04.cnf"),
    (223, "e_rphp065_05.cnf"),
    (224, "e_rphp096_04.cnf"),
    (225, "ecarev-110-1031-23-40-1.cnf"),
    (226, "ecarev-110-1031-23-40-2.cnf"),
    (227, "ecarev-110-1031-23-40-3.cnf"),
    (228, "ecarev-110-1031-23-40-5.cnf"),
    (229, "ecarev-110-1031-23-40-6.cnf"),
    (230, "ecarev-110-1031-23-40-7.cnf"),
    (231, "ecarev-110-1031-23-40-8.cnf"),
    (232, "ecarev-110-4099-22-30-2.cnf"),
    (233, "ecarev-110-4099-22-30-4.cnf"),
    (234, "ecarev-110-4099-22-30-5.cnf"),
    (235, "ecarev-110-4099-22-30-7.cnf"),
    (236, "ex009_10.cnf"),
    (237, "ex015_14.cnf"),
    (238, "ex025_19.cnf"),
    (239, "ex039_33.cnf"),
    (240, "ex045_7.cnf"),
    (241, "ex051_9.cnf"),
    (242, "ex065_24.cnf"),
    (243, "ex065_25.cnf"),
    (244, "ex067_10.cnf"),
    (245, "ex095_8.cnf"),
    (246, "ex145_10.cnf"),
    (247, "ex145_11.cnf"),
    (248, "ex157_10.cnf"),
    (249, "ex175_17.cnf"),
    (250, "ex175_18.cnf"),
    (251, "ex175_20.cnf"),
    (252, "ex177_12.cnf"),
    (253, "ex177_13.cnf"),
    (254, "ex179_11.cnf"),
    (255, "ex187_10.cnf"),
    (256, "exam0.04_2018_3.cnf"),
    (257, "exam_flat_0.04_2018_3.cnf"),
    (258, "exams0.04_2017_9.cnf"),
    (259, "exams_flat_0.04_2017_9.cnf"),
    (260, "factoring29986577x29986577.cnf"),
    (261, "factoring39916801x54018521.cnf"),
    (262, "factoring54018521x562448657.cnf"),
    (263, "factoring87654321x12345678.cnf"),
    (264, "factoring94418953x321534781.cnf"),
    (265, "filter1_true-unreach-call.c.cnf"),
    (266, "filter_iir_true-unreach-call.c.cnf"),
    (267, "frb35-17-5-mgd_ext.cnf"),
    (268, "frb35-17-5_ext.cnf"),
    (269, "full-ax-xa.rules.7-7.cnf"),
    (270, "full-by-za.rules.5-7.cnf"),
    (271, "full-cy-caa.rules.7-7.cnf"),
    (272, "gto_p50c291.cnf"),
    (273, "gto_p50c307.cnf"),
    (274, "gto_p50c311.cnf"),
    (275, "gto_p50c312.cnf"),
    (276, "gto_p50c314.cnf"),
    (277, "gto_p50c314_1.cnf"),
    (278, "gto_p50c345.cnf"),
    (279, "gto_p50c345_1.cnf"),
    (280, "gto_p60c231.cnf"),
    (281, "gto_p60c231_1.cnf"),
    (282, "gto_p60c233.cnf"),
    (283, "gto_p60c234.cnf"),
    (284, "gto_p60c235.cnf"),
    (285, "gto_p60c238.cnf"),
    (286, "gto_p60c239.cnf"),
    (287, "gto_p60c241.cnf"),
    (288, "gto_p60c243.cnf"),
    (289, "gto_p60c295.cnf"),
    (290, "gto_p60c343.cnf"),
    (291, "huck.col.11.cnf"),
    (292, "le450_15a.col.15.cnf"),
    (293, "le450_15b.col.15.cnf"),
    (294, "le450_15c.col.15.cnf"),
    (295, "le450_25a.col.25.cnf"),
    (296, "le450_25b.col.25.cnf"),
    (297, "le450_25c.col.25.cnf"),
    (298, "lei450-05b-05.cnf"),
    (299, "less-cy-caa.rules.4-9.cnf"),
    (300, "less-cy-caa.rules.6-4.cnf"),
    (301, "magicSquare-8_glb.cnf"),
    (302, "magicSquare-9_glb.cnf"),
    (303, "mchess_15.cnf"),
    (304, "mchess_16.cnf"),
    (305, "mchess_17.cnf"),
    (306, "mchess_18.cnf"),
    (307, "mchess_19.cnf"),
    (308, "mchess_20.cnf"),
    (309, "mod8-abd-yad.rules.7-7.cnf"),
    (310, "mod8-bbd-zbd.rules.7-7.cnf"),
    (311, "newton_3_4_true-unreach-call.i.cnf"),
    (312, "newton_3_6_false-unreach-call.i.cnf"),
    (313, "ortholatin-7.cnf"),
    (
        314,
        "pals_floodmax.5_overflow_false-unreach-call.ufo.UNBOUNDED.pals.c.cnf",
    ),
    (
        315,
        "pals_lcr-var-start-time.5_true-unreach-call.ufo.UNBOUNDED.pals.c.cnf",
    ),
    (
        316,
        "pals_lcr-var-start-time.6_true-unreach-call.ufo.UNBOUNDED.pals.c.cnf",
    ),
    (
        317,
        "pals_lcr.8_overflow_false-unreach-call.ufo.UNBOUNDED.pals.c.cnf",
    ),
    (318, "patat-08-comp-3.cnf"),
    (319, "prime_119218851371.cnf"),
    (320, "prime_200560490131.cnf"),
    (321, "quadratic_loose_error.c.cnf"),
    (322, "quadratic_tight_error.c.cnf"),
    (323, "queen12_12.col.12.cnf"),
    (324, "queen13_13.col.13.cnf"),
    (325, "queen14_14.col.14.cnf"),
    (326, "queen15_15.col.15.cnf"),
    (327, "queen8-8-9.cnf"),
    (328, "queen8_12.col.12.cnf"),
    (329, "queen8_8.col.9.cnf"),
    (330, "ramsey-30-4.cnf"),
    (331, "ramsey-32-4.cnf"),
    (332, "satcoin-genesis-SAT-10.cnf"),
    (333, "satcoin-genesis-SAT-1024.cnf"),
    (334, "satcoin-genesis-SAT-128.cnf"),
    (335, "satcoin-genesis-SAT-16.cnf"),
    (336, "satcoin-genesis-SAT-2048.cnf"),
    (337, "satcoin-genesis-SAT-256.cnf"),
    (338, "satcoin-genesis-SAT-3.cnf"),
    (339, "satcoin-genesis-SAT-32.cnf"),
    (340, "satcoin-genesis-SAT-4.cnf"),
    (341, "satcoin-genesis-SAT-4096.cnf"),
    (342, "satcoin-genesis-SAT-5.cnf"),
    (343, "satcoin-genesis-SAT-512.cnf"),
    (344, "satcoin-genesis-SAT-64.cnf"),
    (345, "satcoin-genesis-SAT-7.cnf"),
    (346, "satcoin-genesis-SAT-8.cnf"),
    (347, "satcoin-genesis-SAT-8192.cnf"),
    (348, "satcoin-genesis-SAT-9.cnf"),
    (349, "school1.col.14.cnf"),
    (350, "school1_nsh.col.14.cnf"),
    (351, "sdiv15prop.cnf"),
    (352, "sdiv16prop.cnf"),
    (353, "sdiv17prop.cnf"),
    (354, "sdiv20prop.cnf"),
    (355, "sdiv25prop.cnf"),
    (356, "sdiv30prop.cnf"),
    (357, "sdiv40prop.cnf"),
    (358, "si2-b03m-m400-03.cnf"),
    (359, "si2-b03m-m800-03.cnf"),
    (360, "si2-b06m-m1000-03.cnf"),
    (361, "si2-m4Dr2-m256-08.cnf"),
    (362, "si2-r001-m200-00.cnf"),
    (363, "si2-r001-m200-09.cnf"),
    (
        364,
        "sqrt_Householder_pseudoconstant_true-unreach-call.c.cnf",
    ),
    (365, "sqrt_ineq_2.c.cnf"),
    (366, "sqrt_ineq_3.c.cnf"),
    (367, "sted1_0x0-330.cnf"),
    (368, "sted1_0x0-350.cnf"),
    (369, "sted1_0x0-380.cnf"),
    (370, "sted1_0x0-420.cnf"),
    (371, "sted1_0x1e3-100.cnf"),
    (372, "sted1_0x1e3-200.cnf"),
    (373, "sted1_0x1e3-300.cnf"),
    (374, "sted1_0x24204-330.cnf"),
    (375, "sted1_0x24204-350.cnf"),
    (376, "sted1_0x24204-380.cnf"),
    (377, "sted5_0x0-40.cnf"),
    (378, "sted5_0x0-50.cnf"),
    (379, "sted5_0x0-60.cnf"),
    (380, "sted5_0x0-70.cnf"),
    (381, "sted5_0x1e3-20.cnf"),
    (382, "sted5_0x1e3-40.cnf"),
    (383, "sted5_0x1e3-60.cnf"),
    (384, "sted5_0x24204-50.cnf"),
    (385, "sted5_0x24204-60.cnf"),
    (386, "sted5_0x24204-70.cnf"),
    (
        387,
        "terminator_03_true-unreach-call_true-termination.i.cnf",
    ),
    (388, "udiv35prop.cnf"),
    (389, "udiv40prop.cnf"),
    (390, "udiv45prop.cnf"),
    (391, "udiv46prop.cnf"),
    (392, "udiv47prop.cnf"),
    (393, "udiv48prop.cnf"),
    (394, "uniqinv20prop.cnf"),
    (395, "uniqinv25prop.cnf"),
    (396, "uniqinv30prop.cnf"),
    (397, "uniqinv40prop.cnf"),
    (398, "uniqinv45prop.cnf"),
    (399, "uniqinv46prop.cnf"),
    (400, "uniqinv47prop.cnf"),
];
