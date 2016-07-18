use std::ptr;
use avcodec::AVPacket;

impl Default for AVPacket {
	#[cfg(feature = "ff_api_destruct_packet")]
	fn default() -> AVPacket {
		AVPacket {
			buf: ptr::null_mut(),
			pts: 0,
			dts: 0,
			data: ptr::null_mut(),
			size: 0,
			stream_index: 0,
			flags: 0,
			side_data: ptr::null_mut(),
			side_data_elems: 0,
			duration: 0,
			destruct: None,
			private: ptr::null_mut(),
			pos: 0,
			convergence_duration: 0,
		}
	}
	
	#[cfg(not(feature = "ff_api_destruct_packet"))]
	fn default() -> AVPacket {
		AVPacket {
			buf: ptr::null_mut(),
			pts: 0,
			dts: 0,
			data: ptr::null_mut(),
			size: 0,
			stream_index: 0,
			flags: 0,
			side_data: ptr::null_mut(),
			side_data_elems: 0,
			duration: 0,
			//destruct:
			//private:
			pos: 0,
			convergence_duration: 0,
		}
	}
}