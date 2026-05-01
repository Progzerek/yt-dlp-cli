use std::process::Command;
use std::io;

fn main() {
    loop{
        let default_path = format!("{}\\Downloads", std::env::var("USERPROFILE").expect("Failed to find Current User"));
        let exe_dir = std::env::current_exe()
            .expect("Failed to get exe path")
            .parent()
            .expect("Failed to get exe directory")
            .to_path_buf();
        let yt_dlp = exe_dir.join("yt-dlp.exe");

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
                "n" => {Command::new(&yt_dlp).arg(&url).arg("--merge-output-format").arg("mkv").arg("-P").arg(&path).status().expect("Failed to download"); break },
                "y" => {Command::new(&yt_dlp).arg(&url).arg("-x").arg("--audio-format").arg("m4a").arg("-P").arg(&path).status().expect("Failed to download"); break},
                _ => println!("Try again")                 
            };
        };


        
    }
}
