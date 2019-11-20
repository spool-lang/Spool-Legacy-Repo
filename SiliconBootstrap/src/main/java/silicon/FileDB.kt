package silicon

class FileDB {
    private val map: MutableMap<String, AstNode> = mutableMapOf()

    operator fun set(canonicalName: String, node: AstNode) {
        when (node) {
            is AstNode.TypeNode, is AstNode.FunctionNode, is AstNode.VariableNode -> map[canonicalName] = node
        }
    }

    fun getTypeNode(canonicalName: String): AstNode.TypeNode {
        val node = map[canonicalName]
        if (node is AstNode.TypeNode) return node
        throw Exception()
    }
}