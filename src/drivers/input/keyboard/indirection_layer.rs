extern crate keycodes_ascii; // our own crate in "libs/" dir


use keycodes_ascii::{Keycode, KeyboardModifiers, KEY_RELEASED_OFFSET};
use collections::VecDeque;
use core::default::Default;



// TODO: avoid unsafe static mut using the following: https://www.reddit.com/r/rust/comments/1wvxcn/lazily_initialized_statics/cf61im5/


// TODO: use a lock-free queue (a la Michael Scott): https://aturon.github.io/blog/2015/08/27/epoch/
//                                                   https://blog.rust-lang.org/2015/04/10/Fearless-Concurrency.html


static mut KBD_MODIFIERS: Option<KeyboardModifiers> = None;


pub fn init() { 
    assert_has_not_been_called!("keyboard init was called more than once!");
    
    unsafe {
        KBD_MODIFIERS = Some(KeyboardModifiers::default());
    }

}


#[derive(Debug, Copy, Clone, PartialEq)]
pub enum KeyAction {
    Pressed,
    Released,
}

/// the KeyEvent that should be delivered to applications upon a keyboard action
#[derive(Debug, Copy, Clone)]
pub struct KeyEvent {
    pub keycode: Keycode,
    pub action: KeyAction,
    pub modifiers: KeyboardModifiers,
}

impl KeyEvent {
    pub fn new(keycode: Keycode, action: KeyAction, modifiers: KeyboardModifiers,) -> KeyEvent {
        KeyEvent {
            keycode, 
            action,
            modifiers,
        }
    }
}


#[derive(Debug)]
pub enum KeyboardInputError {
    UnknownScancode,
}




/// returns Ok(()) if everything was handled properly.
/// returns KeyboardInputError 
pub fn handle_keyboard_input(scan_code: u8) -> Result<(), KeyboardInputError> {
    // let kbd_state = KBD_STATE.try_read();
    // if kbd_state.is_none() {
    //     println!("Error: KBD_STATE.try_read() failed, discarding {}!", scan_code);
    //     return Err(KeyboardInputError::TryAcquireFailed);
    // }
    // let kbd_state = kbd_state.unwrap(); // safe, cuz we already checked for is_none()
    let mut modifiers = unsafe { KBD_MODIFIERS.as_mut().expect("Error: KBD_MODIFIERS was uninitialized") };
   
    // first, update the modifier keys
    match scan_code {
        x if x == Keycode::Control as u8 => { modifiers.control = true }
        x if x == Keycode::Alt     as u8 => { modifiers.alt = true }
        x if x == (Keycode::LeftShift as u8) || x == (Keycode::RightShift as u8) => { modifiers.shift = true }

        // trigger caps lock on press only
        x if x == Keycode::CapsLock as u8 => { modifiers.caps_lock ^= true }

        x if x == Keycode::Control as u8 + KEY_RELEASED_OFFSET => { modifiers.control = false }
        x if x == Keycode::Alt     as u8 + KEY_RELEASED_OFFSET => { modifiers.alt = false }
        x if x == ((Keycode::LeftShift as u8) + KEY_RELEASED_OFFSET) || x == ((Keycode::RightShift as u8) + KEY_RELEASED_OFFSET) => { modifiers.shift = false }

        _ => { } // do nothing
    }


    // second,  put the keycode and it's action (pressed or released) in the keyboard queue
    match scan_code {
        x => { 
            let (adjusted_scan_code, action) = 
                if x < KEY_RELEASED_OFFSET { 
                    (scan_code, KeyAction::Pressed) 
                } else { 
                    (scan_code - KEY_RELEASED_OFFSET, KeyAction::Released) 
                };

            let keycode = Keycode::from_scancode(adjusted_scan_code); 
            match keycode {
                Some(keycode) => { // this re-scopes (shadows) keycode
                    use console;
                    use console::{ConsoleEvent, ConsoleInputEvent};
                    console::queue_event(
                        ConsoleEvent::InputEvent(
                            ConsoleInputEvent::new(KeyEvent::new(keycode, action, modifiers.clone()))
                        )
                    );
                    return Ok(());  // successfully queued up KeyEvent 
                }

                _ => { 
                    warn!("Unknown keycode: {:?}", keycode);
                    return Err(KeyboardInputError::UnknownScancode); 
                }
            }
        }
    }

}
