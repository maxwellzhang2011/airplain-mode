use std::process::Command;
use std::env;

fn set_airplane_mode(enabled: bool) {
    let radio_state = if enabled { "off" } else { "on" };

    Command::new("nmcli")
        .args(["radio", "all", radio_state])
        .status()
        .expect("Failed to change radio state");
}

fn set_bluetooth(block: bool) {
    let action = if block { "block" } else { "unblock" };

    Command::new("rfkill")
        .args([action, "bluetooth"])
        .status()
        .expect("Failed to change Bluetooth state");
}

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();

    if args.len() == 0{
        let output = Command::new("nmcli")
        .args(["radio", "wifi"])
        .output()
        .unwrap();

        let onoff = String::from_utf8_lossy(&output.stdout)
            .trim() == "disabled";

        println!("airplain mode {}", match onoff{
            true => "on",
            false => "off",
        });
    }else if args[0] == "on" {
        set_airplane_mode(true);
        set_bluetooth(true);
    }else if args[0] == "off" {
        set_airplane_mode(false);
        set_bluetooth(false);
    }else if args[0] == "help"{
        println!("
airplain off  -- set it off
airplain on   -- set it on
airplain      -- list of its on or off
airplain help -- list all the command
            ");
    }else{
        println!("unknown command!");
    }

}
