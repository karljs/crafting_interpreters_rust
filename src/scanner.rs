pub struct Scanner {
    source: String,
}

impl Scanner {
    pub fn from_source(source: String) -> Self {
        println!("source is {:?}", source);
        Scanner { source }
    }

    pub fn scan(&self) -> std::str::Chars<'_> {
        self.source.chars()
    }
}
