/// Syncmers as defined by Dutta et al. 2022, https://www.biorxiv.org/content/10.1101/2022.01.10.475696v2.full
/// Esp Fig 1b
/// Planning to implement other methods soon
///
/// TODO: Add Iterator impl's

use std::iter::{FilterMap, Enumerate};
use std::slice::Windows;

// TODO:Denote the reverse complement of x by Embedded Image. For a given order, the canonical form of a k-mer x, denoted by Canonical(x), is the smaller of x and Embedded Image. For example, under the lexicographic order, Canonical(CGGT) = ACCG.
// Canonical(x) = min(x, revcomp(x))

// NOTE: "By convention, ties are broken by choosing the leftmost position"

/// 1-parameter syncmer method
/// t is 0-based (unlike in the paper)
/// NOTE: Sequence should be all upper case (or all lower case)
// TODO: Remove?
pub struct Syncmers {
    pub k: usize,
    pub s: usize,
    pub t: usize,
    // pub downsample: f32,
}

// type FilterMapIter<'a> = FilterMap<Enumerate<Windows<'a, u8>>, &'static fn ((usize, &'a [u8])) -> Option<usize>>;

impl Syncmers {
    pub fn new(k: usize, s: usize, t: usize) -> Self {

        assert!(s < k);
        assert!(t < k);
        Syncmers { k, s, t }
    }

    // TODO: Find a way to return just the iter (FilterMap Iter and it's long return type)

    pub fn find_all(&self, seq: &[u8]) -> Vec<usize> {
        assert!(seq.len() >= self.k);
        seq.windows(self.k)
            .enumerate()
            .filter_map(|(i, kmer)| {
                let min_pos = kmer
                    .windows(self.s)
                    .enumerate()
                    .min_by(|(_, a), (_, b)| a.cmp(b));

                if min_pos.unwrap().0 == self.t {
                    Some(i)
                } else {
                    None
                }
            })
            .collect::<Vec<_>>()
    }

    /*
    type FilterMapIter<'a> = FilterMap<Enumerate<Windows<'a, u8>>, &'static fn ((usize, &'a [u8])) -> Option<usize>>;

    pub fn find<'a>(&self, seq: &'a [u8]) -> FilterMapIter<'a> {
        assert!(seq.len() >= self.k);
        seq.windows(self.k)
            .enumerate()
            .filter_map(|(i, kmer)| {
                if let Some(min_pos) = kmer
                    .windows(self.s)
                    .enumerate()
                    .min_by(|(_, a), (_, b)| a.cmp(b)) {

                        if min_pos == self.t {
                            Some(i)
                        } else {
                            None        
                        }
                    } else {
                        None
                    }
            }) 

    } */
}

impl Iterator for Syncmers {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        unimplemented!()
    }
}

/// Multi-parameter syncmer method
/// t is 0-based (unlike in the paper)
pub struct ParameterizedSyncmers<'a> {
    pub k: usize,
    pub s: usize,
    pub t: &'a [usize],
}

impl<'a> ParameterizedSyncmers<'a> {
    pub fn new(k: usize, s: usize, t: &'a [usize]) -> Self {
        assert!(s < k);
        assert!(t.iter().all(|&t| t < k));
        ParameterizedSyncmers { k, s, t }
    }

    pub fn find_all(&self, seq: &[u8]) -> Vec<usize> {
        seq.windows(self.k)
            .enumerate()
            .filter_map(|(i, kmer)| {
                let min_pos = kmer
                    .windows(self.s)
                    .enumerate()
                    .min_by(|(_, a), (_, b)| a.cmp(b));

                if self.t.contains(&min_pos.unwrap().0) {
                    Some(i)
                } else {
                    None
                }
            })
            .collect::<Vec<_>>()
    }
}

mod test {
    use super::*;

    #[test]
    pub fn test_syncmers_fig1b() {
        let sequence = b"CCAGTGTTTACGG";
        let syncmers = Syncmers::new(5, 2, 2);
        let syncmer_positions = syncmers.find_all(sequence);
        assert!(syncmer_positions == vec![0, 7]);

        let ts: [usize; 1] = [2];

        let psyncmers = ParameterizedSyncmers::new(5, 2, &ts);
        let syncmer_positions = psyncmers.find_all(sequence);
        assert!(syncmer_positions == vec![0, 7]);
    }
}