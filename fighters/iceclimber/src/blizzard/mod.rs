use super::*;

mod acmd;

pub fn install() {
    let popo = &mut Agent::new("popo_blizzard");
    let nana = &mut Agent::new("nana_blizzard");

    acmd::install(popo);
    acmd::install(nana);

    popo.install();
    nana.install();
}