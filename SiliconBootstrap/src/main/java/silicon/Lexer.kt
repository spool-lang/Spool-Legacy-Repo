package silicon

import java.lang.Exception

class Lexer(private val source: String) {
    var tokens: ArrayList<Token> = arrayListOf()
    var row: Long = 0;
    var column: Long = 0;

    @Throws(Exception::class)
    fun lex(): List<Token> {

        for (char in source.toCharArray()) {
            when (char) {
                '+' -> symbol(TokenType.PLUS, '+')
                '-' -> symbol(TokenType.MINUS, '-')
                '*' -> symbol(TokenType.MULTIPLY, '*')
                '/' -> symbol(TokenType.DIVIDE, '/')
                '^' -> symbol(TokenType.POW, '^')
            }
        }

        val lexedTokens = tokens
        tokens = arrayListOf()
        return lexedTokens
    }


    private fun  symbol(type: TokenType, lexeme: Char) = symbol(type, "$lexeme")

    private fun symbol(type: TokenType, lexeme: String) = tokens.add(Token(type, row, column, lexeme, null))
}