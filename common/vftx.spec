VME_GSI_VFTX2(id) {
        MEMBER(DATA16 time_fine[32]);
        MEMBER(DATA16 time_coarse[32]);
        MEMBER(DATA16 time_trigger);

        U32 custom_header {
                0..4   => id = MATCH(id);
                5..8   => internal..status;
                9..17  => count;
                24..31 => 0xab;
        };
        
        U32 event_header {
                0..7   => 0xaa;
                29..30 => 0b01;
                31     => 0b1;
                ENCODE(11..23 => time_trigger);
        };
		
		local! u32 channel;
		dyn! U32 event {
			25..29 => @channel;

			ENCODE(0..10  => time_fine[channel]);
			ENCODE(11..23 => time_coarse[channel]);
        }
};

VME_GSI_VFTX2_MVLC(id) {
        MEMBER(DATA16 time_fine[32]);
        MEMBER(DATA16 time_coarse[32]);
        MEMBER(DATA16 time_trigger);

        U32 custom_header {
                0..4   => id = MATCH(id);
                5..8   => 0;
                9..17  => 0;
                24..31 => 0xab;
        };

        U32 trigger_window_register {
                0..10  => second..boundary;
                15	   => future..bit;
                16..27 => first..boundary;
                28..31 => 0;
        };

        U32 status_register;

        U32 mvlc_stack_header {
                0..15  => word_count; 
                16..31 => MATCH(0xf500); /* MVLC tag */
        };

        U32 event_header {
                0..7   => 0xaa;
                29..30 => 0b01;
                31     => 0b1;
                ENCODE(11..23 => time_trigger);
        };

		local u32 channel;
		dyn! U32 event {
			0..10  => time_fine;
			11..23 => time_coarse;
			25..29 => channel;

			ENCODE(0..10  => time_fine[channel]);
			ENCODE(11..23 => time_coarse[channel]);
        }
};

