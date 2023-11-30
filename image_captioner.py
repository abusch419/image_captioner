import sys
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
    caption = processor.decode(out[0], skip_special_tokens=True)
    print(caption)
    return caption

if __name__ == "__main__":
    if len(sys.argv) > 1:
        img_path = sys.argv[1]  # Get the image path from the command line argument
        get_caption(img_path)
    else:
        print("No image path provided.")
