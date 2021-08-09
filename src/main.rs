extern crate midir;

use std::error::Error;
use std::io::{stdin, stdout, Write};

use enigo::{Enigo, Key, KeyboardControllable};
use midir::{Ignore, MidiInput};

fn main() {
    match run() {
        Ok(_) => (),
        Err(err) => println!("Error: {}", err),
    }
}

fn run() -> Result<(), Box<dyn Error>> {
    let mut input = String::new();
    let mut enigo = Enigo::new();
    let mut midi_in = MidiInput::new("midir reading input")?;
    midi_in.ignore(Ignore::None);

    // Get an input port (read from console if multiple are available)
    let in_ports = midi_in.ports();
    let in_port = match in_ports.len() {
        0 => return Err("no input port found".into()),
        1 => {
            println!(
                "Choosing the only available input port: {}",
                midi_in.port_name(&in_ports[0]).unwrap()
            );
            &in_ports[0]
        }
        _ => {
            println!("\nAvailable input ports:");
            for (i, p) in in_ports.iter().enumerate() {
                println!("{}: {}", i, midi_in.port_name(p).unwrap());
            }
            print!("Please select input port: ");
            stdout().flush()?;
            let mut input = String::new();
            stdin().read_line(&mut input)?;
            in_ports
                .get(input.trim().parse::<usize>()?)
                .ok_or("invalid input port selected")?
        }
    };
    let mut format: bool = false;
    let mut build: bool = false;
    let mut jumptodef: bool = false;
    let mut showdef: bool = false;
    let mut enter: bool = false;
    println!("\nOpening connection");
    let in_port_name = midi_in.port_name(in_port)?;

    let mut fmt: u8 = 0;
    let mut bld: u8 = 0;
    let mut jtd: u8 = 0;
    let mut sdef: u8 = 0;
    let mut enter_value: u8 = 0;

    // _conn_in needs to be a named parameter, because it needs to be kept alive until the end of the scope
    let _conn_in = midi_in.connect(
        in_port,
        "midir-read-input",
        move |stamp, message, _| {
            if message[0] == 128 {
                return;
            }
            if format == false {
                println!("{}: {:?} (len inside = {})", stamp, message, message.len());
                fmt = message[1];
                println!("Key For Format {}", fmt);
                format = true;
                println!("Press Key For Build");
                return;
            }

            if build == false {
                println!(
                    "{}: {:?} (len inside iff = {})",
                    stamp,
                    message,
                    message.len()
                );
                bld = message[1];
                println!("Key For Build {}", bld);
                build = true;
                println!("Press Key For Opening Definition");
                return;
            }

            if jumptodef == false {
                println!("{}: {:?} (len inside = {})", stamp, message, message.len());
                jtd = message[1];
                println!("Key For Opening Definition {}", jtd);
                jumptodef = true;
                println!("Press Key For Inline Def");
                return;
            }
            if showdef == false {
                println!("{}: {:?} (len inside= {})", stamp, message, message.len());
                sdef = message[1];
                println!("Key For Inline Def {}", sdef);
                showdef = true;
                println!("Press Key For Enter");
                return;
            }
            if enter == false {
                println!("{}: {:?} (len = {})", stamp, message, message.len());
                enter_value = message[1];
                println!("Key For Enter {}", enter_value);
                enter = true;
                return;
            }
            // println!("{}: {:?} (len = {})", stamp, message, message.len());
            if message[1] == fmt {
                // println!("Press Key For Format {}", fmt);
                enigo.key_sequence_parse("{+CTRL}{+SHIFT}i{-CTRL}{-SHIFT}");
            }
            if message[1] == bld {
                enigo.key_sequence_parse("{+CTRL}{+SHIFT}b{-CTRL}{-SHIFT}");
            }
            if message[1] == jtd {
                enigo.key_click(Key::F12);
            }
            if message[1] == sdef {
                enigo.key_sequence_parse("{+CTRL}{+SHIFT}");
                enigo.key_click(Key::F10);
                enigo.key_sequence_parse("{-CTRL}{-SHIFT}");
            }
            if message[1] == enter_value {
                enigo.key_click(Key::Return);
            }
        },
        (),
    )?;

    println!(
        "Connection open, reading input from '{}' (press enter to exit) ...",
        in_port_name
    );
    println!("Press Key For Format");

    loop {
        input.clear();
        stdin().read_line(&mut input)?; // wait for next enter key press
        if input == "Q\n" {
            println!("Closing connection: {}", input);
            break;
        }
    }
    Ok(())
}
