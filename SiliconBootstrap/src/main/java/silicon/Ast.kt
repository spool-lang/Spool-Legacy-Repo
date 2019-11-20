package silicon

interface AstVisitor {
    fun visitFile(file: AstNode.FileNode)

    fun visitClass(clazz: AstNode.ClassNode)

    fun visitVariable(variable: AstNode.VariableNode)

    fun visitFunction(function: AstNode.FunctionNode)

    fun visitBlock(block: AstNode.BlockNode)
}

sealed class AstNode {

    abstract fun visit(visitor: AstVisitor)

    class FileNode(val statements: Map<String, AstNode>, val namespace: String, val imports: Map<String, String>): AstNode() {
        override fun visit(visitor: AstVisitor) {
            visitor.visitFile(this)
        }
    }

    class ClassNode(val name: String): AstNode() {
        override fun visit(visitor: AstVisitor) {
            visitor.visitClass(this)
        }
    }

    class VariableNode(val name: String, val type: String, val const: Boolean): AstNode() {
        override fun visit(visitor: AstVisitor) {
            visitor.visitVariable(this)
        }
    }

    class FunctionNode(val name: String?, val block: BlockNode, val params: List<Pair<String, String>>): AstNode() {
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