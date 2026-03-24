use std::process::Command;

#[derive(serde::Serialize, serde::Deserialize, Clone)]
pub struct MediaInfo {
    pub title: String,
    pub artist: String,
    pub album: String,
    pub playing: bool,
}

#[derive(serde::Serialize, serde::Deserialize, Clone)]
pub struct AudioStatus {
    pub volume: u32,
    pub muted: bool,
    pub media: Option<MediaInfo>,
}


pub fn get_audio_status() -> AudioStatus {
    #[cfg(target_os = "macos")]
    {
        let vol_out = Command::new("osascript")
            .arg("-e")
            .arg("output volume of (get volume settings)")
            .output();
        let mute_out = Command::new("osascript")
            .arg("-e")
            .arg("output muted of (get volume settings)")
            .output();

        let volume = if let Ok(out) = vol_out {
            String::from_utf8_lossy(&out.stdout).trim().parse::<u32>().unwrap_or(0)
        } else { 0 };

        let muted = if let Ok(out) = mute_out {
            String::from_utf8_lossy(&out.stdout).trim() == "true"
        } else { false };

        let media = get_media_info();

        AudioStatus { volume, muted, media }
    }


    #[cfg(target_os = "windows")]
    {
        // Use PowerShell to get volume (0.0 to 1.0) and mute status
        let script = "(Get-AudioDevice -Playback).Volume; (Get-AudioDevice -Playback).Muted";
        // Note: This requires the AudioDeviceCmdlets module usually, 
        // fallback to a generic WMI approach if not available.
        // For simplicity, let's try a standard PS core command if possible or just return a mock if it fails.
        let out = Command::new("powershell")
            .arg("-Command")
            .arg("$vol = (Get-WmiObject -Query 'Select * from Win32_SoundDevice').Volume; $vol")
            .output();
        
        // This is complex on Windows without a crate.
        // Let's use a simpler approach: 
        // (New-Object -ComObject WScript.Shell).SendKeys([char]174) // Vol Down
        // For now, return a placeholder or try to find a better PS script.
        let media = get_media_info();
        AudioStatus { volume: 50, muted: false, media }
    }


    #[cfg(not(any(target_os = "macos", target_os = "windows")))]
    {
        AudioStatus { volume: 0, muted: true }
    }
}

pub fn set_mute(mute: bool) {
    #[cfg(target_os = "macos")]
    {
        let val = if mute { "with" } else { "without" };
        let _ = Command::new("osascript")
            .arg("-e")
            .arg(format!("set volume {} output muted", val))
            .output();
    }

    #[cfg(target_os = "windows")]
    {
        let val = if mute { "$true" } else { "$false" };
        // This is just a placeholder, real Windows audio toggle is hard via CMD.
        // One could use `nircmd.exe` if available.
    }
}

pub fn set_volume(volume: u32) {
    #[cfg(target_os = "macos")]
    {
        let _ = Command::new("osascript")
            .arg("-e")
            .arg(format!("set volume output volume {}", volume))
            .output();
    }
}

pub fn get_media_info() -> Option<MediaInfo> {
    #[cfg(target_os = "macos")]
    {
        // Try Music.app
        let music_script = "if application \"Music\" is running then tell application \"Music\" to get {name, artist, album, player state} as string";
        if let Ok(o) = Command::new("osascript").arg("-e").arg(music_script).output() {
            let s = String::from_utf8_lossy(&o.stdout).trim().to_string();
            if !s.is_empty() {
                let parts: Vec<&str> = s.split(", ").collect();
                if parts.len() >= 4 {
                    return Some(MediaInfo {
                        title: parts[0].to_string(),
                        artist: parts[1].to_string(),
                        album: parts[2].to_string(),
                        playing: parts[3] == "playing",
                    });
                }
            }
        }
        
        // Try Spotify
        let spotify_script = "if application \"Spotify\" is running then tell application \"Spotify\" to get {name, artist, album, player state} as string";
        if let Ok(o) = Command::new("osascript").arg("-e").arg(spotify_script).output() {
            let s = String::from_utf8_lossy(&o.stdout).trim().to_string();
            if !s.is_empty() {
                let parts: Vec<&str> = s.split(", ").collect();
                if parts.len() >= 4 {
                    return Some(MediaInfo {
                        title: parts[0].to_string(),
                        artist: parts[1].to_string(),
                        album: parts[2].to_string(),
                        playing: parts[3] == "playing",
                    });
                }
            }
        }
        None
    }

    #[cfg(target_os = "windows")]
    {
        // PowerShell script to get GSMTC info
        let script = r#"
            $m = [Windows.Media.Control.GlobalSystemMediaTransportControlsSessionManager]::RequestAsync().GetResults()
            $s = $m.GetCurrentSession()
            if ($s) {
                $p = $s.TryGetMediaPropertiesAsync().GetResults()
                $st = $s.GetPlaybackInfo().PlaybackStatus
                "$($p.Title)|$($p.Artist)|$($p.AlbumTitle)|$($st)"
            }
        "#;
        if let Ok(o) = Command::new("powershell").arg("-Command").arg(script).output() {
            let s = String::from_utf8_lossy(&o.stdout).trim().to_string();
            if !s.is_empty() {
                let parts: Vec<&str> = s.split('|').collect();
                if parts.len() >= 4 {
                    return Some(MediaInfo {
                        title: parts[0].to_string(),
                        artist: parts[1].to_string(),
                        album: parts[2].to_string(),
                        playing: parts[3] == "Playing",
                    });
                }
            }
        }
        None
    }

    #[cfg(not(any(target_os = "macos", target_os = "windows")))]
    { None }
}
