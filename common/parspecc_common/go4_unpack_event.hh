#ifndef __GO4_PARSPECC_EVENT_H__
#define __GO4_PARSPECC_EVENT_H__

#include <cstdint>
#include <algorithm>
#include "go4_unpack_subevent.hh"

// DEFAULT values will match any event's metadata 
constexpr uint16_t DEFAULT_EV_TYPE      = 10;
constexpr uint16_t DEFAULT_EV_SUBTYPE   = 1;
constexpr uint16_t DEFAULT_EV_TRIG_TYPE = 1;
constexpr size_t MAX_SUBEV_COUNT = 100;
constexpr size_t MAX_EV_COUNT = 16;

class TGo4MbsEvent;
class Go4ParspeccBaseEvent {
	const uint16_t type      = DEFAULT_EV_TYPE;
	const uint16_t subtype   = DEFAULT_EV_SUBTYPE;
	const uint16_t trig_type = DEFAULT_EV_TRIG_TYPE;
	const uint16_t __dummy   = 0xe1e1; // to pad to 32-bit boundary
public:
	uint16_t subev_n;
	Go4ParspeccSubevent* subev[MAX_SUBEV_COUNT];
	
	uint32_t event_num;
	uint32_t l_dlen;
	bool is_valid;

	Go4ParspeccBaseEvent() = default;
	Go4ParspeccBaseEvent(uint16_t type, uint16_t subtype, uint16_t trig_type) :
		type(type),
		subtype(subtype),
		trig_type(trig_type),
		subev_n(0) {
			std::fill(subev, subev+MAX_SUBEV_COUNT, (Go4ParspeccSubevent*)&__invalid_subevent);
		}

	~Go4ParspeccBaseEvent() {} /* Event doesn't own the subev instances. They are owned by subevent classes themselves. */
	
	virtual void init() = 0;
	virtual void fill(TGo4MbsEvent* ev) = 0;
	virtual void clear() = 0;
	virtual bool check() = 0;

	virtual bool match_event(TGo4MbsEvent* ev) = 0;
};

/* Real events will specialize the class below,
 * just by adding the correct subev pointers to point to correct instances. */
template<uint16_t t_type, uint16_t t_subtype, uint16_t t_trig_type>
class Go4ParspeccEvent : public Go4ParspeccBaseEvent {
	Go4ParspeccEvent() : Go4ParspeccSubevent(t_type, t_subtype, t_trig_type) {}
	
	template<uint32_t n> 
	void __init() {
		(std::static_cast<ParspeccGetSubeventType<n-1>::type *>subev[n-1])->init();
		__init<n-1>();
	}
	template<> 
	void __init<0>() {}
	
	void init() { __init<MAX_SUBEV_COUNT>(); }

	/* ---------------------- */

	template<uint32_t n> 
	void __fill(TGo4MbsEvent* ev) {
		(std::static_cast<ParspeccGetSubeventType<n-1>::type *>subev[n-1])->fill(ev);
		__fill<n-1>(ev);
	}
	template<> 
	void __fill<0>(TGo4MbsEvent* ev) {}
	
	void fill(TGo4MbsEvent* ev) { __fill<MAX_SUBEV_COUNT>(TGo4MbsEvent* ev); }

	/* ---------------------- */
	template<uint32_t n> 
	void __clear() {
		(std::static_cast<ParspeccGetSubeventType<n-1>::type *>subev[n-1])->clear();
		__clear<n-1>();
	}
	template<> 
	void __clear<0>() {}

	void clear() { __clear<MAX_SUBEV_COUNT>(); }
	
	/* ---------------------- */

	template<uint32_t n> 
	bool __check() {
		return (std::static_cast<ParspeccGetSubeventType<n-1>::type *>subev[n-1])->check() && __check<n-1>();
	}
	template<>
	bool __check<MAX_SUBEV_COUNT>() {
		bool __is_valid = 1;
		__is_valid &= std::static_cast<ParspeccGetSubeventType<MAX_SUBEV_COUNT-1>::type *>subev[MAX_SUBEV_COUNT-1]->check();
		__is_valid &= __check<MAX_SUBEV_COUNT-1>();
	}
	template<> 
	bool __check<0>() { return true; }

	bool check() { is_valid = __check<MAX_SUBEV_COUNT>(); return is_valid; }

	/* ---------------------- */

	bool match_event(TGo4MbsEvent* ev) { 
		return ((ev->GetType()    == t_type)    &&
			(ev->GetSubtype() == t_subtype) &&
			(ev->GetTrigger() == t_trig_type));
	}
};

/* Invalid event. The `never` type. */
template<>
class Go4ParspeccEvent<UINT16_MAX, UINT16_MAX, UINT16_MAX> : Go4ParspeccBaseEvent {
	void init() {}
	void fill(TGo4MbsEvent* ev) { (void)ev; }
	void clear() {}
	bool check() { return true; }
	bool match_event(TGo4MbsEvent* ev) { return false; }
};

using InvalidEvent = Go4ParspeccEvent<UINT16_MAX, UINT16_MAX, UINT16_MAX>;
constexpr InvalidEvent __invalid_event{};

/* Real event with type/subtype/trig_type will specialize the 
 * ParspeccGetEventType<n>::type (n = 0,1,2, ... SUBEV_NUM) to return the proper
 * type, given as the name in the spec file that the parser produces. */
template<uint32_t a> 
struct ParspeccGetEventType { using type = InvalidEvent; }

#endif
