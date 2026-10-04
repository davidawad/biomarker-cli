# /// script
# requires-python = ">=3.9"
# dependencies = ["pillow>=10", "matplotlib>=3.7"]
# ///
r"""Render an ANSI terminal transcript (stdin) as a terminal-window PNG.

    uv run scripts/render-terminal.py OUT.png [--title TITLE] [--cols N] < transcript

Lines starting with `$ `, and their `\` continuation lines, are drawn as
commands; everything else is drawn with the SGR colours it carries. Long
lines soft-wrap at --cols, like a terminal. The font is DejaVu Sans Mono,
which ships inside matplotlib, so the output does not depend on the fonts
installed on the machine.
"""

import argparse
import os
import re
import sys

import matplotlib
from PIL import Image, ImageDraw, ImageFont

SCALE = 2
FONT_PX = 15 * SCALE
LINE_H = round(FONT_PX * 1.4)
PAD_X = 22 * SCALE
PAD_Y = 18 * SCALE
BAR_H = 34 * SCALE
RADIUS = 10 * SCALE

BG = (30, 33, 40)
BAR = (44, 48, 57)
FG = (205, 210, 218)
DIM = (120, 127, 140)
PROMPT = (110, 200, 140)
COMMAND = (235, 238, 243)
TITLE = (150, 156, 168)
# Normal and bright ANSI colours 0-7.
PALETTE = [
    (60, 64, 72), (240, 100, 100), (110, 200, 140), (230, 190, 100),
    (100, 160, 240), (200, 130, 220), (90, 190, 200), (205, 210, 218),
]
BRIGHT = [
    (110, 116, 128), (255, 130, 130), (140, 225, 165), (245, 210, 130),
    (130, 185, 255), (220, 160, 235), (120, 215, 225), (245, 247, 250),
]
DOTS = [(255, 95, 87), (254, 188, 46), (40, 200, 64)]

SGR = re.compile(r"\x1b\[([0-9;]*)m")


def fonts():
    d = os.path.join(os.path.dirname(matplotlib.__file__), "mpl-data", "fonts", "ttf")
    return (
        ImageFont.truetype(os.path.join(d, "DejaVuSansMono.ttf"), FONT_PX),
        ImageFont.truetype(os.path.join(d, "DejaVuSansMono-Bold.ttf"), FONT_PX),
    )


def cells(line):
    """Split one line into (char, fg, bold) cells, applying SGR codes."""
    out, fg, bold, pos = [], None, False, 0
    for m in list(SGR.finditer(line)) + [None]:
        end = m.start() if m else len(line)
        out += [(ch, fg, bold) for ch in line[pos:end]]
        if m is None:
            break
        pos = m.end()
        for code in [int(c) for c in m.group(1).split(";") if c] or [0]:
            if code == 0:
                fg, bold = None, False
            elif code == 1:
                bold = True
            elif code == 22:
                bold = False
            elif code == 2:
                fg = DIM
            elif 30 <= code <= 37:
                fg = PALETTE[code - 30]
            elif 90 <= code <= 97:
                fg = BRIGHT[code - 90]
            elif code == 39:
                fg = None
    return out


def rows(text, cols):
    """Terminal rows: SGR-styled cells, soft-wrapped at `cols`."""
    out, continued = [], False
    for line in text.rstrip("\n").split("\n"):
        if line.startswith("$ ") or continued:
            plain = SGR.sub("", line)
            c = [(ch, COMMAND, True) for ch in plain]
            if not continued:
                c[0] = ("$", PROMPT, True)
            continued = plain.endswith("\\")
        else:
            c = cells(line)
        out += [c[i : i + cols] for i in range(0, max(len(c), 1), cols)]
    return out


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("out")
    ap.add_argument("--title", default="biomarker")
    ap.add_argument("--cols", type=int, default=100)
    args = ap.parse_args()

    regular, bold = fonts()
    grid = rows(sys.stdin.read(), args.cols)
    cell_w = regular.getlength("M")
    width = round(PAD_X * 2 + cell_w * max(len(r) for r in grid))
    height = BAR_H + PAD_Y * 2 + LINE_H * len(grid)

    img = Image.new("RGB", (width, height), (255, 255, 255))
    mask = Image.new("L", (width, height), 0)
    ImageDraw.Draw(mask).rounded_rectangle((0, 0, width - 1, height - 1), RADIUS, fill=255)
    win = Image.new("RGB", (width, height), BG)
    d = ImageDraw.Draw(win)
    d.rectangle((0, 0, width, BAR_H), fill=BAR)
    for i, colour in enumerate(DOTS):
        cx, cy, r = PAD_X + i * 20 * SCALE, BAR_H // 2, 6 * SCALE
        d.ellipse((cx - r, cy - r, cx + r, cy + r), fill=colour)
    d.text((width / 2, BAR_H / 2), args.title, font=regular, fill=TITLE, anchor="mm")

    for y, row in enumerate(grid):
        top = BAR_H + PAD_Y + y * LINE_H
        for x, (ch, fg, is_bold) in enumerate(row):
            if ch != " ":
                d.text((PAD_X + x * cell_w, top), ch, font=bold if is_bold else regular, fill=fg or FG)

    img.paste(win, (0, 0), mask)
    # Transparent corners so the rounded window sits on any page background.
    img.putalpha(mask)
    img.save(args.out, optimize=True)


if __name__ == "__main__":
    main()
