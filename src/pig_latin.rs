//use std::collections::HashSet;

pub fn convert(word : String) -> String{

    // let mut set = HashSet::new();
    // set.insert("a");
    // set.insert("e");
    // set.insert("i");
    // set.insert("o");
    // set.insert("u");

    //let mut set: HashSet<String> = vec!["a", "e", "i", "o", "u"].into_iter().collect();

    let set = vec!["a", "e", "i", "o", "u"];
    let (head, tail) = word.split_at(1);

    if set.contains(&head) {
        let result = format!("{}-hay", word);
        return result;
    }

    let result = format!("{}-{}ay", tail, head);
    result
}