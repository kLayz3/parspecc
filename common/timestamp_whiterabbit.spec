#ifndef TIMESTAMP_WHITERABBIT_SPEC
#define TIMESTAMP_WHITERABBIT_SPEC

TIMESTAMP_WHITERABBIT(id) {
	MEMBER(DATA16 subsystem_id);
	MEMBER(DATA16 tll);
	MEMBER(DATA16 tlh);
	MEMBER(DATA16 thl);
	MEMBER(DATA16 thh);

	local! u8 err_bit; 
	u32 header {
		0..11  => id; /* WR ID */
		12..15 => 0;
		16     => err_bit;
		17..31 => 0;
		ENCODE(0..11 => subsystem_id);
		assert!(err_bit == 0);
	};

	u32 d1 {
		16..31 => 0x03e1;
		ENCODE(0..15 => tll);
	};
	u32 d2 {
		16..31 => 0x04e1;
		ENCODE(0..15 => tlh);
	};
	u32 d3 {
		16..31 => 0x05e1;
		ENCODE(0..15 => thl);
	};
	u32 d4 {
		16..31 => 0x06e1;
		ENCODE(0..15 => thh);
	};
};

TIMESTAMP_SYNC_CHECK() { 
        MEMBER(DATA16 ref_recv); 
        MEMBER(DATA16 value); 
 
        UINT32 u32 { 
                20..31 => 0xf1a; 
                ENCODE(16..19 => ref_recv); 
                ENCODE(0..15 => value); 
        };
};


#endif
