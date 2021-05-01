pub fn build_proverb(list: &[&str]) -> String {
    if list.len() == 0 {
        return String::new();
    }
    let mut s = String::new();
    for i in 0..list.len()-1 {
        s.push_str(&format!("For want of a {} the {} was lost.\n", 
            list[i], list[i+1])[..]);
    }
    s.push_str(&format!("And all for the want of a {}.", list[0])[..]);
    s
}
