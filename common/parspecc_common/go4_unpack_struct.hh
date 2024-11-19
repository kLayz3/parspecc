#ifndef __GO4_UNPACK_STRUCT__
#define __GO4_UNPACK_STRUCT__

#include <exception>
#include <iostream>
#include <cstdio>
#include <cstdint>

#define MAX_DYN_DEFAULT 128

/* This define creates the alias for uint(N)_t as DATA##N, also wraps the
 * DATA##N into a monadic type used later during the composition. */
#define __IMPL_D__(N) \
	using DATA##N = uint##N##_t; \
	template<typename _T = void> \
	struct __D##N { \
		using T = DATA##N; \
		using S = __D##N; \
		T x; \
		S() = default; \
		S(T x) : x(x) {} \
		static constexpr size_t min_size() { return sizeof(T); } \
		void init() {} \
		inline bool check_event() { return true; } \
		inline void clear() noexcept { x = std::static_cast<T>(0); } \
		void fill(uint8_t* event_handle, size_t& bytes_available, size_t& bytes_read) { \
			if(min_size() > bytes_available) throw std::runtime_error("Subevent boundary reached. Cannot read anymore."); \
			x = *(T*)event_handle; \
			bytes_read = sizeof(T); \
			bytes_available -= bytes_read; \
		} \
	}; \
	template<typename _T = void> \
	using __u##N      = __D##N<_T>; \
	template<typename _T = void> \
	using __U##N      = __D##N<_T>; \
	template<typename _T = void> \
	using __I##N      = __D##N<_T>; \
	template<typename _T = void> \
	using __uint##N_t = __D##N<_T>; \
	template<typename _T = void> \
	using __int##N_t = __D##N<_T>; \
	template<typename _T = void> \
	using __UInt##N_t = __D##N<_T>; \
	template<typename _T = void> \
	using __Int##N_t = __D##N<_T>;

__IMPL_D__ (64)
__IMPL_D__ (32)
__IMPL_D__ (16)
__IMPL_D__ ( 8)

// Container for ENCODE ptrs, doesn`t own the pointer
template<typename T>
class Go4UnpackPtr {
	T* p;
	uint8_t l;
	uint8_t h;
	T mask;
public:
	Go4UnpackPtr() : p((T*)nullptr), l(0), h(0), mask(0) {}
	void assign(void* p, uint8_t l, uint8_t h) {
		this->p = (T*)p;
		this->l = l;
		this->h = h;
		this->mask = ((T)((1ull << (h-l+1)) -1));
		if(h-l+1 > 63) this->mask = (T)0xffffffffffffffff;
	}
	T operator*() const noexcept {
		return mask & (*p >> l);
	}
	T get_data() const noexcept {
		return this->operator*();
	}
};

#endif
