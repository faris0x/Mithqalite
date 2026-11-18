// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use itertools::Itertools;
use mithqalite_base::random::EXAMPLE_SEED;
use mithqalite_base::strings::random::random_fixed_length_strings;
use mithqalite_base::test_util::stats::common_values_map::common_values_map;
use mithqalite_base::test_util::stats::median;

fn random_fixed_length_strings_helper(
    len: u64,
    expected_values: &[&str],
    expected_common_values: &[(&str, usize)],
    expected_median: (&str, Option<&str>),
) {
    let ss = random_fixed_length_strings(EXAMPLE_SEED, len);
    let values = ss.clone().take(20).collect_vec();
    let common_values = common_values_map(1000000, 10, ss.clone());
    let (median_lo, median_hi) = median(ss.take(1000000));
    assert_eq!(
        (
            values.iter().map(String::as_str).collect_vec().as_slice(),
            common_values
                .iter()
                .map(|(s, f)| (s.as_str(), *f))
                .collect_vec()
                .as_slice(),
            (median_lo.as_str(), median_hi.as_deref())
        ),
        (expected_values, expected_common_values, expected_median)
    );
}

#[test]
fn test_random_fixed_length_strings() {
    random_fixed_length_strings_helper(0, &[""; 20], &[("", 1000000)], ("", None));
    random_fixed_length_strings_helper(
        1,
        &[
            "\u{5f771}",
            "\u{87234}",
            "\u{bcd36}",
            "\u{9e195}",
            "\u{5da07}",
            "\u{36553}",
            "\u{45028}",
            "\u{1cdfd}",
            "\u{d8530}",
            "\u{c7f2e}",
            "\u{ba4bc}",
            "\u{ff677}",
            "\u{a12e2}",
            "\u{d775c}",
            "\u{f827b}",
            "\u{bdf7a}",
            "簅",
            "\u{15aca}",
            "\u{4e5e2}",
            "\u{bb286}",
        ],
        &[
            ("\u{1e21d}", 8),
            ("\u{bd934}", 8),
            ("\u{dc941}", 8),
            ("䄡", 7),
            ("霜", 7),
            ("\u{ee5a}", 7),
            ("𐊩", 7),
            ("\u{163a4}", 7),
            ("\u{1e7ff}", 7),
            ("𣡝", 7),
        ],
        ("\u{88629}", Some("\u{8862a}")),
    );
    random_fixed_length_strings_helper(
        2,
        &[
            "\u{5f771}\u{87234}",
            "\u{bcd36}\u{9e195}",
            "\u{5da07}\u{36553}",
            "\u{45028}\u{1cdfd}",
            "\u{d8530}\u{c7f2e}",
            "\u{ba4bc}\u{ff677}",
            "\u{a12e2}\u{d775c}",
            "\u{f827b}\u{bdf7a}",
            "簅\u{15aca}",
            "\u{4e5e2}\u{bb286}",
            "\u{18eeb}\u{bac4f}",
            "\u{5b55a}\u{65709}",
            "\u{b2626}\u{31a93}",
            "\u{8757b}\u{b3524}",
            "\u{fcc17}\u{32c01}",
            "\u{aada8}\u{57e7c}",
            "\u{eb738}栨",
            "\u{51a21}\u{ef6af}",
            "\u{b9caa}\u{d5099}",
            "\u{e397f}\u{32518}",
        ],
        &[
            ("\u{6}욵", 1),
            ("\u{19}此", 1),
            ("\u{1c}훶", 1),
            ("+쳭", 1),
            (">\u{f454}", 1),
            ("B韧", 1),
            ("¦×", 1),
            ("Ϊǔ", 1),
            ("ӚĐ", 1),
            ("ۥä", 1),
        ],
        ("\u{882fd}\u{b44d9}", Some("\u{882fe}\u{6d6a7}")),
    );
    random_fixed_length_strings_helper(
        3,
        &[
            "\u{5f771}\u{87234}\u{bcd36}",
            "\u{9e195}\u{5da07}\u{36553}",
            "\u{45028}\u{1cdfd}\u{d8530}",
            "\u{c7f2e}\u{ba4bc}\u{ff677}",
            "\u{a12e2}\u{d775c}\u{f827b}",
            "\u{bdf7a}簅\u{15aca}",
            "\u{4e5e2}\u{bb286}\u{18eeb}",
            "\u{bac4f}\u{5b55a}\u{65709}",
            "\u{b2626}\u{31a93}\u{8757b}",
            "\u{b3524}\u{fcc17}\u{32c01}",
            "\u{aada8}\u{57e7c}\u{eb738}",
            "栨\u{51a21}\u{ef6af}",
            "\u{b9caa}\u{d5099}\u{e397f}",
            "\u{32518}\u{6952d}\u{93ad5}",
            "\u{65c6e}\u{dc7bd}\u{aec4c}",
            "\u{dd524}\u{c0bc1}\u{795d0}",
            "\u{dbb9d}\u{a50fb}紐",
            "\u{4effe}\u{794af}\u{1b5a0}",
            "\u{56be3}\u{78fc9}\u{5870a}",
            "\u{106f48}\u{dfffa}\u{3cc01}",
        ],
        &[
            ("\u{19}此\u{2f2bd}", 1),
            ("(\u{6c2b8}\u{f70}", 1),
            ("+쳭\u{93ded}", 1),
            (",\u{e5cc2}\u{ea33}", 1),
            ("6\u{a341e}엕", 1),
            (">\u{f454}\u{5e168}", 1),
            ("G\u{47b77}䊹", 1),
            ("K\u{58609}\u{e08e}", 1),
            ("O┑\u{37d86}", 1),
            ("R\u{f41e}\u{41c45}", 1),
        ],
        (
            "\u{884b7}\u{6ad70}\u{3d86d}",
            Some("\u{884b8}\u{10f66a}\u{da539}"),
        ),
    );
    random_fixed_length_strings_helper(
        10,
        &[
            "\u{5f771}\u{87234}\u{bcd36}\u{9e195}\u{5da07}\u{36553}\u{45028}\u{1cdfd}\u{d8530}\
            \u{c7f2e}",
            "\u{ba4bc}\u{ff677}\u{a12e2}\u{d775c}\u{f827b}\u{bdf7a}簅\u{15aca}\u{4e5e2}\u{bb286}",
            "\u{18eeb}\u{bac4f}\u{5b55a}\u{65709}\u{b2626}\u{31a93}\u{8757b}\u{b3524}\u{fcc17}\
            \u{32c01}",
            "\u{aada8}\u{57e7c}\u{eb738}栨\u{51a21}\u{ef6af}\u{b9caa}\u{d5099}\u{e397f}\u{32518}",
            "\u{6952d}\u{93ad5}\u{65c6e}\u{dc7bd}\u{aec4c}\u{dd524}\u{c0bc1}\u{795d0}\u{dbb9d}\
            \u{a50fb}",
            "紐\u{4effe}\u{794af}\u{1b5a0}\u{56be3}\u{78fc9}\u{5870a}\u{106f48}\u{dfffa}\u{3cc01}",
            "\u{91290}\u{4628e}\u{bee71}\u{70e90}\u{b48bb}\u{3a445}\u{10a645}𬆠\u{e59de}\u{61b5a}",
            "\u{f4783}\u{c5ab2}幔\u{fdb07}\u{abccb}\u{ba750}\u{88d5a}\u{a706e}\u{969a2}\u{1089e3}",
            "\u{102189}\u{5f066}\u{10ea66}\u{435bb}\u{bcbd6}\u{4bc59}𱆞\u{f50a0}\u{47bc1}\u{5fd98}",
            "\u{91a7a}\u{100a8d}\u{e0017}\u{9db06}\u{1ab04}\u{780f6}ㅚ\u{5fc0a}\u{fb714}\u{c62cd}",
            "\u{b22dc}\u{10364e}\u{ee477}\u{f0983}\u{b5c36}\u{41f7b}\u{bdf28}\u{b27f7}\u{94dc8}\
            \u{73381}",
            "\u{34609}\u{52911}\u{e56bf}\u{100af4}\u{396ff}\u{1051a8}𬅑\u{815dc}\u{fd1e7}\u{e6e9c}",
            "攠\u{eceaa}\u{10029d}\u{5e236}\u{d963b}\u{bbb1a}\u{108b67}\u{e5bc3}\u{97108}𢔱",
            "\u{9f166}\u{dedb4}\u{52752}\u{45bf5}\u{86d73}\u{ff2fd}쫵\u{78f74}\u{93bc0}\u{c308f}",
            "\u{e8b6}\u{89619}\u{1cdf9}\u{b9c86}\u{9f375}\u{c2487}\u{e1c3e}\u{f6e29}\u{79cc5}𬑎",
            "\u{9a803}\u{bf22e}\u{7e88e}\u{c50e8}\u{58c32}\u{79248}𰥦\u{b238b}\u{104b92}\u{8cc78}",
            "\u{eecc5}𢇡\u{e1fb6}\u{625ab}\u{a1988}䮨\u{bbaa5}\u{143be}\u{a12d4}\u{1028e1}",
            "\u{1c105}\u{9493f}\u{efa70}\u{13487}紋\u{b1948}\u{89052}\u{8c3cb}\u{b82d2}\u{729c3}",
            "\u{10c5ba}\u{dec07}𰢫\u{d277f}\u{3e5dc}\u{52431}\u{4867e}\u{75774}𪲨\u{b865a}",
            "\u{105191}\u{93891}\u{c4975}\u{c2f58}\u{d387c}\u{157dd}\u{77a83}\u{d6eec}\u{b2581}\
            \u{9bb09}",
        ],
        &[
            ("崕浆\u{3da8f}뽑\u{d3fa5}\u{3139e}X\u{48918}\u{108376}♟", 1),
            ("馑霸\u{fdd49}\u{d2b16}튞ଓ儳輋韨\u{be78f}", 1),
            ("𨟿\u{a025e}\u{4d531}席ےᥒ\"𤭉\u{b30a9}\u{988d4}", 1),
            (
                "\u{c6771}\u{c979e}᳇\u{f4e2}能Ŷ\u{474f6}\u{f872}窚\u{6ad4b}",
                1,
            ),
            ("+쳭\u{93ded}\u{84fa9}㱆\u{b3aac}곖𦸌\u{593c4}\u{57c2f}", 1),
            (
                "w\u{6bb08}\u{e8b68}\u{f961c}\u{c362e}\u{2e6b}\u{95326}\u{7ff5b}受\u{f715}",
                1,
            ),
            (
                "ȵ\u{48470}\u{104dcc}扞×ⴞ\u{1ebce}\u{1065e4}\u{34780}\u{46560}",
                1,
            ),
            ("ɿ\u{13f25}\u{bb3ca}垾\u{7f077}酓\u{3f2d3}\u{86f0b}壪床", 1),
            (
                "ӯ\u{83676}\u{d50cb}ꧏ\u{f9dcd}\u{d1466}\u{1de8e}\u{13bb5}搜\u{592}",
                1,
            ),
            (
                "\u{5cb}\u{cc480}ⶺǠ\u{95c2c}\u{6d588}㬪𪮊\u{147b9}\u{d7fa9}",
                1,
            ),
        ],
        (
            "\u{880b8}\u{7ba91}\u{615f9}𝟂\u{1a34d}\u{4bb86}\u{363e3}\u{f113b}\u{fada1}\u{88aa9}",
            Some("\u{880b8}\u{9f3aa}\u{d7dab}𪿾\u{100550}\u{fd341}𬐿鄓𬫦\u{e718}"),
        ),
    );
}
