package silicon

interface AstVisitor {
    fun visitFile(file: AstNode.FileNode)

    fun visitClass(clazz: AstNode.TypeNode)

    fun visitVariable(variable: AstNode.VariableNode)

    fun visitFunction(function: AstNode.FunctionNode)

    fun visitBlock(block: AstNode.BlockNode)
}

data class Type(val canonicalName: String, var node: AstNode.TypeNode? = null) {
    fun resolveType(node: AstNode.TypeNode) {
        if (this.node != null) this.node = node
        else throw Exception("Attempted to resolve node type has already been resolved!")
    }
}

sealed class AstNode {

    abstract fun visit(visitor: AstVisitor)

    class FileNode(val statements: Map<String, AstNode>, val namespace: String, val imports: Map<String, String>): AstNode() {
        override fun visit(visitor: AstVisitor) {
            visitor.visitFile(this)
        }
    }

    class TypeNode(val name: String, val superType: Type): AstNode() {
        override fun visit(visitor: AstVisitor) {
            visitor.visitClass(this)
        }
    }

    class VariableNode(val name: String, val type: Type, val const: Boolean): AstNode() {
        override fun visit(visitor: AstVisitor) {
            visitor.visitVariable(this)
        }
    }

    class FunctionNode(val name: String?, val block: BlockNode, val params: List<Pair<String, Type>>): AstNode() {
        override fun visit(visitor: AstVisitor) {
            visitor.visitFunction(this)
        }
    }

    class BlockNode(val statements: List<AstNode>): AstNode() {
        override fun visit(visitor: AstVisitor) {
            visitor.visitBlock(this)
        }
    }
}