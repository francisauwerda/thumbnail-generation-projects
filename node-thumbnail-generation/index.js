import path from "path";
import { readdir, mkdir } from "fs/promises";
import { promises as fs } from "fs";
import sharp from "sharp";
import heicConvert from "heic-convert";

const INPUT_DIR = "/app/images";
const OUTPUT_DIR = "/app/thumbnails";

async function measureTime(label, operation) {
  const start = process.hrtime.bigint();
  await operation();
  const end = process.hrtime.bigint();
  const durationMs = Number(end - start) / 1_000_000;
  console.log(`✅ ${label}: ${durationMs.toFixed(2)} ms`);
}

async function processFile(file) {
  const sourcePath = path.join(INPUT_DIR, file);
  const extension = path.extname(file).toLowerCase();
  const outputFilename = `${path.basename(file, extension)}-thumbnail.png`;
  const destinationPath = path.join(OUTPUT_DIR, outputFilename);

  try {
    if ([".jpg", ".jpeg", ".png", ".webp", ".gif"].includes(extension)) {
      await measureTime(`Processed image [${file}]`, () =>
        sharp(sourcePath)
          .rotate()
          .resize({ width: 100, height: 100, fit: "cover" })
          .toFile(destinationPath)
      );
    } else if (extension === ".heic" || extension === ".heif") {
      await measureTime(`Processed HEIC [${file}]`, async () => {
        const inputBuffer = await fs.readFile(sourcePath);
        const outputBuffer = await heicConvert({
          buffer: inputBuffer,
          format: "PNG",
        });
        await sharp(outputBuffer)
          .resize({ width: 100, height: 100, fit: "cover" })
          .toFile(destinationPath);
      });
    } else {
      console.log(`- Skipping unsupported file: ${file}`);
    }
  } catch (error) {
    console.error(`❌ Error processing ${file}:`, error.message);
  }
}

async function main() {
  console.log("--- Starting Node.js Thumbnail Generation ---");
  const totalStart = process.hrtime.bigint();

  try {
    await mkdir(OUTPUT_DIR, { recursive: true });

    const files = await readdir(INPUT_DIR);

    if (files.length === 0) {
      console.log("No files found in the images directory.");
      return;
    }

    await Promise.all(files.map(processFile));
  } catch (error) {
    console.error("An unexpected error occurred:", error);
  } finally {
    const totalEnd = process.hrtime.bigint();
    const totalDurationMs = Number(totalEnd - totalStart) / 1_000_000;
    const totalDurationInSeconds = totalDurationMs / 1000;
    console.log(`\n--- Finished in ${totalDurationInSeconds.toFixed(2)}s ---`);
  }
}

main();
