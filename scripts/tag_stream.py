import io
import time
import picamera
import picamera.array
import grpc
from concurrent import futures
import things_pb2
import things_pb2_grpc
from dt_apriltags import Detector
import numpy as np


width = 640
height = 480

def detection_to_proto(detection):
    # print('-------', detection)
    message = things_pb2.ApriltagDetection()
    message.id = detection.tag_id
    message.hamming = detection.hamming
    message.decision_margin = detection.decision_margin
    message.homography.h11 = detection.homography[0,0]
    message.homography.h12 = detection.homography[0,1]
    message.homography.h13 = detection.homography[0,2]
    message.homography.h21 = detection.homography[1,0]
    message.homography.h22 = detection.homography[1,1]
    message.homography.h23 = detection.homography[1, 2]
    message.homography.h31 = detection.homography[2, 0]
    message.homography.h32 = detection.homography[2, 1]
    message.homography.h33 = detection.homography[2, 2]
    message.center.x = detection.center[0]
    message.center.y = detection.center[1]
    message.corners.a.x = detection.corners[0][0]
    message.corners.a.y = detection.corners[0][1]
    message.corners.b.x = detection.corners[1][0]
    message.corners.b.y = detection.corners[1][1]
    message.corners.c.x = detection.corners[2][0]
    message.corners.c.y = detection.corners[2][1]
    message.corners.d.x = detection.corners[3][0]
    message.corners.d.y = detection.corners[3][1]
    message.pose.translation.x = detection.pose_t[0][0]
    message.pose.translation.y = detection.pose_t[1][0]
    message.pose.translation.z = detection.pose_t[2][0]
    message.pose.rotation.extend(detection.pose_R.flatten())
    message.pose.error = detection.pose_err
    return message


def rgb2gray(rgb):
    r, g, b = rgb[:, :, 0], rgb[:, :, 1], rgb[:, :, 2]
    gray = 0.2989 * r + 0.5870 * g + 0.1140 * b

    return np.array(gray, dtype=np.uint8)

class ProcessedImageStreamer(things_pb2_grpc.ProcessedImageStreamer):
    def StreamProcessedImages(self, request, context):
        try:
            detector = Detector(searchpath=['/usr/local/lib'],
                                families='tag36h11',
                                nthreads=3,
                                quad_decimate=1.0,
                                quad_sigma=0.8,
                                refine_edges=1,
                                decode_sharpening=0.25,
                                debug=0)

            with picamera.PiCamera(resolution=(width, height), framerate=1) as camera:
                # camera.resolution = (width, height)
                print("camera", camera)
                # Start a preview and let the camera warm up for 2 seconds
                camera.start_preview()
                time.sleep(2)
                with picamera.array.PiRGBArray(camera) as stream:
                    for foo in camera.capture_continuous(stream, 'rgb', use_video_port=True):
                        stream.flush()
                        # gray = np.mean(stream.array, axis=2, dtype=np.uint8)
                        gray =  rgb2gray(stream.array)
                        K = (329.8729619143081, 332.94611303946357, 528.0, 396.0)
                        detections = detector.detect(gray, estimate_tag_pose=True, camera_params=K, tag_size=0.08 )
                        print("gray.shape", gray.shape)
                        print("stream.array.shape", stream.array.shape)
                        gray3 = np.zeros((height, width, 3))
                        gray3[:, :, 1] = gray
                        
                        print("detected {} images".format(len(detections)))
                        
                        image_data = stream.array
                        # image_data = image_data.reshape((320, 2,
                        #                                 240, 2, 3)).max(3).max(1)
                        proto_image = things_pb2.Image(
                            width=width, height=height, image_data=image_data.tobytes())
                        proto_detections = map(detection_to_proto, detections)
                        message = things_pb2.ProcessedImage(
                            image=proto_image, 
                            apriltag_detections=proto_detections
                            )
                        stream.seek(0)
                        yield message
        except KeyboardInterrupt:
            print('interrupted!')
        except Exception as e:
            print(e)
            raise e
        finally:
            print('ended')

def serve():
  server = grpc.server(futures.ThreadPoolExecutor(max_workers=10))
  things_pb2_grpc.add_ProcessedImageStreamerServicer_to_server(
      ProcessedImageStreamer(), server)
  server.add_insecure_port('0.0.0.0:50053')
  print('connected')
  server.start()
  server.wait_for_termination()

serve()

    # connection.close()
    # client_socket.close()



