use swaybar_server_logic::{i3BarJsonObject,pango_markup,i3Array};
fn main() {
    swaybar_server_logic::init();
    loop {
        let i3block = battery_indicator("#101010");
        i3Array::new().push(i3block).send().expect("This should never crash, if you see this what the fuck.");
        std::thread::sleep(std::time::Duration::from_secs(1));
    }
}

fn battery_indicator (color :&str) -> i3BarJsonObject {
    let batterylevel = std::fs::read_to_string("/sys/class/power_supply/BAT0/charge_now").unwrap();
    let batterylevel :u32= batterylevel.trim().parse().unwrap();
    let max_battery = std::fs::read_to_string("/sys/class/power_supply/BAT0/charge_full").unwrap();
    let max_battery :u32= max_battery.trim().parse().unwrap();
    let batterypercentage :f64= (batterylevel as f64/max_battery as f64 * 100 as f64).round();
    let mut i3block = swaybar_server_logic::i3BarJsonObject::default();
    i3block.markup = Some("pango".into());
    i3block.color =Some(color.into());
    i3block.border_right = Some(15);
    i3block.border = Some("#424242".into());
    i3block.full_text=pango_markup::background_color("#1ed2f4",&format!("Battery: {}%",batterypercentage)).into();
    i3block
}

