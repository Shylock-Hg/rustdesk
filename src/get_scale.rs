#[cfg(target_os = "linux")]
use gdk4;
#[cfg(target_os = "linux")]
use gtk4;
#[cfg(target_os = "linux")]
use gdk4::prelude::{MonitorExt, DisplayExt, ListModelExt, Cast};
use display_info::DisplayInfo;

fn main() {
  let display_infos = DisplayInfo::all().unwrap();
  let display_info = display_infos[0].clone();
  let scale = display_info.scale_factor;
  print!("{}", scale)
}
