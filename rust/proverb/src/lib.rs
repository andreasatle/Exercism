//! For want of a horseshoe nail, a kingdom was lost, or so the saying goes.
//! 
//! Given a list of inputs, generate the relevant proverb. For example, given the 
//! list ["nail", "shoe", "horse", "rider", "message", "battle", "kingdom"], 
//! you will output the full text of this proverbial rhyme:
//! ```comment
//! For want of a nail the shoe was lost.
//! For want of a shoe the horse was lost.
//! For want of a horse the rider was lost.
//! For want of a rider the message was lost.
//! For want of a message the battle was lost.
//! For want of a battle the kingdom was lost.
//! And all for the want of a nail.
//! ```
//! Note that the list of inputs may vary; your solution should be able to handle lists 
//! of arbitrary length and content. No line of the output text should be a static, 
//! unchanging string; all should vary according to the input given.

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
