from pathlib import Path

from PIL import Image, ImageDraw

ACCENT = (249, 115, 22, 255)
INK = (43, 22, 8, 255)
GRID = 64
SCALE = 16
PIXELS = GRID * SCALE
ASSETS = Path(__file__).resolve().parent.parent / "assets"


def box(pen, x, y, w, h, r, fill):
    pen.rounded_rectangle(
        [x * SCALE, y * SCALE, (x + w) * SCALE, (y + h) * SCALE],
        radius=r * SCALE,
        fill=fill,
    )


def render(pixels):
    art = Image.new("RGBA", (PIXELS, PIXELS), (0, 0, 0, 0))
    pen = ImageDraw.Draw(art)
    box(pen, 4, 4, 56, 56, 14, ACCENT)
    box(pen, 27, 13, 10, 5, 2.5, INK)
    box(pen, 19, 20, 26, 5, 2.5, INK)
    pen.polygon(
        [
            (23 * SCALE, 27 * SCALE),
            (41 * SCALE, 27 * SCALE),
            (38.6 * SCALE, 49.4 * SCALE),
            (25.4 * SCALE, 49.4 * SCALE),
        ],
        fill=INK,
    )
    box(pen, 28.6, 33, 2.8, 11, 1.4, ACCENT)
    box(pen, 32.6, 33, 2.8, 11, 1.4, ACCENT)
    return art.resize((pixels, pixels), Image.LANCZOS)


render(64).save(ASSETS / "icon-64.png")
render(256).save(
    ASSETS / "icon.ico", sizes=[(16, 16), (32, 32), (48, 48), (256, 256)]
)
print("icons written")
