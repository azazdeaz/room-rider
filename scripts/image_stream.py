import io
import time
import picamera
import grpc
from concurrent import futures
import things_pb2
import things_pb2_grpc


width = 640
height = 480

class ImageStreamer(things_pb2_grpc.ImageStreamer):

    def StreamImages(self, request, context):
        try:
            with picamera.PiCamera() as camera:
                camera.resolution = (width, height)
                print("camera", camera)
                # Start a preview and let the camera warm up for 2 seconds
                camera.start_preview()
                time.sleep(2)

                # Note the start time and construct a stream to hold image data
                # temporarily (we could write it directly to connection but in this
                # case we want to find out the size of each capture first to keep
                # our protocol simple)
                start = time.time()
                stream = io.BytesIO()
                for foo in camera.capture_continuous(stream, 'jpeg', use_video_port=True):
                    # Write the length of the capture to the stream and flush to
                    # ensure it actually gets sent
                    # connection.write(struct.pack('<L', stream.tell()))
                    # connection.flush()
                    # Rewind the stream and send the image data over the wire
                    stream.seek(0)
                    message = stream.read()
                    message = things_pb2.Image(width=width, height=height, image_data=message)
                    yield message
                    print("image sent")


                    # Reset the stream for the next capture
                    stream.seek(0)
            # Write a length of zero to the stream to signal we're done
            # connection.write(struct.pack('<L', 0))
        except KeyboardInterrupt:
            print('interrupted!')
        finally:
            print('ended')

def serve():
  server = grpc.server(futures.ThreadPoolExecutor(max_workers=10))
  things_pb2_grpc.add_ImageStreamerServicer_to_server(
      ImageStreamer(), server)
  server.add_insecure_port('0.0.0.0:50052')
  print('connected')
  server.start()
  server.wait_for_termination()

serve()

    # connection.close()
    # client_socket.close()