const fs = require("fs");
const path = require("path");
const sharp = require("sharp");

// Get all files in the out dir
const files = fs.readdirSync("./out").sort((a, b) => {
  const aVal = +path.basename(a).split(".")[0];
  const bVal = +path.basename(b).split(".")[0];
  return aVal - bVal;
});

function toMax(c) {
  if (c > 128) return 255;
  return 0;
}

function getStruct(structData) {
  // Layout in memory:
  // 16-bit // repeat // top bit next // second bit black // 14-bit i

  // Is an uin8array
  // Remember little endian

  if (structData.count > 0x3fff) {
    throw new Error("Count is too big");
  }
  // if (structData.i > 0x7fff) {
  //   throw new Error("i is too big");
  // }

  // const i = (structData.i & 0x7fff) | (structData.black ? 0x8000 : 0);
  const count =
    structData.count |
    (structData.next ? 0x8000 : 0) |
    (structData.black ? 0x4000 : 0);

  // const final = (count << 16) | i;
  // if (final > 0xffffff) {
  //   throw new Error("Final is too big");
  // }

  return [(count & 0xff) >> 0, (count & 0xff00) >> 8];
}

const framesPromise = Promise.all(
  files.map(async (file) => {
    const data = await sharp(path.join("./out", file))
      .raw()
      .toBuffer();

    const imageData = new Array(160 * 120).fill(0);

    const height = 480;
    const width = 360;

    for (let x = 0; x < 160; x++) {
      for (let y = 0; y < 120; y++) {
        let final = 0;

        for (let cx = 0; cx < 3; cx++) {
          for (let cy = 0; cy < 3; cy++) {
            const pixelIndex = ((y * 3 + cy) * width + (x * 3 + cx)) * 3;
            const r = toMax(data[pixelIndex]); // Red channel
            const g = toMax(data[pixelIndex + 1]); // Green channel
            const b = toMax(data[pixelIndex + 2]); // Blue channel
            const a = (r + g + b) / 3;
            final += a;
          }
        }

        final /= 9;
        final = toMax(final);
        final = toMax(data[(y * width + x) * 8]);

        imageData[y * 160 + x] = final;
      }
    }

    return imageData;
  }),
);

(async () => {
  const frames = await framesPromise;

  const structs = [];
  const actual = [];

  for (const frame of frames) {
    let lastBlack = null;
    let count = 0;
    let lastI = 0;
    let next = true;

    for (let x = 0; x < 160; x++) {
      for (let y = 0; y < 120; y++) {
        let i = y * 160 + x;
        const black = frame[i] === 0;

        if (lastBlack === null) {
          lastBlack = black;
          lastI = i;
          count = 1;
          continue;
        } else if (lastBlack !== black || count === 0x3fff) {
          const data = {
            black: lastBlack,
            count,
            next,
          };
          actual.push(data);
          structs.push(getStruct(data));
          next = false;
          lastBlack = black;
          lastI = i;
          count = 1;
        } else {
          count++;
          continue;
        }
      }
    }

    const data = {
      black: lastBlack,
      count,
      next,
    };
    actual.push(data);
    structs.push(getStruct(data));
  }

  console.log(actual[2]);
  console.log(structs[2]);
  const buffer = Buffer.alloc(structs.length * 2);
  for (const [i, [a, b]] of structs.entries()) {
    buffer[i * 2] = a;
    buffer[i * 2 + 1] = b;
  }
  fs.writeFileSync("out.bin", buffer);
})();
