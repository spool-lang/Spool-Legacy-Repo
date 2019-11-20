package silicon

class Parser(val tokens: List<Token>) {

    private var current: Int = 0

    fun parse(): AstNode.FileNode {
        val statements: MutableMap<String, AstNode> = mutableMapOf()

        while (!isAtEnd()) {
            val node = declaration()
            if (node != null) {
                when (node) {
                    is AstNode.ClassNode -> statements[node.name] = node
                    is AstNode.VariableNode -> statements[node.name] = node
                    is AstNode.FunctionNode -> {
                        val name = node.name
                        if (name != null) {
                            statements[name] = node
                        }
                    }
                }
            }
        }

        return AstNode.FileNode(statements, "", mapOf())
    }

    private fun declaration(): AstNode? {
        try {
            if (match(TokenType.CLASS));
            if (match(TokenType.VAR));
            if (match(TokenType.CONST));
            if (match(TokenType.FUNC));
        } catch (e: Exception) {
            e.printStackTrace()
        }
        return null
    }

    private fun match(vararg types: TokenType): Boolean {
        for (type in types) {
            if (check(type)) {
                advance()
                return true
            }
        }
        return false
    }

    private fun check(type: TokenType): Boolean = isAtEnd() || peek().type == type

    private fun advance(): Token {
        if (!isAtEnd()) current++
        return previous()
    }

    private fun isAtEnd() = peek().type != TokenType.EOF

    private fun peek() = tokens[current]

    private fun previous() = tokens[current - 1]
}