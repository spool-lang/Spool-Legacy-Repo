package silicon

class Parser(private val tokens: List<Token>) {

    private var current: Int = 0
    private var namespace: String? = null

    fun parse(): FileDB {
        val fileDB = FileDB()

        resolveNamespaceImports()

        while (!isAtEnd()) {
            val node = topLevelDeclaration()
            if (node != null) {
                when (node) {
                    is AstNode.TypeNode -> fileDB[node.name] = node
                    is AstNode.VariableNode -> fileDB[node.name] = node
                    is AstNode.FunctionNode -> {
                        val name = node.name
                        if (name != null) {
                            fileDB[name] = node
                        }
                    }
                }
            }
        }

        return fileDB
    }

    private fun resolveNamespaceImports() {
        if (match(TokenType.NAMESPACE)) {
            namespace = ""

            while (match(TokenType.ID)) {
                namespace = "$namespace${previous().lexeme}"
                if (match(TokenType.DOT)) namespace = "$namespace."
                else break
            }
        }
        else {
            throw Exception()
        }
    }

    private fun topLevelDeclaration(): AstNode? {
        try {
            if (match(TokenType.CLASS));
            // if (match(TokenType.VAR));
            // if (match(TokenType.CONST));
            // if (match(TokenType.FUNC));
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

    private fun check(type: TokenType): Boolean = !isAtEnd() && peek().type == type

    private fun advance(): Token {
        if (!isAtEnd()) current += 1
        return previous()
    }

    private fun isAtEnd() = peek().type == TokenType.EOF

    private fun peek() = tokens[current]

    private fun previous() = tokens[current - 1]
}