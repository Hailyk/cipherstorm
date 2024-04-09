// cracker used cracking hashes

use crate::ciphers::{generate_hash, Algorithm};
use crossterm;
use std::collections::{HashMap, HashSet};
use std::sync::mpsc;
use std::sync::Arc;
use std::thread;
use std::thread::JoinHandle;
use std::time;

static SPINNER: [char; 4] = ['|', '/', '-', '\\'];
static SPINNER_SPEED_REDUCER: usize = 80;

// struct for storing the result of the cracker
pub struct CrackResult {
    pub hash: String,
    pub password: String,
    exit_done: bool,
    prefix: String,
}

/// spawn crack manager on a separate thread, communicate with the main thread through a channel
/// @param hash_list: &HashSet<String>
/// @param password_charset: &Vec<String>
/// @param password_length: usize
/// @param algorithm: Algorithm
/// @param multi_thread: bool
/// @return ()
/// @public
/// @description: manage the cracking process
pub fn crack_manager(
    hash_set: Arc<HashSet<String>>,
    password_charset: Arc<Vec<String>>,
    password_length: usize,
    algorithm: Algorithm,
    multi_thread: bool,
) {
    // determine the number of threads to spawn
    let mut available_parallelism: usize = 1;
    if multi_thread {
        let available_paralism_result = thread::available_parallelism();
        if available_paralism_result.is_ok() {
            available_parallelism = available_paralism_result.unwrap().get();
            println!("Available parallelism: {}", available_parallelism);
        } else {
            println!(
                "Error getting available parallelism: {}",
                available_paralism_result.err().unwrap()
            );
            println!("Reducing parallelism to 1");
        }
    } else {
        println!("Running in single thread mode");
    }

    let charset_length = password_charset.len();

    // determine the password list size per thread, 4 seems to be the sweet spot for MD5
    // TODO: make this configurable via command line or a dynamic algorithm
    let suffix_size = 3;

    let mut suffix_visual = String::new();
    for _ in 0..suffix_size {
        suffix_visual.push_str("?");
    }

    // calculate the password prefix last index
    let last_index = password_length - suffix_size;

    // generate password_list_indexing for password_list_generator
    let mut password_indexer: Vec<isize> = Vec::new();
    for _ in 0..last_index {
        password_indexer.push(-1);
    }

    // create vector for storing the thread handles
    let mut thread_handles: HashMap<String, JoinHandle<()>> = HashMap::new();

    // create mpsc::channel for communication between threads
    let (tx, rx) = mpsc::channel();

    // calculate the total number of batches needed, used for progress tracking
    let total_batches_needed: usize = charset_length.pow((password_length - suffix_size) as u32);
    let mut progress: usize = 0;

    // start the timer
    let start_time = time::Instant::now();

    // spawn threads
    for _ in 0..available_parallelism {
        // spawn the thread
        let thread_handle = crack_spawner_helper(
            tx.clone(),
            hash_set.clone(),
            password_charset.clone(),
            password_length,
            &mut password_indexer,
            suffix_size,
            charset_length,
            algorithm,
        );

        // make the copy of the thread name
        let thread_name = thread_handle.0.clone();

        thread_handles.insert(thread_handle.0, thread_handle.1);

    }

    // flag for when the password generation is done
    let mut generation_done = false;

    // spiny thingy keeper tracker
    let mut spinner_counter = 0;

    // listen for result and spawn new thread when a thread finishes
    loop {
        // spinner
        spinner_counter += 1;
        if spinner_counter == 4 * SPINNER_SPEED_REDUCER {
            spinner_counter = 0;
        }

        // get the spinner character
        let spinner_char = SPINNER[spinner_counter / SPINNER_SPEED_REDUCER];
        
        // percentage progress
        let progress_percentage = (progress as f32 / total_batches_needed as f32) * 100.0;

        // get terminal size
        let terminal_size = crossterm::terminal::size().unwrap();
        let terminal_height = terminal_size.1;

        // print the spinner and progress
        crossterm::execute!(
            std::io::stdout(),
            crossterm::cursor::Hide,
            crossterm::cursor::MoveTo(0, terminal_height),
            crossterm::cursor::SavePosition,
            crossterm::style::Print(format!(
                "{} Progress:{:.2}%:{}/{}: ",
                spinner_char, progress_percentage, progress, total_batches_needed
            )),
        )
        .unwrap();

        // try to get the result in a nonblocking way
        let result = rx.try_recv();

        // check if the result is done
        if result.is_ok() {
            let result = result.unwrap();
            if result.exit_done {
                // remove the thread handle
                thread_handles.remove(&result.prefix);

                // increment the progress
                progress += 1;

                // print the completed thread to the bottom line
                crossterm::execute!(
                    std::io::stdout(),
                    crossterm::cursor::Hide,
                    crossterm::style::Print(format!(
                        "Batch {}{} done", result.prefix, suffix_visual
                    )),
                    crossterm::cursor::RestorePosition,
                )
                .unwrap();

                // spawn new thread
                let thread_handle = crack_spawner_helper(
                    tx.clone(),
                    hash_set.clone(),
                    password_charset.clone(),
                    password_length,
                    &mut password_indexer,
                    suffix_size,
                    charset_length,
                    algorithm,
                );
                if thread_handle.2 {
                    // all password generated, wait for all threads to finish
                    generation_done = true;

                    // wait for all threads to finish
                    if thread_handles.is_empty() {
                        break;
                    }
                }

                // insert the new thread handle
                if !generation_done {
                    thread_handles.insert(thread_handle.0, thread_handle.1);
                }
            } else {
                // calculate the time taken
                let time_taken = start_time.elapsed().as_secs();

                //print the result
                crossterm::execute!(
                    std::io::stdout(),
                    crossterm::cursor::MoveTo(0, terminal_height - 1),
                    crossterm::cursor::SavePosition,
                    crossterm::cursor::Hide,
                    crossterm::style::Print(format!(
                        "Hash: {} Password: {} Time: {}s \n",
                        result.hash, result.password, time_taken
                    )),
                    crossterm::cursor::RestorePosition,
                )
                .unwrap();
            }
        } else {
            // sleep
            thread::sleep(time::Duration::from_millis(10));
        }
    }
}

/// spawn next instance of the cracker, spawns a thread for password cracking, returns the thread
/// handle

fn crack_spawner_helper(
    tx: mpsc::Sender<CrackResult>,
    hash_set: Arc<HashSet<String>>,
    password_charset: Arc<Vec<String>>,
    password_length: usize,
    password_indexer: &mut Vec<isize>,
    suffix_size: usize,
    charset_length: usize,
    algorithm: Algorithm,
) -> (String, JoinHandle<()>, bool) {
    // clone the tx so multiple threads can send result to single listener
    let tx_clone = tx.clone();

    // calculate the password prefix last index
    let last_index = password_length - suffix_size;

    // create the password prefix for the thread
    let mut prefix: String = String::new();
    for index in 0..last_index {
        if password_indexer[index] != -1 {
            prefix.push_str(password_charset[password_indexer[index] as usize].as_str());
        }
    }

    // increment the password_indexer prefix
    password_indexer[last_index - 1] += 1;

    // fix any overflow
    for cursor in (0..last_index).rev() {
        // check for overflow
        if password_indexer[cursor] == charset_length as isize {
            if cursor == 0 {
                // all password suffix generated
                return (
                    String::new(),
                    thread::Builder::new()
                        .spawn(|| {
                            return ();
                        })
                        .unwrap(),
                    true,
                );
            }
            // reset the current index and increment the previous index
            password_indexer[cursor] = 0;
            password_indexer[cursor - 1] += 1;
        } else {
            // no more overflow
            break;
        }
    }

    // create the cracker instance
    let instance = crack_instance(
        hash_set.clone(),
        password_charset.clone(),
        prefix.clone(),
        suffix_size.clone(),
        algorithm.clone(),
        tx_clone,
    );

    // return the thread handle
    (prefix, instance, false)
}

/// instance of the cracker, spawns a thread for password cracking, result in returned through a
/// channel to reduce delay and overhead multi password list
/// @private
/// @description: instance of the cracker
/// @return thread::Thread
fn crack_instance(
    hash_list: Arc<HashSet<String>>,
    password_charset: Arc<Vec<String>>,
    password_prefix: String,
    password_gen_size: usize,
    algorithm: Algorithm,
    password_receiver: mpsc::Sender<CrackResult>,
) -> JoinHandle<()> {
    // build the thread
    let thread_name = format!("Cracker-{}", password_prefix);
    let built_thread = thread::Builder::new().name(thread_name);

    // spawn the thread
    let spawned_thread: thread::JoinHandle<_> = built_thread
        .spawn(move || {
            // Create a vector representing the password suffix to iterate through
            let mut password_indexing: Vec<isize> = Vec::new();
            let beginning_set = password_prefix.is_empty();
            for _ in 0..password_gen_size {
                if beginning_set {
                    password_indexing.push(-1);
                } else {
                    password_indexing.push(0);
                }
            }

            // loop through the password charset
            loop {
                // clone the password prefix
                let mut password_generated = password_prefix.clone();

                // increment the password_indexing
                password_indexing[password_gen_size - 1] += 1;

                // check for overflow
                for curser in (0..password_gen_size).rev() {
                    if password_indexing[curser] == password_charset.len() as isize {
                        if curser == 0 {
                            // notify the main thread that the thread is done
                            let sent_result = password_receiver.send(CrackResult {
                                hash: String::new(),
                                password: String::new(),
                                exit_done: true,
                                prefix: password_prefix.clone(),
                            });

                            // check if the result was sent
                            if sent_result.is_err() {
                                println!("Error sending result: {}", sent_result.err().unwrap());
                            }
                            return ();
                        }
                        // reset the current index and increment the previous index
                        password_indexing[curser] = 0;
                        password_indexing[curser - 1] += 1;
                    } else {
                        // no more overflow
                        break;
                    }
                }

                // generate password
                for index in 0..password_gen_size {
                    if password_indexing[index] != -1 {
                        password_generated
                            .push_str(password_charset[password_indexing[index] as usize].as_str());
                    }
                }

                // generate hash
                let hash: String = generate_hash(algorithm, password_generated.as_str());

                // check if hash is in hash_list
                if hash_list.contains(&hash) {
                    // send the result to the main thread
                    let sent_result = password_receiver.send(CrackResult {
                        hash,
                        password: password_generated,
                        exit_done: false,
                        prefix: password_prefix.clone(),
                    });

                    // check if the result was sent
                    if sent_result.is_err() {
                        println!("Error sending result: {}", sent_result.err().unwrap());
                    }
                }
            }
        })
        .unwrap();

    // return the thread handle
    spawned_thread
}
