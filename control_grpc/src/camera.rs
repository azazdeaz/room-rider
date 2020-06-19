use things::image_streamer_client::ImageStreamerClient;
use things::{ Empty};
use std::error::Error;
use std::path::Path;
use std::io::Read;
use image::jpeg::JpegDecoder;


pub mod things {
    tonic::include_proto!("things");
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let mut client = ImageStreamerClient::connect("http://192.168.50.222:50052").await?;
    let mut stream = client
    .stream_images(tonic::Request::new(Empty{}))
    .await?
    .into_inner();
    
    while let Some(img) = stream.message().await? {
        println!("NOTE = {:?}", img.image_data);
        let loaded_image = image::load_from_memory_with_format(&img.image_data, image::ImageFormat::Jpeg).unwrap();
        loaded_image.save("image.jpeg");
        
        
    }
    
    Ok(())
}
