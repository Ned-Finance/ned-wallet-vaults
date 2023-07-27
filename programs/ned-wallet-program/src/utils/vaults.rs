
pub fn get_name_array(name: &Vec<u8>) -> [u8;30] {
    let mut fixed:[u8; 30] = [0; 30];
    let name_as_array:&[u8] = &name;
    for (index, byte) in fixed.iter_mut().enumerate() {
        if let Some(found) = name_as_array.get(index) {
            *byte = *found
        }
    }
    return fixed
}

pub fn name_is_empty(name: &Vec<u8>) -> bool {
    let name_with_chars = name.iter().find(|x| **x != 0);
    return name_with_chars.is_none();
}