# -*- coding: utf-8 -*-
# Generated by the protocol buffer compiler.  DO NOT EDIT!
# source: things.proto

from google.protobuf import descriptor as _descriptor
from google.protobuf import message as _message
from google.protobuf import reflection as _reflection
from google.protobuf import symbol_database as _symbol_database
# @@protoc_insertion_point(imports)

_sym_db = _symbol_database.Default()




DESCRIPTOR = _descriptor.FileDescriptor(
  name='things.proto',
  package='things',
  syntax='proto3',
  serialized_options=None,
  serialized_pb=b'\n\x0cthings.proto\x12\x06things\":\n\x05Image\x12\r\n\x05width\x18\x01 \x01(\r\x12\x0e\n\x06height\x18\x02 \x01(\r\x12\x12\n\nimage_data\x18\x03 \x01(\x0c\"\x07\n\x05\x45mpty2A\n\rImageStreamer\x12\x30\n\x0cStreamImages\x12\r.things.Empty\x1a\r.things.Image\"\x00\x30\x01\x62\x06proto3'
)




_IMAGE = _descriptor.Descriptor(
  name='Image',
  full_name='things.Image',
  filename=None,
  file=DESCRIPTOR,
  containing_type=None,
  fields=[
    _descriptor.FieldDescriptor(
      name='width', full_name='things.Image.width', index=0,
      number=1, type=13, cpp_type=3, label=1,
      has_default_value=False, default_value=0,
      message_type=None, enum_type=None, containing_type=None,
      is_extension=False, extension_scope=None,
      serialized_options=None, file=DESCRIPTOR),
    _descriptor.FieldDescriptor(
      name='height', full_name='things.Image.height', index=1,
      number=2, type=13, cpp_type=3, label=1,
      has_default_value=False, default_value=0,
      message_type=None, enum_type=None, containing_type=None,
      is_extension=False, extension_scope=None,
      serialized_options=None, file=DESCRIPTOR),
    _descriptor.FieldDescriptor(
      name='image_data', full_name='things.Image.image_data', index=2,
      number=3, type=12, cpp_type=9, label=1,
      has_default_value=False, default_value=b"",
      message_type=None, enum_type=None, containing_type=None,
      is_extension=False, extension_scope=None,
      serialized_options=None, file=DESCRIPTOR),
  ],
  extensions=[
  ],
  nested_types=[],
  enum_types=[
  ],
  serialized_options=None,
  is_extendable=False,
  syntax='proto3',
  extension_ranges=[],
  oneofs=[
  ],
  serialized_start=24,
  serialized_end=82,
)


_EMPTY = _descriptor.Descriptor(
  name='Empty',
  full_name='things.Empty',
  filename=None,
  file=DESCRIPTOR,
  containing_type=None,
  fields=[
  ],
  extensions=[
  ],
  nested_types=[],
  enum_types=[
  ],
  serialized_options=None,
  is_extendable=False,
  syntax='proto3',
  extension_ranges=[],
  oneofs=[
  ],
  serialized_start=84,
  serialized_end=91,
)

DESCRIPTOR.message_types_by_name['Image'] = _IMAGE
DESCRIPTOR.message_types_by_name['Empty'] = _EMPTY
_sym_db.RegisterFileDescriptor(DESCRIPTOR)

Image = _reflection.GeneratedProtocolMessageType('Image', (_message.Message,), {
  'DESCRIPTOR' : _IMAGE,
  '__module__' : 'things_pb2'
  # @@protoc_insertion_point(class_scope:things.Image)
  })
_sym_db.RegisterMessage(Image)

Empty = _reflection.GeneratedProtocolMessageType('Empty', (_message.Message,), {
  'DESCRIPTOR' : _EMPTY,
  '__module__' : 'things_pb2'
  # @@protoc_insertion_point(class_scope:things.Empty)
  })
_sym_db.RegisterMessage(Empty)



_IMAGESTREAMER = _descriptor.ServiceDescriptor(
  name='ImageStreamer',
  full_name='things.ImageStreamer',
  file=DESCRIPTOR,
  index=0,
  serialized_options=None,
  serialized_start=93,
  serialized_end=158,
  methods=[
  _descriptor.MethodDescriptor(
    name='StreamImages',
    full_name='things.ImageStreamer.StreamImages',
    index=0,
    containing_service=None,
    input_type=_EMPTY,
    output_type=_IMAGE,
    serialized_options=None,
  ),
])
_sym_db.RegisterServiceDescriptor(_IMAGESTREAMER)

DESCRIPTOR.services_by_name['ImageStreamer'] = _IMAGESTREAMER

# @@protoc_insertion_point(module_scope)
