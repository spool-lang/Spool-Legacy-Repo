package silicon

data class Token(val type: TokenType, val line: Long, val column: Long, val lexeme: String?, val literal: Any?)

enum class TokenType {
    //Operators
    PLUS,
    MINUS,
    MULTIPLY,
    DIVIDE,
    POW
}