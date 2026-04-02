// Copyright 2022 Google LLC

// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::ops::Range;

#[derive(Debug, Default)]
struct ConsumedRanges {
    ranges: Vec<Range<usize>>,
    covered_len: usize,
}

impl ConsumedRanges {
    fn insert(&mut self, mut range: Range<usize>) {
        if range.is_empty() {
            return;
        }

        let mut insert_at = 0;
        while insert_at < self.ranges.len() && self.ranges[insert_at].end < range.start {
            insert_at += 1;
        }

        let merge_start = insert_at;
        while insert_at < self.ranges.len() && self.ranges[insert_at].start <= range.end {
            let current = &self.ranges[insert_at];
            range.start = range.start.min(current.start);
            range.end = range.end.max(current.end);
            self.covered_len -= current.end - current.start;
            insert_at += 1;
        }

        self.ranges.splice(merge_start..insert_at, [range.clone()]);
        self.covered_len += range.end - range.start;
    }

    fn covers_len(&self, len: usize) -> bool {
        self.covered_len == len
    }
}

pub(super) struct CacheCell {
    data: Vec<u8>,
    bytes_read: ConsumedRanges,
}

impl CacheCell {
    pub(super) fn new(data: Vec<u8>) -> Self {
        Self {
            data,
            bytes_read: ConsumedRanges::default(),
        }
    }

    pub(super) fn read(&mut self, range: Range<usize>) -> &[u8] {
        self.bytes_read.insert(range.clone());
        &self.data[range]
    }

    pub(super) fn len(&self) -> usize {
        self.data.len()
    }

    pub(super) fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    pub(super) fn entirely_consumed(&self) -> bool {
        self.bytes_read.covers_len(self.data.len())
    }
}

#[cfg(test)]
mod tests {
    use super::{CacheCell, ConsumedRanges};

    #[test]
    fn test_cachecell() {
        let mut cell = CacheCell::new(vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9]);
        assert_eq!(cell.len(), 10);
        assert!(!cell.entirely_consumed());

        assert_eq!(cell.read(0..2), &[0, 1]);
        assert!(!cell.entirely_consumed());

        assert_eq!(cell.read(3..10), &[3, 4, 5, 6, 7, 8, 9]);
        assert!(!cell.entirely_consumed());

        assert_eq!(cell.read(0..2), &[0, 1]);
        assert!(!cell.entirely_consumed());

        assert_eq!(cell.read(1..4), &[1, 2, 3]);
        assert!(cell.entirely_consumed());
    }

    #[test]
    fn test_consumed_ranges_merges_overlapping_and_adjacent_ranges() {
        let mut consumed = ConsumedRanges::default();

        consumed.insert(2..4);
        consumed.insert(0..2);
        consumed.insert(3..7);
        consumed.insert(7..10);

        assert!(consumed.covers_len(10));
    }
}
