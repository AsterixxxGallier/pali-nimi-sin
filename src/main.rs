use std::fmt::{Debug, Display, Formatter};
use std::str::FromStr;

use clap::Parser;
use rand::{Rng, thread_rng};
use rand::prelude::SliceRandom;

#[derive(Copy, Clone)]
enum SuliAlaSuli {
    Suli,
    Ala,
}

impl FromStr for SuliAlaSuli {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "suli" => Ok(SuliAlaSuli::Suli),
            "ala" => Ok(SuliAlaSuli::Ala),
            _ => Err("nimi ni li nimi 'suli' ala li nimi 'ala' ala".into()),
        }
    }
}

impl Debug for SuliAlaSuli {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            SuliAlaSuli::Suli => f.write_str("suli"),
            SuliAlaSuli::Ala => f.write_str("ala")
        }
    }
}

impl Display for SuliAlaSuli {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Debug::fmt(self, f)
    }
}

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Wile {
    #[clap(short = 'l', long, default_value_t = 1)]
    suli_nimi_pi_lili_nanpa_wan: usize,
    #[clap(short = 's', long, default_value_t = 4)]
    suli_nimi_pi_suli_nanpa_wan: usize,
    #[clap(short = 'n', long, default_value_t = SuliAlaSuli::Suli)]
    mu_n_li_suli_ala_suli: SuliAlaSuli,
    #[clap(short = 'N', long, default_value_t = 0.5)]
    mute_pi_mu_n: f64,
    #[clap(short = 'm', long, default_value_t = 20)]
    mute_nimi: isize,
}

fn main() {
    let wile = Wile::parse();
    let mut rng = thread_rng();
    let mut mute_nimi_pali = 0;
    while mute_nimi_pali < wile.mute_nimi || wile.mute_nimi == 0 {
        println!("{}", pali_nimi(
            rng.gen_range(wile.suli_nimi_pi_lili_nanpa_wan..=wile.suli_nimi_pi_suli_nanpa_wan),
            wile.mute_pi_mu_n,
            wile.mu_n_li_suli_ala_suli,
            &mut rng,
        ));
        mute_nimi_pali += 1;
    }
}

const KIPISI_NIMI_PONA: [&str; 41] = [
    "ka", "ke", "ki", "ko", "ku",
    "sa", "se", "si", "so", "su",
    "ta", "te", "to", "tu",
    "na", "ne", "ni", "no", "nu",
    "pa", "pe", "pi", "po", "pu",
    "ma", "me", "mi", "mo", "mu",
    "ja", "je", "jo", "ju",
    "la", "le", "li", "lo", "lu",
    "wa", "we", "wi",
];
const KIPISI_NIMI_PONA_LON_MU_N: [&str; 36] = [
    "ka", "ke", "ki", "ko", "ku",
    "sa", "se", "si", "so", "su",
    "ta", "te", "to", "tu",
    "pa", "pe", "pi", "po", "pu",
    "ma", "me", "mi", "mo", "mu",
    "ja", "je", "jo", "ju",
    "la", "le", "li", "lo", "lu",
    "wa", "we", "wi",
];
const KIPISI_NIMI_PONA_LON_OPEN: [&str; 46] = [
    "a", "e", "i", "o", "u",
    "ka", "ke", "ki", "ko", "ku",
    "sa", "se", "si", "so", "su",
    "ta", "te", "to", "tu",
    "na", "ne", "ni", "no", "nu",
    "pa", "pe", "pi", "po", "pu",
    "ma", "me", "mi", "mo", "mu",
    "ja", "je", "jo", "ju",
    "la", "le", "li", "lo", "lu",
    "wa", "we", "wi",
];

fn pali_nimi(suli: usize, mute_pi_mu_n: f64, nimi_n_li_suli_ala_suli: SuliAlaSuli,
             rng: &mut (impl Rng + ?Sized)) -> String {
    let mut suli_pali = 0;
    let mut nimi = String::new();
    let mut mu_pini_li_mu_n = false;
    while suli_pali < suli {
        if suli_pali == 0 {
            nimi.push_str(KIPISI_NIMI_PONA_LON_OPEN.choose(rng).unwrap());
        } else if mu_pini_li_mu_n {
            nimi.push_str(KIPISI_NIMI_PONA_LON_MU_N.choose(rng).unwrap());
        } else {
            nimi.push_str(KIPISI_NIMI_PONA.choose(rng).unwrap());
        }
        suli_pali += 1;
        if suli_pali == suli {
            break;
        }
        if rng.gen_bool(mute_pi_mu_n) {
            nimi.push('n');
            mu_pini_li_mu_n = true;
            if let SuliAlaSuli::Suli = nimi_n_li_suli_ala_suli {
                suli_pali += 1;
            }
        }
    }
    nimi
}
