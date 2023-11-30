# image_captioner.py
import requests
from PIL import Image
from transformers import BlipProcessor, BlipForConditionalGeneration

def get_caption(img_path):
    processor = BlipProcessor.from_pretrained("Salesforce/blip-image-captioning-large")
    model = BlipForConditionalGeneration.from_pretrained("Salesforce/blip-image-captioning-large")

    img = Image.open(img_path)

    # unconditional image captioning
    inputs = processor(img, return_tensors="pt")
    out = model.generate(**inputs, max_new_tokens=100)
    print(processor.decode(out[0], skip_special_tokens=True))
    return processor.decode(out[0], skip_special_tokens=True)
