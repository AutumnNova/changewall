	#[cfg(test)]
	mod tests {
		use crate::colors;

		#[test]
		fn test_rgb2hex() {
			assert_eq!(colors::rgb2hex(vec![ 255, 255, 255]), "#ffffff");
			assert_eq!(colors::rgb2hex(vec![ 127, 127, 127]), "#7f7f7f");
			assert_eq!(colors::rgb2hex(vec![ 0, 0, 0]), "#000000");
		}

		#[test]
		fn test_hex2rgb() {
			assert_eq!(colors::hex2rgb("#FFFFFF"), vec![ 255, 255, 255]);
			assert_eq!(colors::hex2rgb("#7F7F7F"), vec![ 127, 127, 127]);
			assert_eq!(colors::hex2rgb("#000000"), vec![ 0, 0, 0]);
		}

		#[test]
		fn test_hex2xrgb() {
			assert_eq!(colors::hex2xrgb("#FFFFFF"), "255/255/255/ff");
			assert_eq!(colors::hex2xrgb("#7F7F7F"), "127/127/127/ff");
			assert_eq!(colors::hex2xrgb("#000000"), "0/0/0/ff");
		}

		#[test]
		fn test_hex2rgbdisplay() {
			assert_eq!(colors::hex2rgbdisplay("#FFFFFF"), "255,255,255");
			assert_eq!(colors::hex2rgbdisplay("#7F7F7F"), "127,127,127");
			assert_eq!(colors::hex2rgbdisplay("#000000"), "0,0,0");
		}
	}
