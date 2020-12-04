use std::str::FromStr;
use structopt::StructOpt;
use rppal::gpio::{Gpio, InputPin};
use anyhow::anyhow;

#[derive(StructOpt)]
struct Cli {
    #[structopt(subcommand)]
    subcommand: SubCommand
}

enum Value {
    High,
    Low
}

impl FromStr for Value {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> anyhow::Result<Self> {
        match s.trim().to_lowercase().as_str() {
            "0" | "low" | "l" => Ok(Value::Low),
            "1" | "high" | "hi" | "h" => Ok(Value::High),
            _ => Err(anyhow!("Invalid value for on off value"))
        }
    }
}

enum Type {
    Output,
    Input
}

impl FromStr for Type {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> anyhow::Result<Self> {
        match s.trim().to_lowercase().as_str() {
            "out" | "output" | "o" => Ok(Type::Output),
            "in" | "input" | "i" => Ok(Type::Input),
            _ => Err(anyhow!("Invalid value for type output input"))
        }
    }
}

enum Pull {
    Up,
    Down
}

impl FromStr for Pull {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> anyhow::Result<Self> {
        match s.trim().to_lowercase().as_str() {
            "0" | "down" | "d" => Ok(Pull::Down),
            "1" | "up" | "u" => Ok(Pull::Up),
            _ => Err(anyhow!("Invalid value for pull up down"))
        }
    }
}

#[derive(StructOpt)]
enum SubCommand {
    Setup {
        #[structopt(
            help="output",
        )]
        output_type: Type,
        pins: Vec<u8>,
        #[structopt(short, long)]
        reset_on_drop: bool,
        #[structopt(short, long, required_if("output_type", "input"))]
        pull: Option<Pull>
    },
    Test {
        value: Value,
        #[structopt(
            help="pins you want to test BCM",
        )]
        pins: Vec<u8>,
        #[structopt(short, long)]
        reset_on_drop: bool
    }
}

fn main() -> anyhow::Result<()> {
    let cli = Cli::from_args();

    let gpio = Gpio::new()?;
    match cli.subcommand {
        SubCommand::Test {
            pins,
            value,
            reset_on_drop
        } => {
            for pin_num in pins {
                let pin = gpio.get(pin_num)?;
                let mut output_pin = pin.into_output();
                output_pin.set_reset_on_drop(reset_on_drop);
                match value {
                    Value::High => output_pin.set_high(),
                    Value::Low => output_pin.set_low()
                };
            }
        },
        SubCommand::Setup {
            output_type,
            pins,
            reset_on_drop,
            pull
        } => {
            match output_type {
                Type::Output => {
                    for pin_num in pins {
                        let pin = gpio.get(pin_num)?;
                        let mut output_pin = pin.into_output();
                        output_pin.set_reset_on_drop(reset_on_drop);
                    }
                },
                Type::Input => {
                    for pin_num in pins {
                        let pin = gpio.get(pin_num)?;
                        let mut input_pin: InputPin = match pull {
                            Some(ref p) => {
                                match p {
                                    Pull::Down => {
                                        pin.into_input_pulldown()
                                    },
                                    Pull::Up => {
                                        pin.into_input_pullup()
                                    }
                                }
                            }
                            None => {
                                pin.into_input()
                            }
                        };
                        input_pin.set_reset_on_drop(reset_on_drop);
                    }
                }
            }
        },
        _ => return Err(anyhow!("Invalid subcommand type"))
    };
    Ok(())
}
