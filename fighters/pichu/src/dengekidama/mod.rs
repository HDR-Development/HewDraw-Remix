use super::*;

mod acmd;

pub fn install() {
    let agent = &mut Agent::new("pichu_dengekidama");
    acmd::install(agent);
    agent.install();
}