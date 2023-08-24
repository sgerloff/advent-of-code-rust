#[derive(Debug)]
pub struct TopNGroupCounter {
    top_n: usize,
    _current_sum: usize,
    pub top_sums: Vec::<usize>
}

impl TopNGroupCounter {
    pub fn new(top_n: usize) -> TopNGroupCounter {
        TopNGroupCounter { 
            top_n: top_n, 
            _current_sum: 0, 
            top_sums: Vec::<usize>::new() 
        }
    }

    pub fn add(&mut self, number: usize) {
        self._current_sum = self._current_sum + number;
    }

    pub fn complete(&mut self) {
        self.top_sums.push(self._current_sum);
        self.top_sums.sort();
        self.top_sums.reverse();
        self.top_sums.truncate(self.top_n);
        self.reset();
    }

    fn reset(&mut self) {
        self._current_sum = 0;
    }
}