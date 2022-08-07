
pub mod binarychannel;
pub use binarychannel::BinaryChannel;


    // pub fn AppendLinecompressedToFile(path : &Path, bytes : &[u8]) -> u64{
    //     // let mut buf: ByteBuf = bincode::deserialize(&BinaryChannel::file_to_bytes(path)).unwrap();
    //     // buf.append(&mut bytes.to_owned());
    //     let mut f = OpenOptions::new().append(true).open(path).expect("cannot open file");
    //     let pos_i = f.seek(SeekFrom::End(0)).unwrap();
    //     f.write_all(&bytes.chunks(2).map(|x| 
    //         EncodeMap::compress(
    //             x.get(0).unwrap_or(&EncodeMap::char_to_bytes(&'-').unwrap()),
    //             x.get(1).unwrap_or(&EncodeMap::char_to_bytes(&'-').unwrap())
    //         )
    //     ).collect::<Vec<u8>>()).expect("write failed");
    //     pos_i 
    // }
    // pub fn compressFile(path : &Path) {
    //     println!("{:?}", Command::new("./bin/zstd")
    //     .arg(path.to_str().unwrap().to_owned())
    //     .arg("--rm")
    //     .output()
    //     .expect("failed to execute process"));
    // }

    // pub fn uncompressFile(path : &Path) {
    //     Command::new("./bin/zstd")
    //     .arg("-d")
    //     .arg(path.to_str().unwrap().to_owned())
    //     .arg("--rm")
    //     .output()
    //     .expect("failed to execute process");
    //     fs::remove_file(path.to_str().unwrap().to_owned() + ".bloc");
    // }

    

    // pub fn AppendLineToFile(path : &Path, bytes : &[u8]) -> u64{
    //     // let mut buf: ByteBuf = bincode::deserialize(&BinaryChannel::file_to_bytes(path)).unwrap();
    //     // buf.append(&mut bytes.to_owned());
    //     let mut f = OpenOptions::new().append(true).open(path).expect("cannot open file");
    //     let pos_i = f.seek(SeekFrom::End(0)).unwrap();
    //     f.write_all(&bytes).expect("write failed");
    //     pos_i
    // }