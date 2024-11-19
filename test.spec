TIMESTAMP_WHITERABBIT() {
	MEMBER(DATA16 subsystem_id);
	MEMBER(DATA64 t1);
	MEMBER(DATA32 t2);

	UINT32 header {
		0 ..11  => id = MATCH(id);
		12..15 => 0;
		16..16 => error_bit;
		17..31 => 0;
	   ENCODE(subsystem_id, (value=id));
	}
	UINT64 d1 {
		0..15  => t1;
		16..31 => 0x04e1;
		32..47 => t2;
		48..63 => 0x03e1; 
	   ENCODE(t1, (value = raw!((t1<<16) + t2) ) );
	};
	UINT64 d4 {
		0..15  => t1;
		16..31 => 0x06e1;
		32..47 => t2;
		48..63 => 0x05e1;
	   ENCODE(t2, (value= raw!((t1<<16) + t2 )));
	};
}

V1190_TDC() {
	MEMBER(DATA32 v1190_tdc_value[5000] );
	MEMBER(DATA32 v1190_tdc_channel[5000] );
	MEMBER(DATA32 v1190_tdc_value_calib[5000] );
	MEMBER(DATA32 v1190_tdc_value_sign[5000] );

	UINT32 header {
		0..4 => meh;
		5..26 => event_number;
		27..29 => 0;
		30 => 1;
		31 => 0;
	}
	
	UINT32 kek;

	for(0 <= i < 4) { 
		UINT32 tdc_header {
			0..26 => meh;
			27 => 1;
			28 => 0;
		}

		dyn! {
			UINT32 t;
			UINT32 ch_data {
				0..18 => v1190_time;
				19..25 => v1190_channel;
				26 => leading_or_trailing;
				27..31 => 0;
				ENCODE(v1190_tdc_value APPEND_LIST, (value = v1190_time));
				ENCODE(v1190_tdc_channel APPEND_LIST, (value = v1190_channel));
			}
		};

		UINT32 tdc_trailer {
			0..26 => meh;
			27..28 => 0b11;
			29..31 => 0;
		};	
	}

	UINT32 the_trailer {
		0..23 => meh;
		24..26 => err;
		27..31 => 0b10000; 
		ENCODE(err, (value=err));
	};
}

/*
COMPO() {}

SUBEVENT(hehehehe) {}

EVENT[trig_type = 4] {}

EVENT {}

	*/

