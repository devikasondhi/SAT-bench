#!/bin/sh
TIMEOUT=1200
SOLVER=$HOME/.local/bin/mios-gp
DIR=$HOME/Documents/ownCloud/SR15m

export LC_ALL=C
export TIMEFORMAT=" %2U"
run () {
    PROB=$DIR/$1
    echo ${PROB}
    time timeout -k ${TIMEOUT} -s 9 ${TIMEOUT} ${SOLVER} -1 6.0 -2 0.35 -3 2 -4 3 ${PROB} | ${SOLVER} -X -: ${PROB}
}

run 009-80-4.cnf
run 38bits_10.dimacs.cnf
run 40bits_10.dimacs.cnf
run 42bits_12.dimacs.cnf
run 44bits_11.dimacs.cnf
run 46bits_11.dimacs.cnf
run 46bits_12.dimacs.cnf
run 46bits_14.dimacs.cnf
run 48bits_11.dimacs.cnf
run 48bits_14.dimacs.cnf
run 50bits_10.dimacs.cnf
run 6s168-opt.cnf
run ACG-20-5p0.cnf
run E02F20.cnf
run E02F22.cnf
run UCG-20-5p0.cnf
run aes_32_2_keyfind_1.cnf
run aes_32_3_keyfind_1.cnf
run atco_enc1_opt1_04_32.cnf
run atco_enc1_opt2_10_14.cnf
run atco_enc2_opt2_05_9.cnf
run atco_enc2_opt2_20_11.cnf
run complete-400-0.1-16-98765432140016.cnf
run complete-400-0.1-3-9876543214003.cnf
run complete-500-0.1-1-9876543215001.cnf
run complete-500-0.1-15-98765432150015.cnf
run complete-500-0.1-17-98765432150017.cnf
run complete-500-0.1-8-9876543215008.cnf
run gss-18-s100.cnf
run gss-19-s100.cnf
run itox_vc1130.cnf
run jgiraldezlevy.2200.9086.08.40.158.cnf
run jgiraldezlevy.2200.9086.08.40.194.cnf
run jgiraldezlevy.2200.9086.08.40.22.cnf
run jgiraldezlevy.2200.9086.08.40.35.cnf
run jgiraldezlevy.2200.9086.08.40.41.cnf
run jgiraldezlevy.2200.9086.08.40.62.cnf
run jgiraldezlevy.2200.9086.08.40.81.cnf
run jgiraldezlevy.2200.9086.08.40.85.cnf
run k2fix_gr_rcs_w9.shuffled.cnf
run manthey_DimacsSorterHalf_28_4.cnf
run manthey_DimacsSorterHalf_29_2.cnf
run manthey_DimacsSorterHalf_30_0.cnf
run manthey_DimacsSorterHalf_30_3.cnf
run manthey_DimacsSorterHalf_30_7.cnf
run manthey_DimacsSorterHalf_31_2.cnf
run manthey_DimacsSorterHalf_31_8.cnf
run manthey_DimacsSorterHalf_32_1.cnf
run manthey_DimacsSorterHalf_33_1.cnf
run manthey_DimacsSorterHalf_37_6.cnf
run manthey_DimacsSorterHalf_37_8.cnf
run manthey_DimacsSorterHalf_37_9.cnf
run manthey_DimacsSorter_28_0.cnf
run manthey_DimacsSorter_28_3.cnf
run manthey_DimacsSorter_28_4.cnf
run manthey_DimacsSorter_28_7.cnf
run manthey_DimacsSorter_30_1.cnf
run manthey_DimacsSorter_30_4.cnf
run manthey_DimacsSorter_30_5.cnf
run manthey_DimacsSorter_30_6.cnf
run manthey_DimacsSorter_31_0.cnf
run manthey_DimacsSorter_31_6.cnf
run manthey_DimacsSorter_31_8.cnf
run manthey_DimacsSorter_32_1.cnf
run manthey_DimacsSorter_35_7.cnf
run manthey_DimacsSorter_36_4.cnf
run manthey_DimacsSorter_37_6.cnf
run manthey_single-ordered-initialized-w12-b6.cnf
run manthey_single-ordered-initialized-w14-b7.cnf
run manthey_single-ordered-initialized-w18-b7.cnf
run manthey_single-ordered-initialized-w18-b8.cnf
run manthey_single-ordered-initialized-w20-b10.cnf
run manthey_single-ordered-initialized-w20-b7.cnf
run manthey_single-ordered-initialized-w20-b8.cnf
run manthey_single-ordered-initialized-w22-b6.cnf
run manthey_single-ordered-initialized-w22-b8.cnf
run manthey_single-ordered-initialized-w24-b7.cnf
run manthey_single-ordered-initialized-w24-b9.cnf
run manthey_single-ordered-initialized-w26-b7.cnf
run manthey_single-ordered-initialized-w26-b8.cnf
run manthey_single-ordered-initialized-w28-b8.cnf
run manthey_single-ordered-initialized-w32-b9.cnf
run manthey_single-ordered-initialized-w40-b10.cnf
run manthey_single-ordered-initialized-w40-b8.cnf
run manthey_single-ordered-initialized-w42-b8.cnf
run manthey_single-ordered-initialized-w42-b9.cnf
run manthey_single-ordered-initialized-w44-b6.cnf
run manthey_single-ordered-initialized-w44-b7.cnf
run manthey_single-ordered-initialized-w44-b9.cnf
run manthey_single-ordered-initialized-w46-b7.cnf
run manthey_single-ordered-initialized-w48-b6.cnf
run manthey_single-ordered-initialized-w48-b8.cnf
run manthey_single-ordered-initialized-w50-b6.cnf
run manthey_single-ordered-initialized-w54-b9.cnf
run minxorminand064.cnf
run mrpp_4x4-10_20.cnf
run mrpp_4x4-10_9.cnf
run mrpp_4x4-12_12.cnf
run mrpp_4x4-4_24.cnf
run mrpp_4x4-4_4.cnf
run mrpp_4x4-4_5.cnf
run mrpp_4x4-6_16.cnf
run mrpp_4x4-6_20.cnf
run mrpp_4x4-6_5.cnf
run mrpp_4x4-8_8.cnf
run mrpp_6x6-10_10.cnf
run mrpp_6x6-10_8.cnf
run mrpp_6x6-12_16.cnf
run mrpp_6x6-12_8.cnf
run mrpp_6x6-14_24.cnf
run mrpp_6x6-16_12.cnf
run mrpp_6x6-16_9.cnf
run mrpp_6x6-18_12.cnf
run mrpp_6x6-18_9.cnf
run mrpp_6x6-20_10.cnf
run mrpp_8x8-12_9.cnf
run mrpp_8x8-16_12.cnf
run mrpp_8x8-18_12.cnf
run mrpp_8x8-18_13.cnf
run mrpp_8x8-22_10.cnf
run mrpp_8x8-22_11.cnf
run mrpp_8x8-24_11.cnf
run mrpp_8x8-24_14.cnf
run mrpp_8x8-24_16.cnf
run mrpp_8x8-8_9.cnf
run openstacks-p30_3.085-SAT.cnf
run partial-5-15-s.cnf
run q_query_3_L150_coli.sat.cnf
run q_query_3_L200_coli.sat.cnf
run q_query_3_L80_coli.sat.cnf
run vmpc_29.cnf