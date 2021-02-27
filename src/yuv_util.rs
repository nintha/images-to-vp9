/// Converts an RGB image to YUV420p (planar/3 planes)
///
/// # Arguments
///
/// * `img` - should contain the pixel data in the following format:
/// `[r, g, b, ... , r, g, b, ... , r, g, b, ...]`
///
/// * `bytes_per_pixel` - should contain the number of bytes used by one pixel
/// (eg.: RGB is 3 bytes and RGBA is 4 bytes)
///
/// # Return
///
/// `[y, y, y, ... , u, u, u, ... , v, v, v, ...]`
///
/// # Examples
///
/// ```
/// let rgb = vec![0u8; 12];
/// let yuv = rgb2yuv420::convert_rgb_to_yuv420p(&rgb, 2, 2, 3);
/// assert_eq!(yuv.len(), rgb.len() / 2);
/// ```
#[allow(unused)]
pub fn convert_rgb_to_yuv420p(img: &[u8], width: u32, height: u32, bytes_per_pixel: usize) -> Vec<u8> {
    convert_rgb_to_yuv420(img, width, height, bytes_per_pixel, |yuv, uv_index, chroma_size, u, v| {
        yuv[*uv_index] = u;
        yuv[*uv_index + chroma_size] = v;
        *uv_index += 1;
    })
}

/// Converts an RGB image to YUV420sp NV12 (semi-planar/2 planes)
///
/// # Arguments
///
/// * `img` - should contain the pixel data in the following format:
/// [r, g, b, ... , r, g, b, ... , r, g, b, ...]
///
/// * `bytes_per_pixel` - should contain the number of bytes used by one pixel
/// (eg.: RGB is 3 bytes and RGBA is 4 bytes)
///
/// # Return
///
/// `[y, y, y, ... , u, v, u, v, ...]`
///
/// # Examples
///
/// ```
/// let rgb = vec![0u8; 12];
/// let yuv = rgb2yuv420::convert_rgb_to_yuv420sp_nv12(&rgb, 2, 2, 3);
/// assert_eq!(yuv.len(), rgb.len() / 2);
/// ```
#[allow(unused)]
pub fn convert_rgb_to_yuv420sp_nv12(img: &[u8], width: u32, height: u32, bytes_per_pixel: usize) -> Vec<u8> {
    convert_rgb_to_yuv420(img, width, height, bytes_per_pixel, |yuv, uv_index, _cs, u, v| {
        yuv[*uv_index] = u;
        *uv_index += 1;
        yuv[*uv_index] = v;
        *uv_index += 1;
    })
}

fn convert_rgb_to_yuv420<T>(img: &[u8], width: u32, height: u32, bytes_per_pixel: usize, store_uv: T) -> Vec<u8>
    where T: Fn(&mut Vec<u8>, &mut usize, usize, u8, u8) -> () {
    let frame_size: usize = (width * height) as usize;
    let chroma_size: usize = frame_size / 4;
    // println!("frame_size={}, chroma_size={}", frame_size, chroma_size);

    let mut y_index: usize = 0;
    let mut uv_index = frame_size;
    let mut yuv = vec![0; (width * height * 3 / 2) as usize];
    let mut index: usize = 0;

    for j in 0..height {
        for _ in 0..width {
            let r = i32::from(img[index * bytes_per_pixel]);
            let g = i32::from(img[index * bytes_per_pixel + 1]);
            let b = i32::from(img[index * bytes_per_pixel + 2]);
            index += 1;
            yuv[y_index] = clamp((77 * r + 150 * g + 29 * b + 128) >> 8);
            y_index += 1;
            if j % 2 == 0 && index % 2 == 0 {
                store_uv(&mut yuv,
                         &mut uv_index,
                         chroma_size,
                         clamp(((-43 * r - 84 * g + 127 * b + 128) >> 8) + 128),
                         clamp(((127 * r - 106 * g - 21 * b + 128) >> 8) + 128));
            }
        }
    }
    yuv
}

fn clamp(val: i32) -> u8 {
    match val {
        ref v if *v < 0 => 0,
        ref v if *v > 255 => 255,
        v => v as u8,
    }
}
