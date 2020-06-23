use apriltag::{DetectorBuilder, Family};
use std::error::Error;
use std::string::String;
use things::image_streamer_client::ImageStreamerClient;
use things::processed_image_streamer_server::{
    ProcessedImageStreamer, ProcessedImageStreamerServer,
};
use things::{apriltag_detection, ApriltagDetection, Empty, Ping, ProcessedImage};
use tokio::sync::mpsc;
use tonic::transport::Server;
use tonic::{Request, Response, Status};

pub mod things {
    tonic::include_proto!("things");
}

#[derive(Debug)]
struct ProcessedImageStreamerService;

#[tonic::async_trait]
impl ProcessedImageStreamer for ProcessedImageStreamerService {
    async fn echo(&self, request: Request<Ping>) -> Result<Response<Ping>, Status> {
        println!("request {:?}", request);
        Ok(Response::new(Ping {
            content: String::from("pong"),
        }))
    }

    type StreamProcessedImagesStream = mpsc::Receiver<Result<ProcessedImage, Status>>;

    async fn stream_processed_images(
        &self,
        _request: Request<Empty>,
    ) -> Result<Response<Self::StreamProcessedImagesStream>, Status> {
        let (mut tx, rx) = mpsc::channel(4);
        let mut client = ImageStreamerClient::connect("http://192.168.50.222:50052").await.unwrap();
        let mut stream = client
            .stream_images(tonic::Request::new(Empty{}))
            .await?
            .into_inner();
        tokio::spawn(async move {
            while let Some(img) = stream.message().await.unwrap() {
                println!("message in");
                // println!("NOTE = {:?}", img.image_data);
                let loaded_image = image::load_from_memory_with_format(&img.image_data, image::ImageFormat::Jpeg).unwrap();
                // loaded_image.save("image.jpeg");
                let apriltag_detections = detect(loaded_image);
                let message = ProcessedImage {
                    image: Some(img),
                    apriltag_detections: apriltag_detections,
                };
                tx.send(Ok(message)).await.unwrap();
            }
        });

        Ok(Response::new(rx))
    }

    type ListFeaturesStream = mpsc::Receiver<Result<Ping, Status>>;

    async fn list_features(
        &self,
        request: Request<Ping>,
    ) -> Result<Response<Self::ListFeaturesStream>, Status> {
        println!("into da stream");
        let (mut tx, rx) = mpsc::channel(4);

        tokio::spawn(async move {
            tx.send(Ok(Ping {
                content: String::from("pong"),
            }))
            .await
            .unwrap();
            tx.send(Ok(Ping {
                content: String::from("pong2"),
            }))
            .await
            .unwrap();
        });

        Ok(Response::new(rx))
    }
}

pub fn detect(loaded_image: image::DynamicImage) -> Vec<ApriltagDetection> {
    let mut detector = DetectorBuilder::new()
        .add_family_bits(Family::tag_36h11(), 1)
        .build()
        .unwrap();
    let image = loaded_image;
    let detections = detector.detect(image.to_luma());

    println!("{} tags found", detections.len());
    // detections.into_iter().enumerate().for_each(|(index, det)| {
    //     println!(
    //         "- detection {}\n\
    //             id: {}\n\
    //             hamming: {}\n\
    //             decision_margin: {}\n\
    //             center: {:?}\n\
    //             corners: {:?}\n\
    //             homography: {:?}\n\
    //             ",
    //         index,
    //         det.id(),
    //         det.hamming(),
    //         det.decision_margin(),
    //         det.center(),
    //         det.corners(),
    //         det.homography().data()
    //     );
    // });

    let detections = detections
        .into_iter()
        .map(|det| -> ApriltagDetection {
            let center = det.center();
            let corners = det.corners();
            let homography = det.homography().data();
            let msg = ApriltagDetection {
                id: det.id() as u32,
                hamming: 0.0, //det.hamming(),
                decision_margin: det.decision_margin(),
                center: Some(apriltag_detection::Point {
                    x: center[0] as f32,
                    y: center[1] as f32,
                }),
                corners: Some(apriltag_detection::Corners {
                    a: Some(apriltag_detection::Point {
                        x: corners[0][0] as f32,
                        y: corners[0][1] as f32,
                    }),
                    b: Some(apriltag_detection::Point {
                        x: corners[1][0] as f32,
                        y: corners[1][1] as f32,
                    }),
                    c: Some(apriltag_detection::Point {
                        x: corners[2][0] as f32,
                        y: corners[2][1] as f32,
                    }),
                    d: Some(apriltag_detection::Point {
                        x: corners[3][0] as f32,
                        y: corners[3][1] as f32,
                    }),
                }),
                homography: Some(apriltag_detection::Homography {
                    h11: homography[0] as f32,
                    h12: homography[1] as f32,
                    h13: homography[2] as f32,
                    h21: homography[3] as f32,
                    h22: homography[4] as f32,
                    h23: homography[5] as f32,
                    h31: homography[6] as f32,
                    h32: homography[7] as f32,
                    h33: homography[8] as f32,
                }),
            };
            println!("msg {:?}", msg);
            msg
        })
        .collect();

    detections
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    println!("running...");

    let addr = "127.0.0.1:10000".parse().unwrap();

    let servicer = ProcessedImageStreamerService {};

    let svc = ProcessedImageStreamerServer::new(servicer);

    Server::builder().add_service(svc).serve(addr).await?;
    Ok(())
}
