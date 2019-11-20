package silicon

import kotlin.Exception
import kotlin.system.exitProcess

val test = """
    namespace silicon.bootstrap
"""


fun main(args: Array<String>) {

    if (args.size != 1) {
        println("Please specify the project file.")
        exitProcess(-1)
    }

    val lexer = Lexer(test)
    val tokens: List<Token>

    try {
        tokens = lexer.lex()
    } catch (e: Exception) {
        e.printStackTrace()
        exitProcess(-2)
    }

    tokens.forEach { println(it) }

    val parser = Parser(tokens)

    try {
        parser.parse()
    } catch (e: Exception) {
        e.printStackTrace()
        exitProcess(-2)
    }
}