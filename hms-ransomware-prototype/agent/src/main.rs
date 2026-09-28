
use notify::{Config,RecommendedWatcher,RecursiveMode,Watcher};
use std::{path::Path,sync::mpsc::channel};
fn main(){
 let (tx,rx)=channel();
 let mut w=RecommendedWatcher::new(tx,Config::default()).unwrap();
 let p=Path::new(".");
 w.watch(p,RecursiveMode::Recursive).unwrap();
 println!("Watching current folder...");
 for e in rx { println!("{:?}",e); }
}
