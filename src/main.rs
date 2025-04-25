use clap::Parser;
use rand::seq::SliceRandom;
use std::env;
use std::path::PathBuf;
use std::process::{Command, ExitStatus};

/// A spicy little wrapper that yells at you depending on command success.
#[derive(Parser)]
struct Cli {
    /// The shell command to run
    #[arg(required = false)]
    command: Vec<String>,
}

fn main() {
    let args = Cli::parse();
    
    if args.command.is_empty() {
        // No command provided, just announce completion
        speak_completion();
    } else {
        // Command provided, run it and report status
        let cmd_str = args.command.join(" ");

        // Run the command using the shell
        let status: ExitStatus = if cfg!(windows) {
            Command::new("cmd")
                .args(&["/C", &cmd_str])
                .status()
                .expect("Failed to run command")
        } else {
            execute_with_shell(&cmd_str)
        };

        if status.success() {
            speak_success();
        } else {
            speak_failure();
        }
    }
}

/// Execute a command string with the user's shell, preserving aliases
fn execute_with_shell(cmd_str: &str) -> ExitStatus {
    // Get user's shell
    let shell = env::var("SHELL").unwrap_or_else(|_| String::from("sh"));
    let shell_path = PathBuf::from(&shell);
    let shell_name = shell_path.file_name()
        .and_then(|name| name.to_str())
        .unwrap_or("sh");
    
    // Use different strategies depending on the shell
    match shell_name {
        "zsh" => {
            // For zsh, create a special command that forces alias expansion
            // Using -i for interactive mode and wrapping command with 'eval'
            let eval_cmd = format!("eval '{}'", cmd_str.replace("'", "'\\''"));
            Command::new(&shell)
                .args(&["-i", "-c", &eval_cmd])
                .status()
                .expect("Failed to run command")
        },
        "bash" => {
            // For bash, source profile and then execute
            let home = env::var("HOME").unwrap_or_else(|_| String::from(""));
            let exec_cmd = format!("source {0}/.bashrc 2>/dev/null || source {0}/.bash_profile 2>/dev/null; {1}", 
                                  home, cmd_str);
            Command::new(&shell)
                .args(&["-c", &exec_cmd])
                .status()
                .expect("Failed to run command")
        },
        "fish" => {
            // For fish shell
            let home = env::var("HOME").unwrap_or_else(|_| String::from(""));
            let exec_cmd = format!("source {0}/.config/fish/config.fish 2>/dev/null; {1}", home, cmd_str);
            Command::new(&shell)
                .args(&["-c", &exec_cmd])
                .status()
                .expect("Failed to run command")
        },
        _ => {
            // For unknown shells, just run the command directly
            Command::new(&shell)
                .args(&["-c", cmd_str])
                .status()
                .expect("Failed to run command")
        }
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

/// Say a random completion phrase using text-to-speech
fn speak_completion() {
    let completion_messages = [
        "Alright, that command finished.",
        "Okay, the damn thing is done running.",
        "Finished! Fucking finally",
        "That shit's complete. What next now?",
        "Command execution wrapped up.",
        "All done with that crap.",
        "It finished, for fuck's sake.",
        "Task complete. Happy now?",
        "Finally! That command is done.",
        "The process ended. Took its fucking time.",
    ];
    speak(completion_messages.choose(&mut rand::thread_rng()).unwrap())
}

/// Say a phrase using platform-specific text-to-speech
fn speak(phrase: &str) {
    #[cfg(target_os = "macos")]
    {
        let _ = Command::new("say").arg(phrase).status();
    }

    #[cfg(target_os = "linux")]
    {
        // Try espeak first, fall back to espeak-ng if available
        let espeak_result = Command::new("espeak").arg(phrase).status();

        if espeak_result.is_err() {
            let _ = Command::new("espeak-ng").arg(phrase).status();
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
