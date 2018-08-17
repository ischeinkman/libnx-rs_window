


use input::{HatState, ControllerButton, ControllerHat, Event, Input, ButtonArgs, Button, ButtonState};
use libnx_rs::libnx::{HidControllerKeys};

pub struct LibnxButtonId {}

/// Constants for possible values of the event 
impl LibnxButtonId {
    pub const A : u8 = 1; 
    pub const B : u8 = 2; 
    pub const X : u8 = 3; 
    pub const Y : u8 = 4; 

    pub const LSTICK : u8 = 5; 
    pub const RSTICK : u8 = 6; 

    pub const L : u8 = 7; 
    pub const R : u8 = 8; 
    pub const ZL : u8 = 9; 
    pub const ZR : u8 = 10; 

    pub const PLUS : u8 = 11;
    pub const MINUS : u8 = 12; 

    pub const DLEFT : u8 = 13; 
    pub const DUP : u8 = 14; 
    pub const DRIGHT : u8 = 15;
    pub const DDOWN : u8 = 16; 

    /// TODO: Right now the sticks are treated as just more d-pads. 
    pub const LSTICK_LEFT : u8 = 17;
    pub const LSTICK_UP : u8 = 18; 
    pub const LSTICK_RIGHT : u8 = 19;
    pub const LSTICK_DOWN : u8 = 20;
    
    pub const RSTICK_LEFT : u8 = 21;
    pub const RSTICK_UP : u8 = 22; 
    pub const RSTICK_RIGHT : u8 = 23;
    pub const RSTICK_DOWN : u8 = 24;

    pub const SL : u8 = 25;
    pub const SR : u8 = 26;

    //TODO: The single joycon values.

}

#[derive(Clone, Copy, Eq, PartialEq, Debug, Hash)]
pub enum LibnxKeyState {
    Up, 
    Down, 
    Held
}

/// Converts a number from LibNX's hidKeysUp, hidKeysDown, and hidKeysHeld to a 
/// piston Event. 
/// Note that the values for LibNX's HidControllerKeys enum are NOT the same as the 
/// id's returned from these events, due to piston requiring the id to fit in a u8. 
/// In addition, since piston has no concept of "holding" a key at the moment, 
/// a state of LibnxKeyState::Held produces no events. This may change in the future, however.
pub fn parse_key_events(controller : i32, state : LibnxKeyState, keys : u32) -> Vec<Input> {

    let mut retval = Vec::new();
    
    if state == LibnxKeyState::Held {
        return retval;
    }

    for idx in 0 .. 32 {
        let mask = 1 << idx; 
        if mask & keys == 0 {
            continue;
        }
        let button = idx + 1; 
        let nevent = Input::Button(parse_args(controller, state, button));
        retval.push(nevent);
    }
    retval
}


#[inline]
fn parse_args(controller : i32, state : LibnxKeyState, button : u8) -> ButtonArgs {
    let btn = parse_button(controller, button);
    let argState = match state {
        LibnxKeyState::Up => ButtonState::Release, 
        _ => ButtonState::Press
    };
    ButtonArgs {
        state : argState, 
        button : btn, 
        scancode : None
    }
}

#[inline]
fn parse_button(controller : i32, button : u8) -> Button {
    parse_hat_event(controller, button)
        .map(|hat| Button::Hat(hat))
        .unwrap_or(Button::Controller(parse_button_event(controller, button)))
}

#[inline]
fn parse_button_event(controller : i32, button : u8) -> ControllerButton {
    ControllerButton::new(controller, button)
}

#[inline]
fn parse_hat_event(controller : i32, button : u8) -> Option<ControllerHat> {
    if button < 13 || button > 24 {
        None
    }
    else {
        let dirnum = button % 4;
        let state : HatState = match dirnum {
            0 => HatState::Down,
            1 => HatState::Left,
            2 => HatState::Up,
            3 => HatState::Right,
            
            _ => return None //Should be mathematically impossible
        };
        Some(ControllerHat::new(controller, button, state))
    }
}