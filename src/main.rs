use std::process::Command;
use std::io;

fn main() {
    loop{
        let default_path = format!("{}\\Downloads", std::env::var("USERPROFILE").expect("Failed to find Current User"));
        println!("||Youtube downloader||");
        println!("Youtube link (type exit if want to leave): ");
        let mut url = String::new();
        io::stdin().read_line(&mut url).expect("Failed to get link");
        let url = url.trim();
        if &url.to_lowercase() == "exit" {break;};

        //path ask
        println!("Path to download (press enter for default):");
        let mut path = String::new();
        io::stdin().read_line(&mut path).expect("Failed to read line");
        let mut path = path.trim();
        if path == ""{
            path = &default_path;
        }

        //audio only question
        loop {
            println!("Audio only? (y/n)");
            let mut audio_only = String::new();
            io::stdin().read_line(&mut audio_only).expect("Failed to read line");
            let audio_only = audio_only.trim();
            match audio_only {
                "n" => {Command::new("./yt-dlp.exe").arg(&url).arg("-P").arg(&path).status().expect("Failed to download"); break },
                "y" => {Command::new("./yt-dlp.exe").arg(&url).arg("-x").arg("--audio-format").arg("m4a").arg("-P").arg(&path).status().expect("Failed to download"); break},
                _ => println!("Try again")                 
            };
        };


        
    }
}
