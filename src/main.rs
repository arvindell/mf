use clap::Parser;
use rand::seq::SliceRandom;
use std::process::{Command, ExitStatus};

/// A spicy little wrapper that yells at you depending on command success.
#[derive(Parser)]
struct Cli {
    /// The shell command to run
    #[arg(required = true)]
    command: Vec<String>,
}

fn main() {
    let args = Cli::parse();
    let cmd_str = args.command.join(" ");

    // Run the command using the shell
    let status: ExitStatus = if cfg!(windows) {
        Command::new("cmd")
            .args(&["/C", &cmd_str])
            .status()
            .expect("Failed to run command")
    } else {
        Command::new("sh")
            .arg("-c")
            .arg(&cmd_str)
            .status()
            .expect("Failed to run command")
    };

    if status.success() {
        speak_success();
    } else {
        speak_failure();
    }
}

/// Say a random success phrase using text-to-speech
fn speak_success() {
    let success_messages = [
        "Hell yeah, it fucking worked!",
        "Holy shit, that actually worked!",
        "Fuck yes! Command executed successfully.",
        "Goddamn right it worked!",
        "Success, you magnificent bastard!",
        "That shit just worked like a charm!",
        "Nailed that shit!",
        "Fuck yeah, task complete!",
        "That's what I'm talking about, you badass!",
        "Success! You're not as dumb as you look!",
    ];
    speak(success_messages.choose(&mut rand::thread_rng()).unwrap())
}

/// Say a random failure phrase using text-to-speech
fn speak_failure() {
    let failure_messages = [
        "That shit broke!",
        "Well, you fucked that up!",
        "Command failed, dumbass!",
        "What the hell did you do wrong?",
        "Shit's broken as fuck!",
        "That command crashed harder than my ex's car!",
        "Goddamn it! It failed!",
        "Another fucking failure. Surprise, surprise.",
        "Error! Did you even try?",
        "Holy shit, that went sideways fast!",
    ];
    speak(failure_messages.choose(&mut rand::thread_rng()).unwrap())
}

/// Say a phrase using platform-specific text-to-speech
fn speak(phrase: &str) {
    println!("{}", phrase); // Always print the message as fallback

    #[cfg(target_os = "macos")]
    {
        let _ = Command::new("say")
            .arg(phrase)
            .status();
    }

    #[cfg(target_os = "linux")]
    {
        // Try espeak first, fall back to espeak-ng if available
        let espeak_result = Command::new("espeak")
            .arg(phrase)
            .status();
            
        if espeak_result.is_err() {
            let _ = Command::new("espeak-ng")
                .arg(phrase)
                .status();
        }
    }

    #[cfg(target_os = "windows")]
    {
        // Use PowerShell to speak the text
        let ps_script = format!(
            "Add-Type -AssemblyName System.Speech; \
             $synth = New-Object -TypeName System.Speech.Synthesis.SpeechSynthesizer; \
             $synth.Speak('{}');",
            // Replace single quotes with escaped single quotes for PowerShell
            phrase.replace("'", "''")
        );
        
        let _ = Command::new("powershell")
            .arg("-Command")
            .arg(&ps_script)
            .status();
    }
}