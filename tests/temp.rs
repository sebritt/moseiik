#[cfg(test)]
mod tests {
    use moseiik::main::Options;
    use moseiik::main::compute_mosaic;
    use image::{GenericImageView, ImageReader, RgbaImage};

    #[test]
    #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
    fn test_x86() {
        // Set up the parameters for the mosaic
        let args = Options {
            image: String::from("tests/moseiik_test_images/kit.jpeg"),
            output: String::from("tests/moseiik_test_images/test_x86_out.png"),
            tiles: String::from("tests/moseiik_test_images/images"),
            scaling: 1,
            tile_size: 25,
            remove_used: false,
            verbose: false,
            simd: true, // test avx2 or sse2 if available
            num_thread: 1,
        };

        //Compute the mosaic
        compute_mosaic(args);

        // Load the ground truth and the produced mosaic
        let img_gt = ImageReader::open("assets/ground-truth-kit.png").unwrap().decode().unwrap().into_rgb8();
        let img_mosaic = ImageReader::open("tests/moseiik_test_images/test_x86_out.png").unwrap().decode().unwrap().into_rgb8();

        // Ensure both images have the same dimensions
        assert_eq!(img_gt.dimensions(), img_mosaic.dimensions());

        let mut difference = 0;

        // Compare pixel by pixel
        for i in 0..img_gt.width() { // i = {| 0, width-1 |}
            for j in 0..img_gt.height() { // j = {| 0, height-1 |}
                for rgb in 0..3 { // rgb = {0, 1, 2}
                    difference += i32::abs((img_gt.get_pixel(i,j)[rgb] as i32)-(img_mosaic.get_pixel(i,j)[rgb] as i32));
                }
            }
        }

        // Is there a difference ?
        assert_eq!(difference, 0);
    }

    #[test]
    #[cfg(target_arch = "aarch64")]
    fn test_aarch64() {
        // Set up the parameters for the mosaic
        let args = Options {
            image: String::from("tests/moseiik_test_images/kit.jpeg"),
            output: String::from("tests/moseiik_test_images/test_aarch_out.png"),
            tiles: String::from("tests/moseiik_test_images/images"),
            scaling: 1,
            tile_size: 25,
            remove_used: false,
            verbose: false,
            simd: true, // test neon if available
            num_thread: 1,
        };

        //Compute the mosaic
        compute_mosaic(args);

        // Load the ground truth and the produced mosaic
        let img_gt = ImageReader::open("assets/ground-truth-kit.png").unwrap().decode().unwrap().into_rgb8();
        let img_mosaic = ImageReader::open("tests/moseiik_test_images/test_aarch_out.png").unwrap().decode().unwrap().into_rgb8();

        // Ensure both images have the same dimensions
        assert_eq!(img_gt.dimensions(), img_mosaic.dimensions());

        let mut difference = 0;

        // Compare pixel by pixel
        for i in 0..img_gt.width() { // i = {| 0, width-1 |}
            for j in 0..img_gt.height() { // j = {| 0, height-1 |}
                for rgb in 0..3 { // rgb = {0, 1, 2}
                    difference += i32::abs((img_gt.get_pixel(i,j)[rgb] as i32)-(img_mosaic.get_pixel(i,j)[rgb] as i32));
                }
            }
        }

        // Is there a difference ?
        assert_eq!(difference, 0);
    }

    #[test]
    fn test_generic() {
        // Set up the parameters for the mosaic
        let args = Options {
            image: String::from("tests/moseiik_test_images/kit.jpeg"),
            output: String::from("tests/moseiik_test_images/test_generic_out.png"),
            tiles: String::from("tests/moseiik_test_images/images"),
            scaling: 1,
            tile_size: 25,
            remove_used: false,
            verbose: false,
            simd: false,
            num_thread: 1,
        };

        //Compute the mosaic
        compute_mosaic(args);

        // Load the ground truth and the produced mosaic
        let img_gt = ImageReader::open("assets/ground-truth-kit.png").unwrap().decode().unwrap().into_rgb8();
        let img_mosaic = ImageReader::open("tests/moseiik_test_images/test_generic_out.png").unwrap().decode().unwrap().into_rgb8();

        // Ensure both images have the same dimensions
        assert_eq!(img_gt.dimensions(), img_mosaic.dimensions());

        let mut difference = 0;

        // Compare pixel by pixel
        for i in 0..img_gt.width() { // i = {| 0, width-1 |}
            for j in 0..img_gt.height() { // j = {| 0, height-1 |}
                for rgb in 0..3 { // rgb = {0, 1, 2}
                    difference += i32::abs((img_gt.get_pixel(i,j)[rgb] as i32)-(img_mosaic.get_pixel(i,j)[rgb] as i32));
                }
            }
        }

        // Is there a difference ?
        assert_eq!(difference, 0);
    }
}
