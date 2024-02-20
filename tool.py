import argparse
from pathlib import Path

from shrink_img import resize_image_buffer, guess_image_format


parser = argparse.ArgumentParser()
parser.add_argument("src")
parser.add_argument("max_size", help="WxH")
parser.add_argument("dest")

args = parser.parse_args()

src_data = Path(args.src).resolve().read_bytes()
width, height = [int(s) for s in args.max_size.split("x")]
mime_type = guess_image_format(src_data)
print("mime type:", mime_type)
dest_data = resize_image_buffer(src_data, width, height)
Path(args.dest).resolve().write_bytes(dest_data)
