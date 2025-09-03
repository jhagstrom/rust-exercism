pub fn recite(start_bottles: u32, take_down: u32) -> String {
   let words = [
        "No", "One", "Two", "Three", "Four",
        "Five", "Six", "Seven", "Eight", "Nine", "Ten"
    ];
        let mut song: String = String::new();
        let start: usize = start_bottles as usize;
        let stop: usize = take_down as usize;
        let stop_at = if stop >= start { 1 } else { start - stop + 1 };
        for num in (stop_at..=start).rev() {
            let bottle_word = if num == 1 { "bottle" } else { "bottles" };
            let next_num = num - 1;
            let next_bottle_word = if next_num == 1 { "bottle" } else { "bottles" };
            song.push_str(&format!(
                "{} green {} hanging on the wall,\n",
                words[num], bottle_word
            ));
            song.push_str(&format!(
                "{} green {} hanging on the wall,\n",
                words[num], bottle_word
            ));
            song.push_str("And if one green bottle should accidentally fall,\n");
    
            if next_num > 0 {
                song.push_str(&format!(
                    "There'll be {} green {} hanging on the wall.\n\n",
                    words[next_num].to_lowercase(), next_bottle_word
                ));
            } else {
                song.push_str("There'll be no green bottles hanging on the wall.\n\n");
            }
        }
    song
}


