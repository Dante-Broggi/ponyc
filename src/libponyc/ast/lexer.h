#ifndef LEXER_H
#define LEXER_H

#include "../../common/platform.h"

#include "source.h"
#include "token.h"

PONY_EXTERN_C_BEGIN

typedef struct lexer_t lexer_t;
typedef struct errors_t errors_t;

/** Create a new lexer to handle the given source.
 * The created lexer must be freed later with lexer_close().
 * Never returns NULL.
 */
lexer_t* lexer_open(source_t* source, errors_t* errors,
  bool allow_test_symbols);

/** Free a previously opened lexer.
 * The lexer argument may be NULL.
 */
void lexer_close(lexer_t* lexer);

/** Get the next token from the given lexer.
 * The returned token must be freed later with token_free().
 */
token_t* lexer_next(lexer_t* lexer);

/** Report the lexical text for the given token ID.
 * The returned string should not be freed by the caller and is valid
 * indefinitely.
 * Only reports IDs with fixed text, ie operators and keywords.
 * Returns NULL for non-fixed tokens.
 */
const char* lexer_print(token_id id);

PONY_EXTERN_C_END

#endif
