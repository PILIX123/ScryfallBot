import os
import urllib.request
from multiprocessing import Pool

import requests
from cairosvg import svg2png

sympolgy = "https://api.scryfall.com/symbology"


def get_svg(d):
    uri = d.get("svg_uri")
    name: str = d.get("symbol")
    name = name.replace("{", "").replace("}", "").replace("/", "\\")
    urllib.request.urlretrieve(uri, f"svg/{name}.svg")
    with open(f"svg/{name}.svg") as f:
        svg2png(f.read(), output_height=128,
                output_width=128, write_to=f"png/{name}.png", background_color=0)


if __name__ == "__main__":
    os.makedirs("svg")
    os.makedirs("png")
    res = requests.get(sympolgy, headers={
        "User-Agent": "PILIXSrcyfallBot/0.1", "Accepts": "*/*"})
    json = res.json()
    data = json.get("data")
    print(len(data))
    with Pool(32) as p:
        p.map(get_svg, data)
