use things::image_streamer_client::ImageStreamerClient;
use things::{ Empty};
use std::error::Error;
use std::path::Path;
use std::io::Read;
use image::jpeg::JpegDecoder;
use failure::Fallible;
use apriltag::{DetectorBuilder, Family};



pub mod things {
    tonic::include_proto!("things");
}

pub fn detect(loaded_image:image::DynamicImage) -> Fallible<()> {

    let mut detector = DetectorBuilder::new()
            .add_family_bits(Family::tag_16h5(), 1)
            .build()
            .unwrap();
    
    let image = loaded_image;
    let detections = detector.detect(image.to_luma());


    detections.into_iter().enumerate().for_each(|(index, det)| {
        println!(
            "- detection {}\n\
                id: {}\n\
                hamming: {}\n\
                decision_margin: {}\n\
                center: {:?}\n\
                corners: {:?}\n\
                homography: {:?}\n\
                ",
            index,
            det.id(),
            det.hamming(),
            det.decision_margin(),
            det.center(),
            det.corners(),
            det.homography().data()
        );
    });

    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let mut client = ImageStreamerClient::connect("http://192.168.50.222:50052").await?;
    let mut stream = client
    .stream_images(tonic::Request::new(Empty{}))
    .await?
    .into_inner();
    
    while let Some(img) = stream.message().await? {
        // println!("NOTE = {:?}", img.image_data);
        let loaded_image = image::load_from_memory_with_format(&img.image_data, image::ImageFormat::Jpeg).unwrap();
        // loaded_image.save("image.jpeg");
        detect(loaded_image);
        
    }
    
    Ok(())
}
