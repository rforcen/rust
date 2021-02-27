// epub word frequency counter

#![allow(dead_code)]

use epub::doc::EpubDoc;
use html2text;
use std::collections::HashMap;
use glob::glob;
use std::env::args;
use num_cpus;
use scoped_pool::{Pool};
use std::{char, {time::{Instant}}, sync::{Arc, Mutex, MutexGuard}};

const SEPARATORS : &str = " ,ºª\\`©…«»—[]{}\n;*-?¿¡!\"#$%&/()=:._<>─┼“”·│×’";

type WordMap = HashMap::<String, u32>;

// Arc<Mutex<T>>
#[derive(Clone, Debug)]
struct ArcMutex<T> (Arc<Mutex<T>>);

impl<T> ArcMutex<T> {
	pub fn new(v : T) -> Self { ArcMutex( Arc::new(Mutex::new(v)) ) } 
	pub fn get_val(&self) ->  T where T: Copy { *self.0.lock().unwrap() }
	pub fn get(&self) -> MutexGuard<T> { self.0.lock().unwrap() }
	pub fn clone(&self) -> Self { ArcMutex( Arc::clone(&self.0)) }
	pub fn set(&mut self, v : T) { *self.0.lock().unwrap() = v }
}


fn main() {
    index_mt()
}

fn compare_st_mt_timings() { // mt is x3 times faster in a 4 core cpu
    let t = Instant::now();   index_mt();    println!("mt : {:?}", Instant::now()-t);
    let t = Instant::now();   index_st();    println!("st : {:?}", Instant::now()-t);
}

fn print_wfc(word_map : &WordMap) { // print wordmap in freq. reversed order
    let mut count_vec: Vec<(&String, &u32)> = word_map.iter().collect(); // sort in count order
    count_vec.sort_by(|a, b| b.1.cmp(a.1));
    let sum : u32 = count_vec.iter().map(|&kv| kv.1).sum(); // total words
    for i in &count_vec { println!("{:18} {count:>8.*}  {frac:.1}%", i.0, count=*i.1 as usize, frac=100. * *i.1 as f32 / sum as f32);}    
    println!("word count: {}", sum);
}

fn index_mt() { // multithreaded freq. index version
    if let Some(mut base_folder) = args().nth(1) { 
        base_folder += "/**/*.epub";
    
        let mut word_map = WordMap::new();
        let paths = ArcMutex::new(vec![]);
        let nth = num_cpus::get();

        fn process_paths(paths : ArcMutex<Vec<String>>, word_map : &mut WordMap) {
        
            let np = paths.get().len();  

            if np!=0 {
                let wms = ArcMutex::new(vec![WordMap::new(); np]);

                let pool = Pool::new(np);

                pool.scoped( |scope| {          
                    
                    for i in 0..np {
                        let path = paths.get()[i].clone();
                        let wms = wms.clone();

                        scope.execute(move || {                             
                            wms.get()[i] = index_epub(&path);
                        });          
                    }                        
                });
                
                pool.shutdown();               

                for wm in wms.get().iter() { // aggregate in word_map <= wms[]
                    for (k, v) in wm.iter() {
                        *word_map.entry(k.clone()).or_insert(0) += v
                    }  
                }
            }
        }

        for entry in glob(&base_folder[..]).expect("Failed to read glob pattern")  {
            if let Ok(path) = entry {

                let path = path.into_os_string().into_string().unwrap();
                paths.get().push(path.clone());

                if paths.get().len() == nth { // do the 'nth' epubs
                    process_paths(paths.clone(), &mut word_map);
                    paths.get().clear();
                }                     
            }
        }
        // process pending paths if any
        if paths.get().len() != 0 {  process_paths(paths.clone(), &mut word_map) }

        print_wfc(&word_map)
    }
}

fn index_st() {
    if let Some(mut base_folder) = args().nth(1) { 
        base_folder += "/**/*.epub";
    
        let mut word_map = WordMap::new();

        for entry in glob(&base_folder[..]).expect("Failed to read glob pattern")  {
            if let Ok(path) = entry {
                for (k, v) in index_epub(&path.into_os_string().into_string().unwrap()).iter() {
                    *word_map.entry(k.clone()).or_insert(0) += v
                }             
            }
        }

        print_wfc(&word_map)
    }
}

fn index_epub(epub_file : &String) -> WordMap { // map of epub 

    let mut word_map = WordMap::new();
    let separators : Vec<char> = SEPARATORS.chars().collect();

    if let Ok(ref mut doc) = EpubDoc::new(epub_file) {        
        for spine in doc.spine.clone() {  // spine travers
            if let Ok(content) = doc.get_resource_str(&spine[..]) { // 
                for c in html2text::from_read(&(content.as_bytes())[..], 60).split(&separators[..]) {
                    if c.len() > 0 { *word_map.entry(c.to_lowercase().to_string()).or_insert(0) += 1 }
                }
            }
        }
    }

    word_map
}


