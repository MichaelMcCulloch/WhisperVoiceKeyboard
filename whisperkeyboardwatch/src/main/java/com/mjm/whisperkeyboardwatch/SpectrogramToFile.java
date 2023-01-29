package com.mjm.whisperkeyboardwatch;


import android.graphics.Bitmap;
import android.graphics.Color;

import java.io.FileOutputStream;
import java.io.IOException;

public class SpectrogramToFile {
    /**
     * This method is used to save a float array as a spectrogram image.
     *
     * @param spectrogram The input array to be saved as an image.
     * @param savePath    The path to save the image.
     */
    public static void save(float[] spectrogram, String savePath) {
        Bitmap bitmap = Bitmap.createBitmap(3000, 80, Bitmap.Config.ARGB_8888);

        //Loop through the float array and save each value as a pixel in the bitmap
        int x = 0;
        int y = 0;
        for (float f : spectrogram) {
            int color = (int) ((f + 1) / 2 * 255);
            bitmap.setPixel(y, x, Color.argb(255, color, color, color));
            x++;
            if (x >= 80) {
                x = 0;
                y++;
            }
        }
        //write the bitmap to file
        try {
            FileOutputStream out = new FileOutputStream(savePath + "/spectrogram" + System.nanoTime() + ".png");
            bitmap.compress(Bitmap.CompressFormat.PNG, 100, out);
            out.close();
        } catch (IOException e) {
            e.printStackTrace();
        }
    }
}
