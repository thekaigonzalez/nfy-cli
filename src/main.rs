fn main() {
    use std::fs::File;
    use std::io::BufReader;
    
    use rodio::{Decoder, OutputStream, Sink};
    use argparse::{ArgumentParser, StoreTrue, Store};

    let mut verbose = false;
    let mut dir = "songs".to_string();
    let mut volume = 3.0;
    let mut song_name = "".to_string();

    
    {  // this block limits scope of borrows by ap.refer() method
        let mut ap = ArgumentParser::new();
        
        ap.set_description("Play music on the NFy API.");
        
        ap.refer(&mut verbose)
            .add_option(&["-v", "--verbose"], StoreTrue,
            "Be verbose");
        ap.refer(&mut dir)
            .add_option(&["--sdir"], Store,
            "Song Directory. \"Songs\" by default.");
        
        ap.refer(&mut song_name).add_option(&["--song"], Store, 
        "Song name");

        ap.refer(&mut volume).add_option(&["-o"], Store, 
        "Change volume: For hearing it's recommended around 3.0.");

        ap.parse_args_or_exit();
    }
    
    // m_s.sleep_until_end();

    // let rd = glob(format!("./{}/*.{}", dir, name)).unwrap();

    let array: Vec<String> = Vec::new();

    
    // for i in rd {
    //     let mut f: File = File::create("./test.txt").expect("Failed to create test.txt");
    //     let mut oop = OpenOptions::new()
    //         .append(true)
    //         .open("test1.txt")
    //         .unwrap();

    //     oop.write(b"Hello!\n").expect("Error appending");
    //     oop.write(b"Hello!").expect("Error appending again");
    //     f.write(b"Hello!").expect("failed to write to test.txt buffer");
    //     array.push((&i.as_ref().unwrap().display()).to_string());
    // }

    
    println!("{:?}", array);
    println!("Hello, world!");


    let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    let file = BufReader::new(File::open(format!("./{}/{}.ogg", dir, song_name)).unwrap());
    let source = Decoder::new(file).unwrap();
    let m_s = Sink::try_new(&stream_handle).unwrap();
    m_s.append(source);
    m_s.set_volume(volume);
    m_s.sleep_until_end();
    
}
