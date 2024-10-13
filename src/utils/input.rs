use raylib::prelude::KeyboardKey;

pub fn key_to_char(key: KeyboardKey) -> Option<char> {
    match key {
        KeyboardKey::KEY_A => Some('a'),
        KeyboardKey::KEY_B => Some('b'),
        KeyboardKey::KEY_C => Some('c'),
        KeyboardKey::KEY_D => Some('d'),
        KeyboardKey::KEY_E => Some('e'),
        KeyboardKey::KEY_F => Some('f'),
        KeyboardKey::KEY_G => Some('g'),
        KeyboardKey::KEY_H => Some('h'),
        KeyboardKey::KEY_I => Some('i'),
        KeyboardKey::KEY_J => Some('j'),
        KeyboardKey::KEY_K => Some('k'),
        KeyboardKey::KEY_L => Some('l'),
        KeyboardKey::KEY_M => Some('m'),
        KeyboardKey::KEY_N => Some('n'),
        KeyboardKey::KEY_O => Some('o'),
        KeyboardKey::KEY_P => Some('p'),
        KeyboardKey::KEY_Q => Some('q'),
        KeyboardKey::KEY_R => Some('r'),
        KeyboardKey::KEY_S => Some('s'),
        KeyboardKey::KEY_T => Some('t'),
        KeyboardKey::KEY_U => Some('u'),
        KeyboardKey::KEY_V => Some('v'),
        KeyboardKey::KEY_W => Some('w'),
        KeyboardKey::KEY_X => Some('x'),
        KeyboardKey::KEY_Y => Some('y'),
        KeyboardKey::KEY_Z => Some('z'),
        _ => None,
    }
}

pub fn key_to_digit(key: KeyboardKey) -> Option<char> {
    match key {
        KeyboardKey::KEY_ZERO | KeyboardKey::KEY_KP_0 => Some('0'),
        KeyboardKey::KEY_ONE | KeyboardKey::KEY_KP_1 => Some('1'),
        KeyboardKey::KEY_TWO | KeyboardKey::KEY_KP_2 => Some('2'),
        KeyboardKey::KEY_THREE | KeyboardKey::KEY_KP_3 => Some('3'),
        KeyboardKey::KEY_FOUR | KeyboardKey::KEY_KP_4 => Some('4'),
        KeyboardKey::KEY_FIVE | KeyboardKey::KEY_KP_5 => Some('5'),
        KeyboardKey::KEY_SIX | KeyboardKey::KEY_KP_6 => Some('6'),
        KeyboardKey::KEY_SEVEN | KeyboardKey::KEY_KP_7 => Some('7'),
        KeyboardKey::KEY_EIGHT | KeyboardKey::KEY_KP_8 => Some('8'),
        KeyboardKey::KEY_NINE | KeyboardKey::KEY_KP_9 => Some('9'),
        _ => None,
    }
}
