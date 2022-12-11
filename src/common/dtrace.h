#ifndef DTRACE_H
#define DTRACE_H

#if PRESERVE_FUNCTION_MACROS

bool macro__DTRACE_ENABLED(char const*);
bool macro__DTRACE(char const*, int, ...);

#define DTRACE_ENABLED(name)                         \
  macro__DTRACE_ENABLED(#name)

#if defined(USE_DYNAMIC_TRACE)
#include "dtrace_probes.h"

#define DTRACE0(name)                                \
  macro__DTRACE(#name, 0)
#define DTRACE1(name, a0)                            \
  macro__DTRACE(#name, 1, a0)
#define DTRACE2(name, a0, a1)                        \
  macro__DTRACE(#name, 2, (a0), (a1))
#define DTRACE3(name, a0, a1, a2)                    \
  macro__DTRACE(#name, 3, (a0), (a1), (a2))
#define DTRACE4(name, a0, a1, a2, a3)                \
  macro__DTRACE(#name, 4, (a0), (a1), (a2), (a3))
#define DTRACE5(name, a0, a1, a2, a3, a4)            \
  macro__DTRACE(#name, 5, (a0), (a1), (a2), (a3), (a4))

#else

#define DTRACE0(name)                                \
  macro__DTRACE(#name, 0)
#define DTRACE1(name, a0)                            \
  macro__DTRACE(#name, 1, #a0)
#define DTRACE2(name, a0, a1)                        \
  macro__DTRACE(#name, 2, (#a0), (#a1))
#define DTRACE3(name, a0, a1, a2)                    \
  macro__DTRACE(#name, 3, (#a0), (#a1), (#a2))
#define DTRACE4(name, a0, a1, a2, a3)                \
  macro__DTRACE(#name, 4, (#a0), (#a1), (#a2), (#a3))
#define DTRACE5(name, a0, a1, a2, a3, a4)            \
  macro__DTRACE(#name, 5, (#a0), (#a1), (#a2), (#a3), (#a4))

#endif

#elif defined(USE_DYNAMIC_TRACE)

#include "dtrace_probes.h"

#define DTRACE_ENABLED(name)                         \
  PONY_##name##_ENABLED()
#define DTRACE0(name)                                \
  PONY_##name()
#define DTRACE1(name, a0)                            \
  PONY_##name(a0)
#define DTRACE2(name, a0, a1)                        \
  PONY_##name((a0), (a1))
#define DTRACE3(name, a0, a1, a2)                    \
  PONY_##name((a0), (a1), (a2))
#define DTRACE4(name, a0, a1, a2, a3)                \
  PONY_##name((a0), (a1), (a2), (a3))
#define DTRACE5(name, a0, a1, a2, a3, a4)            \
  PONY_##name((a0), (a1), (a2), (a3), (a4))

#else

#define DTRACE_ENABLED(name)                         0
#define DTRACE0(name)                                do {} while (0)
#define DTRACE1(name, a0)                            do {} while (0)
#define DTRACE2(name, a0, a1)                        do {} while (0)
#define DTRACE3(name, a0, a1, a2)                    do {} while (0)
#define DTRACE4(name, a0, a1, a2, a3)                do {} while (0)
#define DTRACE5(name, a0, a1, a2, a3, a4)            do {} while (0)

#endif

#endif
