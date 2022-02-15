extern crate hex;
extern crate generic_array;
extern crate ripline;
extern crate hash_hasher;

use std::io::{Write, stdin, stdout};
use std::env;
use std::error::Error;
use std::fs::File;
use hex::FromHex;
use generic_array::{typenum::U16, GenericArray};
use md4::{Md4, Digest};
use ripline::{lines::LineIter};
use memmap2::Mmap;
// Special hasher for already hashed data - NTLM is a hash
use hash_hasher::HashedMap;
use std::time::Instant;
use std::thread;
use std::thread::JoinHandle;
use crossbeam_channel::unbounded;

// This is used for sending words to the threads as it takes ownership of
// the data and avoids problems with mmap not having a 'static lifetime
struct Message {
  text: Vec<u8>,
}

fn main() -> Result<(), Box<dyn Error>> {

  // Open and read the input hashes
  let mmap = unsafe { 
    Mmap::map(
      &File::open(
        &env::args().nth(1).expect("Failed to provide hash input file")
      )?
    )?
  };
  let iter = LineIter::new(b'\n', &mmap);

  // store the first and last byte of input hashes, so for small input hash lists
  // we can do a cheaper check than a hashmap lookup
  let mut starts = [0;256];
  let mut ends = [0;256];

  // Convert input hashes file to HashMap of GenericArray's
  // Since searching these hashes is the biggest cost of this whole thing
  // we use a HashMap for 0(1)~ performance
  let to_find: HashedMap<GenericArray<u8, U16>,_> = iter.map(|l| {
    let inhash = <[u8; 16]>::from_hex(&l[0..l.len()-1]).unwrap(); 
    let in_hash: GenericArray<u8, U16> = *GenericArray::from_slice(&inhash);
    starts[inhash[0] as usize] = 1;
    ends[inhash[15] as usize] = 1;
    (in_hash,0)
  }).collect();

  // For big input hash lists we want to skip the fast byte check below
  let big = match to_find.len() {
    e if e > 512 => true,
    e if e < 512 => false,
    _ => false,
  };
  // This decides when a thread should notify the main that it's cracked stuff
  let crackthresh = match big {
    true => 10,
    false => 1,
  };

  // Fire off our worker threads to wait for the data from the wordlist
  let threadnum = 10;
  let mut threadhand: Vec<JoinHandle<_>> = Vec::new();
  // We clone the reciever multiple times which is how the threads pick up new clears
  // Can't do that with mpsc which only allows cloning the sender, need crossbeam
  let (tx,rx): (crossbeam_channel::Sender<Option<Message>>, crossbeam_channel::Receiver<Option<Message>>) = unbounded();
  let (tx2,rx2): (crossbeam_channel::Sender<usize>, crossbeam_channel::Receiver<usize>) = unbounded();
  
  for _ in 0..threadnum {
    // Make copies of these two for the threads
    let rx_thread = rx.clone();
    let tx2_thread = tx2.clone();
    let to_find_thread = to_find.clone();
    threadhand.push(thread::spawn(move || {

      // Pre-allocate the objects we'll reuse to reduce alloc's
      let mut utf16: Vec<u8> = Vec::with_capacity(1024); // utf16 encoded string as bytes
      let mut out: Vec<u8> = Vec::with_capacity(8192);
      let mut b = [0;2]; // needed for utf16 encoding, but not used
      let mut cracked = 0;

      // Fetch clears from the channel
      for recv in rx_thread {
        // We wrap the message in an Option to allow for a kill signal (see below)
        if let Some(message) = recv {
          for clear in message.text.split(|c| *c == 10 as u8) {
            //println!("Thread {} recieved: '{:?}'",j,clear);
            if clear.len() == 0 { continue; } //we get a lot of blanks for some reason
            // faster to iter & encode chars than the encode_utf16 str iter
            clear.iter()
              .for_each(|n| {
                let c = char::from(*n).encode_utf16(&mut b);
                // align_to is unsafe, but faster than to_le_bytes
                unsafe {
                  utf16.extend_from_slice(c.align_to::<u8>().1);
                }
              });
            // encoding error
            if utf16.len() == 0 && clear.len() != 0 { continue; }
            // doing this single Md4 digest is faster than multiple updates() + finalize()
            let hash = Md4::digest(&utf16);

            if !big {
              // for small hashlists, can we get away with this cheaper check
              if starts[hash[0] as usize] != 1 || ends[hash[15] as usize] != 1 { 
                utf16.clear(); continue;
              }
            }

            // check if the generated hash is in our input hash list
            if to_find_thread.contains_key(&hash) {
              cracked += 1;
              // extend_from_slice is faster than push
              out.extend_from_slice(&clear);
              out.extend_from_slice(&[58]);
              //writing each character is faster than doing it in one go
              for x in hash {
                write!(&mut out,"{:02x}",x).unwrap();
              }
              out.extend_from_slice(&[10]);
              // check if our output buffer should be flushed
              if out.len() >= 8192 { // make sure this comparison aligns with capacity
                stdout().write_all(&out).unwrap();
                out.clear();
              }
              if cracked == crackthresh {
                tx2_thread.send(cracked).unwrap();
                cracked = 0;
              }
            }
            // clear our reused buffers
            utf16.clear();
          }
        // Our thread recieved None lets dump our buffer and exit
        } else {
          //println!("Break {}",j);
          stdout().write_all(&out).unwrap();
          break;
        } 
      } 
    }));  
  }   

  // Reading from stdin is faster than opening a file
  let mmap = unsafe { Mmap::map(&stdin()).expect("No wordlist provided via stdin") };

  let iter = LineIter::new(b'\n', &mmap);
  let num = 64*31; // number of clears to send each thread
  let mut buf: Vec<u8> = Vec::with_capacity(num);
  let mut cracked= 0;
  let mut hashed= 0;
  let start = Instant::now();

  // Got through the wordlist and add the words to a buffer, then send to a thread
  for clear in iter {
    hashed += 1;
    buf.extend_from_slice(clear);
    // we hit out buffer size, send
    if buf.len() >= num-256 {
      let message = Message {
          text: buf.clone(),
      };
      tx.send(Some(message)).unwrap();
      buf.clear();
      // check if we can exit early because we cracked everything
      if let Ok(i) = rx2.try_recv() {
        cracked += i;
        // if we can tell all the threads to exit
        if cracked == to_find.len() {
          for _ in 0..threadnum {
            tx.send(None).unwrap();
          }
          break;
        }
      }
    }
  }

  // Send any last clears in the buffer
  if buf.len() != 0 {
    let message = Message {
        text: buf.clone(),
    };
    tx.send(Some(message)).unwrap();
  }

  // tell the threads to exit
  for _ in 0..threadnum {
    tx.send(None).unwrap();
  }

  // wait for threads to exit
  for thread in threadhand {
    thread.join().unwrap();
  }
  
  let elapsed = (start.elapsed().as_secs() as f64) + (f64::from(start.elapsed().subsec_nanos()) / 1_000_000_000.0);
  write!(stdout(),"Time: {:.2}s, Hashed: {}, Cracked: {}, Speed: {:.2} kH/s\n",
    elapsed,hashed,cracked,
    (hashed as f64/elapsed)/1024 as f64)
    .unwrap();

  Ok(())
}
