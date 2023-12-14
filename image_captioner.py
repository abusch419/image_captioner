import sys
import requests
from PIL import Image
from transformers import BlipProcessor, BlipForConditionalGeneration
import logging

import os


logging.basicConfig(level=logging.INFO, format='%(message)s')

def get_caption(img_path):
    model_path = "Salesforce/blip-image-captioning-base"
    # General cache directory
    cache_dir = os.path.join(os.path.expanduser("~"), ".cache/huggingface")

    # Rely on transformers library for caching
    logging.info("🚨 Loading BLIP model.")
    processor = BlipProcessor.from_pretrained(model_path, cache_dir=cache_dir)
    model = BlipForConditionalGeneration.from_pretrained(model_path, cache_dir=cache_dir)
    logging.info("🤗 Model loaded successfully.")

    img = Image.open(img_path)

    # unconditional image captioning
    inputs = processor(img, return_tensors="pt")
    out = model.generate(**inputs, max_new_tokens=100)
    caption = processor.decode(out[0], skip_special_tokens=True)
    print(caption)
    return caption

if __name__ == "__main__":
    if len(sys.argv) > 1:
        img_path = sys.argv[1]  # Get the image path from the command line argument
        get_caption(img_path)
    else:
        print("No image path provided.")
