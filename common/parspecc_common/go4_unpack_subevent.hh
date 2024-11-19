#ifndef __GO4_PARSPECC_SUBEVENT_H__
#define __GO4_PARSPECC_SUBEVENT_H__

#include <cstdint>

// DEFAULT values will match any subevent's metadata 
constexpr uint16_t DEFAULT_SUBEV_TYPE     = 10;
constexpr uint16_t DEFAULT_SUBEV_SUBTYPE  = 1;
constexpr uint16_t DEFAULT_SUBEV_PROCID   = 0;
constexpr uint16_t DEFAULT_SUBEV_SUBCRATE = 0;
constexpr uint16_t DEFAULT_SUBEV_CONTROL  = 0;

class TGo4MbsSubEvent;
class Go4ParspeccSubevent {
	const uint16_t type     = DEFAULT_SUBEV_TYPE;
	const uint16_t subtype  = DEFAULT_SUBEV_SUBTYPE;
	const uint16_t procid   = DEFAULT_SUBEV_PROCID;
	const uint16_t subcrate = DEFAULT_SUBEV_SUBCRATE;
	const uint16_t control  = DEFAULT_SUBEV_CONTROL;
	const uint16_t __dummy  = 0xdead; // to pad to 64-bit boundary 
public:
	uint32_t l_dlen;
	Go4ParspeccSubevent(uint16_t type, uint16_t subtype, uint16_t procid, uint16_t subcrate, uint16_t control) : 
		type(type),
		subtype(subtype),
		procid(procid),
		subcrate(subcrate),
		control(control) {}

	Go4ParspeccSubevent() = default;

	virtual void init() = 0;
	virtual void fill(TGo4MbsSubEvent* subevt) = 0;
	virtual void clear() = 0;
	virtual bool check() = 0;
	
	virtual bool match_subevent(TGo4MbsSubEvent* subevt) {
		return ((subevt->GetType()     == type)     &&
				(subevt->GetSubtype()  == subtype)  &&
				(subevt->GetProcId()   == procid)   &&
				(subevt->GetSubcrate() == subcrate) &&
				(subevt->GetControl()  == control));
	}
};

/* The `never` type. */
class InvalidSubevent : public Go4ParspeccSubevent {
	Go4ParspeccSubevent() : Go4ParspeccSubevent(UINT16_MAX, UINT16_MAX, UINT16_MAX, UINT16_MAX, UINT16_MAX) {}
	void init() {}
	void fill(TGo4MbsSubEvent* subevt) {}
	void clear() {}
	bool check() { return true; }

	bool match_subevent(TGo4MbsSubEvent* subevt) { return false; }
}
using InvalidSubevent = Go4ParspeccSubevent<UINT16_MAX, UINT16_MAX, UINT16_MAX, UINT16_MAX, UINT16_MAX>;
constexpr InvalidSubevent __invalid_subevent{} ;

/* Real subevent with type/subtype/subcrate/control/procid will specialize the 
 * ParspeccGetSubeventType<n>::type (n = 0,1,2, ... SUBEV_NUM) to return the proper
 * type, given as the name in the spec file that the parser produces. */
template<uint32_t a> 
struct ParspeccGetSubeventType { using type = InvalidSubevent; }

#endif
