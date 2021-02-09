// scan entry
pub struct ScanEntry {
    result: bool
}
impl ScanEntry {
    fn scan() {

    }
}


// scan batch that contains scan entries
pub struct ScanBatch {
    // private scan entry slice
    entries: [ScanEntry]
}

impl ScanBatch {
    // adds scan entries into scan batch
    pub fn addEntry(&mut self, e: ScanEntry) {
        self.entries += e
    }

    pub fn run(self) {
        for i in self.entries {
            i
        }
    }
}
