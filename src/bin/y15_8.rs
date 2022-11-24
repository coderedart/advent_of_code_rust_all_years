fn calculate_in_memory_size() -> usize {
    let mut lines = INPUT.lines();

    let mut total_count = 0;
    while let Some(line) = lines.next() {
        if line.is_empty() {
            continue;
        }
        let line = line.strip_prefix('"').unwrap();
        let line = line.strip_suffix('"').unwrap();
        let mut chars = line.chars();
        let mut previous_backslash = false;
        let mut count = 0;
        while let Some(c) = chars.next() {
            if previous_backslash {
                previous_backslash = false;
                match c {
                    '\\' => continue,
                    'x' => {
                        chars.next();
                        chars.next();
                        continue;
                    }
                    '"' => {
                        continue;
                    }
                    _ => unreachable!(),
                }
            }
            if c == '\\' {
                previous_backslash = true;
            }
            if c == '"' {
                continue;
            }
            count += 1;
        }
        total_count += count;
    }
    total_count
}
fn calculate_in_code_size() -> usize {
    INPUT.lines().map(|s| s.len()).sum()
}
fn calculate_in_code_size_new_string() -> usize {
    INPUT
        .lines()
        .map(|s| {
            let char_count = s.len();
            // "" becomes "\"\"". so, we should add 4 for every string.
            let char_count = char_count + 4; // for escaping wrapped double quotes using \" on each side
                                             // this counts the wrapping double quotes too
                                             // so make sure to delete count of two (for wrapping double quotes)
            let backslash_count = s.chars().filter(|c| *c == '\\' || *c == '"').count();

            let total = char_count + backslash_count - 2;
            assert!(format!("{:#?}", s).len() == total);
            // println!("{s:#?}, {total}");
            total
        })
        .sum()
}
fn main() {
    let code_size = calculate_in_code_size();
    let memory_size = calculate_in_memory_size();
    let code_size_new_string = calculate_in_code_size_new_string();
    dbg!(
        memory_size,
        code_size,
        code_size_new_string,
        code_size - memory_size,
        code_size_new_string - code_size
    );
}

const INPUT: &str = r#"
"\xa8br\x8bjr\""
"nq"
"zjrfcpbktjmrzgsz\xcaqsc\x03n\"huqab"
"daz\\zyyxddpwk"
"draes\xa2n\\g\x27ek\"lj\"\\viqych"
"nnx\\krnrfomdnt\x2flbl\xd2xpo\"cp\"k"
"kwdaapalq"
"u\"ptk"
"ckhorczuiudfjmmcc\\u\"wozqxibsfjma"
"ydctdrxat\"pd\"lwi\"bjesevfw\xe8"
"v\"\xa8rrzep\"\"r"
"nbydghkfvmq\\\xe0\"lfsrsvlsj\"i\x61liif"
"jsas\"u"
"odipikxlo"
"\"rnubsgwltqkbsu\"pcpcs"
"eitk\\f\\mhcqqoym\\ji"
"vnedc"
"\"lhcaurdqzyjyu"
"haxzsa\"zcn\"y\"foclgtjfcnv\"m\x68krc"
"\"eoeggg\"tmiydvcay\"vfavc"
"snqvyqoncwxcvwbdktoywch"
"rnfgjsyr\xd5wacy"
"ik\"hebrpvsts"
"txw"
"\x15pxtdkogd\"urbm\"gevhh\"nxr\x3erxtk"
"cetqtcy"
"inleep\\mgl"
"uflwbxvww\x2cxzezqnaply\"yh\"qlllzk"
"eepak\"xqtedzt"
"na\x61qzfieafvyrsnwkssznohjmc"
"yceaonylz\xc1\\jrlbbkzwsidfi"
"ybqafngkcqpbp"
"\xaft"
"yidjpaobqydso"
"ju\\ldxig\\lrdrhjcmm\x77rc"
"tylacqeslnwj\x48ds\"tjxa"
"efbfm"
"\\fxkgoprgdcjgyajykg\\dtbrz"
"eujvva"
"h\x7acwfpikme\\vwthyvrqdnx\""
"rbpbrxm\\\"\"\"voxx"
"ykiw\"tkb\\lforu\"rsf\\tf\"x\"rqti"
"e\\wh\x77aqeugiq\\ihhfqfuaij"
"g\"t\\o"
"nxzo\"hf\\xp"
"dxiaqfo\xea"
"kali\\zczhiqkqzybjj\"fgdjnik"
"zdkgrqmdv"
"bimxim\xb6lrwsaj\"ui\"a"
"\"rrznitibgx\\olpsjmjqzctxaubdifsq"
"zb\"khzixaacmhuzmlymoformipdzml"
"qfwi"
"hjwsxfpphttjy\"\"zixais\xbblgnqfto"
"puj\\qmyu\"nqgaqfthbwjokbmrpbhpi"
"cyxdpkh\\\""
"q"
"m"
"tbxdzzllarlo"
"gbtys"
"gytilk\\vlqxvcuutjunrqc"
"uugkvcuzan\\eyhb"
"yaxr\"genlbgw\"\\uc"
"nrgecjeip\\sjdvgqaqxwsqactopu"
"pu\"r\"txpyrkfny\\zmwfneyvwmnkkdipv"
"jm\xa3bhwvq"
"qxojmnml\"w\x9airr"
"xbzsuihs\x4dcedy\xaclrhgii\\\""
"drgjirusrekrwmvxllwdm"
"\x28hfxnfpycmpnkku\"csuf\xaarxlqyg\"x"
"\"zvz\\rmg\"\\sxxoifffyqfyn\"iq\"ps"
"\"z"
"zbwkmk\"sgzos\x93gtc\""
"bvm\x28aa\\\\\"pywuhaniox\\z\\hbp\xd7mold"
"aszgvsyna"
"qf\"vdwuss"
"lnohni\"qwiacjsjegstlbfq\\kyjhyd"
"c\\naawulxlqplnacvytspry\xf5ytxxqq"
"razwqmsqgbaaxcd\\f"
"radggyrjrg\"zx"
"\"pu\x11t\\ajcjuieinlkvya"
"veggiskh"
"eglfhjxiet\"kouqfskwsy\"hpthsldel"
"mv\xc1b\"f\\shrssnjwcpmurepdxdlcj"
"dlayjd\"suvzotgdtc"
"\xa9pvxeopn"
"lpplsaxy\"oiwaq"
"hqwh\\lusv"
"hykykwlx\"\xa5atkgh\\d\x63dff"
"vfktanpjy\"xxetc"
"dnhwkgjnsmsswfuelvihvjl\"jtf"
"x\"dwvzra\"nbbsewftehczgbvfzd\"rau"
"csfi\"mzejnjqkqupwadrgti\"von"
"xckf\xf7xsm\\pgvlpetjndpyblais\\z"
"yecy\x6fuj\x58bwpgeuiw\"mdu"
"fgb"
"c\\lx\x3efthet\xfdelgvwvpem"
"kgyrmarvfwjinlowt"
"yzte"
"vc\"z"
"sxevqfzmmdwsuu\""
"fxbaercmcy\xb6md"
"f"
"m\x44gqbcppho\\b"
"gtafr\x57m\x11jy\"\"erwmmpiwjkbckuw"
"ufdjt\"kssprzxqixzxmq\x58q"
"yzbyo\"lfdbyaxexyfbnyv\\\xe8xmre"
"u\x43ntr\\\\byyfjr\"iveujvnwsqbnpuvrta"
"us\xf6bai"
"c\\edh"
"tzckolphexfq\\\x23\xfbdqv\\\"m"
"yjafhbvhhj\x1b\"bplb"
"\"o"
"rubahvmp\""
"qmkukrnrmqumh"
"wdpxyvyidhwjf\\nabbijwhr\xc5bksvy\"p"
"u\"prlpg\""
"nsvcquyxbwilsxxemf\xd9leq"
"y\xcetxuafl"
"it"
"kwdlysf\\xjpelae"
"viwh\x58wpjjlnvryuti\x2chngrx\\nhtkui"
"vhn\x9ehre\xc3ncsqbozms\"nl"
"ytc\xa3mgeeogjcqavmmmd"
"xzlexlitseozoxtpzzutfq"
"cish\x07lmovj"
"ekbflwqzaiivdr\"pq\\azrfbntqwkn"
"uc\"xdbegmlmhksofzohavtrnxf"
"xfdnrdqdrcjzbe"
"ndg\"ckgrpisib\"rg\"p\\lmpfzlssnvk"
"witfjwpbyyzlop"
"zonlww\"emrbcsgdtrg\"rjzy\x64zqntlw"
"dvgb\"zn\\vrbzema\"ckmd"
"\\vdlmxhlvldk\"pmzazeip"
"\"\"r"
"rsntinv"
"iy"
"lr\x20efh"
"csgexlb\"zqdavlxxhtdbh\"\"\x0fkpvhiphm"
"ouwhp\"ogbft"
"cm\\ckltng\"dw\x8brf\xf0eppgckd"
"zmnlsgalhpkejsizfsbtnfliu\"nhc"
"pnrkaayqvwpdjbhcrbb\"yfeq\"aq"
"ozh\\hoxow\x2csrtr\\r\""
"bqxabj\"u\\s"
"cpsjti\"gy"
"aa\"p\\nki\\ajijkqev"
"q\"\"lfdentjgd\\"
"bmokvpoebutfki"
"pielvcbne\xf6efvzxn"
"kx"
"zlgmqagcrbhrwtwtmmg"
"aiyhmntcqjbpv\xb5hhswxbryoedvos"
"tdpaxrb"
"fu\"\x7dttkyvhrlwko"
"oirc\\\"cqlnqffjqt\\k"
"edxlia\\tcyby"
"jpeybgwfayerfrfbvfog\"ol"
"ysr"
"bzwzilgwfugjk"
"tlcc\x75nukvwjgftetjcs\xaecwc"
"dsqssa\"vzrf\"sewbp\\ahhlmhbeihlh"
"qtgmjck\"n\"guki\"gmdivwqxismqj"
"\"f"
"wuorvlovucngbzdszqpikyk"
"dfrdsacoukmgvhbq\"\"iwto"
"\"ey\"ch\\wcgioe\\\"ouvligmsw"
"ciqlszzgs"
"\\tzyrkaoi\"sopjaq"
"lmtnv"
"ar\"fqoroigiertjjlm\"ymgi\\kkjewsxd"
"wehcimlvudpxtamdn\"rwy"
"hr\"zvrwthr\"vruzqfrldn\"b"
"sggekodkiwvym\"mhsco"
"ltlkfbrrdvk\\"
"uut\"sfjnz\"\\ef"
"hxilg\\"
"zsredsiwlzrpedibn"
"vtfi"
"\\h"
"qekfrc\xf6wduodbwrguqcng\\n"
"\"lljlfdrxftwidn\\pkv\xd9ij"
"mrvgqynpehkliuijlpp"
"gikjph"
"yoxcdrdt\"wbaurnyhoyxoihu"
"onmomwuxuammbzxe"
"rnrr\\twviz\x61gqaljr\x0dmtw"
"r\"vupaoi"
"l"
"sei"
"jwxtdtbkd\\kxd"
"\x22v\\"
"ahd"
"j\"bjqxs"
"\\i\x24gglxub\\nzsajokt"
"lviwpu\"uxdlh\\zuy\"xqy\"ytdzlx\"r"
"kptfmys"
"fwxzikfhczkjwyjszqdbkepaeellc"
"nlqpsvbrbd\\ns"
"qryuwkjiodw\"\"vaqyq\"dmyifm"
"tw\x15kdmaudjl\\zorhp\"alwh"
"aatrvczesykekkjfyb\"kb"
"usqcutbqbxxhucwxo\xc1ltb\"j\"bghjcvws"
"ilhsrnzxkz"
"bianqfdfdhvw"
"hqibqs\x7ax\"qoxqoaqtcsz"
"htxtoojbbauztwxuiq\\ngyfy\\obzc"
"rxn\\moxlj"
"mtus\x84erh\"dbe"
"asx\x50huvsitcxadt"
"\"bugggtnrc\"\"kl\"hmpu\x83hqrvhpo"
"ewisbp\"\"vuzf\\w\x5fvalszdhl"
"scusplpwxfnxu\x57\"zynpn\x99xerc\\ri"
"m\\kinmkke\x0cl"
"xhuzit\x7fd"
"kfbo\x04\x50ruqirn"
"t\"\"xpbdscmdoug"
"punvpsgnbgyxe\"sptmpz"
"bxukkazijr"
"nxyrcdaoo\"rjkk\"wntehcvcip\"vrd"
"rdpvqskmihqaw"
"p\\gwdhtqnpgthod"
"nwnuf\"\"yebycearom\"nqym\"\xd4sii\xccle"
"alda\"ptspo\"wkkv\"zoi\"hkb\"vnntyd"
"ixpgpfzbqv"
"znui\"\\fzn\x03qozabh\"rva\"pv\x67"
"e\"zswmwuk"
"hcccygwfa"
"ngmace\\rtyllolr\"\x68bw"
"\\c\"jyufbry\"ryo\"xpo\x26ecninfeckh\\s"
"hdnpngtuc\"dzbvvosn\x31fwtpzbrt"
"hesbpd\xd4"
"dsdbstuzrdfmrnyntufs\"dmv"
"d\xeeibcwhcvkt"
"fvzwrsfjdqdmy\"\"v"
"ns\"dqafz\\lkyoflnazv\"mn\x37\"o\"yj\"e"
"dypilgbwzccayxa\"bnmuernx"
"q\xa9ztqrhreb\"\"kxfeyodqb"
"iz\xa5qjxqulaawuwz\"rqmpcj\\yel"
"z\"\\pq\"\"y\x67zpjtn"
"ifxqvivp\"kiiftdoe"
"jxzebj\"\x35\"qr\"ecglcutuoyywqumcs\"kk"
"q"
"yob\x85qmpuwexptczbkrl"
"cjiavv\"uudpozvibyycnmxhxpxmpjoz"
"xro\\uiqyrcid"
"nod\\k"
"d\"neiec"
"tqyrqvwyvmz\\pzgzzcqsqsrgbqbtapoz"
"r\"xvocpeuxfxslgueb\x05kzyyie\"aoec"
"\"du\\uirlhcbgv\\cjqhfreqnvn"
"zp\x04\x15\"pbjwhrjtmiba"
"\\cv\""
"k\"rwnb\\hiu\"rqd\"rc\\nyakrhly"
"klrmafjzandiddodgz"
"xipzhqzhvlpykzcuppx"
"zdvrvn\xd0mtfvpylbn\\\\sxcznrzugwznl"
"ody\\pvm\"kpjiudzhxazirgxzvumeat\"o"
"kllvhdp\"prjikzrrc\"adgpegc\\\"gk"
"sqtpug\xbcaauxaamw"
"wegxxrrxdvpivrqievfeokmnojsk"
"\\bo"
"gijhz"
"ylowluvabwrigssdgtxdwsiorxev\xdd"
"\""
"ghnsrnsqtxpygikahkrl"
"\"rcfqkbjf\"sgxg\"vnd\\rotn"
"ap\"smgsuexjrbuqs\"mpbstogj\"x"
"koaunz\\sgt\"opv"
"yialiuzwix"
"yp\"ndxgwzml\"bt"
"lpcjxmggfsy\\szbxccarjkqzasqkb\xcfd\x0c"
"x"
"mgakc"
"vjieunoh\x73fjwx"
"erbvv\"qulsd"
"mimycrbfhqkarmz"
"tihfbgcszuej\"c\xfbvoqskkhbgpaddioo"
"mziavkwrmekriqghw"
"izk\\tnjd\\ed\\emokvjoc"
"c\"nhbqzndro\\g"
"usfngdo"
"aypljdftvptt"
"ym\"afvq\xbcc"
"zabi\"wjpvugwhl"
"ebvptcjqjhc\"n\"p\"dxrphegr\\"
"mzlqqxokhye\xd9\\rffhnzs"
"hnipqknwpsjakanuewe"
"rqgbfcjdrmiz\"h"
"kzzp\\z\\txmkwaouxictybwx"
"yzmspjkqrteiydswlvb"
"gjpxklgpzv\"txri\\hotpuiukzzzd"
"p\"rxergtbsxmjmkeeqwvoagnki\""
"santipvuiq"
"\"ihjqlhtwbuy\"hdkiv\"mtiqacnf\\"
"oliaggtqyyx"
"fwwnpmbb"
"yrtdrieazfxyyneo"
"nywbv\\"
"twc\\ehfqxhgomgrgwpxyzmnkioj"
"qludrkkvljljd\\xvdeum\x4e"
"#;
