import { v2 as cloudinary } from "cloudinary";
import multer from "multer";

cloudinary.config({
  cloud_name: "daobz9adz",
  api_key: "837525829992755",
  api_secret: "AokrmT4Eft-dx73JZdGlIcao5q0",
});

const storage = new multer.memoryStorage();

export async function imageUploadUtil(file) {
  const result = await cloudinary.uploader.upload(file, {
    resource_type: "auto",
  });
  return result;
}

export const upload = multer({
  storage,
});
