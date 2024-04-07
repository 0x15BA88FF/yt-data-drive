fn run() {
    let mut encoding_time: f64;
    let mut encoding_blob_size: f64;

    let mut decoding_time: f64;
    let mut decoding_blob_size: f64;

    println!("Encoding time average was {} Mb/s", encoding_blob_size / encoding_time);
    println!("Decoding time average was {} Mb/s", decoding_blob_size / decoding_time);
}
