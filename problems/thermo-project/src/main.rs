#[allow(unused)]
fn main() {
    let t1_temp = 288.0;

    let pressure_ratio = 4.0;

    let gamma1 = get_gamma(t1_temp);

    let t2_temp = get_t2_temp(t1_temp, gamma1, pressure_ratio);

    let isen_enth_t2 = 288.15 + (429.3 - 288.15) * 0.81;

    let t1_entropy = 1.662;

    let t2_entropy = 2.061;

    let ideal_cell_voltage = 0.94;

    let cell_voltage = 0.75;

    let fuel_cell_dc_pow = cell_voltage * 0.3 * 834.0;

    let entropy_gen_fc = 0.3 * 834.0 * (0.94 - cell_voltage) * (10u32 as f64).powi(-6);

    let air_usage = 3.57 * (10u32 as f64).powi(-7) * 2.5 * (190.0 / cell_voltage);

    let turbine_efficiency = 0.84;

    let h5 = 1336.55;

    let h6 = 1198.0;

    let h6s = h5 - (h5 - h6) / 0.84;

    let s5 = 3.227;

    let s6 = 3.111;

    let h7 = 641.8;

    let h7s = h6 - (h6 - h7) / 0.89;

    let s7 = 2.461;

    let m1_rate = 14842.8 / 3600.00;

    let mfuel_fc_rate = 0.062;

    let mfuel_comb_rate = 0.01731;

    let qcomb = (mfuel_fc_rate * (1.0 - 0.85) + mfuel_comb_rate) * 50050.0;

    let qloss = (mfuel_fc_rate * (1.0 - 0.85) + mfuel_comb_rate) * (1.0 - 0.98) * 50050.0;

    let t3 = 0.8 * (640.0 - t2_temp) + t2_temp;

    let h8 = h7 - ((m1_rate * (605.0 - 429.3)) / 4.2029);

    let wpt = 4.20291 * (h6 - h7);

    let qtotal = 0.06258 * 0.85 * 50050.0 + qcomb;

    println!("Qtotal: {}", qtotal);

    let wnet = 0.89 * fuel_cell_dc_pow + 0.95 * wpt;

    println!("Wnet: {}", wnet);

    let final_eff = wnet / qtotal;

    println!("Final efficiency: {}", final_eff);
}

fn get_gamma(t1_temp: f64) -> f64 {
    let a = 5.0 * (10u32 as f64).powi(-5);

    let b = 3.0 * (10u32 as f64).powi(-8);

    let gamma = 1.4197 - a * t1_temp - b * t1_temp.powi(2);

    gamma
}

fn get_t2_temp(t1_temp: f64, gamma1: f64, pressure_ratio: f64) -> f64 {
    let t2_temp = t1_temp * (pressure_ratio).powf((gamma1 - 1.0) / gamma1);
    t2_temp
}
