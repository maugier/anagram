use std::{collections::{HashMap}, path::{PathBuf, Path}, error::Error, io::{BufRead, stdin, BufReader}, fs::File};
use clap::Parser;

#[derive(Parser)]
#[command(author, version, about, long_about=None)]
struct CLI {
    /// Word whose anagrams to look for
    target: String,

    /// Dictionary file, or "-" to read from stdin (default)
    #[arg(short, value_name = "FILE")]
    dictfile: Option<PathBuf>,

    /// Minimum word length. Equivalent to filtering the dictionary.
    #[arg(short)]
    minlen: Option<usize>,
}


fn main() -> Result<(), Box<dyn Error>>{

    let args = CLI::parse();

    let words = WordMap::load(args.dictfile.as_deref(), args.minlen)?;
    let target = Letters::new(&args.target);

    for_anagrams(&words, &target, |ws| {
        println!("{}", ws.join(" "));
    });

    Ok(())

}

fn for_anagrams<F: FnMut(&[&str])>(words: &WordMap, target: &Letters, mut action: F) {

    let limit = words.maxlen();
    let mut buf = vec![];
    go(words, target, &mut buf, limit, &mut action);

    fn go<'a, F: FnMut(&[&str])>(words: &'a WordMap, target: &Letters, buffer: &mut Vec<&'a str>, limit: usize, action: &mut F) {
        //println!("find_anagrams {:?}", target);
        if target.len == 0 { 
            action(&buffer[..]);
            return
        }

        for len in (1..=target.len.min(limit)).rev() {
            if let Some(len_words) = words.0.get(&len) {
                for word in len_words {
                    if let Some(new_target) = target.subtract(&*word) {
                        buffer.push(&*word);
                        go(words, &new_target, buffer, len, action);
                        buffer.pop();
                    }
                }
            }
        }
    }

}

#[derive(Debug,Clone)]
struct Letters {
    map: HashMap<char,usize>,
    len: usize,
}

impl Letters {

    fn new(s: &str) -> Self {
        let mut len = 0;
        let mut map = HashMap::new();

        for char in s.chars() {
            *map.entry(char).or_default() += 1;
            len += 1;
        }
        Self { map, len }
    }

    fn subtract(&self, s: &str) -> Option<Self> {
        if s.chars().count() > self.len { return None }
        let mut new = self.clone();

        for char in s.chars() {
            let count = new.map.get_mut(&char)?;
            *count = count.checked_sub(1)?;
            new.len -= 1;
        }

        Some(new)
    }

}

struct WordMap(HashMap<usize, Vec<String>>);

impl WordMap {

    fn maxlen(&self) -> usize {
        *self.0.keys().max().unwrap() // safe because we assert the map not to be empty on construction
    }

    fn load(path: Option<&Path>, minlen: Option<usize>) -> Result<Self, Box<dyn Error>> {
        let mut words: HashMap<usize, Vec<String>> = HashMap::new();
    
        let source: Box<dyn Iterator<Item=_>> = match path {
            None  => Box::new(stdin().lines()),
            Some(dash) if dash == Path::new("-") => Box::new(stdin().lines()),
            Some(path) => {
                Box::new(BufReader::new(File::open(path)?).lines())
            }
        };

        for word in source {
            let word = word?;
            let len = word.chars().count();
            if let Some(min) = minlen {
                if len < min { continue }
            }
            words.entry(len).or_default().push(word);
        }
    
        if words.len() == 0 {
            Err("Empty dictionary")?;
        }

        Ok(Self(words))
    
    }
}