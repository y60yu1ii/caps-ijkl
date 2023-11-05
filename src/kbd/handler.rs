use std::ffi::CString;

use libc::{c_void, input_event, ioctl, open, read, O_RDONLY};

use super::event_codes::*;

const EVIOCGRAB: u64 = 1074021776;

pub struct KeyboardHandler {
    fd: i32,
    uinput: uinput::Device,
    is_grabbed: bool,
    debug: bool,
    device_path: String,
}

impl KeyboardHandler {
    pub fn new(device_path: &str, debug: bool) -> KeyboardHandler {
        unsafe {
            let c_str = CString::new(device_path).unwrap();
            let fd = open(c_str.as_ptr(), O_RDONLY);
            if fd == -1 {
                panic!("Cannot open input device: {}", device_path);
            }

            KeyboardHandler {
                device_path: device_path.to_string(),
                is_grabbed: false,
                uinput: uinput::open("/dev/uinput")
                    .unwrap()
                    .name(format!("C-HJKL Output for {}", device_path))
                    .unwrap()
                    .event(uinput::event::Keyboard::All)
                    .unwrap()
                    .create()
                    .unwrap(),
                debug,
                fd,
            }
        }
    }

    fn grab(&mut self) {
        unsafe {
            if !self.is_grabbed && ioctl(self.fd, EVIOCGRAB, 1) != -1 {
                self.is_grabbed = true;
            }
        }
    }

    #[allow(dead_code)]
    fn ungrab(&mut self) {
        unsafe {
            ioctl(self.fd, EVIOCGRAB, 0);
            self.is_grabbed = false;
        }
    }

    fn read(&self) -> input_event {
        unsafe {
            let mut ev: input_event = std::mem::zeroed();
            if read(
                self.fd,
                &mut ev as *mut _ as *mut c_void,
                std::mem::size_of::<input_event>(),
            ) != (std::mem::size_of::<input_event>() as _)
            {
                panic!("Read a partial event");
            }
            ev.clone()
        }
    }

    fn write(&mut self, ev: &input_event) {
        self.uinput
            .write(ev.type_ as _, ev.code as _, ev.value)
            .unwrap();
    }

    pub fn run_forever(&mut self) {
        let mut caps = false;

        std::thread::sleep(std::time::Duration::from_secs(1));

        self.grab();

        let mut caps_keys = Vec::new();
        let mut other_keys = Vec::new();

        loop {
            let mut input = self.read();

            if self.debug {
                println!(
                    "[{}] caps: {}, ev: {} {} {}",
                    self.device_path, caps, input.type_, input.code, input.value
                );
            }

            if input.code == KEY_CAPSLOCK {
                caps = input.value != 0;

                if input.value == 0 {
                    for x in caps_keys.drain(..) {
                        self.write(&input_event {
                            time: input.time,
                            type_: 1,
                            code: x,
                            value: 0,
                        });
                    }
                }

                continue;
            }

            if caps && !other_keys.contains(&input.code) {
                let key_to_press = match input.code {
                    KEY_K => Some(KEY_UP),
                    KEY_H => Some(KEY_LEFT),
                    KEY_J => Some(KEY_DOWN),
                    KEY_L => Some(KEY_RIGHT),
                    KEY_U => Some(KEY_HOME),
                    KEY_O => Some(KEY_END),
                    KEY_P => Some(KEY_PAGEUP),
                    KEY_SEMICOLON => Some(KEY_PAGEDOWN),
                    KEY_BACKSPACE => Some(KEY_DELETE),
                    KEY_1 => Some(KEY_F1),
                    KEY_2 => Some(KEY_F2),
                    KEY_3 => Some(KEY_F3),
                    KEY_4 => Some(KEY_F4),
                    KEY_5 => Some(KEY_F5),
                    KEY_6 => Some(KEY_F6),
                    KEY_7 => Some(KEY_F7),
                    KEY_8 => Some(KEY_F8),
                    KEY_9 => Some(KEY_F9),
                    KEY_0 => Some(KEY_F10),
                    KEY_MINUS => Some(KEY_F11),
                    KEY_EQUAL => Some(KEY_F12),
                    _ => None,
                };
                // println!("{:?} => {:?}", input.code, key_to_press);

                if let Some(key_to_press) = key_to_press {
                    add_or_remove_key(&mut caps_keys, input.value, key_to_press);

                    input.code = key_to_press;
                    self.write(&input);
                    continue;
                }
            }

            // Pass-through
            add_or_remove_key(&mut other_keys, input.value, input.code);
            self.write(&input);
        }
    }
}

fn add_or_remove_key(keys: &mut Vec<u16>, value: i32, code: u16) {
    if value != 0 {
        keys.push(code);
    } else {
        keys.retain(|x| *x != code);
    }
}
