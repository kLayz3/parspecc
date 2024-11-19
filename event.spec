#include "common/timestamp_whiterabbit.spec"

BASIC(chann_num, bit_id) {
	MEMBER(DATA8 y_id);
	MEMBER(DATA32 z_id);
	MEMBER(DATA16 hit_id[chann_num]);

	dyn![max = 4] U32 x = MATCH(0xfefefefe); 
	U64 y {
		0..10 => T;
		13..25 => 0xfea;
		11 => 0x1;
		12 => 0x0;
		ENCODE(26..31 => y_id);
	};

	for(0 <= i < 4) {
		U32 word;
	}

	dyn! U32 z {
		0..bit_id => 0xfeaa;
		ENCODE(bit_id .. 31 => z_id);
	};
};

SOMETHING_ELSE() {
	MEMBER(DATA16 _u_enc[200]);
	MEMBER(DATA8 _uu_enc[200]);
	dyn! U32 _x = MATCH(0xfefefefe);
	dyn! b = BASIC(chann_num = 12, bit_id = 5);
	dyn! U32 _u {
		0..5 => 0xf;
		6 => 0;
		ENCODE(7..15 => _u_enc); 
	};
};

SUBEVENT(frs_user) {
	ts = TIMESTAMP_WHITERABBIT(id = 0x1);
	a = BASIC(chann_num = 16, bit_id = 2);
	b = SOMETHING_ELSE();
};

EVENT[event](type=10, subtype=1, trig_type=1) {
	frs_user = frs_user(type=10, subtype=1, procid=20);
};


