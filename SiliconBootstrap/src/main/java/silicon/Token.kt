package silicon

data class Token(val type: TokenType, val line: Long, val column: Long, val lexeme: String?, val literal: Any?)

enum class TokenType {
    //Operators
    PLUS,
    MINUS,
    MULTIPLY,
    DIVIDE,
    POW,
    DOT,
    //Other
    COMMA,
    COLIN,
    R_ARROW,
    // Logic Operators
    LESS,
    GREATER,
    LESS_EQUAL,
    GREATER_EQUAL,
    NOT,
    NOT_EQUAL,
    EQUAL,
    //Assignment
    ASSIGN,
    PLUS_ASSIGN,
    MINUS_ASSIGN,
    MULTIPLY_ASSIGN,
    DIVIDE_ASSIGN,
    POW_ASSIGN,
    //Brackets
    SQUARE_LEFT,
    SQUARE_RIGHT,
    ANGLE_LEFT,
    ANGLE_RIGHT,
    BRACE_LEFT,
    BRACE_RIGHT,
    PAREN_LEFT,
    PAREN_RIGHT,
    //Literals
    ID,
    NUMBER,
    STRING,
    //Keywords
    FUNC,
    CLASS,
    VAR,
    CONST,
    NAMESPACE,
    USE,
    NATIVE,
    IF,
    ELSE,
    RETURN,

    EOF
}