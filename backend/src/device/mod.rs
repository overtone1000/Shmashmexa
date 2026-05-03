use std::error::Error;

pub(crate) fn set_screen_state(desired_state:bool, kiosk_uid:&u64)->Result<(),Box<dyn Error>>
{
    use std::process::Command;

    let arg = match desired_state
    {
        true=>"--on",
        false=>"--off"
    };

    let result = Command::new("sh")
        .env("XDG_RUNTIME_DIR", "/run/user/".to_owned()+&kiosk_uid.to_string())
        .env("WAYLAND_DISPLAY", "wayland-0")
        .arg("-c")
        .arg("wlr-randr --output HDMI-A-1")
        .arg(arg)
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