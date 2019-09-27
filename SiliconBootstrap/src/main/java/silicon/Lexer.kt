package silicon

import java.lang.Exception

class Lexer(private val source: String) {
    var tokens: ArrayList<Token> = arrayListOf()
    var index: Int = 0
    var row: Long = 0
    var column: Long = 0

    @Throws(Exception::class)
    fun lex(): List<Token> {

        var char = ' '
        while (char != '\u0000') {

            char = next()
            when (char) {
                // Operators
                '+' -> pattern(TokenType.PLUS_ASSIGN, "+=") || symbol(TokenType.PLUS, '+')
                '-' ->  pattern(TokenType.MINUS_ASSIGN, "-=") || symbol(TokenType.MINUS, '-')
                '*' -> pattern(TokenType.MULTIPLY_ASSIGN, "*=") || symbol(TokenType.MULTIPLY, '*')
                '/' -> pattern(TokenType.DIVIDE_ASSIGN, "/=") || symbol(TokenType.DIVIDE, '/')
                '^' -> pattern(TokenType.POW_ASSIGN, "^=") || symbol(TokenType.POW, '^')
                '=' -> pattern(TokenType.EQUAL, "==") || symbol(TokenType.ASSIGN, '=')
                '.' -> symbol(TokenType.DOT, '.')

                // Logic Operators
                '<' -> pattern(TokenType.LESS_EQUAL, "<=") || symbol(TokenType.LESS, '<')
                '>' -> pattern(TokenType.GREATER_EQUAL, ">=") || symbol(TokenType.GREATER, ">")
                '!' -> pattern(TokenType.NOT_EQUAL, "!=") || symbol(TokenType.NOT, '!')

                // Brackets & Braces
                '[' -> symbol(TokenType.SQUARE_LEFT, '[')
                ']' -> symbol(TokenType.SQUARE_RIGHT, ']')
                '{' -> symbol(TokenType.BRACE_LEFT, '{')
                '}' -> symbol(TokenType.BRACE_RIGHT, '}')
                '(' -> symbol(TokenType.PAREN_LEFT, '(')
                ')' -> symbol(TokenType.PAREN_RIGHT, ')')
            }
        }

        val lexedTokens = tokens
        tokens = arrayListOf()
        return lexedTokens
    }

    private fun pattern(type: TokenType, pattern: String): Boolean {
        val length = pattern.length
        for (i in 1 until length) {
            if (peek(i - 1) != pattern[i]) {
                return false
            }
        }
        index += length - 1
        symbol(type, pattern)
        return true
    }

    private fun  symbol(type: TokenType, lexeme: Char): Boolean = symbol(type, "$lexeme")

    private fun symbol(type: TokenType, lexeme: String): Boolean {
        tokens.add(Token(type, row, column, lexeme, null))
        return true
    }

    private fun next(): Char = if (index < source.length) { source[index++] } else { '\u0000'}

    private fun peek(count: Int): Char = if (index + count < source.length) { source[index + count] } else { '\u0000' }
}