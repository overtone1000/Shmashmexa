use std::error::Error;

pub(crate) fn set_screen_state(screen_on:bool, kiosk_uid:&u64)->Result<(),Box<dyn Error>>
{
    use std::process::Command;

    let arg = match screen_on
    {
        true=>"--on",
        false=>"--off"
    };

    println!("{:?}",std::env::var("PATH"));

    //Have to add path to wlr-randr as packaged rust application path is limited
    let path = "/run/current-system/sw/bin/";

    //run directory for kiosk user, which is the user for the cage session
    let xdg_runtime_dir = "/run/user/".to_owned()+&kiosk_uid.to_string();

    //command to control display
    let command = "wlr-randr --output HDMI-A-1 ".to_owned() + arg;

    let result = Command::new("sh")
        .env("XDG_RUNTIME_DIR", xdg_runtime_dir)
        .env("WAYLAND_DISPLAY", "wayland-0")
        .env("PATH",path)
        .arg("-c")
        .arg(command)
        .output();

    match result
    {
        Ok(res)=>{
            println!("{:?}",res);
            Ok(())
        }
        Err(e)=>{
            println!("{:?}",e);
            Err(Box::new(e))
        }
    }
}