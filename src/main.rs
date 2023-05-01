use std::fs::File;
use rust_stemmers::{Algorithm, Stemmer};
use std::collections::HashMap;
use std::collections::BTreeMap;
use std::io::{BufReader,BufRead};
use text_analysis::count_words;

fn cosine_similarity(val: &[f64], va: &[f64]) -> f64 {
    let dot: f64 = val.iter().zip(va.iter()).map(|(a,b)| a * b).sum();
    let sq1 = val.iter().map(|x| x * x).sum::<f64>();
    let sq2 = va.iter().map(|y| y * y).sum::<f64>();
    let prods = sq1.sqrt() * sq2.sqrt();
    if prods == 0.0 { 0.0 } else { dot/prods }
}

fn median(vector: Vec<f64>) -> f64 {
    let n = vector.len();
    let half = n/2;
    if n % 2 == 0 {
       (vector[half] + vector[half-1])/2.0
    } else {
       vector[half]
       }
}

fn standard_deviation(vector: &Vec<f64>) -> f64 {
    let mut sum_of_squares = 0.0;
    let mean = vector.iter().sum::<f64>() / vector.len() as f64;
    for value in vector {
        sum_of_squares += (value - mean).powi(2);
	}
    (sum_of_squares/vector.len() as f64).sqrt()
}

fn add_entries(matrix_hash: &BTreeMap<(usize, usize), f64>, vocab: &BTreeMap<String,usize>, total: &mut HashMap<String, Vec<f64>>, start: usize, end: usize) {
    //function for unsparsing the matrix ahead of the cosine similarity comparisons
    for x in 0..=vocab.len() {
        for y in start..=end {
            let item = total.entry(y.to_string()).or_insert(Vec::new());
            let value = matrix_hash.get(&(x, y)).cloned().unwrap_or(0.0);
            item.push(value);
        }
    }
}

fn receive_stats(hash1: HashMap<String,Vec<f64>>, vocablen: usize, po_wd_cnt: &mut Vec<f64>) -> (f64,f64,f64,f64,f64,f64,f64,f64,f64) {
      let mut poem_vocab: Vec<f64> = Vec::new();
      let mut poem_repeats: Vec<f64> = Vec::new();
      let mut cosine_sim: HashMap<(String,String),f64> = HashMap::new();
          for (poem,val) in hash1.clone() {
             let sum_of_ones = val.iter().filter(|&x| *x == 1.0).sum::<f64>();
	     poem_repeats.push((vocablen as f64 - sum_of_ones)/vocablen as f64);
	     poem_vocab.push(val.iter().filter(|value| value > &&0.0).count() as f64);
             for (po,va) in hash1.clone() {
                let result = cosine_similarity(&val, &va);
                cosine_sim.insert((poem.to_string(), po.to_string()),result);
	        }
          }
       let mut stats = vec![];
       for ((_poem,_po),similarity) in cosine_sim {
          stats.push(similarity);
          }
       let sum: f64 = stats.iter().sum();
       let mean: f64 = sum/stats.len() as f64;
       let std_dev: f64 = standard_deviation(&stats);
       let repeats_mean: f64 = poem_repeats.iter().sum::<f64>()/poem_repeats.len() as f64;
       let poem_vocab_mean: f64 = poem_vocab.iter().sum::<f64>()/poem_vocab.len() as f64;
       stats.sort_by(|a, b| b.partial_cmp(a).unwrap().reverse());
       po_wd_cnt.sort_by(|a,b| b.partial_cmp(a).unwrap().reverse());
       let min = stats.first().unwrap();
       let max = stats.last().unwrap();
       let range = max - min;
       let wrd_range = po_wd_cnt.last().unwrap() - po_wd_cnt.first().unwrap();
       let wrd_mean: f64 = po_wd_cnt.iter().sum::<f64>() / po_wd_cnt.len() as f64;
       let dist_median: f64 = median(stats);
       let word_cnt_median: f64 = median(po_wd_cnt.to_vec());
       (mean, std_dev, range, dist_median, wrd_range, wrd_mean, word_cnt_median, repeats_mean, poem_vocab_mean)
 }


fn main() {
    let files = ["ChatGPTsbestpoems.txt", "BardBestPoems.txt"];
    let mut vocab: BTreeMap<String,usize> = BTreeMap::new();
    let mut matrix_hash: BTreeMap<(usize,usize),f64> = BTreeMap::new();
    let mut poem_word_cnt: i32 = 0;
    let mut po_wd_cnt = vec![];
    let mut total_po_wd_cnt: HashMap<String, Vec<f64>> = HashMap::new();
    let mut poem_cnt: i32 = 0;
    for f in files.iter() {
       let fname = format!("{:?}", f);
       let f = File::open(f).expect("file did not open");
       let reader = BufReader::new(f);
       let en_stemmer = Stemmer::create(Algorithm::English);
       let en_stop_words = stop_words::get(stop_words::LANGUAGE::English);
       let mut poem_words: Vec<String> = Vec::new();
       for poem_line in reader.lines() {
        //check if the poem is finished
   	   let poem_line = &poem_line.unwrap();
           if !poem_line.is_empty() {
	       let poem_vec: Vec<&str> = poem_line.split_whitespace().collect();
	       for po_v in poem_vec {
	           let stem = en_stemmer.stem(po_v);
		   let s = stem.replace(&['!', '.','\'','\"',',','?'][..],"");
		   poem_word_cnt+=1;
		   if !en_stop_words.contains(&s.to_lowercase()) {
		         let index = vocab.len();
		         vocab.entry(s.to_lowercase()).or_insert(index);
		         poem_words.push(s.to_lowercase());
		         }
		      }
	   } else {
	          poem_cnt+=1;
		  po_wd_cnt.push(poem_word_cnt as f64);
		  let count_po = count_words(&poem_words);
		  for (k,v) in count_po.iter() {
		     matrix_hash.insert((vocab[k],poem_cnt as usize),*v as f64);
		     }
	          poem_words = Vec::new();
	          poem_word_cnt = 0;
	          }
	   }
	total_po_wd_cnt.entry(fname).or_insert(po_wd_cnt.clone());
    }
    let mut totalchat: HashMap<String,Vec<f64>> = HashMap::new();
    let mut totalbard: HashMap<String,Vec<f64>> = HashMap::new();
    add_entries(&matrix_hash, &vocab, &mut totalchat, 0, 35);
    add_entries(&matrix_hash, &vocab, &mut totalbard, 36, 70);
    let totalchatresults = receive_stats(totalchat, vocab.len(), total_po_wd_cnt.get_mut("\"ChatGPTsbestpoems.txt\"").unwrap());
    let totalbardresults = receive_stats(totalbard, vocab.len(), total_po_wd_cnt.get_mut("\"BardBestPoems.txt\"").unwrap());
    println!("Name:  mean, std_dev, range, median_sim, word_range, word_mean, word_count_median, repeats_mean, poem_vocab_mean");
    println!("ChatGPT: {:.3?}", totalchatresults);
    println!("Bard: {:.3?}", totalbardresults);
}
